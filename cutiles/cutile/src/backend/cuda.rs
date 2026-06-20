use crate::core::entropy::compute_entropy_diagnostic;
use crate::error::CutileError;
use crate::traits::ManifoldCompute;

#[cfg(feature = "cuda")]
use cudarc::driver::{CudaDevice, LaunchAsync, LaunchConfig};

const MODULE: &str = "blackwell_entropy_v2";
const KERNEL: &str = "entropy_reduction_v2";
const BLOCK: u32 = 256;

/// GPU entropy launch result (scalar reductions + surge proxy).
#[derive(Debug, Clone, Copy)]
pub struct CudaEntropyResult {
    pub w: f32,
    pub visc: f32,
    pub stretch: f32,
    pub surge: f32,
    pub betti_proxy: f32,
    pub used_gpu_kernel: bool,
}

pub struct CudaBackend {
    #[cfg(feature = "cuda")]
    device: CudaDevice,
    #[cfg(feature = "cuda")]
    kernel_ready: bool,
}

impl CudaBackend {
    pub fn new() -> Result<Self, CutileError> {
        #[cfg(feature = "cuda")]
        {
            let device = CudaDevice::new(0).map_err(|e| CutileError::Cuda(e.to_string()))?;
            let kernel_ready = Self::try_load_kernel(&device);
            Ok(Self {
                device,
                kernel_ready,
            })
        }
        #[cfg(not(feature = "cuda"))]
        {
            Err(CutileError::BackendUnavailable(
                "compile with --features cuda".into(),
            ))
        }
    }

