/// Shared entropy kernel inputs (CUDA, wgpu, CPU).
#[derive(Debug, Clone)]
pub struct EntropyParams {
    pub omega_tilde: Vec<f32>,
    pub d_perp_rho_sq: Vec<f32>,
    pub rho: Vec<f32>,
    pub strain_norms: Vec<f32>,
    pub tau: f32,
    pub nu: f32,
    pub surge_threshold: f32,
    pub prev_w_avg: f32,
}

impl EntropyParams {
    pub fn validate(&self) -> Result<(), crate::CutileError> {
        if self.omega_tilde.len() != self.rho.len() {
            return Err(crate::CutileError::InvalidDimensions {
                expected: self.rho.len(),
                actual: self.omega_tilde.len(),
            });
        }
        Ok(())
    }
}

/// Unified entropy + surge result across all backends.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntropyResult {
    pub w: f32,
    pub visc: f32,
    pub stretch: f32,
    pub surge: f32,
    pub betti_proxy: f32,
    pub used_gpu_kernel: bool,
}

impl From<super::cuda::CudaEntropyResult> for EntropyResult {
    fn from(r: super::cuda::CudaEntropyResult) -> Self {
        Self {
            w: r.w,
            visc: r.visc,
            stretch: r.stretch,
            surge: r.surge,
            betti_proxy: r.betti_proxy,
            used_gpu_kernel: r.used_gpu_kernel,
        }
    }
}