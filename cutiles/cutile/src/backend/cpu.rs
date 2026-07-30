use crate::core::entropy::{betti_proxy, compute_entropy_diagnostic};
use crate::error::CutileError;
use crate::traits::ManifoldCompute;

use super::types::{EntropyParams, EntropyResult};

#[derive(Debug, Default)]
pub struct CpuBackend;

impl CpuBackend {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_entropy_v2(&self, params: &EntropyParams) -> Result<EntropyResult, CutileError> {
        params.validate()?;
        let (w, visc, stretch) = compute_entropy_diagnostic(
            &params.omega_tilde,
            &params.d_perp_rho_sq,
            &params.rho,
            &params.strain_norms,
            f64::from(params.tau),
            f64::from(params.nu),
        );
        let base = params.prev_w_avg.abs().max(1e-12);
        let rel = ((w as f32) - params.prev_w_avg).abs() / base;
        let betti = betti_proxy(&params.d_perp_rho_sq, params.surge_threshold);
        Ok(EntropyResult {
            w: w as f32,
            visc: visc as f32,
            stretch: stretch as f32,
            surge: if rel > params.surge_threshold { 1.0 } else { 0.0 },
            betti_proxy: betti,
            used_gpu_kernel: false,
        })
    }

    pub fn compute_entropy_batch(
        &self,
        batch: &[EntropyParams],
    ) -> Result<Vec<EntropyResult>, CutileError> {
        batch.iter().map(|p| self.compute_entropy_v2(p)).collect()
    }
}

impl ManifoldCompute for CpuBackend {
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
        let r = self.compute_entropy_v2(&EntropyParams {
            omega_tilde: omega_tilde.to_vec(),
            d_perp_rho_sq: d_perp_rho_sq.to_vec(),
            rho: rho.to_vec(),
            strain_norms: strain_norms.to_vec(),
            tau,
            nu,
            surge_threshold: 0.05,
            prev_w_avg: 0.0,
        })?;
        Ok((r.w, r.visc, r.stretch))
    }
}