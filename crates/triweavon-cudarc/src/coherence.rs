//! GPU SRAC coherence metrics — feeds Stitch Viviani Peak / OTLP reson8.* dimensions.

use crate::Result;

/// SRAC metric bundle for observability export.
#[derive(Debug, Clone, Copy)]
pub struct SracGpuMetrics {
    pub w: f32,
    pub surge: f32,
    pub betti_proxy: f32,
    pub used_gpu: bool,
}

pub fn compute_srac_metrics(
    omega_tilde: &[f32],
    d_perp_rho_sq: &[f32],
    rho: &[f32],
    strain_norms: &[f32],
) -> Result<SracGpuMetrics> {
    let backend = cutile::Backend::auto();
    let r = backend.compute_entropy_v2(&cutile::EntropyParams {
        omega_tilde: omega_tilde.to_vec(),
        d_perp_rho_sq: d_perp_rho_sq.to_vec(),
        rho: rho.to_vec(),
        strain_norms: strain_norms.to_vec(),
        tau: 1.0,
        nu: 0.01,
        surge_threshold: 0.05,
        prev_w_avg: 0.0,
    })?;
    Ok(SracGpuMetrics {
        w: r.w,
        surge: r.surge,
        betti_proxy: r.betti_proxy,
        used_gpu: r.used_gpu_kernel,
    })
}