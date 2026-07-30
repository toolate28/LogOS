//! Persistent homology — H0 intervals via union-find.
//!
//! H1 is a stub — see `TODO(gemini)` below.
//!
//! ## Invariant
//! For any n-point cloud with k connected components at ε_max, the barcode
//! output contains exactly n H0 intervals, of which k have `death == eps_max`
//! (or `f64::INFINITY` if the caller passes it).

use crate::vr::Edge;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Barcode {
    pub birth: f64,
    pub death: f64,
    pub dim: u8,
}

/// Compute H0 persistent homology barcodes via the standard union-find
/// algorithm over a distance-sorted edge stream.
///
/// # Arguments
/// - `n`        : number of points
/// - `edges`    : pairwise distances, **pre-sorted ascending** by distance
/// - `eps_max`  : treat surviving components as dying at this value
///                (use `f64::INFINITY` for true ∞ semantics)
pub fn compute_h0(n: usize, edges: &[Edge], eps_max: f64) -> Vec<Barcode> {
    if n == 0 {
        return vec![];
    }
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank: Vec<u8> = vec![0; n];
    // Every component is born at ε=0 and defaults to surviving to eps_max.
    let mut death: Vec<f64> = vec![eps_max; n];

    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]]; // path compression
            x = parent[x];
        }
        x
    }

    for &(u, v, d) in edges {
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru == rv {
            continue;
        }
        // Union by rank: the younger (smaller rank) root dies at ε=d.
        // Ties broken by index so the result is deterministic.
        let (dying, surviving) = match rank[ru].cmp(&rank[rv]) {
            std::cmp::Ordering::Less => (ru, rv),
            std::cmp::Ordering::Greater => (rv, ru),
            std::cmp::Ordering::Equal => {
                if ru < rv {
                    (rv, ru)
                } else {
                    (ru, rv)
                }
            }
        };
        parent[dying] = surviving;
        if rank[dying] == rank[surviving] {
            rank[surviving] = rank[surviving].saturating_add(1);
        }
        death[dying] = d;
    }

    (0..n)
        .map(|i| Barcode {
            birth: 0.0,
            death: death[i],
            dim: 0,
        })
        .collect()
}

/// H1 stub — 1D boundary-matrix reduction.
///
/// TODO(gemini): implement standard left-to-right column reduction with
/// low-pivot tracking over a sparse boundary matrix. Return 1D Barcode
/// intervals. This is the Scale-strand lift.
pub fn compute_h1(_n: usize, _edges: &[Edge], _eps_max: f64) -> Vec<Barcode> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point_cloud, vr};

    #[test]
    fn h0_count_equals_point_count() {
        let pts = point_cloud::generate(point_cloud::Cloud::Circle, 20);
        let es = vr::edges(&pts);
        let eps_max = es.last().map(|e| e.2).unwrap_or(0.0);
        let bars = compute_h0(pts.len(), &es, eps_max);
        assert_eq!(bars.len(), 20, "H0 count must equal point count");
    }

    #[test]
    fn circle_has_exactly_one_persistent_component() {
        let pts = point_cloud::generate(point_cloud::Cloud::Circle, 12);
        let es = vr::edges(&pts);
        let eps_max = es.last().unwrap().2;
        let bars = compute_h0(pts.len(), &es, eps_max);
        let persistent = bars.iter().filter(|b| b.death >= eps_max).count();
        assert_eq!(persistent, 1, "unit circle is connected → 1 persistent bar");
    }

    #[test]
    fn two_blobs_have_two_persistent_components_below_gap() {
        let pts = point_cloud::generate(point_cloud::Cloud::TwoBlobs, 20);
        let es = vr::edges(&pts);
        // Cut off ε before the inter-blob bridge: within-blob max ≈ 0.6,
        // inter-blob min ≈ 2.4. Pick 1.0.
        let eps_cut = 1.0;
        let es_cut: Vec<_> = es.iter().copied().take_while(|e| e.2 <= eps_cut).collect();
        let bars = compute_h0(pts.len(), &es_cut, eps_cut);
        let persistent = bars.iter().filter(|b| b.death >= eps_cut).count();
        assert_eq!(persistent, 2, "two disjoint blobs at ε<gap → 2 persistent bars");
    }
}
