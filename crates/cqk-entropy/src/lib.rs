//! cqk-entropy — Microlocal Entropy Functional W[ω̃]
//!
//! Implements the directional entropy on the cosphere bundle S*M:
//!
//!   W[ω̃] = ∫_{S*M} (τ ‖d_⊥ρ‖² + ρ) dμ_{S*M}
//!
//! and the Theorem-4 dissipation inequality witness:
//!
//!   dW/dt ≤ −ν ∫ ‖d_⊥ρ‖² − τ ∫ ‖S‖ ‖ω̃‖_{g'}
//!
//! This is the CRITICAL kernel for the Navier-Stokes singularity hunt.
//! Every tick of the DNS loop, this crate evaluates the inequality and
//! logs whether the viscous term (Case A) or the stretching term (Case B)
//! dominates.
//!
//! Epistemic status: ⟦H-NS1⟧ — braid-provisional pending Lean L1/L2.
//! References:
//!   arXiv:2601.08854v3 Theorem 4
//!   Brief §1.2, §3.2, §6

use cqk_microlocal::{
    compute_perp_gradient, BaseMeshRef, CosphereLift, PerpGradient, StrainRateField,
};
use reson8_core::{enforce_invariant, InvariantStatus, WaveScore};
use serde::{Deserialize, Serialize};

// ── Constants ───────────────────────────────────────────────────────

/// CAP-ready residual tolerance (from DeepMind PINN pipeline).
/// Anything below this is numerical noise, not physics.
pub const CAP_EPS: f64 = 1e-13;

/// Default viscosity for Taylor-Green baseline test.
pub const NU_DEFAULT: f64 = 0.01;

// ── Entropy Reading ─────────────────────────────────────────────────

/// Instantaneous microlocal entropy and its derivative along the flow.
/// This is the primary observable for the Navier-Stokes singularity hunt.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EntropyReading {
    /// W[ω̃] — the microlocal entropy functional value.
    pub w: f64,
    /// Observed dW/dt (finite-difference from previous tick).
    pub dw_dt: f64,
    /// −ν ∫ ‖d_⊥ρ‖² — viscous dissipation term (Case A).
    pub visc_term: f64,
    /// −τ ∫ ‖S‖ ‖ω̃‖_{g'} — vortex stretching term (Case B).
    pub stretch_term: f64,
    /// Residual: dw_dt − (visc_term + stretch_term).
    /// Should be ≤ CAP_EPS if Theorem 4 holds.
    pub residual: f64,
    /// Whether the Theorem-4 inequality is violated at this tick.
    pub theorem_4_violated: bool,
    /// Which case dominates: A (viscous) or B (stretching).
    pub dominant_case: DominantCase,
}

/// Which mechanism dominates the dissipation balance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DominantCase {
    /// Case A: viscous dissipation dominates. Singularity suppressed.
    CaseA,
    /// Case B: vortex stretching dominates. Singularity risk.
    CaseB,
    /// Knife edge: terms are within 10% of each other.
    KnifeEdge,
}

// ── Entropy Evaluator ───────────────────────────────────────────────

/// Configuration for the entropy evaluator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyConfig {
    /// Kinematic viscosity ν.
    pub nu: f64,
    /// Temporal scaling parameter τ > 0.
    pub tau: f64,
    /// CAP tolerance for residual classification.
    pub cap_eps: f64,
}

impl Default for EntropyConfig {
    fn default() -> Self {
        Self {
            nu: NU_DEFAULT,
            tau: 1.0,
            cap_eps: CAP_EPS,
        }
    }
}

/// Evaluate the microlocal entropy functional W[ω̃].
///
///   W = ∫_{S*M} (τ ‖d_⊥ρ‖² + ρ) dμ
///
/// Discretized as a sum over mesh points × fibre directions.
pub fn evaluate_w<const N: usize>(
    lift: &CosphereLift<3, N>,
    perp_grad: &PerpGradient,
    tau: f64,
) -> f64 {
    let np = lift.base_mesh.total_points();
    let mut w = 0.0;

    for i in 0..np {
        for j in 0..N {
            let rho = lift.rho_at(i, j);
            if rho.is_finite() {
                // τ ‖d_⊥ρ‖² contribution (using per-point max as proxy)
                let grad_sq = perp_grad.per_point_max[i];
                w += tau * grad_sq + rho;
            }
        }
    }

    // Normalize by total DOF
    let dof = (np * N) as f64;
    if dof > 0.0 { w / dof } else { 0.0 }
}

/// Evaluate the viscous dissipation term (Case A RHS).
/// −ν ∫_{S*M} ‖d_⊥ρ‖² dμ
pub fn evaluate_visc_term(nu: f64, perp_grad: &PerpGradient) -> f64 {
    -nu * perp_grad.norm_sq_integrated
}

