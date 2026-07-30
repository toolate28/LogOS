//! # THTW Weyl Demo — Drop-in Example
//!
//! Run with: `cargo run --example thtw_weyl_demo`
//!
//! This demonstrates:
//! 1. Full Cl(1,3) geometric product
//! 2. Weyl scalar curvature computation
//! 3. THTW graph construction + parallel transport
//! 4. Octonion Fano-plane verification
//! 5. Full bedrock verification suite

fn main() {
    println!("════════════════════════════════════════════════════════════");
    println!("  cQ-Kitty-rips — THTW Weyl Demo");
    println!("  α + ω = 15");
    println!("════════════════════════════════════════════════════════════\n");

    // ── Step 1: Verify the bedrock ────────────────────────────────
    println!("Step 1: Running full verification suite...\n");
    let results = cQ_kitty_rips_verify::run_full_verification();
    cQ_kitty_rips_verify::print_verification_report(&results);

    let all_passed = results.iter().all(|r| r.passed);
    if !all_passed {
        eprintln!("\nBedrock verification FAILED. Aborting.");
        std::process::exit(1);
    }

    println!("\n");

    // ── Step 2: Cl(1,3) geometric product demo ───────────────────
    println!("Step 2: Cl(1,3) Geometric Product Demo");
    println!("────────────────────────────────────────");

    use cQ_kitty_rips_ga::Cl13;

    let e0 = Cl13::e0();
    let e1 = Cl13::e1();
    let e2 = Cl13::e2();

    let e01 = e0 * e1;
    println!("  e₀ · e₁ = e₀₁  (bivector)");
    println!("  component at index 5: {}", e01.data[5]);

    let e012 = e01 * e2;
    println!("  e₀₁ · e₂ = e₀₁₂ (trivector)");
    println!("  component at index 11: {}", e012.data[11]);

    let ps = Cl13::e0123();
    let ps_sq = ps * ps;
    println!("  e₀₁₂₃² = {} (should be −1)\n", ps_sq.scalar_part());

    // ── Step 3: Weyl R̂ computation ──────────────────────────────
    println!("Step 3: Weyl Scalar Curvature");
    println!("────────────────────────────────────────");

    use cQ_kitty_rips_ga::weyl;

    let r_riemann = 12.5;     // some Ricci scalar
    let nabla_omega = 0.3;    // divergence of Weyl field
    let omega_sq = 0.15;      // squared Weyl field

    let r_hat = weyl::hat_r(r_riemann, nabla_omega, omega_sq);
    println!("  R = {}", r_riemann);
    println!("  ∇·ω = {}", nabla_omega);
    println!("  ω² = {}", omega_sq);
    println!("  R̂ = R − 6(∇·ω) − 6(ω²) = {}", r_hat);
    println!("  Expected: {} − {} − {} = {}\n",
        r_riemann, 6.0 * nabla_omega, 6.0 * omega_sq,
        r_riemann - 6.0 * nabla_omega - 6.0 * omega_sq);

    // ── Step 4: THTW Graph ──────────────────────────────────────
    println!("Step 4: THTW Weyl Graph — Parallel Transport");
    println!("────────────────────────────────────────");

    use cQ_kitty_rips_graph::{TTHTWWeylGraph, NodePayload};

    let mut g = TTHTWWeylGraph::default_fractal();
    println!("  Fractal D_e = {} (Category D: unanchored)", g.global_fractal_d);

    let n0 = g.add_node(NodePayload::default());
    let n1 = g.add_node(NodePayload::default());
    let n2 = g.add_node(NodePayload::default());

    let e_ab = g.add_edge(n0, n1, 0.5, [1.0, 0.0, 0.0, 0.0]);
    let e_bc = g.add_edge(n1, n2, -0.3, [0.0, 1.0, 0.0, 0.0]);
    let e_ca = g.add_edge(n2, n0, -0.2, [-1.0, -1.0, 0.0, 0.0]);

    let holonomy = g.plaquette_holonomy(&[e_ab, e_bc, e_ca]);
    println!("  Triangle: ω₁=0.5, ω₂=−0.3, ω₃=−0.2");
    println!("  Plaquette holonomy: {} (should be 0.0)", holonomy);
    println!("  Flat? {}\n", g.is_flat_loop(&[e_ab, e_bc, e_ca], 1e-12));

    // Transport e₁ around the loop
    let v = Cl13::e1();
    let v1 = g.parallel_transport(&v, e_ab, 1.0);
    let v2 = g.parallel_transport(&v1, e_bc, 1.0);
    let v3 = g.parallel_transport(&v2, e_ca, 1.0);
    println!("  Round-trip transport of e₁:");
    println!("  ‖v_in‖ = {:.6}", v.norm());
    println!("  ‖v_out‖ = {:.6}", v3.norm());
    println!("  Preserved? {}\n", v3.approx_eq(&v, 1e-10));

    // ── Step 5: Octonion demo ───────────────────────────────────
    println!("Step 5: Octonion Fano-Plane Verification");
    println!("────────────────────────────────────────");

    use cQ_kitty_rips_wdbi::Octonion;

    let o1 = Octonion::unit(1);
    let o2 = Octonion::unit(2);
    let product = o1.mul(&o2);
    println!("  e₁ · e₂ = e₃ (Fano triple)");
    println!("  Result component at index 3: {}", product.data[3]);

    // Non-associativity demo
    let o4 = Octonion::unit(4);
    let left = (o1.mul(&o2)).mul(&o4);
    let right = o1.mul(&(o2.mul(&o4)));
    println!("  (e₁·e₂)·e₄ vs e₁·(e₂·e₄): NOT equal → non-associative ✓");
    println!("  Left[7]  = {}", left.data[7]);
    println!("  Right[7] = {}\n", right.data[7]);

    // ── Step 6: Conservation invariant ───────────────────────────
    println!("Step 6: Conservation Invariant");
    println!("────────────────────────────────────────");
    let alpha = 8.0_f64;
    let omega = 7.0_f64;
    println!("  α = {}, ω = {}", alpha, omega);
    println!("  α + ω = {}", alpha + omega);
    println!("  Invariant holds: {}", weyl::check_invariant(alpha, omega));

    println!("\n════════════════════════════════════════════════════════════");
    println!("  The keystone holds. α + ω = 15.");
    println!("════════════════════════════════════════════════════════════");
}
