//! Generic numeric clamping — shared across cutile, coherence-mcp (TS mirror), and inspectors.
//!
//! TypeScript mirror: `coherence-mcp/src/lib/clamping.ts` (keep semantics in sync).

/// Types that support inclusive range clamping.
pub trait Clampable: Sized + Copy + PartialOrd {
    fn clamp_to(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    fn is_in_range(self, min: Self, max: Self) -> bool {
        self >= min && self <= max
    }
}

impl Clampable for f32 {}
impl Clampable for f64 {}
impl Clampable for i32 {}
impl Clampable for u32 {}
impl Clampable for i64 {}
impl Clampable for u64 {}

/// Clamping behaviour configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClampConfig {
    pub enabled: bool,
    pub min: f32,
    pub max: f32,
}

impl Default for ClampConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min: 0.0,
            max: 1.0,
        }
    }
}

/// Result of `apply_clamping`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClampResult<T> {
    pub value: T,
    pub was_clamped: bool,
    pub original: T,
}

/// Applies clamping when `config.enabled`; otherwise returns the input unchanged.
pub fn apply_clamping<T>(value: T, config: &ClampConfig) -> ClampResult<T>
where
    T: Clampable + Into<f32> + From<f32> + Copy,
{
    if !config.enabled {
        return ClampResult {
            value,
            was_clamped: false,
            original: value,
        };
    }

    let min = T::from(config.min);
    let max = T::from(config.max);

    if value.is_in_range(min, max) {
        ClampResult {
            value,
            was_clamped: false,
            original: value,
        }
    } else {
        ClampResult {
            value: value.clamp_to(min, max),
            was_clamped: true,
            original: value,
        }
    }
}

/// Reject-or-clamp resolution for tool parameters (mirrors coherence-mcp `resolveClampedParameter`).
pub fn resolve_clamped_parameter<T>(
    value: T,
    config: &ClampConfig,
    reject_when_out_of_range: bool,
) -> Result<ClampResult<T>, crate::CutileError>
where
    T: Clampable + Into<f32> + From<f32> + Copy,
{
    let min = T::from(config.min);
    let max = T::from(config.max);

    if !config.enabled && reject_when_out_of_range && !value.is_in_range(min, max) {
        return Err(crate::CutileError::ParameterOutOfRange {
            name: "parameter".into(),
            value: value.into(),
            min: config.min,
            max: config.max,
        });
    }

    Ok(apply_clamping(value, config))
}

/// Standard correction-burst intensity range `[0, 1]`.
pub const INTENSITY_MIN: f32 = 0.0;
pub const INTENSITY_MAX: f32 = 1.0;

/// Burst duration in seconds — sync with coherence-mcp `DURATION_CLAMP`.
pub const DURATION_MIN: f32 = 0.05;
pub const DURATION_MAX: f32 = 30.0;
pub const DURATION_DEFAULT: f32 = 1.0;

/// Dispatch priority — sync with coherence-mcp `PRIORITY_CLAMP`.
pub const PRIORITY_MIN: f32 = 1.0;
pub const PRIORITY_MAX: f32 = 10.0;
pub const PRIORITY_DEFAULT: f32 = 5.0;

pub fn duration_clamp_config(clamp_enabled: bool) -> ClampConfig {
    ClampConfig {
        enabled: clamp_enabled,
        min: DURATION_MIN,
        max: DURATION_MAX,
    }
}

pub fn priority_clamp_config(clamp_enabled: bool) -> ClampConfig {
    ClampConfig {
        enabled: clamp_enabled,
        min: PRIORITY_MIN,
        max: PRIORITY_MAX,
    }
}

pub fn intensity_clamp_config(clamp_enabled: bool) -> ClampConfig {
    ClampConfig {
        enabled: clamp_enabled,
        min: INTENSITY_MIN,
        max: INTENSITY_MAX,
    }
}

/// Resolve intensity: reject when `clamp_enabled` is false, else clamp.
pub fn resolve_intensity(
    intensity: f32,
    clamp_enabled: bool,
) -> Result<ClampResult<f32>, crate::CutileError> {
    if !clamp_enabled && !intensity.is_in_range(INTENSITY_MIN, INTENSITY_MAX) {
        return Err(crate::CutileError::ParameterOutOfRange {
            name: "intensity".into(),
            value: intensity,
            min: INTENSITY_MIN,
            max: INTENSITY_MAX,
        });
    }

    Ok(apply_clamping(
        intensity,
        &intensity_clamp_config(clamp_enabled),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_to_bounds() {
        assert_eq!(1.5_f32.clamp_to(0.0, 1.0), 1.0);
        assert_eq!((-0.1_f32).clamp_to(0.0, 1.0), 0.0);
    }

    #[test]
    fn apply_clamping_when_disabled() {
        let r = apply_clamping(2.0_f32, &ClampConfig::default());
        assert_eq!(r.value, 2.0);
        assert!(!r.was_clamped);
    }

    #[test]
    fn resolve_intensity_rejects() {
        assert!(resolve_intensity(1.5, false).is_err());
    }

    #[test]
    fn resolve_intensity_clamps() {
        let r = resolve_intensity(1.5, true).unwrap();
        assert_eq!(r.value, 1.0);
        assert!(r.was_clamped);
    }
}