    #[cfg(feature = "cuda")]
    fn try_load_kernel(device: &CudaDevice) -> bool {
        #[cfg(ptx_embedded)]
        {
            const PTX: &str = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/kernels/blackwell_entropy_v2.ptx"
            ));
            device
                .load_ptx(PTX, MODULE, &[KERNEL])
                .map(|_| true)
                .unwrap_or(false)
        }
        #[cfg(not(ptx_embedded))]
        {
            let _ = device;
            false
        }
    }

    #[cfg(feature = "cuda")]
    fn launch_kernel(
        &self,
        omega_tilde: &[f32],
        d_perp_rho_sq: &[f32],
        rho: &[f32],
        strain_norms: &[f32],
        tau: f32,
        nu: f32,
        surge_threshold: f32,
        prev_w_avg: f32,
    ) -> Result<CudaEntropyResult, CutileError> {
        if !self.kernel_ready {
            return Err(CutileError::BackendUnavailable(
                "PTX missing — run: nvcc -ptx -arch=sm_100 kernels/blackwell_entropy_v2.cu -o kernels/blackwell_entropy_v2.ptx"
                    .into(),
            ));
        }

        let func = self
            .device
            .get_func(MODULE, KERNEL)
            .ok_or_else(|| CutileError::Cuda("entropy_reduction_v2 not loaded".into()))?;

        let total_dof = omega_tilde.len() as i32;
        let np = d_perp_rho_sq.len().max(1) as i32;

        let d_omega = self
            .device
            .htod_sync_copy(omega_tilde)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;
        let d_grad = self
            .device
            .htod_sync_copy(d_perp_rho_sq)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;
        let d_rho = self
            .device
            .htod_sync_copy(rho)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;
        let d_strain = self
            .device
            .htod_sync_copy(strain_norms)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;

        let mut d_w = self
            .device
            .alloc_zeros::<f32>(1)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;
        let mut d_visc = self
            .device
            .alloc_zeros::<f32>(1)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;
        let mut d_stretch = self
            .device
            .alloc_zeros::<f32>(1)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;
        let mut d_surge = self
            .device
            .alloc_zeros::<f32>(1)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;
        let mut d_betti = self
            .device
            .alloc_zeros::<f32>(1)
            .map_err(|e| CutileError::Cuda(e.to_string()))?;

        let grid = ((total_dof as u32).saturating_add(BLOCK - 1)) / BLOCK;
        let cfg = LaunchConfig {
            grid_dim: (grid.max(1), 1, 1),
            block_dim: (BLOCK, 1, 1),
            shared_mem_bytes: 0,
        };

        unsafe {
            func.launch(
                cfg,
                (
                    &d_omega,
                    &d_grad,
                    &d_rho,
                    &d_strain,
                    &mut d_w,
                    &mut d_visc,
                    &mut d_stretch,
                    &mut d_surge,
                    &mut d_betti,
                    total_dof,
                    np,
                    nu,
                    tau,
                    surge_threshold,
                    prev_w_avg,
                ),
            )
            .map_err(|e| CutileError::Cuda(e.to_string()))?;
        }

        self.device
            .synchronize()
            .map_err(|e| CutileError::Cuda(e.to_string()))?;

        let w = self
            .device
            .dtoh_sync_copy(&d_w)
            .map_err(|e| CutileError::Cuda(e.to_string()))?[0];
        let visc = self
            .device
            .dtoh_sync_copy(&d_visc)
            .map_err(|e| CutileError::Cuda(e.to_string()))?[0];
        let stretch = self
            .device
            .dtoh_sync_copy(&d_stretch)
            .map_err(|e| CutileError::Cuda(e.to_string()))?[0];
        let surge = self
            .device
            .dtoh_sync_copy(&d_surge)
            .map_err(|e| CutileError::Cuda(e.to_string()))?[0];
        let betti_proxy = self
            .device
            .dtoh_sync_copy(&d_betti)
            .map_err(|e| CutileError::Cuda(e.to_string()))?[0];

        Ok(CudaEntropyResult {
            w,
            visc,
            stretch,
            surge,
            betti_proxy,
            used_gpu_kernel: true,
        })
    }

    /// Full v2 path with surge outputs; CPU fallback when PTX/device unavailable.
    pub fn compute_entropy_v2(
        &self,
        omega_tilde: &[f32],
        d_perp_rho_sq: &[f32],
        rho: &[f32],
        strain_norms: &[f32],
        tau: f32,
        nu: f32,
        surge_threshold: f32,
        prev_w_avg: f32,
    ) -> Result<CudaEntropyResult, CutileError> {
        if omega_tilde.len() != rho.len() {
            return Err(CutileError::InvalidDimensions {
                expected: rho.len(),
                actual: omega_tilde.len(),
            });
        }

        #[cfg(feature = "cuda")]
        if self.kernel_ready {
            if let Ok(gpu) = self.launch_kernel(
                omega_tilde,
                d_perp_rho_sq,
                rho,
                strain_norms,
                tau,
                nu,
                surge_threshold,
                prev_w_avg,
            ) {
                return Ok(gpu);
            }
        }

        let (w, visc, stretch) = compute_entropy_diagnostic(
            omega_tilde,
            d_perp_rho_sq,
            rho,
            strain_norms,
            f64::from(tau),
            f64::from(nu),
        );
        let base = prev_w_avg.abs().max(1e-12);
        let rel = ((w as f32) - prev_w_avg).abs() / base;
        Ok(CudaEntropyResult {
            w: w as f32,
            visc: visc as f32,
            stretch: stretch as f32,
            surge: if rel > surge_threshold { 1.0 } else { 0.0 },
            betti_proxy: 0.0,
            used_gpu_kernel: false,
        })
    }
}

impl ManifoldCompute for CudaBackend {
    type BackendError = CutileError;

    fn compute_entropy(
        &self,
        omega_tilde: &[f32],
        d_perp_rho_sq: &[f32],
        rho: &[f32],
        strain_norms: &[f32],
        tau: f32,
        nu: f32,
    ) -> Result<(f32, f32, f32), Self::BackendError> {
        let r = self.compute_entropy_v2(
            omega_tilde,
            d_perp_rho_sq,
            rho,
            strain_norms,
            tau,
            nu,
            0.05,
            0.0,
        )?;
        Ok((r.w, r.visc, r.stretch))
    }
}