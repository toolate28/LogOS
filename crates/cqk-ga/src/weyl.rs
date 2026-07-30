//! # Weyl Gauge Theory in Cl(1,3)
//!
//! Implements the Weyl scalar curvature, covariant derivative, and Lagrangian
//! from Ghilencea arXiv:2604.07508, as bridged by @Akitti's THTW stack.
//!
//! Ported from Gemini's battle-tested cQ-kitty-rips-ga.
//!
//! ## Key Formulas
//!
//! - Weyl-modified Ricci scalar: R̂ = R − 6(∇·ω) − 6(ω·ω)
//! - Weyl-covariant derivative: ∇̂T = (∇ + q_T·ω)T
//! - Weyl Lagrangian: L = (1/24ξ²)R̂² − (1/4α²)F̂²
//!
//! ## Rigor Category: A (genuinely derived from local scale invariance)

use crate::cl13::Cl13;

/// Weyl-modified Ricci scalar: R̂ = R − 6(∇·ω) − 6(ω·ω)
#[inline(always)]
pub fn hat_r(r_riemann: f64, nabla_dot_omega: f64, omega_sq: f64) -> f64 {
    r_riemann - 6.0 * nabla_dot_omega - 6.0 * omega_sq
}

/// Weyl gauge field strength as a bivector in Cl(1,3).
/// F̂_μν = ∂_μ ω_ν − ∂_ν ω_μ
pub fn weyl_field_strength(d_mu_omega_nu: &[[f64; 4]; 4]) -> Cl13 {
    let f01 = d_mu_omega_nu[0][1] - d_mu_omega_nu[1][0];
    let f02 = d_mu_omega_nu[0][2] - d_mu_omega_nu[2][0];
    let f03 = d_mu_omega_nu[0][3] - d_mu_omega_nu[3][0];
    let f12 = d_mu_omega_nu[1][2] - d_mu_omega_nu[2][1];
    let f13 = d_mu_omega_nu[1][3] - d_mu_omega_nu[3][1];
    let f23 = d_mu_omega_nu[2][3] - d_mu_omega_nu[3][2];
    Cl13::bivector([f01, f02, f03, f12, f13, f23])
}

/// Squared field strength: F̂² = F̂_μν F̂^μν
/// With signature (+,−,−,−), raising indices flips signs on spatial components.
pub fn field_strength_squared(f: &Cl13) -> f64 {
    let d = &f.data;
    2.0 * (d[5] * d[5] + d[6] * d[6] + d[7] * d[7]
         - d[8] * d[8] - d[9] * d[9] - d[10] * d[10])
}

/// Weyl Lagrangian density: L = (1/24ξ²)R̂² − (1/4α²)F̂²
#[inline]
pub fn weyl_lagrangian(hat_r_val: f64, f_sq: f64, xi: f64, alpha: f64) -> f64 {
    assert!(xi.abs() > 1e-15, "xi must be nonzero");
    assert!(alpha.abs() > 1e-15, "alpha must be nonzero");
    (1.0 / (24.0 * xi * xi)) * hat_r_val * hat_r_val
        - (1.0 / (4.0 * alpha * alpha)) * f_sq
}

/// Weyl-covariant derivative: ∇̂_μ T = ∂_μ T + q_T · ω_μ · T
#[inline]
pub fn weyl_covariant_derivative(
    partial_t: &Cl13,
    omega_mu: f64,
    q_t: f64,
    t: &Cl13,
) -> Cl13 {
    *partial_t + (*t * (q_t * omega_mu))
}

/// Spontaneous breaking: returns (M_P², Λ_stub).
pub fn spontaneous_breaking(vev_phi: f64, xi: f64) -> (f64, f64) {
    let m_planck_sq = xi * vev_phi * vev_phi;
    (m_planck_sq, 0.0) // Λ requires full scalar potential
}

/// Conservation invariant: α + ω = 15
#[inline(always)]
pub fn check_invariant(alpha: f64, omega: f64) -> bool {
    (alpha + omega - 15.0).abs() < 1e-10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hat_r_flat() { assert_eq!(hat_r(0.0, 0.0, 0.0), 0.0); }

    #[test]
    fn test_hat_r_pure_weyl() {
        let r_hat = hat_r(0.0, 1.0, 0.5);
        assert!((r_hat - (-9.0)).abs() < 1e-12);
    }

    #[test]
    fn test_lagrangian_signs() {
        let l = weyl_lagrangian(10.0, 4.0, 1.0, 1.0);
        let expected = (1.0 / 24.0) * 100.0 - (1.0 / 4.0) * 4.0;
        assert!((l - expected).abs() < 1e-12);
    }

    #[test]
    fn test_invariant() {
        assert!(check_invariant(8.0, 7.0));
        assert!(!check_invariant(8.0, 8.0));
    }
}