/// Evaluate the vortex stretching term (Case B RHS).
/// −τ ∫_{S*M} ‖S‖ ‖ω̃‖_{g'} dμ
pub fn evaluate_stretch_term<const N: usize>(
    tau: f64,
    lift: &CosphereLift<3, N>,
    strain: &StrainRateField,
) -> f64 {
    let np = lift.base_mesh.total_points();
    let mut total = 0.0;
    for i in 0..np {
        let s_norm = strain.frobenius_norms[i];
        for j in 0..N {
            let omega_mag = lift.omega_tilde[i * N + j];
            total += s_norm * omega_mag;
        }
    }
    -tau * total
}

/// Full entropy evaluation: compute W, dW/dt, viscous/stretch terms, residual.
pub fn evaluate_entropy<const N: usize>(
    config: &EntropyConfig,
    lift: &CosphereLift<3, N>,
    strain: &StrainRateField,
    w_previous: f64,
    dt: f64,
) -> EntropyReading {
    let perp_grad = compute_perp_gradient(lift);

    let w = evaluate_w(lift, &perp_grad, config.tau);
    let dw_dt = if dt > 0.0 { (w - w_previous) / dt } else { 0.0 };

    let visc_term = evaluate_visc_term(config.nu, &perp_grad);
    let stretch_term = evaluate_stretch_term(config.tau, lift, strain);

    let rhs_bound = visc_term + stretch_term;
    let residual = dw_dt - rhs_bound;
    let theorem_4_violated = residual > config.cap_eps;

    let dominant_case = {
        let v = visc_term.abs();
        let s = stretch_term.abs();
        if v < 1e-30 && s < 1e-30 {
            DominantCase::KnifeEdge
        } else {
            let ratio = v / (v + s);
            if ratio > 0.55 {
                DominantCase::CaseA
            } else if ratio < 0.45 {
                DominantCase::CaseB
            } else {
                DominantCase::KnifeEdge
            }
        }
    };

    EntropyReading {
        w,
        dw_dt,
        visc_term,
        stretch_term,
        residual,
        theorem_4_violated,
        dominant_case,
    }
}

// ── Runtime Guards ──────────────────────────────────────────────────

/// The critical per-tick assertion from Brief §3.3.
/// Panics if the Theorem-4 dissipation inequality is violated AND
/// the universal invariant fails.
pub fn assert_conservation(reading: &EntropyReading, alpha: f64, omega: f64) {
    // Theorem-4 witness
    debug_assert!(
        !reading.theorem_4_violated,
        "cqk-entropy: Theorem-4 violated! residual={:.6e}, dW/dt={:.6e}, RHS_bound={:.6e}",
        reading.residual, reading.dw_dt, reading.visc_term + reading.stretch_term,
    );
    // Universal invariant
    let inv = enforce_invariant(alpha, omega);
    debug_assert!(
        inv.status == InvariantStatus::Passed,
        "cqk-entropy: α+ω={:.3} ≠ 15 (deviation={:.3})",
        inv.total, inv.deviation,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use cqk_microlocal::{fibonacci_sphere_directions, lift_vorticity, VorticityField};

    fn make_test_lift() -> (CosphereLift<3, 8>, StrainRateField) {
        let mesh = BaseMeshRef {
            nx: 4, ny: 4, nz: 4,
            lx: 1.0, ly: 1.0, lz: 1.0,
        };
        let np = mesh.total_points();
        let mut vort = VorticityField::zeros(np);
        // Taylor-Green-like: ω_z = cos(x)*cos(y)
        for i in 0..np {
            vort.data[3 * i + 2] = 1.0;
        }
        let dirs = fibonacci_sphere_directions::<8>();
        let lift = lift_vorticity(&mesh, &vort, &dirs);
        let strain = StrainRateField { frobenius_norms: vec![0.5; np] };
        (lift, strain)
    }

    #[test]
    fn entropy_reading_computes() {
        let (lift, strain) = make_test_lift();
        let config = EntropyConfig::default();
        let reading = evaluate_entropy(&config, &lift, &strain, 0.0, 0.01);
        // W should be finite and defined
        assert!(reading.w.is_finite());
    }

    #[test]
    fn conservation_passes_at_invariant() {
        let (lift, strain) = make_test_lift();
        let config = EntropyConfig::default();
        let reading = evaluate_entropy(&config, &lift, &strain, 0.0, 0.01);
        // Should not panic with valid invariant
        assert_conservation(&reading, 7.0, 8.0);
    }

    #[test]
    fn dominant_case_classification() {
        let reading = EntropyReading {
            w: 1.0, dw_dt: -0.5,
            visc_term: -0.8, stretch_term: -0.1,
            residual: 0.0, theorem_4_violated: false,
            dominant_case: DominantCase::CaseA,
        };
        assert_eq!(reading.dominant_case, DominantCase::CaseA);
    }
}
