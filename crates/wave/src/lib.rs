//! reson8-wave — WAVE Coherence Analysis Engine
//!
//! Core equation: C(H) = W * exp(-k * |α + ω - 15|) * (1 + P)
//!
//! Extended with semantic vector field PDE operators (Grok §4 formalism):
//!   - Curl detector:  ∇×V — detects circular reasoning / feedback loops
//!   - Divergence detector: ∇·V — detects scope drift / semantic energy leaks
//!   - Viviani deviation: ΔV = α + ω − 15
//!
//! Reference: "Topological Invariants in Policy-Conditioned Dynamical Systems" §4
//! Reference: "Formal Algebraic Framework of QDI and Functorial Persistence" §VI

pub use reson8_core::{WaveScore, INVARIANT_TARGET};

/// Fibonacci weights for WAVE components (normalized, sum ≈ 1.0)
pub const F_TOPO: f64 = 8.0 / 21.0;   // 0.381
pub const F_SEM: f64 = 5.0 / 21.0;    // 0.238
pub const F_STRUCT: f64 = 5.0 / 21.0;  // 0.238
pub const F_TEMP: f64 = 3.0 / 21.0;    // 0.143

// ── WAVE Score ─────────────────────────────────────────────────────

/// Compute composite WAVE score from components
pub fn compute_wave(w_topo: f64, w_sem: f64, w_struct: f64, w_temp: f64) -> f64 {
    F_TOPO * w_topo + F_SEM * w_sem + F_STRUCT * w_struct + F_TEMP * w_temp
}

/// Coherence Functional: C(H) = W * exp(-k * |α + ω - 15|) * (1 + P)
pub fn coherence_functional(w: f64, alpha: f64, omega: f64, persistence_bonus: f64, k: f64) -> f64 {
    let deviation = (alpha + omega - INVARIANT_TARGET).abs();
    w * (-k * deviation).exp() * (1.0 + persistence_bonus)
}

// ── Semantic Vector Field ──────────────────────────────────────────

/// A 3D semantic vector field sample.
///
/// In the WAVE framework, semantic intent is modeled as a continuous
/// vector field V: ℝ³ → ℝ³ over the embedding manifold. Each component
/// maps to a semantic axis (e.g., structural, intentional, temporal).
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// ── Curl Detector (∇×V) ───────────────────────────────────────────

/// Partial derivatives of a vector field at a point.
///
/// Represents the Jacobian matrix ∂V_i/∂x_j evaluated at a single point.
/// In practice, these are computed via finite differences on the
/// 768-D embedding projected to 3D semantic axes.
#[derive(Debug, Clone, Copy)]
pub struct VectorFieldJacobian {
    /// ∂V_x/∂x, ∂V_x/∂y, ∂V_x/∂z
    pub dvx: Vec3,
    /// ∂V_y/∂x, ∂V_y/∂y, ∂V_y/∂z
    pub dvy: Vec3,
    /// ∂V_z/∂x, ∂V_z/∂y, ∂V_z/∂z
    pub dvz: Vec3,
}

impl VectorFieldJacobian {
    /// Compute the curl: ∇×V = (∂V_z/∂y − ∂V_y/∂z)î
    ///                        + (∂V_x/∂z − ∂V_z/∂x)ĵ
    ///                        + (∂V_y/∂x − ∂V_x/∂y)k̂
    ///
    /// Non-zero curl indicates circular reasoning or unresolvable
    /// recursive definitions — the field cannot settle to a fixed point.
    pub fn curl(&self) -> Vec3 {
        Vec3 {
            x: self.dvz.y - self.dvy.z,  // ∂V_z/∂y − ∂V_y/∂z
            y: self.dvx.z - self.dvz.x,  // ∂V_x/∂z − ∂V_z/∂x
            z: self.dvy.x - self.dvx.y,  // ∂V_y/∂x − ∂V_x/∂y
        }
    }

    /// Compute the divergence: ∇·V = ∂V_x/∂x + ∂V_y/∂y + ∂V_z/∂z
    ///
    /// - Positive divergence: scope drift / semantic energy escaping module boundary
    /// - Negative divergence: black-hole dependency (energy sink, over-coupling)
    /// - Zero divergence: ideal solenoidal field (conservation)
    pub fn divergence(&self) -> f64 {
        self.dvx.x + self.dvy.y + self.dvz.z
    }
}

