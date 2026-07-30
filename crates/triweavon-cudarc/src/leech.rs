//! Leech density guidance — Construction A norm-4 vectors → hybrid K22 reduction.

use crate::golay::{golay_derived_norm4_vectors, leech_full_kissing_approximation};
use crate::m24::{K22SheafFragment, ReducedK22Fragment, ReductionError};

pub const KISSING_NUMBER_24D: u32 = 196_560;
pub const LEECH_MIN_NORM: f32 = 4.0;

#[derive(Debug, Clone)]
pub struct LeechDensityConfig {
    pub weight: f32,
    pub norm_threshold: f32,
    pub wave_composite: f32,
    pub use_full_kissing: bool,
}

impl Default for LeechDensityConfig {
    fn default() -> Self {
        Self {
            weight: 0.35,
            norm_threshold: LEECH_MIN_NORM,
            wave_composite: 0.995,
            use_full_kissing: false,
        }
    }
}

impl LeechDensityConfig {
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
        self
    }

    pub fn with_full_kissing(mut self, enabled: bool) -> Self {
        self.use_full_kissing = enabled;
        self
    }
}

/// Norm-4 direction table (Golay Construction A; expandable to full kissing).
pub fn leech_norm4_vectors(use_full: bool) -> Vec<(i32, i32, i32)> {
    if use_full {
        leech_full_kissing_approximation()
    } else {
        golay_derived_norm4_vectors()
    }
}

fn alignment_to_directions(fragment: &ReducedK22Fragment, dirs: &[(i32, i32, i32)]) -> f32 {
    if dirs.is_empty() {
        return 0.5;
    }
    let mut sum = 0.0f32;
    for &(a, b, c) in dirs {
        let norm = ((a * a + b * b + c * c) as f32).sqrt();
        if norm <= LEECH_MIN_NORM {
            let align = 1.0 / (1.0 + norm * 0.05);
            sum += align;
        }
    }
    let raw = sum / dirs.len() as f32;
    let betti_bonus = 1.0 / (1.0 + fragment.betti_proxy * 0.001);
    (raw * betti_bonus).clamp(0.0, 1.0)
}

pub fn leech_density_score_with_config(
    fragment: &ReducedK22Fragment,
    config: &LeechDensityConfig,
) -> f32 {
    let dirs = leech_norm4_vectors(config.use_full_kissing);
    alignment_to_directions(fragment, &dirs) * config.weight.max(0.01)
}

pub fn leech_density_score(fragment: &ReducedK22Fragment) -> f32 {
    leech_density_score_with_config(fragment, &LeechDensityConfig::default())
}

pub fn apply_leech_density_guidance(
    _input: &K22SheafFragment,
    base_result: ReducedK22Fragment,
    config: &LeechDensityConfig,
) -> Result<ReducedK22Fragment, ReductionError> {
    let density = leech_density_score_with_config(&base_result, config);
    let adjusted_betti = base_result.betti_proxy * (1.0 - config.weight * 0.15 * density);

    let guided = ReducedK22Fragment {
        betti_proxy: adjusted_betti.max(20.0),
        tomczak_preserved: base_result.tomczak_preserved,
        provenance: format!(
            "{}+Leech(w={:.2},d={:.3},kissing={})",
            base_result.provenance,
            config.weight,
            density,
            if config.use_full_kissing { "full" } else { "rep" }
        ),
    };

    if !guided.rigid_lift_check() {
        return Ok(base_result);
    }

    Ok(guided)
}