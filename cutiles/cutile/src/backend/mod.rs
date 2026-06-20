pub mod cpu;
pub mod cuda;
pub mod types;
pub mod wgpu;

use crate::error::CutileError;
use crate::traits::TilingStrategy;

pub use cpu::CpuBackend;
pub use cuda::{CudaBackend, CudaEntropyResult};
pub use types::{EntropyParams, EntropyResult};
pub use wgpu::WgpuBackend;

/// Tiered backend selection: wgpu (portable) → CUDA (NVIDIA peak) → CPU fallback.
pub enum Backend {
    Cpu(CpuBackend),
    #[cfg(feature = "cuda")]
    Cuda(CudaBackend),
    #[cfg(feature = "wgpu-backend")]
    Wgpu(WgpuBackend),
}

impl Backend {
    /// Prefer CUDA when available, then wgpu, else CPU.
    pub fn auto() -> Self {
        #[cfg(feature = "cuda")]
        if let Ok(cuda) = CudaBackend::new() {
            return Self::Cuda(cuda);
        }
        #[cfg(feature = "wgpu-backend")]
        if let Ok(wgpu) = WgpuBackend::new() {
            return Self::Wgpu(wgpu);
        }
        Self::Cpu(CpuBackend::new())
    }

    pub fn cpu() -> Self {
        Self::Cpu(CpuBackend::new())
    }

    #[cfg(feature = "cuda")]
    pub fn try_cuda() -> Result<Self, CutileError> {
        Ok(Self::Cuda(CudaBackend::new()?))
    }

    #[cfg(feature = "wgpu-backend")]
    pub fn try_wgpu() -> Result<Self, CutileError> {
        Ok(Self::Wgpu(WgpuBackend::new()?))
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Cpu(_) => "cpu",
            #[cfg(feature = "cuda")]
            Self::Cuda(_) => "cuda",
            #[cfg(feature = "wgpu-backend")]
            Self::Wgpu(_) => "wgpu",
        }
    }

    pub fn compute_entropy_v2(&self, params: &EntropyParams) -> Result<EntropyResult, CutileError> {
        params.validate()?;
        match self {
            Self::Cpu(b) => b.compute_entropy_v2(params),
            #[cfg(feature = "cuda")]
            Self::Cuda(b) => Ok(
                b.compute_entropy_v2(
                    &params.omega_tilde,
                    &params.d_perp_rho_sq,
                    &params.rho,
                    &params.strain_norms,
                    params.tau,
                    params.nu,
                    params.surge_threshold,
                    params.prev_w_avg,
                )?
                .into(),
            ),
            #[cfg(feature = "wgpu-backend")]
            Self::Wgpu(b) => b.compute_entropy_v2(params),
        }
    }

    /// Batched entropy evaluation — primary optimization for SRAC loops (reduces per-call overhead).
    pub fn compute_entropy_batch(
        &self,
        batch: &[EntropyParams],
    ) -> Result<Vec<EntropyResult>, CutileError> {
        if batch.is_empty() {
            return Ok(Vec::new());
        }
        match self {
            Self::Cpu(b) => b.compute_entropy_batch(batch),
            #[cfg(feature = "cuda")]
            Self::Cuda(b) => batch
                .iter()
                .map(|p| {
                    Ok(b.compute_entropy_v2(
                        &p.omega_tilde,
                        &p.d_perp_rho_sq,
                        &p.rho,
                        &p.strain_norms,
                        p.tau,
                        p.nu,
                        p.surge_threshold,
                        p.prev_w_avg,
                    )?
                    .into())
                })
                .collect(),
            #[cfg(feature = "wgpu-backend")]
            Self::Wgpu(b) => b.compute_entropy_batch(batch),
        }
    }
}

/// CPU tiling helper shared by backends.
#[derive(Debug, Default)]
pub struct DefaultTiler;

impl TilingStrategy for DefaultTiler {
    fn recommended_tile_size(&self, problem_size: usize) -> usize {
        if problem_size < 256 {
            64
        } else if problem_size < 4096 {
            256
        } else {
            512
        }
    }
}