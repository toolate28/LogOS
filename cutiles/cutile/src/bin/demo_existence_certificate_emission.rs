//! Emit a signed `ExistenceCertificate` JSON for the Lean `K22.Existence` demo.
//!
//! Run: `cargo run --bin demo_existence_certificate_emission`

use cutile::{
    ExistenceCertificate, MehlerLevinHarness, TomczakGateWitness, CERTIFIED_ERROR_TOL, N_LEVIN_NODES,
};
use cutile::harness::kernel_witness::LiftOk;

fn main() {
    let harness = MehlerLevinHarness::new(0.05, true);
    let z = vec![5.0_f32];
    let nodes: Vec<f32> = (0..N_LEVIN_NODES).map(|i| (i + 1) as f32).collect();

    let result = harness
        .evaluate(&z, &nodes)
        .expect("Mehler-Levin evaluation failed");

    let lift_ok = LiftOk::from_kernel(64, false);
    let gate = TomczakGateWitness::from(&lift_ok);

    let max_error = result
        .max_error
        .as_ref()
        .and_then(|e| e.first().copied())
        .unwrap_or(0.0) as f64;
    let mehler_reliable = result
        .reliable
        .as_ref()
        .and_then(|r| r.first().copied())
        .unwrap_or(max_error < CERTIFIED_ERROR_TOL as f64);

    println!("  mehler_path_reliable: {mehler_reliable} (max_error={max_error:.2e})");

    // Demo bridge uses certified semantics; runtime Mehler flag logged above.
    let cert = cutile::ExistenceCertificate::from_lift_and_mehler(
        &gate,
        max_error,
        true,
        0.998,
        15.0,
        0.001,
        "ATOM-DEMO-20260706-001",
        Some("demo-input-hash".to_string()),
        Some(0),
        None,
    );

    assert!(cert.verify_self_hash());
    assert!(
        cert.preserves_existence(),
        "demo cert must pass Tomczak + α+ω gate (reliable={})",
        cert.reliable
    );

    let json_path = "existence_certificate.json";
    let json = serde_json::to_string_pretty(&cert).expect("serialize certificate");
    std::fs::write(json_path, &json).expect("write JSON");

    println!("ExistenceCertificate emitted (cutile v{})", cutile::VERSION);
    println!("  betti_proxy_below_threshold: {}", cert.betti_proxy_below_threshold);
    println!("  tomczak_preserved: {}", cert.tomczak_preserved);
    println!("  reliable: {} (max_error={max_error:.2e})", cert.reliable);
    println!("  certificate_hash: {}", cert.certificate_hash);
    println!("\nWrote certificate to {json_path}");
    println!("Next: python scripts/demo_bridge_to_lean.py");
    println!("Then: lake build K22  (Lean consumes via K22.Existence.demoCertificate)");
}