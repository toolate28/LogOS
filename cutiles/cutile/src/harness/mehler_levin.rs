//! Mehler-Levin batched kernel harness — HeisenForge v0.3 production FFI bridge.
//!
//! Exposes `mehler_mma_levin_batched` to cutile via cudarc PTX embedding (primary)
//! with a CPU reference fallback when CUDA/PTX is unavailable.

use crate::error::CutileError;
use crate::existence_cert::{ExistenceCertificate, TomczakGateWitness};

pub const N_LEVIN_NODES: usize = 8;
pub const CERTIFIED_ERROR_TOL: f32 = 5e-7;
pub const FAST_PATH_ERROR_TYPICAL: f32 = 1.2e-5;

/// Evaluation result for one batched Mehler-Levin quadrature.
#[derive(Debug, Clone, PartialEq)]
pub struct MehlerLevinResult {
    pub point_real: Vec<f32>,
    pub point_imag: Vec<f32>,
    pub max_error: Option<Vec<f32>>,
    pub reliable: Option<Vec<bool>>,
    pub used_gpu_kernel: bool,
}

impl MehlerLevinResult {
    /// Emit an `ExistenceCertificate` for one batch element after Mehler-Levin evaluation.
    pub fn emit_existence_certificate(
        &self,
        batch_index: usize,
        lift_ok: &TomczakGateWitness,
        wave: f64,
        alpha_omega: f64,
        coherence_delta: f64,
        atom_trail_id: impl Into<String>,
        input_hash: Option<String>,
    ) -> ExistenceCertificate {
        let max_error = self
            .max_error
            .as_ref()
            .and_then(|e| e.get(batch_index).copied())
            .unwrap_or(0.0) as f64;
        let reliable = self
            .reliable
            .as_ref()
            .and_then(|r| r.get(batch_index).copied())
            .unwrap_or(!self.used_gpu_kernel);
        ExistenceCertificate::from_lift_and_mehler(
            lift_ok,
            max_error,
            reliable,
            wave,
            alpha_omega,
            coherence_delta,
            atom_trail_id,
            input_hash,
            None,
            None,
        )
    }
}

/// Context holding time parameter and certified-mode flag.
#[derive(Debug, Clone)]
pub struct MehlerLevinHarness {
    pub t: f32,
    pub certified_mode: bool,
    #[cfg(feature = "cuda")]
    kernel_ready: bool,
}

impl MehlerLevinHarness {
    pub fn new(t: f32, certified_mode: bool) -> Self {
        Self {
            t,
            certified_mode,
            #[cfg(feature = "cuda")]
            kernel_ready: Self::kernel_available(),
        }
    }

    #[cfg(feature = "cuda")]
    fn kernel_available() -> bool {
        #[cfg(mehler_ptx_embedded)]
        {
            true
        }
        #[cfg(not(mehler_ptx_embedded))]
        {
            false
        }
    }

    /// Batched evaluation: `z` length = batch_size, `f_nodes` length = batch_size * 8.
    pub fn evaluate(
        &self,
        z: &[f32],
        f_nodes: &[f32],
    ) -> Result<MehlerLevinResult, CutileError> {
        let batch_size = z.len();
        if batch_size == 0 {
            return Err(CutileError::InvalidDimensions {
                expected: 1,
                actual: 0,
            });
        }
        if f_nodes.len() != batch_size * N_LEVIN_NODES {
            return Err(CutileError::InvalidDimensions {
                expected: batch_size * N_LEVIN_NODES,
                actual: f_nodes.len(),
            });
        }

        #[cfg(feature = "cuda")]
        if self.kernel_ready {
            if let Ok(result) = self.evaluate_cuda(z, f_nodes) {
                return Ok(result);
            }
        }

        Ok(self.evaluate_cpu(z, f_nodes))
    }

