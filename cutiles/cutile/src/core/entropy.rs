/// Numerical counterpart to W[ω̃] from the formal layer.
pub fn compute_entropy_diagnostic(
    omega_tilde: &[f32],
    d_perp_rho_sq: &[f32],
    rho: &[f32],
    strain_norms: &[f32],
    tau: f64,
    nu: f64,
) -> (f64, f64, f64) {
    let mut w = 0.0;
    let mut visc = 0.0;
    let mut stretch = 0.0;

    for i in 0..omega_tilde.len() {
        if omega_tilde[i].is_finite() && rho[i].is_finite() {
            let grad_sq = d_perp_rho_sq
                .get(i % d_perp_rho_sq.len())
                .copied()
                .unwrap_or(0.0);
            w += tau * f64::from(grad_sq) + f64::from(rho[i]);
            visc += f64::from(grad_sq);
            stretch += f64::from(
                strain_norms
                    .get(i % strain_norms.len())
                    .copied()
                    .unwrap_or(0.0),
            ) * f64::from(omega_tilde[i]);
        }
    }

    (w, -nu * visc, -tau * stretch)
}

/// Betti proxy: count of mesh points exceeding gradient threshold.
pub fn betti_proxy(d_perp_rho_sq: &[f32], threshold: f32) -> f32 {
    d_perp_rho_sq
        .iter()
        .filter(|&&g| g > threshold)
        .count() as f32
}

/// Default surge detector (relative W jump).
#[derive(Debug, Default)]
pub struct DefaultSurgeDetector;

impl crate::traits::SurgeDetector for DefaultSurgeDetector {
    fn detect_surge(&self, current_w: f32, prev_w_avg: f32, threshold: f32) -> bool {
        let base = prev_w_avg.abs().max(1e-12);
        ((current_w - prev_w_avg).abs() / base) > threshold
    }
}