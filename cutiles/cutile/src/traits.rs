pub trait ManifoldCompute {
    type BackendError;

    fn compute_entropy(
        &self,
        omega_tilde: &[f32],
        d_perp_rho_sq: &[f32],
        rho: &[f32],
        strain_norms: &[f32],
        tau: f32,
        nu: f32,
    ) -> Result<(f32, f32, f32), Self::BackendError>;
}

pub trait SurgeDetector {
    fn detect_surge(&self, current_w: f32, prev_w_avg: f32, threshold: f32) -> bool;
}

pub trait TilingStrategy {
    fn recommended_tile_size(&self, problem_size: usize) -> usize;
}