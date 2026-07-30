//! cqk-wdbi — Weighted-Distance Braid Index
//!
//! Persistent homology pipeline for the cQ-kitty-rips architecture.
//! Computes Vietoris-Rips filtrations and extracts Betti numbers (β₀, β₁, β₂)
//! as topological invariants of the fluid state.
//!
//! β₀ = connected vortex clusters
//! β₁ = stable, non-contractible vortex rings
//! β₂ = 2D enclosed voids → SINGULARITY INDICATOR
//!
//! A sudden β₂ surge correlates with ρ = log‖ω̃‖ spike on the cosphere fibre.
//! This is Hook HM from the execution brief §6.3.
//!
//! References:
//!   Brief §6.3: Hook HM — Topology cross-check
//!   arXiv:2604.08105 (direction-aware TDA — Grok battery-materials pulse)

use reson8_core::{VoidClass, WaveScore};
use serde::{Deserialize, Serialize};

// ── Persistence Pair ────────────────────────────────────────────────

/// A persistence pair (birth, death) in the filtration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PersistencePair {
    pub dimension: usize,
    pub birth: f64,
    pub death: f64,
}

impl PersistencePair {
    pub fn persistence(&self) -> f64 { self.death - self.birth }
    pub fn midpoint(&self) -> f64 { (self.birth + self.death) / 2.0 }

    pub fn void_class(&self) -> VoidClass {
        VoidClass::from_persistence(self.persistence())
    }
}

// ── Persistence Diagram ─────────────────────────────────────────────

/// A full persistence diagram: collection of persistence pairs by dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceDiagram {
    pub pairs: Vec<PersistencePair>,
}

impl PersistenceDiagram {
    pub fn new() -> Self { Self { pairs: Vec::new() } }

    /// Add a persistence pair.
    pub fn add(&mut self, dimension: usize, birth: f64, death: f64) {
        self.pairs.push(PersistencePair { dimension, birth, death });
    }

    /// Extract pairs of a given dimension.
    pub fn dimension(&self, d: usize) -> Vec<&PersistencePair> {
        self.pairs.iter().filter(|p| p.dimension == d).collect()
    }

    /// Count pairs above a persistence threshold for a given dimension.
    pub fn count_significant(&self, dim: usize, threshold: f64) -> usize {
        self.dimension(dim).iter().filter(|p| p.persistence() > threshold).count()
    }
}

// ── Betti Numbers ───────────────────────────────────────────────────

/// Betti number tuple (β₀, β₁, β₂) at a given filtration value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct BettiTuple {
    pub beta_0: usize,
    pub beta_1: usize,
    pub beta_2: usize,
}

impl BettiTuple {
    /// Detect a β₂-spike: more than `threshold` significant 2-voids.
    pub fn beta_2_spike(&self, threshold: usize) -> bool {
        self.beta_2 > threshold
    }

    /// Total topological complexity.
    pub fn total(&self) -> usize {
        self.beta_0 + self.beta_1 + self.beta_2
    }
}

/// Extract Betti numbers from a persistence diagram at a given filtration value.
pub fn betti_at(diagram: &PersistenceDiagram, filtration_value: f64) -> BettiTuple {
    let mut beta = [0usize; 3];
    for pair in &diagram.pairs {
        if pair.dimension < 3 && pair.birth <= filtration_value && pair.death > filtration_value {
            beta[pair.dimension] += 1;
        }
    }
    BettiTuple { beta_0: beta[0], beta_1: beta[1], beta_2: beta[2] }
}

// ── Direction-Aware Filtration ──────────────────────────────────────

/// Direction-aware filtration function for anisotropic structures.
/// Extends standard Vietoris-Rips with cone-based + PCA-based directional
/// weighting (per arXiv:2604.08105, Grok battery-materials pulse).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionalFiltration {
    /// Principal loading/flow direction (unit vector in ℝ³).
    pub direction: [f64; 3],
    /// Cone half-angle in radians for directional weighting.
    pub cone_angle: f64,
    /// Weighting factor for in-cone vs out-of-cone edges.
    pub anisotropy_weight: f64,
}

impl DirectionalFiltration {
    /// Compute the directional weight for an edge vector.
    /// In-cone edges get weight 1.0, out-of-cone get anisotropy_weight.
    pub fn edge_weight(&self, edge_vec: &[f64; 3]) -> f64 {
        let norm = (edge_vec[0].powi(2) + edge_vec[1].powi(2) + edge_vec[2].powi(2)).sqrt();
        if norm < 1e-15 { return 1.0; }
        let dot: f64 = edge_vec.iter().zip(self.direction.iter()).map(|(a, b)| a * b).sum();
        let cos_theta = (dot / norm).abs();
        if cos_theta >= self.cone_angle.cos() {
            1.0
        } else {
            self.anisotropy_weight
        }
    }
}

