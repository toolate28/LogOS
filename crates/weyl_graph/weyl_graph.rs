//! # THTW Weyl Graph
//!
//! Directed pregeometric 2→2 recombination lattice with dynamical Weyl gauge
//! field ω_e on edges. Exact translation of @Akitti's THTW_WeylGraph.
//!
//! ## Architecture
//!
//! Each edge carries:
//! - ω_e = (1/a) ln Σ_e  — the discrete Weyl gauge field
//! - Σ_e = exp(a · ω_e)   — the parallel transport scale factor
//! - D_e ≈ 0.23           — MandelbulbFoam fractal dimension modulation
//!
//! ## Rigor Categories
//!
//! - Edge payload structure: Category B (documented from @Akitti's formulation)
//! - Parallel transport formula: Category A (standard gauge theory)
//! - Fractal dimension D_e ≈ 0.23: Category D (unanchored — from MandelbulbFoam)

use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use cQ_kitty_rips_ga::Cl13;

/// Payload carried by each edge of the Weyl graph.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EdgePayload {
    /// Discrete Weyl gauge field: ω_e = (1/a) ln Σ_e
    pub omega_e: f64,

    /// Lattice spacing (default 1.0 for unit lattice)
    pub lattice_spacing: f64,

    /// Displacement vector components [Δx⁰, Δx¹, Δx², Δx³]
    pub delta_x: [f64; 4],

    /// Fractal dimension modulation from MandelbulbFoam
    /// Category D: unanchored, empirical value ≈ 0.23
    pub fractal_d_e: f64,
}

impl EdgePayload {
    /// Compute the scale factor Σ_e = exp(a · ω_e)
    #[inline]
    pub fn sigma_e(&self) -> f64 {
        (self.lattice_spacing * self.omega_e).exp()
    }

    /// Create a new edge with given gauge field value
    pub fn new(omega_e: f64, delta_x: [f64; 4]) -> Self {
        EdgePayload {
            omega_e,
            lattice_spacing: 1.0,
            delta_x,
            fractal_d_e: 0.23, // Category D default
        }
    }
}

/// Node payload — minimal for now, extensible.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NodePayload {
    /// Position in the embedding space (if any)
    pub position: Option<[f64; 4]>,
}

/// The THTW Weyl Graph: a directed graph with gauge fields on edges.
#[derive(Debug)]
pub struct TTHTWWeylGraph {
    /// The underlying directed graph
    pub graph: DiGraph<NodePayload, EdgePayload>,

    /// Global fractal dimension parameter (MandelbulbFoam)
    /// Category D: unanchored
    pub global_fractal_d: f64,
}

impl TTHTWWeylGraph {
    /// Create an empty THTW graph with the given fractal dimension.
    pub fn new(fractal_d_e: f64) -> Self {
        TTHTWWeylGraph {
            graph: DiGraph::new(),
            global_fractal_d: fractal_d_e,
        }
    }

    /// Create with default fractal dimension D_e ≈ 0.23
    pub fn default_fractal() -> Self {
        Self::new(0.23)
    }

    /// Add a node to the graph.
    pub fn add_node(&mut self, payload: NodePayload) -> NodeIndex {
        self.graph.add_node(payload)
    }

    /// Add a directed edge with a gauge field value.
    pub fn add_edge(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        omega_e: f64,
        delta_x: [f64; 4],
    ) -> EdgeIndex {
        let mut payload = EdgePayload::new(omega_e, delta_x);
        payload.fractal_d_e = self.global_fractal_d;
        self.graph.add_edge(from, to, payload)
    }

    // ── Gauge operations ─────────────────────────────────────────

    /// Parallel transport of a multivector along an edge.
    ///
    /// T → Σ_e^{q_T} · T
    ///
    /// Category A: standard gauge theory formula.
    #[inline]
    pub fn parallel_transport(&self, t: &Cl13, edge: EdgeIndex, q_t: f64) -> Cl13 {
        let payload = &self.graph[edge];
        let scale = payload.sigma_e().powf(q_t);
        *t * scale
    }

    /// Discrete non-metricity along an edge.
    ///
    /// Δ‖T‖² = −2 ω_e ‖T‖²
    ///
    /// This measures how much the norm changes under parallel transport.
    #[inline]
    pub fn discrete_non_metricity(&self, norm_t_sq: f64, edge: EdgeIndex) -> f64 {
        let omega = self.graph[edge].omega_e;
        -2.0 * omega * norm_t_sq
    }

    /// Plaquette holonomy: F̂_plaq = ln(∏ Σ_e) = Σ a · ω_e around the loop.
    ///
    /// For a flat connection, this should be zero.
    /// Non-zero holonomy indicates curvature (field strength).
    pub fn plaquette_holonomy(&self, edges: &[EdgeIndex]) -> f64 {
        edges.iter()
            .map(|&e| {
                let p = &self.graph[e];
                p.lattice_spacing * p.omega_e
            })
            .sum()
    }

    /// Check if a loop has zero holonomy (flat connection test).
    pub fn is_flat_loop(&self, edges: &[EdgeIndex], eps: f64) -> bool {
        self.plaquette_holonomy(edges).abs() < eps
    }

    // ── Graph queries ────────────────────────────────────────────

    /// Number of nodes
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Number of edges
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Get all outgoing edges from a node
    pub fn outgoing_edges(&self, node: NodeIndex) -> Vec<EdgeIndex> {
        self.graph.edges_directed(node, Direction::Outgoing)
            .map(|e| e.id())
            .collect()
    }