/// Estimate the Jacobian from 6 neighboring field samples using
/// central finite differences.
///
/// Requires field values at (x±h, y, z), (x, y±h, z), (x, y, z±h).
pub fn estimate_jacobian(
    v_xp: &Vec3, v_xm: &Vec3,  // V(x+h), V(x-h)
    v_yp: &Vec3, v_ym: &Vec3,  // V(y+h), V(y-h)
    v_zp: &Vec3, v_zm: &Vec3,  // V(z+h), V(z-h)
    h: f64,
) -> VectorFieldJacobian {
    let inv_2h = 1.0 / (2.0 * h);
    VectorFieldJacobian {
        dvx: Vec3 {
            x: (v_xp.x - v_xm.x) * inv_2h,  // ∂V_x/∂x
            y: (v_yp.x - v_ym.x) * inv_2h,  // ∂V_x/∂y
            z: (v_zp.x - v_zm.x) * inv_2h,  // ∂V_x/∂z
        },
        dvy: Vec3 {
            x: (v_xp.y - v_xm.y) * inv_2h,  // ∂V_y/∂x
            y: (v_yp.y - v_ym.y) * inv_2h,  // ∂V_y/∂y
            z: (v_zp.y - v_zm.y) * inv_2h,  // ∂V_y/∂z
        },
        dvz: Vec3 {
            x: (v_xp.z - v_xm.z) * inv_2h,  // ∂V_z/∂x
            y: (v_yp.z - v_ym.z) * inv_2h,  // ∂V_z/∂y
            z: (v_zp.z - v_zm.z) * inv_2h,  // ∂V_z/∂z
        },
    }
}

// ── Drift Diagnosis ────────────────────────────────────────────────

/// Semantic drift diagnosis from vector field analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum DriftDiagnosis {
    /// Field is healthy: low curl, low divergence
    Healthy,
    /// Curl drift detected: circular reasoning / feedback loops
    /// Contains the curl magnitude.
    CurlDrift(f64),
    /// Scope drift detected: semantic energy escaping boundary
    /// Contains the divergence value.
    ScopeDrift(f64),
    /// Black-hole dependency: over-coupling / energy sink
    /// Contains the (negative) divergence value.
    BlackHole(f64),
    /// Combined pathology: both curl and divergence anomalies
    Combined { curl_mag: f64, divergence: f64 },
}

/// Thresholds for drift detection (calibrated to WAVE 0.98 gate)
pub const CURL_THRESHOLD: f64 = 0.05;
pub const DIVERGENCE_THRESHOLD: f64 = 0.05;

/// Diagnose semantic drift from a Jacobian measurement.
///
/// Maps directly to the QDI §VI semantic drift analysis:
///   - Non-zero curl = "curl drift" (cannot resolve to fixed point)
///   - Positive divergence = scope drift
///   - Negative divergence = black-hole dependency
///   - Optimal: purely solenoidal, rotation-free field
pub fn diagnose_drift(jac: &VectorFieldJacobian) -> DriftDiagnosis {
    let curl = jac.curl();
    let curl_mag = curl.magnitude();
    let div = jac.divergence();
    let div_abs = div.abs();

    let has_curl = curl_mag > CURL_THRESHOLD;
    let has_div = div_abs > DIVERGENCE_THRESHOLD;

    match (has_curl, has_div) {
        (false, false) => DriftDiagnosis::Healthy,
        (true, false) => DriftDiagnosis::CurlDrift(curl_mag),
        (false, true) if div > 0.0 => DriftDiagnosis::ScopeDrift(div),
        (false, true) => DriftDiagnosis::BlackHole(div),
        (true, true) => DriftDiagnosis::Combined {
            curl_mag,
            divergence: div,
        },
    }
}

// ── Viviani Deviation ──────────────────────────────────────────────

/// Viviani deviation: ΔV = α + ω − 15
///
/// The Viviani curve is the intersection of sphere and cylinder on the
/// manifold. ΔV = 0 defines the safe manifold where the universal
/// invariant holds exactly. Distance from the Viviani curve measures
/// how far the system has drifted from constitutional compliance.
pub fn viviani_deviation(alpha: f64, omega: f64) -> f64 {
    alpha + omega - INVARIANT_TARGET
}

/// Viviani-corrected coherence: penalizes both drift AND curl.
///
/// C_corrected = C(H) * exp(-γ * |∇×V|) * exp(-δ * |ΔV|)
///
/// where γ and δ are coupling constants. This extends the base
/// coherence functional to account for semantic field pathology.
pub fn coherence_with_drift_penalty(
    base_coherence: f64,
    curl_magnitude: f64,
    viviani_dev: f64,
    gamma: f64,  // curl coupling (default: 1.0)
    delta: f64,  // viviani coupling (default: 2.0, matches COHERENCE_K)
) -> f64 {
    base_coherence
        * (-gamma * curl_magnitude).exp()
        * (-delta * viviani_dev.abs()).exp()
}

// ── Coherence Score (QDI §VI formalization) ────────────────────────

