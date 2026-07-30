//! cqk-microlocal — Cosphere-Bundle Lift for Navier-Stokes Vorticity
//!
//! Lifts the vorticity 2-form ω from the base manifold M (ℝ³) to the
//! cosphere bundle S*M. The log-amplitude ρ = log‖ω̃‖_{g'} and its
//! perpendicular gradient d_⊥ρ are the primary observables for the
//! microlocal entropy functional W[ω̃].
//!
//! References:
//!   ⟦H-NS1⟧ arXiv:2601.08854v3 (braid-provisional)
//!   Brief §1.2: Working hypothesis — microlocal lift
//!   Brief §3.2: CosphereLift type

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

// ── Mesh Types ──────────────────────────────────────────────────────

/// Reference to the base computational mesh (DNS grid).
/// The mesh itself lives in GPU memory; this handle indexes into it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseMeshRef {
    /// Number of grid points per spatial dimension.
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    /// Physical domain extents.
    pub lx: f64,
    pub ly: f64,
    pub lz: f64,
}

impl BaseMeshRef {
    pub fn total_points(&self) -> usize { self.nx * self.ny * self.nz }
    pub fn dx(&self) -> f64 { self.lx / self.nx as f64 }
    pub fn dy(&self) -> f64 { self.ly / self.ny as f64 }
    pub fn dz(&self) -> f64 { self.lz / self.nz as f64 }
}

// ── Vorticity Buffer ────────────────────────────────────────────────

/// Buffer holding the vorticity 2-form components at each mesh point.
/// ω = (ω_x, ω_y, ω_z) = ∇ × u
#[derive(Debug, Clone)]
pub struct VorticityField {
    /// Flat buffer: 3 components × total_points, row-major.
    pub data: Vec<f64>,
    pub num_points: usize,
}

impl VorticityField {
    pub fn zeros(num_points: usize) -> Self {
        Self {
            data: vec![0.0; 3 * num_points],
            num_points,
        }
    }

    /// Get the vorticity vector at grid point `i`.
    pub fn at(&self, i: usize) -> [f64; 3] {
        let base = 3 * i;
        [self.data[base], self.data[base + 1], self.data[base + 2]]
    }

    /// Magnitude |ω(x_i)| at grid point i.
    pub fn magnitude_at(&self, i: usize) -> f64 {
        let w = self.at(i);
        (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt()
    }
}

// ── Cosphere Lift ───────────────────────────────────────────────────

/// Cosphere-bundle lift of a velocity field.
/// `D` = spatial dimension (3 for NS), `N` = angular samples per fibre.
/// Total DOF = N × mesh_size.
///
/// The cosphere bundle S*M at a point x ∈ M is the unit sphere in T*_x M.
/// We sample it with N discrete directions (fibre_angles).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosphereLift<const D: usize, const N: usize> {
    /// Reference to the underlying DNS mesh.
    pub base_mesh: BaseMeshRef,
    /// Angular sample directions on the unit (D-1)-sphere.
    /// Each entry is a unit covector direction ξ ∈ S^{D-1}.
    #[serde(with = "BigArray")]
    pub fibre_angles: [[f64; D]; N],
    /// Lifted vorticity magnitude ‖ω̃(x,ξ)‖_{g'} at each (mesh_point, fibre_direction).
    /// Layout: omega_tilde[i * N + j] = magnitude at point i, direction j.
    pub omega_tilde: Vec<f64>,
}

impl<const D: usize, const N: usize> CosphereLift<D, N> {
    /// Total degrees of freedom in the lifted field.
    pub fn total_dof(&self) -> usize {
        self.base_mesh.total_points() * N
    }

    /// Log-amplitude ρ(x,ξ) = log ‖ω̃(x,ξ)‖_{g'}
    /// Returns f64::NEG_INFINITY where ω̃ = 0 (vacuum).
    pub fn rho_at(&self, point: usize, direction: usize) -> f64 {
        let val = self.omega_tilde[point * N + direction];
        if val <= 0.0 {
            f64::NEG_INFINITY
        } else {
            val.ln()
        }
    }

    /// Compute the maximum ρ across all fibre directions at a given point.
    /// This is the "hottest" microlocal direction — the one driving toward blow-up.
    pub fn rho_max_at(&self, point: usize) -> f64 {
        (0..N)
            .map(|j| self.rho_at(point, j))
            .fold(f64::NEG_INFINITY, f64::max)
    }
}

// ── Perpendicular Gradient ──────────────────────────────────────────

/// The perpendicular gradient d_⊥ρ on the cosphere fibre.
/// This is the key observable in the Theorem-4 viscous dissipation term.
#[derive(Debug, Clone)]
pub struct PerpGradient {
    /// ‖d_⊥ρ‖² integrated over S*M (scalar summary).
    pub norm_sq_integrated: f64,
    /// Per-point maximum ‖d_⊥ρ‖² (for β₂-spike detection).
    pub per_point_max: Vec<f64>,
}

