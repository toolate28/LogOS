//! C₁₂ group averaging on hexaflake discretization (12-fold rotational invariance).
//! ATOM: C12-GROUP-AVERAGING-20260709 | α + ω = 15

use std::collections::HashSet;

use crate::core::hexaflake::hexaflake_nodes;
use crate::traits::TilingStrategy;

/// Rotate a hexagonal lattice point by `k * 30°` (C₁₂ action).
#[inline]
pub fn rotate_hex_point_30deg((x, y): (i32, i32), k: usize) -> (i32, i32) {
    let angle = (k as f64) * std::f64::consts::PI / 6.0;
    let (cos_a, sin_a) = (angle.cos(), angle.sin());
    let xr = (x as f64 * cos_a - y as f64 * sin_a).round() as i32;
    let yr = (x as f64 * sin_a + y as f64 * cos_a).round() as i32;
    (xr, yr)
}

/// Full C₁₂ orbit of a lattice point (deduplicated).
pub fn c12_orbit(point: (i32, i32)) -> Vec<(i32, i32)> {
    let mut orbit = Vec::with_capacity(12);
    let mut seen = HashSet::new();
    for k in 0..12 {
        let rotated = rotate_hex_point_30deg(point, k);
        if seen.insert(rotated) {
            orbit.push(rotated);
        }
    }
    orbit
}

/// Base 6-fold hexaflake nodes closed under C₁₂ rotations.
pub fn c12_hexaflake_nodes(radius: u32) -> Vec<(i32, i32)> {
    let base = hexaflake_nodes(radius);
    let mut full: Vec<(i32, i32)> = Vec::new();
    let mut seen = HashSet::new();
    for &(x, y) in &base {
        for k in 0..12 {
            let p = rotate_hex_point_30deg((x, y), k);
            if seen.insert(p) {
                full.push(p);
            }
        }
    }
    full.sort_unstable();
    full.dedup();
    full
}

/// Average a scalar field over the C₁₂ orbit of `point`.
pub fn c12_group_average<F>(point: (i32, i32), value_fn: F) -> f32
where
    F: Fn((i32, i32)) -> f32,
{
    let orbit = c12_orbit(point);
    if orbit.is_empty() {
        return 0.0;
    }
    let sum: f32 = orbit.iter().map(|&p| value_fn(p)).sum();
    sum / orbit.len() as f32
}

/// Average a pre-computed harmony field over the C₁₂ orbit.
pub fn c12_average_harmony(point: (i32, i32), harmony_field: &[((i32, i32), f32)]) -> f32 {
    c12_group_average(point, |p| {
        harmony_field
            .iter()
            .find(|&&(coord, _)| coord == p)
            .map(|&(_, v)| v)
            .unwrap_or(0.0)
    })
}

/// Rotationally invariant harmony contribution gated by Tomczak + WAVE floor.
pub fn c12_harmony_contribution(
    point: (i32, i32),
    raw_harmony: f32,
    tomczak_preserved: bool,
    wave_score: f32,
) -> f32 {
    if !tomczak_preserved || wave_score < 0.85 {
        return 0.0;
    }
    let averaged = c12_group_average(point, |_| raw_harmony);
    if averaged > 0.6 {
        averaged.powf(1.8)
    } else {
        averaged * 0.3
    }
}

/// C₁₂-aware tile sizing (multiples of 12 in high-symmetry regions).
#[derive(Debug, Default, Clone, Copy)]
pub struct C12HexaflakeTiler;

impl TilingStrategy for C12HexaflakeTiler {
    fn recommended_tile_size(&self, problem_size: usize) -> usize {
        let base = if problem_size < 256 { 64 } else { 256 };
        ((base + 11) / 12) * 12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c12_orbit_has_at_most_12_points() {
        assert!(c12_orbit((3, 0)).len() <= 12);
    }

    #[test]
    fn c12_hexaflake_grows_with_radius() {
        assert!(c12_hexaflake_nodes(2).len() >= hexaflake_nodes(2).len());
    }

    #[test]
    fn c12_group_average_is_rotationally_invariant() {
        let value_fn = |p: (i32, i32)| (p.0 + p.1) as f32 * 0.1;
        let point = (2, 1);
        let avg1 = c12_group_average(point, value_fn);
        let rotated = rotate_hex_point_30deg(point, 3);
        let avg2 = c12_group_average(rotated, value_fn);
        assert!((avg1 - avg2).abs() < 1e-5);
    }

    #[test]
    fn c12_harmony_contribution_respects_tomczak_gate() {
        assert_eq!(c12_harmony_contribution((1, 0), 0.9, false, 0.95), 0.0);
    }

    #[test]
    fn c12_tiler_rounds_to_multiple_of_12() {
        let tiler = C12HexaflakeTiler;
        assert_eq!(tiler.recommended_tile_size(300) % 12, 0);
    }
}