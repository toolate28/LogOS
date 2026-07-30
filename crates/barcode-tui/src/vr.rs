//! Vietoris–Rips edge extraction.
//!
//! For v1 we only materialise 1-simplices (edges). 2-simplices (triangles)
//! are not needed for H0; they will be when H1 lands (`TODO(gemini)` in `ph.rs`).

pub type Edge = (usize, usize, f64);

/// All pairwise edges of a 2D point cloud, sorted ascending by distance.
///
/// Deterministic: identical input → identical output, byte-for-byte.
pub fn edges(points: &[(f64, f64)]) -> Vec<Edge> {
    let n = points.len();
    let mut e = Vec::with_capacity(n * n.saturating_sub(1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].0 - points[j].0;
            let dy = points[i].1 - points[j].1;
            let d = (dx * dx + dy * dy).sqrt();
            e.push((i, j, d));
        }
    }
    // `partial_cmp` yields Some for finite floats; NaN would break ordering
    // but our generators emit only finite coordinates.
    e.sort_by(|a, b| a.2.partial_cmp(&b.2).expect("non-finite distance"));
    e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_edges_sorted() {
        let pts = vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
        let e = edges(&pts);
        assert_eq!(e.len(), 3);
        // Two legs of length 1, one hypotenuse of √2.
        assert!((e[0].2 - 1.0).abs() < 1e-12);
        assert!((e[1].2 - 1.0).abs() < 1e-12);
        assert!((e[2].2 - 2f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn empty_cloud() {
        assert_eq!(edges(&[]).len(), 0);
    }

    #[test]
    fn singleton_cloud() {
        assert_eq!(edges(&[(1.0, 2.0)]).len(), 0);
    }
}