/// Compute the perpendicular gradient of ρ on the cosphere fibre.
/// This is a finite-difference approximation on the angular samples.
pub fn compute_perp_gradient<const D: usize, const N: usize>(
    lift: &CosphereLift<D, N>,
) -> PerpGradient {
    let np = lift.base_mesh.total_points();
    let mut per_point_max = vec![0.0f64; np];
    let mut total = 0.0;

    for i in 0..np {
        let mut local_max = 0.0f64;
        for j in 0..N {
            let rho_j = lift.rho_at(i, j);
            if rho_j.is_finite() {
                // Finite difference on the angular ring
                let j_next = (j + 1) % N;
                let rho_next = lift.rho_at(i, j_next);
                if rho_next.is_finite() {
                    let diff_sq = (rho_next - rho_j).powi(2);
                    total += diff_sq;
                    local_max = local_max.max(diff_sq);
                }
            }
        }
        per_point_max[i] = local_max;
    }

    PerpGradient {
        norm_sq_integrated: total,
        per_point_max,
    }
}

// ── Rate-of-Strain Tensor ───────────────────────────────────────────

/// Symmetric rate-of-strain tensor S = ½(∇u + (∇u)ᵀ).
/// Stores the Frobenius norm ‖S‖ at each mesh point.
#[derive(Debug, Clone)]
pub struct StrainRateField {
    /// ‖S(x_i)‖_F at each grid point.
    pub frobenius_norms: Vec<f64>,
}

impl StrainRateField {
    pub fn zeros(num_points: usize) -> Self {
        Self { frobenius_norms: vec![0.0; num_points] }
    }
}

// ── Lift Construction ───────────────────────────────────────────────

/// Construct equi-spaced angular samples on S^{D-1} for D=3.
/// Uses a Fibonacci sphere for near-optimal distribution.
pub fn fibonacci_sphere_directions<const N: usize>() -> [[f64; 3]; N] {
    let golden_angle = std::f64::consts::PI * (3.0 - 5.0_f64.sqrt());
    let mut dirs = [[0.0f64; 3]; N];
    for i in 0..N {
        let y = 1.0 - (2.0 * i as f64) / (N as f64 - 1.0);
        let radius = (1.0 - y * y).sqrt();
        let theta = golden_angle * i as f64;
        dirs[i] = [radius * theta.cos(), y, radius * theta.sin()];
    }
    dirs
}

/// Lift a vorticity field to the cosphere bundle.
/// Projection: ω̃(x,ξ) = |ω(x) · ξ| (directional vorticity magnitude).
pub fn lift_vorticity<const N: usize>(
    mesh: &BaseMeshRef,
    vorticity: &VorticityField,
    fibre_angles: &[[f64; 3]; N],
) -> CosphereLift<3, N> {
    let np = mesh.total_points();
    let mut omega_tilde = vec![0.0f64; np * N];

    for i in 0..np {
        let w = vorticity.at(i);
        for j in 0..N {
            let xi = &fibre_angles[j];
            let dot = w[0] * xi[0] + w[1] * xi[1] + w[2] * xi[2];
            omega_tilde[i * N + j] = dot.abs();
        }
    }

    CosphereLift {
        base_mesh: mesh.clone(),
        fibre_angles: *fibre_angles,
        omega_tilde,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_sphere_unit_vectors() {
        let dirs = fibonacci_sphere_directions::<32>();
        for d in &dirs {
            let norm = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            assert!((norm - 1.0).abs() < 1e-10, "direction not unit: norm={}", norm);
        }
    }

    #[test]
    fn lift_uniform_vorticity() {
        let mesh = BaseMeshRef { nx: 4, ny: 4, nz: 4, lx: 1.0, ly: 1.0, lz: 1.0 };
        let np = mesh.total_points();
        let mut vort = VorticityField::zeros(np);
        // Uniform vorticity in z-direction: ω = (0, 0, 1)
        for i in 0..np {
            vort.data[3 * i + 2] = 1.0;
        }
        let dirs = fibonacci_sphere_directions::<16>();
        let lift = lift_vorticity(&mesh, &vort, &dirs);
        assert_eq!(lift.total_dof(), np * 16);
        // At each point, ω̃(x,ξ) = |ξ_z|, which varies by direction
        assert!(lift.omega_tilde.iter().all(|v| *v >= 0.0));
    }

    #[test]
    fn rho_max_positive_for_nonzero_vorticity() {
        let mesh = BaseMeshRef { nx: 2, ny: 2, nz: 2, lx: 1.0, ly: 1.0, lz: 1.0 };
        let np = mesh.total_points();
        let mut vort = VorticityField::zeros(np);
        vort.data[2] = 10.0; // strong vorticity at point 0
        let dirs = fibonacci_sphere_directions::<8>();
        let lift = lift_vorticity(&mesh, &vort, &dirs);
        assert!(lift.rho_max_at(0) > 0.0);
    }
}
