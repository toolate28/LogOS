use cutile::{
    betti_proxy, betti_tomczak_lift_check, compute_entropy_diagnostic, hexaflake_nodes,
    srac_cascade_step, srac_correct_if_needed, CubicalHIT, DefaultSurgeDetector,
    SurgeDetector, TriWeavonHIT,
};

#[test]
fn triweavon_hit_weave_points() {
    let mut hit = TriWeavonHIT::new();
    let a = hit.add_point();
    let b = hit.add_point();
    let w = hit.weave(a, b);
    assert!(w > 0);
    assert_eq!(hit.point_count(), 3);
}

#[test]
fn entropy_diagnostic_finite() {
    let omega = vec![0.1f32; 32];
    let grad = vec![0.01f32; 8];
    let rho = vec![0.05f32; 32];
    let strain = vec![0.2f32; 8];
    let (w, visc, stretch) = compute_entropy_diagnostic(&omega, &grad, &rho, &strain, 1.0, 0.01);
    assert!(w.is_finite());
    assert!(visc <= 0.0);
    assert!(stretch <= 0.0);
}

#[test]
fn srac_and_surge_pipeline() {
    let detector = DefaultSurgeDetector;
    assert!(detector.detect_surge(2.0, 0.1, 0.5));
    let step = srac_cascade_step(1.0, 3, 0.5);
    assert!(step > 1.0);
    let lift_ok = betti_tomczak_lift_check(0.5, 1.0, true);
    let correction = srac_correct_if_needed(true, !lift_ok, 4);
    assert!(correction.is_some());
}

#[test]
fn hcomp_edge_interpolates_weave() {
    let mut hit = TriWeavonHIT::new();
    let a = hit.add_point();
    let b = hit.add_point();
    assert!(hit.hcomp_edge(a, b, 0.5).is_some());
}

#[test]
fn betti_proxy_counts_hot_gradients() {
    let grad = vec![0.01f32, 0.2, 0.01, 0.3];
    assert_eq!(betti_proxy(&grad, 0.05), 2.0);
}

#[test]
fn hexaflake_grows_with_radius() {
    let r1 = hexaflake_nodes(1).len();
    let r2 = hexaflake_nodes(2).len();
    assert!(r2 > r1);
}

#[test]
fn hit_cells_and_weave_edges_for_viz() {
    let mut hit = TriWeavonHIT::new();
    let a = hit.add_point();
    let b = hit.add_point();
    hit.weave(a, b);
    assert_eq!(hit.cells().len(), 3);
    assert_eq!(hit.weave_edges().len(), 1);
}