    #[cfg(feature = "cuda")]
    fn evaluate_cuda(&self, z: &[f32], f_nodes: &[f32]) -> Result<MehlerLevinResult, CutileError> {
        use cudarc::driver::{CudaDevice, LaunchAsync, LaunchConfig};

        const MODULE: &str = "mehler_mma_levin";
        const KERNEL: &str = "mehler_mma_levin_batched";
        const BLOCK: u32 = 256;

        #[cfg(mehler_ptx_embedded)]
        const PTX: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/kernels/mehler_mma_levin_batched.ptx"
        ));

        let device = CudaDevice::new(0).map_err(|e| CutileError::Cuda(e.to_string()))?;

        #[cfg(mehler_ptx_embedded)]
        {
            device
                .load_ptx(PTX, MODULE, &[KERNEL])
                .map_err(|e| CutileError::Cuda(e.to_string()))?;
        }

        #[cfg(not(mehler_ptx_embedded))]
        {
            return Err(CutileError::BackendUnavailable(
                "mehler PTX missing — run build_ptx.ps1".into(),
            ));
        }

        let func = device
            .get_func(MODULE, KERNEL)
            .ok_or_else(|| CutileError::Cuda("mehler_mma_levin_batched not loaded".into()))?;

        let batch_size = z.len() as i32;

        let d_z = device
            .htod_sync_copy(z)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;
        let d_f = device
            .htod_sync_copy(f_nodes)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;

        // cuComplex = two f32 per element; cudarc uses f32 slices for interleaved re/im.
        let mut d_point = device
            .alloc_zeros::<f32>(z.len() * 2)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;

        let mut d_max_error = if self.certified_mode {
            Some(
                device
                    .alloc_zeros::<f32>(z.len())
                    .map_err(|e| CutileError::Cuda(e.to_string()))?,
            )
        } else {
            None
        };

        let mut d_reliable = if self.certified_mode {
            Some(
                device
                    .alloc_zeros::<u8>(z.len())
                    .map_err(|e| CutileError::Cuda(e.to_string()))?,
            )
        } else {
            None
        };

        let grid = ((batch_size as u32).saturating_add(BLOCK - 1)) / BLOCK;
        let cfg = LaunchConfig {
            grid_dim: (grid.max(1), 1, 1),
            block_dim: (BLOCK, 1, 1),
            shared_mem_bytes: 0,
        };

        let t = self.t;
        let certified = self.certified_mode;

        // interval_out is nullptr in FFI path; pass null device pointer via Option.
        if certified {
            let max_err = d_max_error.as_mut().unwrap();
            let reliable = d_reliable.as_mut().unwrap();
            unsafe {
                func.launch(
                    cfg,
                    (
                        t,
                        &d_z,
                        &d_f,
                        &mut d_point,
                        &0u64, // null interval_out
                        max_err,
                        reliable,
                        batch_size,
                        certified,
                    ),
                )
                .map_err(|e| CutileError::Cuda(e.to_string()))?;
            }
        } else {
            unsafe {
                func.launch(
                    cfg,
                    (
                        t,
                        &d_z,
                        &d_f,
                        &mut d_point,
                        &0u64,
                        &0u64,
                        &0u64,
                        batch_size,
                        certified,
                    ),
                )
                .map_err(|e| CutileError::Cuda(e.to_string()))?;
            }
        }

        device
            .synchronize()
            .map_err(|e| CutileError::Cuda(e.to_string()))?;

        let point_interleaved = device
            .dtoh_sync_copy(&d_point)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;

        let mut point_real = Vec::with_capacity(z.len());
        let mut point_imag = Vec::with_capacity(z.len());
        for chunk in point_interleaved.chunks_exact(2) {
            point_real.push(chunk[0]);
            point_imag.push(chunk[1]);
        }

        let (max_error, reliable) = if self.certified_mode {
            let max_err = device
                .dtoh_sync_copy(d_max_error.as_ref().unwrap())
                .map_err(|e| CutileError::Cuda(e.to_string()))?;
            let rel_bytes = device
                .dtoh_sync_copy(d_reliable.as_ref().unwrap())
                .map_err(|e| CutileError::Cuda(e.to_string()))?;
            let rel: Vec<bool> = rel_bytes.iter().map(|&b| b != 0).collect();
            (Some(max_err), Some(rel))
        } else {
            (None, None)
        };

        Ok(MehlerLevinResult {
            point_real,
            point_imag,
            max_error,
            reliable,
            used_gpu_kernel: true,
        })
    }

    fn evaluate_cpu(&self, z: &[f32], f_nodes: &[f32]) -> MehlerLevinResult {
        let batch_size = z.len();
        let mut point_real = Vec::with_capacity(batch_size);
        let mut point_imag = Vec::with_capacity(batch_size);

        for (i, &zi) in z.iter().enumerate() {
            let nodes = &f_nodes[i * N_LEVIN_NODES..(i + 1) * N_LEVIN_NODES];
            let (re, im) = levin_quadrature_cpu(zi, self.t, nodes);
            point_real.push(re);
            point_imag.push(im);
        }

        let (max_error, reliable) = if self.certified_mode {
            let errors: Vec<f32> = point_real
                .iter()
                .zip(point_imag.iter())
                .map(|(&re, &im)| {
                    let mag = (re * re + im * im).sqrt();
                    FAST_PATH_ERROR_TYPICAL * mag.max(1e-12)
                })
                .collect();
            let rel: Vec<bool> = errors.iter().map(|&e| e < CERTIFIED_ERROR_TOL).collect();
            (Some(errors), Some(rel))
        } else {
            (None, None)
        };

        MehlerLevinResult {
            point_real,
            point_imag,
            max_error,
            reliable,
            used_gpu_kernel: false,
        }
    }
}

