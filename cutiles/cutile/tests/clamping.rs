use cutile::{
    apply_clamping, resolve_intensity, ClampConfig, CutileError, INTENSITY_MAX, INTENSITY_MIN,
};

#[test]
fn intensity_range_constants() {
    assert_eq!(INTENSITY_MIN, 0.0);
    assert_eq!(INTENSITY_MAX, 1.0);
}

#[test]
fn apply_clamping_clamps_high() {
    let cfg = ClampConfig {
        enabled: true,
        min: 0.0,
        max: 1.0,
    };
    let r = apply_clamping(1.8_f32, &cfg);
    assert_eq!(r.value, 1.0);
    assert!(r.was_clamped);
}

#[test]
fn resolve_intensity_matches_mcp_semantics() {
    assert!(matches!(
        resolve_intensity(1.5, false),
        Err(CutileError::ParameterOutOfRange { .. })
    ));
    let ok = resolve_intensity(0.4, false).unwrap();
    assert_eq!(ok.value, 0.4);
    assert!(!ok.was_clamped);
}