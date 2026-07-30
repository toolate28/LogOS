//! Deterministic point-cloud generators.
//!
//! No RNG. Every cloud is a closed-form function of `n` (and shape params)
//! so output is reproducible bit-for-bit.

use clap::ValueEnum;
use std::f64::consts::TAU;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Cloud {
    /// n points evenly on a unit circle — exactly 1 persistent H0 bar.
    Circle,
    /// Two disjoint clusters of n/2 points — 2 persistent H0 bars until
    /// ε exceeds the inter-cluster gap.
    TwoBlobs,
    /// Uniform √n × √n lattice — stress test.
    Grid,
}

/// Generate a point cloud of the requested shape. Deterministic.
pub fn generate(cloud: Cloud, n: usize) -> Vec<(f64, f64)> {
    match cloud {
        Cloud::Circle => (0..n)
            .map(|i| {
                let theta = TAU * (i as f64) / (n as f64);
                (theta.cos(), theta.sin())
            })
            .collect(),

        Cloud::TwoBlobs => {
            // Half around (-1.5, 0), half around (+1.5, 0), each on a
            // small circle of radius 0.3.
            let half = n / 2;
            let right = n - half;
            let mut pts = Vec::with_capacity(n);
            for i in 0..half {
                let t = TAU * (i as f64) / (half.max(1) as f64);
                pts.push((-1.5 + 0.3 * t.cos(), 0.3 * t.sin()));
            }
            for i in 0..right {
                let t = TAU * (i as f64) / (right.max(1) as f64);
                pts.push((1.5 + 0.3 * t.cos(), 0.3 * t.sin()));
            }
            pts
        }

        Cloud::Grid => {
            let s = (n as f64).sqrt().ceil() as usize;
            let span = if s > 1 { 2.0 / (s - 1) as f64 } else { 0.0 };
            (0..n)
                .map(|i| {
                    let col = i % s;
                    let row = i / s;
                    (-1.0 + col as f64 * span, -1.0 + row as f64 * span)
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_has_n_points_on_unit_circle() {
        let pts = generate(Cloud::Circle, 12);
        assert_eq!(pts.len(), 12);
        for (x, y) in pts {
            let r = (x * x + y * y).sqrt();
            assert!((r - 1.0).abs() < 1e-12, "point off unit circle: r={}", r);
        }
    }

    #[test]
    fn two_blobs_straddle_origin() {
        let pts = generate(Cloud::TwoBlobs, 20);
        assert_eq!(pts.len(), 20);
        let left = pts.iter().filter(|(x, _)| *x < 0.0).count();
        let right = pts.iter().filter(|(x, _)| *x > 0.0).count();
        assert_eq!(left, 10);
        assert_eq!(right, 10);
    }

    #[test]
    fn deterministic_bitwise() {
        let a = generate(Cloud::Circle, 24);
        let b = generate(Cloud::Circle, 24);
        assert_eq!(a, b, "circle generator is not deterministic");
    }
}