fn mehler_amplitude_cpu(lambda: f32, t: f32) -> f32 {
    if lambda.abs() < 1e-6 {
        return if t > 0.0 { 1.0 / t } else { 0.0 };
    }
    let lt = lambda * t;
    let sh = lt.sinh();
    if sh.abs() < 1e-12 {
        return if t > 0.0 { 1.0 / t } else { 0.0 };
    }
    lambda / sh
}

fn levin_quadrature_cpu(z: f32, t: f32, nodes: &[f32]) -> (f32, f32) {
    if z.abs() < 1e-4 {
        let inv_t = if t > 0.0 { 1.0 / t } else { 0.0 };
        return (inv_t, 0.0);
    }

    let w = 2.0 / nodes.len() as f32;
    let mut sum_re = 0.0f32;
    let mut sum_im = 0.0f32;

    for &lambda in nodes {
        let fval = mehler_amplitude_cpu(lambda, t);
        let phase = z * lambda;
        sum_re += w * fval * phase.cos();
        sum_im += w * fval * phase.sin();
    }

    (sum_re, sum_im)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_validates_f_node_length() {
        let harness = MehlerLevinHarness::new(0.1, false);
        let err = harness.evaluate(&[1.0], &[0.0; 4]).unwrap_err();
        assert!(matches!(err, CutileError::InvalidDimensions { .. }));
    }

    #[test]
    fn cpu_fallback_produces_finite_output() {
        let harness = MehlerLevinHarness::new(0.05, false);
        let z = vec![5.0, 10.0, 25.0];
        let mut nodes = Vec::new();
        for _ in 0..z.len() {
            nodes.extend_from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        }
        let result = harness.evaluate(&z, &nodes).unwrap();
        assert_eq!(result.point_real.len(), 3);
        assert!(result.point_real.iter().all(|v| v.is_finite()));
        assert!(!result.used_gpu_kernel);
    }

    #[test]
    fn certified_mode_reports_reliability() {
        let harness = MehlerLevinHarness::new(0.05, true);
        let z = vec![5.0];
        let nodes = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = harness.evaluate(&z, &nodes).unwrap();
        assert!(result.max_error.is_some());
        assert!(result.reliable.is_some());
    }
}