// ── Distance Matrix (Ripser++ interface) ────────────────────────────

/// Compressed lower-triangular distance matrix for Vietoris-Rips.
/// This is the format expected by Ripser++ / GPU Ripser.
#[derive(Debug, Clone)]
pub struct DistanceMatrix {
    /// Lower-triangular entries in row-major order.
    pub data: Vec<f64>,
    /// Number of points.
    pub n: usize,
}

impl DistanceMatrix {
    pub fn zeros(n: usize) -> Self {
        Self { data: vec![0.0; n * (n - 1) / 2], n }
    }

    /// Index into the lower-triangular storage.
    fn idx(i: usize, j: usize) -> usize {
        let (i, j) = if i > j { (i, j) } else { (j, i) };
        i * (i - 1) / 2 + j
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        if i == j { 0.0 } else { self.data[Self::idx(i, j)] }
    }

    pub fn set(&mut self, i: usize, j: usize, val: f64) {
        if i != j { self.data[Self::idx(i, j)] = val; }
    }
}

// ── Hook HM: Topology Cross-Check ──────────────────────────────────

/// Hook HM result: cross-checks topology against entropy readings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyCrossCheck {
    pub betti: BettiTuple,
    pub beta_2_spike: bool,
    pub max_persistence_dim2: f64,
    /// If HM and HA disagree → mesh is inadequate, not victory.
    pub mesh_refinement_needed: bool,
}

/// Run Hook HM: compute Betti numbers and cross-check against entropy.
pub fn hook_hm(
    diagram: &PersistenceDiagram,
    filtration_value: f64,
    entropy_case_a_holds: bool,
    beta_2_threshold: usize,
) -> TopologyCrossCheck {
    let betti = betti_at(diagram, filtration_value);
    let spike = betti.beta_2_spike(beta_2_threshold);

    let max_p2 = diagram.dimension(2)
        .iter()
        .map(|p| p.persistence())
        .fold(0.0f64, f64::max);

    // If β₂ surges while Case-A holds → mesh inadequacy, not physics
    let mesh_refinement_needed = spike && entropy_case_a_holds;

    TopologyCrossCheck {
        betti,
        beta_2_spike: spike,
        max_persistence_dim2: max_p2,
        mesh_refinement_needed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistence_pair_basics() {
        let p = PersistencePair { dimension: 1, birth: 0.1, death: 0.5 };
        assert!((p.persistence() - 0.4).abs() < 1e-12);
        assert!((p.midpoint() - 0.3).abs() < 1e-12);
    }

    #[test]
    fn betti_extraction() {
        let mut diag = PersistenceDiagram::new();
        diag.add(0, 0.0, 1.0);
        diag.add(0, 0.0, 0.5);
        diag.add(1, 0.2, 0.8);
        diag.add(2, 0.3, 0.9);
        let betti = betti_at(&diag, 0.4);
        assert_eq!(betti, BettiTuple { beta_0: 2, beta_1: 1, beta_2: 1 });
    }

    #[test]
    fn beta_2_spike_detection() {
        let betti = BettiTuple { beta_0: 5, beta_1: 3, beta_2: 10 };
        assert!(betti.beta_2_spike(5));
        assert!(!betti.beta_2_spike(15));
    }

    #[test]
    fn distance_matrix_symmetry() {
        let mut dm = DistanceMatrix::zeros(4);
        dm.set(2, 0, 3.14);
        assert!((dm.get(0, 2) - 3.14).abs() < 1e-12);
        assert!((dm.get(2, 0) - 3.14).abs() < 1e-12);
    }

    #[test]
    fn directional_filtration_in_cone() {
        let filt = DirectionalFiltration {
            direction: [0.0, 0.0, 1.0],
            cone_angle: 0.5,    // ~29 degrees
            anisotropy_weight: 0.3,
        };
        // Edge along z-axis should be in-cone
        assert!((filt.edge_weight(&[0.0, 0.0, 1.0]) - 1.0).abs() < 1e-12);
        // Edge perpendicular to z should be out-of-cone
        assert!((filt.edge_weight(&[1.0, 0.0, 0.0]) - 0.3).abs() < 1e-12);
    }
}