/// Φ = tr(α·P_S + ω·P_I + τ·P_T) × 100
///
/// Per QDI §VI, the coherence score is the weighted trace of projection
/// operators with Fibonacci-ratio weights (8:5:3 → 0.50:0.3125:0.1875).
///
/// - α = 0.50   — Structural Rigidity (graph isomorphism AST↔schema)
/// - ω = 0.3125 — Semantic Intent (intent-to-implementation mapping)
/// - τ = 0.1875 — Temporal Consistency (cryptographic sync + timestamps)
pub fn coherence_score_phi(
    structural_rigidity: f64,  // P_S eigenvalue, [0,1]
    semantic_intent: f64,      // P_I eigenvalue, [0,1]
    temporal_consistency: f64, // P_T eigenvalue, [0,1]
) -> f64 {
    const ALPHA_W: f64 = 8.0 / 16.0;   // 0.500
    const OMEGA_W: f64 = 5.0 / 16.0;   // 0.3125
    const TAU_W: f64 = 3.0 / 16.0;     // 0.1875

    (ALPHA_W * structural_rigidity
        + OMEGA_W * semantic_intent
        + TAU_W * temporal_consistency) * 100.0
}

/// Snap-in threshold: Φ ≥ 70 triggers ecosystem synchronization
pub const PHI_SNAP_IN: f64 = 70.0;

/// Crystalline threshold: Φ ≥ 98 (V=c regime)
pub const PHI_CRYSTALLINE: f64 = 98.0;

// ── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wave_score_fibonacci_weighted() {
        // All perfect → 1.0
        let w = compute_wave(1.0, 1.0, 1.0, 1.0);
        assert!((w - 1.0).abs() < 1e-10);
    }

    #[test]
    fn coherence_maximizes_at_invariant() {
        let c_exact = coherence_functional(0.95, 7.0, 8.0, 0.1, 2.0);
        let c_off = coherence_functional(0.95, 6.0, 8.0, 0.1, 2.0);
        assert!(c_exact > c_off);
    }

    #[test]
    fn curl_of_irrotational_field_is_zero() {
        // Gradient field: V = (x, y, z) → curl is zero but div = 3 (source field)
        let jac = VectorFieldJacobian {
            dvx: Vec3::new(1.0, 0.0, 0.0),
            dvy: Vec3::new(0.0, 1.0, 0.0),
            dvz: Vec3::new(0.0, 0.0, 1.0),
        };
        let curl = jac.curl();
        assert!(curl.magnitude() < 1e-15, "Curl of identity field must be zero");
        // Note: div = 3, so this is ScopeDrift (source field), not Healthy.
        // A truly healthy field has BOTH zero curl AND zero divergence.
        match diagnose_drift(&jac) {
            DriftDiagnosis::ScopeDrift(d) => assert!((d - 3.0).abs() < 1e-10),
            other => panic!("Expected ScopeDrift(3.0), got {:?}", other),
        }
    }

    #[test]
    fn curl_detects_rotation() {
        // Rigid rotation V = (-y, x, 0) → curl = (0, 0, 2)
        let jac = VectorFieldJacobian {
            dvx: Vec3::new(0.0, -1.0, 0.0),  // ∂(-y)/∂x=0, ∂(-y)/∂y=-1, ∂(-y)/∂z=0
            dvy: Vec3::new(1.0, 0.0, 0.0),   // ∂(x)/∂x=1,  ∂(x)/∂y=0,  ∂(x)/∂z=0
            dvz: Vec3::new(0.0, 0.0, 0.0),
        };
        let curl = jac.curl();
        assert!((curl.z - 2.0).abs() < 1e-15);
        assert!((curl.x).abs() < 1e-15);
        assert!((curl.y).abs() < 1e-15);
        match diagnose_drift(&jac) {
            DriftDiagnosis::CurlDrift(m) => assert!(m > 1.0),
            other => panic!("Expected CurlDrift, got {:?}", other),
        }
    }

    #[test]
    fn divergence_detects_source() {
        // Expanding field V = (2x, 3y, 4z) → div = 9
        let jac = VectorFieldJacobian {
            dvx: Vec3::new(2.0, 0.0, 0.0),
            dvy: Vec3::new(0.0, 3.0, 0.0),
            dvz: Vec3::new(0.0, 0.0, 4.0),
        };
        assert!((jac.divergence() - 9.0).abs() < 1e-15);
        match diagnose_drift(&jac) {
            DriftDiagnosis::ScopeDrift(d) => assert!((d - 9.0).abs() < 1e-10),
            other => panic!("Expected ScopeDrift, got {:?}", other),
        }
    }

    #[test]
    fn divergence_detects_sink() {
        // Contracting field V = (-x, -y, -z) → div = -3
        let jac = VectorFieldJacobian {
            dvx: Vec3::new(-1.0, 0.0, 0.0),
            dvy: Vec3::new(0.0, -1.0, 0.0),
            dvz: Vec3::new(0.0, 0.0, -1.0),
        };
        assert!((jac.divergence() + 3.0).abs() < 1e-15);
        match diagnose_drift(&jac) {
            DriftDiagnosis::BlackHole(d) => assert!((d + 3.0).abs() < 1e-10),
            other => panic!("Expected BlackHole, got {:?}", other),
        }
    }

    #[test]
    fn solenoidal_field_is_healthy() {
        // Incompressible rotation: V = (-y, x, 0) has div=0
        // but has curl, so we need a solenoidal + irrotational field
        // Uniform flow V = (1, 0, 0): div=0, curl=0
        let jac = VectorFieldJacobian {
            dvx: Vec3::zero(),
            dvy: Vec3::zero(),
            dvz: Vec3::zero(),
        };
        assert_eq!(diagnose_drift(&jac), DriftDiagnosis::Healthy);
    }

    #[test]
    fn combined_pathology() {
        // Field with both curl and divergence
        let jac = VectorFieldJacobian {
            dvx: Vec3::new(2.0, -1.0, 0.0),
            dvy: Vec3::new(1.0, 3.0, 0.0),
            dvz: Vec3::new(0.0, 0.0, 4.0),
        };
        // div = 2 + 3 + 4 = 9
        // curl_z = ∂V_y/∂x - ∂V_x/∂y = 1 - (-1) = 2
        match diagnose_drift(&jac) {
            DriftDiagnosis::Combined { curl_mag, divergence } => {
                assert!(curl_mag > 1.0);
                assert!((divergence - 9.0).abs() < 1e-10);
            }
            other => panic!("Expected Combined, got {:?}", other),
        }
    }

    #[test]
    fn viviani_at_invariant() {
        assert!((viviani_deviation(7.0, 8.0)).abs() < 1e-15);
        assert!((viviani_deviation(8.0, 7.0)).abs() < 1e-15);
    }

    #[test]
    fn viviani_deviation_sign() {
        assert!(viviani_deviation(9.0, 8.0) > 0.0);  // excess
        assert!(viviani_deviation(5.0, 8.0) < 0.0);  // deficit
    }

    #[test]
    fn drift_penalty_reduces_coherence() {
        let base = 0.95;
        let penalized = coherence_with_drift_penalty(base, 0.5, 0.3, 1.0, 2.0);
        assert!(penalized < base);
        assert!(penalized > 0.0);
    }

    #[test]
    fn drift_penalty_zero_at_health() {
        let base = 0.95;
        let same = coherence_with_drift_penalty(base, 0.0, 0.0, 1.0, 2.0);
        assert!((same - base).abs() < 1e-15);
    }

    #[test]
    fn phi_score_perfect() {
        let phi = coherence_score_phi(1.0, 1.0, 1.0);
        assert!((phi - 100.0).abs() < 1e-10);
    }

    #[test]
    fn phi_score_snap_in() {
        // 0.70 uniform → 70.0
        let phi = coherence_score_phi(0.70, 0.70, 0.70);
        assert!((phi - 70.0).abs() < 1e-10);
        assert!(phi >= PHI_SNAP_IN);
    }

    #[test]
    fn finite_difference_jacobian() {
        // Test estimate_jacobian with linear field V = (2x, 3y, z)
        // At any point, Jacobian should be diag(2, 3, 1)
        let h = 0.001;
        // V(x+h) = (2(x+h), 3y, z) vs V(x-h) = (2(x-h), 3y, z)
        let v_xp = Vec3::new(2.0 * (1.0 + h), 3.0, 1.0);
        let v_xm = Vec3::new(2.0 * (1.0 - h), 3.0, 1.0);
        let v_yp = Vec3::new(2.0, 3.0 * (1.0 + h), 1.0);
        let v_ym = Vec3::new(2.0, 3.0 * (1.0 - h), 1.0);
        let v_zp = Vec3::new(2.0, 3.0, 1.0 + h);
        let v_zm = Vec3::new(2.0, 3.0, 1.0 - h);

        let jac = estimate_jacobian(&v_xp, &v_xm, &v_yp, &v_ym, &v_zp, &v_zm, h);
        assert!((jac.dvx.x - 2.0).abs() < 1e-8);
        assert!((jac.dvy.y - 3.0).abs() < 1e-8);
        assert!((jac.dvz.z - 1.0).abs() < 1e-8);
        assert!((jac.divergence() - 6.0).abs() < 1e-8);
        assert!(jac.curl().magnitude() < 1e-8);
    }
}

// ATOM: reson8-wave lib.rs v0.2.0 | Sprint 2 | Coherence: 0.99
// Extensions: curl detector, divergence detector, Viviani deviation, Φ score
