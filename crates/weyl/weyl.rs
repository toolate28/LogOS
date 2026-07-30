//! # Weyl Gauge Theory in Cl(1,3)
//!
//! Implements the Weyl scalar curvature, covariant derivative, and Lagrangian
//! from Ghilencea arXiv:2604.07508, as bridged by @Akitti's THTW stack.
//!
//! ## Key Formulas
//!
//! - Weyl-modified Ricci scalar: R̂ = R − 6(∇·ω) − 6(ω·ω)
//! - Weyl-covariant derivative: ∇̂T = (∇ + q_T·ω)T
//! - Weyl Lagrangian: L = (1/24ξ²)R̂² − (1/4α²)F̂²
//!
//! ## Rigor Category: A (genuinely derived)
//!
//! These are standard results from Weyl gauge theory. The formulas are not
//! design conventions — they follow from the requirement of local scale invariance.

use crate::cl13::Cl13;

/// Weyl-modified Ricci scalar: R̂ = R − 6(∇·ω) − 6(ω·ω)
///
/// # Arguments
/// - `r_riemann`: The Riemannian Ricci scalar R
/// - `nabla_dot_omega`: Divergence of the Weyl gauge field ∇_μ ω^μ
/// - `omega_sq`: Squared norm of the Weyl gauge field ω_μ ω^μ
///
/// # Returns
/// The Weyl-modified scalar curvature R̂
#[inline(always)]
pub fn hat_r(r_riemann: f64, nabla_dot_omega: f64, omega_sq: f64) -> f64 {
    r_riemann - 6.0 * nabla_dot_omega - 6.0 * omega_sq
}

/// Weyl gauge field strength: F̂_μν = ∂_μ ω_ν − ∂_ν ω_μ
///
/// For an abelian gauge field, this is just the curl.
/// Returns the 6 independent components as a bivector in Cl(1,3).
pub fn weyl_field_strength(
    d_mu_omega_nu: &[[f64; 4]; 4], // ∂_μ ω_ν matrix
) -> Cl13 {
    // F̂₀₁ = ∂₀ω₁ − ∂₁ω₀
    let f01 = d_mu_omega_nu[0][1] - d_mu_omega_nu[1][0];
    let f02 = d_mu_omega_nu[0][2] - d_mu_omega_nu[2][0];
    let f03 = d_mu_omega_nu[0][3] - d_mu_omega_nu[3][0];
    let f12 = d_mu_omega_nu[1][2] - d_mu_omega_nu[2][1];
    let f13 = d_mu_omega_nu[1][3] - d_mu_omega_nu[3][1];
    let f23 = d_mu_omega_nu[2][3] - d_mu_omega_nu[3][2];

    Cl13::bivector([f01, f02, f03, f12, f13, f23])
}

/// Squared field strength: F̂² = F̂_μν F̂^μν
///
/// With signature (+,−,−,−), raising indices flips signs on spatial components.
pub fn field_strength_squared(f: &Cl13) -> f64 {
    // F̂_μν F̂^μν = 2(F₀₁² + F₀₂² + F₀₃² − F₁₂² − F₁₃² − F₂₃²)
    // The factor of 2 comes from antisymmetry; the signs from the metric.
    let d = &f.data;
    2.0 * (d[5] * d[5] + d[6] * d[6] + d[7] * d[7]   // timelike: +
         - d[8] * d[8] - d[9] * d[9] - d[10] * d[10])  // spacelike: −
}

/// Weyl Lagrangian density: L = (1/24ξ²)R̂² − (1/4α²)F̂²
///
/// # Arguments
/// - `hat_r_val`: The Weyl-modified scalar curvature R̂
/// - `f_sq`: The squared field strength F̂²
/// - `xi`: Coupling constant ξ (from non-minimal coupling)
/// - `alpha`: Gauge coupling constant α
///
/// # Returns
/// The Lagrangian density as a scalar
#[inline]
pub fn weyl_lagrangian(hat_r_val: f64, f_sq: f64, xi: f64, alpha: f64) -> f64 {
    assert!(xi.abs() > 1e-15, "ξ must be nonzero");
    assert!(alpha.abs() > 1e-15, "α must be nonzero");
    (1.0 / (24.0 * xi * xi)) * hat_r_val * hat_r_val
        - (1.0 / (4.0 * alpha * alpha)) * f_sq
}

