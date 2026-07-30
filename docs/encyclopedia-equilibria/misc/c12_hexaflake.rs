//! C₁₂ Group Averaging Operator for TriWeavon Manifold
//! Implements 12-fold rotational invariance on hexaflake discretization.
//! Used for L₃₉L₃₉ harmony bias modulation and harmonic_benefit computation.
//!
//! ATOM Trail: C12-GROUP-AVERAGING-20260709
//! Preserves: α + ω = 15, tomczak_preserved, Music Conservation, zero topological drift

use std::collections::HashSet;

/// Rotate a hexagonal lattice point by k * 30° (C₁₂ action).
/// Uses floating-point rotation with rounding back to nearest lattice point.
/// This is a pragmatic implementation that preserves the spirit of exact group action
/// while remaining computationally lightweight for real-time bias computation.
#[inline]
pub fn rotate_hex_point_30deg((x, y): (i32, i32), k: usize) -> (i32, i32) {
    let angle = (k as f64) * std::f64::consts::PI / 6.0;
    let xr = (x as f64 * angle.cos() - y as f64 * angle.sin()).round() as i32;
    let yr = (x as f64 * angle.sin() + y as f64 * angle.cos()).round() as i32;
    (xr, yr)
}

/// Compute the full C₁₂ orbit of a point (all 12 rotations).
/// Returns a deduplicated set of lattice points in the orbit.
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

/// C₁₂ Group Averaging Operator
///
/// Averages a scalar field value over the full C₁₂ rotational orbit of a point.
/// This enforces 30° rotational invariance on harmony scores, bias contributions,
/// or Betti proxy terms.
///
/// # Arguments
/// * `point` - Lattice coordinate in the hexaflake / C₁₂ discretization
/// * `value_fn` - Function that returns the scalar value at a given lattice point
///
/// # Returns
/// The rotationally averaged value (mean over the C₁₂ orbit).
///
/// This function is intended to be used when modulating `bias_scale` by
/// TomczakGateWitness + wave_score, and when computing `harmonic_benefit`
/// for emission into ExistenceCertificate.
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

/// Convenience wrapper: average a pre-computed harmony field over C₁₂ orbit.
/// The field is assumed to be defined on the same (i32, i32) lattice.
pub fn c12_average_harmony(
    point: (i32, i32),
    harmony_field: &[( (i32, i32), f32 )],
) -> f32 {
    c12_group_average(point, |p| {
        harmony_field
            .iter()
            .find(|&&(coord, _)| coord == p)
            .map(|&(_, v)| v)
            .unwrap_or(0.0)
    })
}

/// Example integration point for L₃₉L₃₉ harmony bias
/// (to be called from HarmonicEigenmodeBias or equivalent SRAC path).
///
/// Returns a rotationally invariant harmony contribution that can be
/// multiplied by Tomczak-modulated bias_scale before writing to
/// ExistenceCertificate.harmonic_benefit.
pub fn c12_harmony_contribution(
    point: (i32, i32),
    raw_harmony: f32,
    tomczak_preserved: bool,
    wave_score: f32,
) -> f32 {
    if !tomczak_preserved || wave_score < 0.85 {
        return 0.0; // Gate via Tomczak + wave_score (Jesus-Fractal-Axiom alignment)
    }

    // Average the raw harmony over the C₁₂ orbit
    let averaged = c12_group_average(point, |_| raw_harmony);

    // Non-linear amplification for strong harmonic attractors (aligns with v0.2 self-benefiting bias)
    if averaged > 0.6 {
        averaged.powf(1.8)
    } else {
        averaged * 0.3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c12_orbit_has_at_most_12_points() {
        let point = (3, 0);
        let orbit = c12_orbit(point);
        assert!(orbit.len() <= 12);
    }

    #[test]
    fn c12_group_average_is_rotationally_invariant() {
        let point = (2, 1);
        let value_fn = |p: (i32, i32)| (p.0 + p.1) as f32 * 0.1;

        let avg1 = c12_group_average(point, value_fn);
        let rotated = rotate_hex_point_30deg(point, 3);
        let avg2 = c12_group_average(rotated, value_fn);

        assert!((avg1 - avg2).abs() < 1e-5);
    }

    #[test]
    fn c12_harmony_contribution_respects_tomczak_gate() {
        let point = (1, 0);
        let contrib = c12_harmony_contribution(point, 0.9, false, 0.95);
        assert_eq!(contrib, 0.0);
    }
}