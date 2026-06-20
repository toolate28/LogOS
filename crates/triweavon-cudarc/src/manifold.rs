use crate::error::TriweavonCudarcError;
use crate::Result;
use crate::UNIVERSAL_INVARIANT_SUM;

/// Viviani crossing constraint — α + ω must equal 15.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VivianiConstraint;

#[derive(Clone, Debug)]
pub struct ManifoldConfig {
    pub alpha: u32,
    pub omega: u32,
    pub wave_target: f32,
    pub k22_sheaf_dim: usize,
}

impl ManifoldConfig {
    pub fn new(alpha: u32, omega: u32, wave_target: f32, k22_sheaf_dim: usize) -> Result<Self> {
        let sum = alpha.saturating_add(omega);
        if sum != UNIVERSAL_INVARIANT_SUM {
            return Err(TriweavonCudarcError::InvariantViolation { sum });
        }
        Ok(Self {
            alpha,
            omega,
            wave_target,
            k22_sheaf_dim,
        })
    }

    pub fn alpha_omega(&self) -> (u32, u32) {
        (self.alpha, self.omega)
    }
}

/// GPU manifold state (CPU fallback uses cutile entropy path).
pub struct GpuManifold {
    config: ManifoldConfig,
    coherence_buffer: Vec<f32>,
}

impl GpuManifold {
    pub fn new(config: ManifoldConfig) -> Result<Self> {
        Ok(Self {
            config,
            coherence_buffer: Vec::new(),
        })
    }

    pub fn config(&self) -> &ManifoldConfig {
        &self.config
    }

    pub fn upload_coherence_state(&mut self, data: &[f32]) -> Result<()> {
        self.coherence_buffer = data.to_vec();
        Ok(())
    }

    /// SRAC coherence metric — delegates to cutile CPU path until Jones/K22 kernels land.
    pub fn compute_coherence(&self) -> Result<f32> {
        if self.coherence_buffer.is_empty() {
            return Ok(0.0);
        }
        let n = self.coherence_buffer.len();
        let grad = vec![0.01f32; n.max(1)];
        let rho = vec![0.05f32; n];
        let strain = vec![0.2f32; n.max(1)];
        let backend = cutile::Backend::cpu();
        let params = cutile::EntropyParams {
            omega_tilde: self.coherence_buffer.clone(),
            d_perp_rho_sq: grad,
            rho,
            strain_norms: strain,
            tau: 1.0,
            nu: 0.01,
            surge_threshold: 0.05,
            prev_w_avg: 0.0,
        };
        let r = backend.compute_entropy_v2(&params)?;
        Ok(r.w)
    }

    pub fn execute_k22_sheaf(&self) -> Result<()> {
        // Skeleton — K22_SHEAF GPU traversal TBD
        Ok(())
    }
}