    /// Total gauge field magnitude (L1 norm over all edges)
    pub fn total_gauge_magnitude(&self) -> f64 {
        self.graph.edge_weights()
            .map(|e| e.omega_e.abs())
            .sum()
    }

    // ── Spectral gap (adjacency Laplacian) ───────────────────────

    /// Compute the spectral gap of the graph Laplacian.
    ///
    /// The spectral gap λ₁ (smallest nonzero eigenvalue) determines
    /// the convergence rate of diffusion on the graph.
    /// For Berry monopole protection, λ₁ must be positive.
    ///
    /// Uses power iteration on L = D − A (unnormalised Laplacian).
    /// Returns None if the graph is too small.
    pub fn spectral_gap_estimate(&self, max_iter: usize) -> Option<f64> {
        let n = self.node_count();
        if n < 3 { return None; }

        // Build adjacency matrix (dense — only for small graphs)
        let mut adj = vec![vec![0.0f64; n]; n];
        for edge in self.graph.edge_indices() {
            let (src, dst) = self.graph.edge_endpoints(edge).unwrap();
            adj[src.index()][dst.index()] = 1.0;
            adj[dst.index()][src.index()] = 1.0; // symmetrise for Laplacian
        }

        // Degree matrix
        let degree: Vec<f64> = (0..n).map(|i| adj[i].iter().sum()).collect();

        // Laplacian L = D − A applied to vector
        let lap_mul = |v: &[f64]| -> Vec<f64> {
            (0..n).map(|i| {
                degree[i] * v[i] - adj[i].iter().zip(v.iter()).map(|(a, x)| a * x).sum::<f64>()
            }).collect()
        };

        // Power iteration for smallest nonzero eigenvalue
        // (deflate by the constant eigenvector first)
        let mut v: Vec<f64> = (0..n).map(|i| (i as f64 + 1.0).sin()).collect();
        let norm_const = 1.0 / (n as f64).sqrt();

        for _ in 0..max_iter {
            // Project out constant eigenvector
            let proj: f64 = v.iter().sum::<f64>() * norm_const;
            for x in v.iter_mut() { *x -= proj * norm_const; }

            // Apply Laplacian
            let lv = lap_mul(&v);

            // Normalise
            let norm: f64 = lv.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-15 { return Some(0.0); }
            v = lv.into_iter().map(|x| x / norm).collect();
        }

        // Rayleigh quotient
        let lv = lap_mul(&v);
        let numerator: f64 = v.iter().zip(lv.iter()).map(|(a, b)| a * b).sum();
        let denominator: f64 = v.iter().map(|x| x * x).sum();
        Some(numerator / denominator)
    }
}

// ──────────────────────────────────────────────────────────────────
// MandelbulbFoam fractal modulation
// ──────────────────────────────────────────────────────────────────

/// MandelbulbFoam fractal dimension modulation.
///
/// Category D: unanchored. The D_e ≈ 0.23 value comes from @Akitti's
/// MandelbulbFoam simulations. It modulates edge weights to create
/// fractal self-similar structure in the pregeometric graph.
pub fn mandelbulb_modulate(omega_e: f64, fractal_d: f64, iteration: u32) -> f64 {
    // Power-8 Mandelbulb recursion on the edge scalar
    let base = omega_e.abs().max(1e-15);
    let modulated = base.powf(8.0 * fractal_d) * (1.0 + (-0.1 * iteration as f64).exp());
    modulated.copysign(omega_e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigma_e() {
        let e = EdgePayload::new(0.0, [0.0; 4]);
        assert!((e.sigma_e() - 1.0).abs() < 1e-12, "Σ_e(ω=0) should be 1");
    }

    #[test]
    fn test_flat_transport() {
        let mut g = TTHTWWeylGraph::default_fractal();
        let a = g.add_node(NodePayload::default());
        let b = g.add_node(NodePayload::default());
        let e = g.add_edge(a, b, 0.0, [1.0, 0.0, 0.0, 0.0]);

        let v = Cl13::e1();
        let transported = g.parallel_transport(&v, e, 1.0);
        assert!(transported.approx_eq(&v, 1e-12), "Flat transport should preserve");
    }

    #[test]
    fn test_non_metricity_sign() {
        let mut g = TTHTWWeylGraph::default_fractal();
        let a = g.add_node(NodePayload::default());
        let b = g.add_node(NodePayload::default());
        let e = g.add_edge(a, b, 0.5, [1.0, 0.0, 0.0, 0.0]);

        let nm = g.discrete_non_metricity(1.0, e);
        assert!(nm < 0.0, "Positive ω should give negative non-metricity");
    }

    #[test]
    fn test_plaquette_flat() {
        let mut g = TTHTWWeylGraph::default_fractal();
        let a = g.add_node(NodePayload::default());
        let b = g.add_node(NodePayload::default());
        let c = g.add_node(NodePayload::default());
        let e1 = g.add_edge(a, b, 0.3, [1.0, 0.0, 0.0, 0.0]);
        let e2 = g.add_edge(b, c, 0.2, [0.0, 1.0, 0.0, 0.0]);
        let e3 = g.add_edge(c, a, -0.5, [-1.0, -1.0, 0.0, 0.0]);

        // 0.3 + 0.2 + (-0.5) = 0.0 — flat
        assert!(g.is_flat_loop(&[e1, e2, e3], 1e-12));
    }
}