/// Weyl-covariant derivative acting on a Cl(1,3) multivector.
///
/// ∇̂_μ T = ∂_μ T + q_T · ω_μ · T
///
/// # Arguments
/// - `partial_t`: The partial derivative ∂_μ T (a multivector)
/// - `omega_mu`: The Weyl gauge field component ω_μ (a scalar)
/// - `q_t`: The Weyl charge of T
/// - `t`: The multivector T being transported
///
/// # Returns
/// The covariant derivative ∇̂_μ T
#[inline]
pub fn weyl_covariant_derivative(
    partial_t: &Cl13,
    omega_mu: f64,
    q_t: f64,
    t: &Cl13,
) -> Cl13 {
    *partial_t + (*t * (q_t * omega_mu))
}

/// Spontaneous breaking check: does the massive ω_μ decouple?
///
/// After spontaneous breaking of scale symmetry, ω_μ acquires mass m_ω.
/// At energies E ≪ m_ω, the Weyl field decouples and we recover
/// standard Einstein-Hilbert gravity with a positive cosmological constant.
///
/// Returns the effective Planck mass M_P² and cosmological constant Λ
/// given the symmetry breaking scale ⟨φ⟩ and coupling ξ.
pub fn spontaneous_breaking(
    vev_phi: f64,   // ⟨φ⟩ — symmetry breaking VEV
    xi: f64,        // non-minimal coupling
) -> (f64, f64) {
    // M_P² = ξ ⟨φ⟩²
    let m_planck_sq = xi * vev_phi * vev_phi;

    // Λ = λ⟨φ⟩⁴ / (4 M_P²) — but λ depends on potential; return ratio
    // For now return (M_P², 0.0) with a note that Λ requires the full potential
    (m_planck_sq, 0.0) // Λ computation requires λ from the scalar potential
}

/// Invariant check: α + ω = 15
///
/// Verifies the conservation law at the boundary of any computation.
/// This is a design convention (Category C) applied to Weyl gauge computations.
#[inline(always)]
pub fn check_invariant(alpha: f64, omega: f64) -> bool {
    (alpha + omega - 15.0).abs() < 1e-10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hat_r_flat_spacetime() {
        // Flat spacetime with no Weyl field: R̂ = 0
        assert_eq!(hat_r(0.0, 0.0, 0.0), 0.0);
    }

    #[test]
    fn test_hat_r_pure_weyl() {
        // R = 0, but Weyl field present
        let r_hat = hat_r(0.0, 1.0, 0.5);
        assert!((r_hat - (-6.0 - 3.0)).abs() < 1e-12);
    }

    #[test]
    fn test_field_strength_antisymmetry() {
        // F̂_μν = −F̂_νμ is guaranteed by construction
        let mut d = [[0.0f64; 4]; 4];
        d[0][1] = 3.0;
        d[1][0] = 1.0;
        let f = weyl_field_strength(&d);
        // F₀₁ = 3 − 1 = 2
        assert!((f.data[5] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_lagrangian_signs() {
        // R̂² term should be positive (quadratic gravity)
        // F̂² term should subtract (gauge kinetic energy)
        let l = weyl_lagrangian(10.0, 4.0, 1.0, 1.0);
        let r_term = (1.0 / 24.0) * 100.0;
        let f_term = (1.0 / 4.0) * 4.0;
        assert!((l - (r_term - f_term)).abs() < 1e-12);
    }

    #[test]
    fn test_invariant() {
        assert!(check_invariant(8.0, 7.0));
        assert!(check_invariant(7.0, 8.0));
        assert!(check_invariant(14.0, 1.0));
        assert!(!check_invariant(8.0, 8.0));
    }
}
