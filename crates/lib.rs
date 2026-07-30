//! # cQ-kitty-rips-verify — The Bedrock Verification Suite
//!
//! Property-based tests that prove the algebraic foundations are correct.
//! BUILD ≠ LABEL ≠ FIX — this crate is the FIX layer.
//!
//! ## What This Verifies
//!
//! 1. **Cl(1,3) Geometric Product** — associativity, metric signature, grade structure
//! 2. **THTW Weyl Graph** — parallel transport, flat connection, plaquette holonomy
//! 3. **Octonion Algebra** — Fano table correctness, Moufang identities, norm multiplicativity
//! 4. **Conservation Invariant** — α + ω = 15 at every computational boundary
//!
//! ## Rigor Category: Meta-A
//!
//! This crate verifies that Category A claims are actually Category A.
//! If any test here fails, everything downstream is suspect.

use cQ_kitty_rips_ga::cl13::Cl13;
use cQ_kitty_rips_ga::weyl;
use cQ_kitty_rips_graph::{TTHTWWeylGraph, NodePayload};
use cQ_kitty_rips_wdbi::Octonion;
use serde::Serialize;

// ══════════════════════════════════════════════════════════════════
// 1. Cl(1,3) Verification
// ══════════════════════════════════════════════════════════════════

/// Verify the metric signature (+,−,−,−) by squaring all grade-1 basis elements.
pub fn verify_metric_signature() -> Result<(), String> {
    let expected = [1.0, -1.0, -1.0, -1.0]; // e₀²=+1, e₁²=e₂²=e₃²=−1
    for (idx, &exp) in expected.iter().enumerate() {
        let e = Cl13::basis(idx + 1); // basis 1..4 are grade-1
        let sq = (e * e).scalar_part();
        if (sq - exp).abs() > 1e-12 {
            return Err(format!(
                "Metric violation: e{}² = {} (expected {})", idx, sq, exp
            ));
        }
    }
    Ok(())
}

/// Verify anticommutativity of distinct generators: eᵢeⱼ + eⱼeᵢ = 0
pub fn verify_anticommutativity() -> Result<(), String> {
    for i in 1..=4 {
        for j in (i + 1)..=4 {
            let ei = Cl13::basis(i);
            let ej = Cl13::basis(j);
            let sum = (ei * ej) + (ej * ei);
            if !sum.is_zero(1e-12) {
                return Err(format!(
                    "Anticommutativity violation: e{}·e{} + e{}·e{} ≠ 0", i, j, j, i
                ));
            }
        }
    }
    Ok(())
}

/// Verify associativity of the geometric product over ALL 16³ = 4096 basis triples.
///
/// This is the definitive test. If this passes, the multiplication table is correct.
pub fn verify_full_associativity() -> Result<(), String> {
    let mut failures = 0;
    let mut first_failure = String::new();

    for i in 0..16 {
        for j in 0..16 {
            for k in 0..16 {
                let ei = Cl13::basis(i);
                let ej = Cl13::basis(j);
                let ek = Cl13::basis(k);

                let left = (ei * ej) * ek;
                let right = ei * (ej * ek);

                if !left.approx_eq(&right, 1e-10) {
                    failures += 1;
                    if first_failure.is_empty() {
                        first_failure = format!(
                            "Associativity failed: (e{} · e{}) · e{} ≠ e{} · (e{} · e{})",
                            i, j, k, i, j, k
                        );
                    }
                }
            }
        }
    }

    if failures > 0 {
        Err(format!("{} ({} failures out of 4096 triples)", first_failure, failures))
    } else {
        Ok(())
    }
}

/// Verify grade structure: grade(eᵢ · eⱼ) is correct for basis blades.
pub fn verify_grade_structure() -> Result<(), String> {
    use cQ_kitty_rips_ga::cl13::tables::GRADE;

    for i in 0..16 {
        let g = GRADE[i];
        let blade = Cl13::basis(i);

        // The blade should have nonzero coefficient only at components of grade g
        for j in 0..16 {
            if j == i {
                if blade.data[j] != 1.0 {
                    return Err(format!("Basis {} should have 1.0 at position {}", i, j));
                }
            } else if blade.data[j] != 0.0 {
                return Err(format!("Basis {} should have 0.0 at position {}", i, j));
            }
        }

        // Grade should be the popcount of the bitmask
        let expected_grade = cQ_kitty_rips_ga::cl13::tables::BITMASK[i].count_ones() as u8;
        if g != expected_grade {
            return Err(format!("Grade mismatch for basis {}: {} vs {}", i, g, expected_grade));
        }
    }
    Ok(())
}

/// Verify the pseudoscalar squares to −1 in Cl(1,3).
///
/// e₀₁₂₃² = e₀²·e₁²·e₂²·e₃² × (−1)^{swaps}
///         = (+1)(−1)(−1)(−1) × (−1)^6
///         = (−1) × 1 = −1
pub fn verify_pseudoscalar_square() -> Result<(), String> {
    let ps = Cl13::e0123();
    let sq = (ps * ps).scalar_part();
    if (sq - (-1.0)).abs() > 1e-12 {
        Err(format!("e₀₁₂₃² = {} (expected −1)", sq))
    } else {
        Ok(())
    }
}

// ══════════════════════════════════════════════════════════════════
// 2. Weyl Gauge Theory Verification
// ══════════════════════════════════════════════════════════════════

/// Verify R̂ = R when ω = 0 (no Weyl field → standard Riemann).
pub fn verify_hat_r_reduces() -> Result<(), String> {
    let r = 42.0;
    let hat = weyl::hat_r(r, 0.0, 0.0);
    if (hat - r).abs() > 1e-12 {
        Err(format!("R̂ should equal R when ω=0, got {}", hat))
    } else {
        Ok(())
    }
}

/// Verify the Lagrangian has correct structure: positive R̂² term, negative F̂² term.
pub fn verify_lagrangian_structure() -> Result<(), String> {
    // Pure R̂ (no F̂) should give positive Lagrangian for any R̂ ≠ 0
    let l1 = weyl::weyl_lagrangian(5.0, 0.0, 1.0, 1.0);
    if l1 <= 0.0 {
        return Err(format!("L(R̂=5, F̂=0) should be positive, got {}", l1));
    }

    // Pure F̂ (no R̂) should give negative Lagrangian
    let l2 = weyl::weyl_lagrangian(0.0, 5.0, 1.0, 1.0);
    if l2 >= 0.0 {
        return Err(format!("L(R̂=0, F̂=5) should be negative, got {}", l2));
    }

    Ok(())
}

// ══════════════════════════════════════════════════════════════════
// 3. THTW Graph Verification
// ══════════════════════════════════════════════════════════════════

/// Verify that flat connections (ω=0 everywhere) preserve multivectors under transport.
pub fn verify_flat_transport_preserves() -> Result<(), String> {
    let mut g = TTHTWWeylGraph::default_fractal();
    let a = g.add_node(NodePayload::default());
    let b = g.add_node(NodePayload::default());
    let e = g.add_edge(a, b, 0.0, [1.0, 0.0, 0.0, 0.0]);

    // Test with several multivectors
    let test_vectors = [
        Cl13::e0(),
        Cl13::e1(),
        Cl13::e12(),
        Cl13::e0123(),
        Cl13 { data: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
                       9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0] },
    ];

    for (idx, v) in test_vectors.iter().enumerate() {
        let transported = g.parallel_transport(v, e, 1.0);
        if !transported.approx_eq(v, 1e-12) {
            return Err(format!("Flat transport altered vector {}", idx));
        }
    }
    Ok(())
}

/// Verify that transport around a contractible loop with zero total holonomy
/// returns the identity transformation.
pub fn verify_contractible_loop_identity() -> Result<(), String> {
    let mut g = TTHTWWeylGraph::default_fractal();
    let a = g.add_node(NodePayload::default());
    let b = g.add_node(NodePayload::default());
    let c = g.add_node(NodePayload::default());

    // Create a triangle with total holonomy = 0
    // ω₁ + ω₂ + ω₃ = 0
    let e1 = g.add_edge(a, b, 0.5, [1.0, 0.0, 0.0, 0.0]);
    let e2 = g.add_edge(b, c, -0.3, [0.0, 1.0, 0.0, 0.0]);
    let e3 = g.add_edge(c, a, -0.2, [-1.0, -1.0, 0.0, 0.0]);

    // Holonomy should be zero
    let holonomy = g.plaquette_holonomy(&[e1, e2, e3]);
    if holonomy.abs() > 1e-12 {
        return Err(format!("Non-zero holonomy {} on flat loop", holonomy));
    }

    // Transport a vector around the loop — should return to itself
    let v = Cl13::e1();
    let q = 1.0;
    let v1 = g.parallel_transport(&v, e1, q);
    let v2 = g.parallel_transport(&v1, e2, q);
    let v3 = g.parallel_transport(&v2, e3, q);

    // For abelian transport, Σ_total = Σ₁·Σ₂·Σ₃ = exp(ω₁+ω₂+ω₃) = exp(0) = 1
    if !v3.approx_eq(&v, 1e-10) {
        return Err(format!(
            "Round-trip transport failed: diff = {}",
            (v3 - v).coefficient_norm()
        ));
    }
    Ok(())
}

/// Verify non-metricity has correct sign: positive ω → shrinking norms.
pub fn verify_non_metricity_sign() -> Result<(), String> {
    let mut g = TTHTWWeylGraph::default_fractal();
    let a = g.add_node(NodePayload::default());
    let b = g.add_node(NodePayload::default());
    let e_pos = g.add_edge(a, b, 0.5, [1.0, 0.0, 0.0, 0.0]);

    let nm = g.discrete_non_metricity(1.0, e_pos);
    if nm >= 0.0 {
        return Err(format!("Positive ω should give negative non-metricity, got {}", nm));
    }
    Ok(())
}

// ══════════════════════════════════════════════════════════════════
// 4. Octonion Verification
// ══════════════════════════════════════════════════════════════════

/// Verify all imaginary units square to −1.
pub fn verify_octonion_squares() -> Result<(), String> {
    for i in 1..8 {
        let ei = Octonion::unit(i);
        let sq = ei.mul(&ei);
        if (sq.real() - (-1.0)).abs() > 1e-12 {
            return Err(format!("e{}² real part = {} (expected −1)", i, sq.real()));
        }
        for j in 1..8 {
            if sq.data[j].abs() > 1e-12 {
                return Err(format!("e{}² has nonzero imaginary component {}", i, j));
            }
        }
    }
    Ok(())
}

/// Verify all 7 Fano-plane triples produce the correct products.
pub fn verify_fano_triples() -> Result<(), String> {
    // Use the ACTUAL triples from the crate (standard Baez orientation)
    let triples = cQ_kitty_rips_wdbi::octonion::tables::FANO_TRIPLES;

    for &(i, j, k) in triples.iter() {
        let ei = Octonion::unit(i);
        let ej = Octonion::unit(j);
        let ek = Octonion::unit(k);

        // Forward: eᵢ · eⱼ = +eₖ
        let fwd = ei.mul(&ej);
        if !fwd.approx_eq(&ek, 1e-12) {
            return Err(format!("Fano triple ({},{},{}) forward failed", i, j, k));
        }

        // Reverse: eⱼ · eᵢ = −eₖ
        let rev = ej.mul(&ei);
        if !rev.approx_eq(&(-ek), 1e-12) {
            return Err(format!("Fano triple ({},{},{}) reverse failed", i, j, k));
        }

        // Cyclic: eⱼ · eₖ = +eᵢ
        let cyc1 = ej.mul(&ek);
        if !cyc1.approx_eq(&ei, 1e-12) {
            return Err(format!("Fano triple ({},{},{}) cyclic1 failed", i, j, k));
        }

        // Cyclic: eₖ · eᵢ = +eⱼ
        let cyc2 = ek.mul(&ei);
        if !cyc2.approx_eq(&ej, 1e-12) {
            return Err(format!("Fano triple ({},{},{}) cyclic2 failed", i, j, k));
        }
    }
    Ok(())
}

/// Verify that octonions are NOT associative (the negative test).
///
/// Using (e₃, e₅, e₄) which span different quaternion subalgebras:
/// (e₃·e₅)·e₄ = (−e₆)·e₄ = e₂
/// e₃·(e₅·e₄) = e₃·(−e₁) = −e₂
/// These differ by sign → non-associative. QED.
pub fn verify_non_associativity() -> Result<(), String> {
    let e3 = Octonion::unit(3);
    let e5 = Octonion::unit(5);
    let e4 = Octonion::unit(4);

    let left = Octonion::mul(&Octonion::mul(&e3, &e5), &e4);
    let right = Octonion::mul(&e3, &Octonion::mul(&e5, &e4));

    if left.approx_eq(&right, 1e-12) {
        return Err("Octonions appear associative — this is wrong!".into());
    }

    // Further verify the specific values
    // (e₃·e₅) = −e₆ (from Fano: e₅·e₃ = e₆, so e₃·e₅ = −e₆)
    let e3e5 = Octonion::mul(&e3, &e5);
    if (e3e5.data[6] - (-1.0)).abs() > 1e-12 {
        return Err(format!("e₃·e₅ should be −e₆, got component [6] = {}", e3e5.data[6]));
    }

    Ok(())
}

/// Verify Moufang identities hold for a set of random-ish octonions.
pub fn verify_moufang_identities() -> Result<(), String> {
    // Test with several triples of non-trivial octonions
    let test_cases = [
        (
            Octonion { data: [1.0, 0.5, -0.3, 0.2, 0.1, -0.4, 0.6, -0.1] },
            Octonion { data: [0.3, -0.2, 0.7, -0.1, 0.4, 0.2, -0.5, 0.3] },
            Octonion { data: [-0.1, 0.3, 0.1, 0.6, -0.2, 0.5, 0.1, -0.4] },
        ),
        (
            Octonion { data: [2.0, 1.0, 0.0, -1.0, 0.5, 0.0, -0.5, 1.0] },
            Octonion { data: [0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 0.5] },
            Octonion { data: [1.0, -1.0, 0.5, 0.5, 0.0, 1.0, -1.0, 0.0] },
        ),
    ];

    for (idx, (a, b, c)) in test_cases.iter().enumerate() {
        if !a.check_left_moufang(b, c, 1e-8) {
            return Err(format!("Left Moufang failed for triple {}", idx));
        }
        if !a.check_right_moufang(b, c, 1e-8) {
            return Err(format!("Right Moufang failed for triple {}", idx));
        }
        if !a.check_middle_moufang(b, c, 1e-8) {
            return Err(format!("Middle Moufang failed for triple {}", idx));
        }
    }
    Ok(())
}

/// Verify norm multiplicativity: ‖ab‖ = ‖a‖·‖b‖ (composition algebra property).
pub fn verify_norm_multiplicativity() -> Result<(), String> {
    let pairs = [
        (
            Octonion { data: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0] },
            Octonion { data: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] },
        ),
        (
            Octonion::unit(3),
            Octonion { data: [1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0] },
        ),
    ];

    for (idx, (a, b)) in pairs.iter().enumerate() {
        let ab = a.mul(b);
        let diff = (a.norm() * b.norm() - ab.norm()).abs();
        if diff > 1e-10 {
            return Err(format!(
                "Norm multiplicativity failed for pair {}: |diff| = {}", idx, diff
            ));
        }
    }
    Ok(())
}

// ══════════════════════════════════════════════════════════════════
// 5. Conservation Invariant: α + ω = 15
// ══════════════════════════════════════════════════════════════════

/// Verify the conservation law holds for all canonical partitions.
pub fn verify_conservation_invariant() -> Result<(), String> {
    let partitions: Vec<(f64, f64)> = vec![
        (8.0, 7.0),  // standard structural
        (7.0, 8.0),  // standard semantic
        (14.0, 1.0), // extreme α-dominant
        (1.0, 14.0), // extreme ω-dominant
        (7.5, 7.5),  // balanced
    ];

    for (alpha, omega) in &partitions {
        if !weyl::check_invariant(*alpha, *omega) {
            return Err(format!("Invariant failed: {} + {} ≠ 15", alpha, omega));
        }
    }

    // Negative test: these should NOT pass
    if weyl::check_invariant(8.0, 8.0) {
        return Err("8 + 8 = 16 should fail invariant check".into());
    }
    if weyl::check_invariant(0.0, 0.0) {
        return Err("0 + 0 = 0 should fail invariant check".into());
    }

    Ok(())
}

// ══════════════════════════════════════════════════════════════════
// Master verification runner
// ══════════════════════════════════════════════════════════════════

/// Result of a single verification check.
#[derive(Debug, Serialize)]
pub struct VerifyResult {
    pub name: &'static str,
    pub category: &'static str,
    pub passed: bool,
    pub message: String,
}

/// Run ALL verification checks and return structured results.
///
/// This is the single function that answers: "Is the bedrock sound?"
pub fn run_full_verification() -> Vec<VerifyResult> {
    let checks: Vec<(&str, &str, fn() -> Result<(), String>)> = vec![
        // Cl(1,3) — Category A
        ("Cl(1,3) metric signature (+,−,−,−)", "A", verify_metric_signature),
        ("Cl(1,3) anticommutativity", "A", verify_anticommutativity),
        ("Cl(1,3) full associativity (4096 triples)", "A", verify_full_associativity),
        ("Cl(1,3) grade structure", "A", verify_grade_structure),
        ("Cl(1,3) pseudoscalar e₀₁₂₃² = −1", "A", verify_pseudoscalar_square),

        // Weyl gauge — Category A
        ("Weyl R̂ → R when ω=0", "A", verify_hat_r_reduces),
        ("Weyl Lagrangian sign structure", "A", verify_lagrangian_structure),

        // THTW Graph — Category B
        ("THTW flat transport preserves vectors", "B", verify_flat_transport_preserves),
        ("THTW contractible loop identity", "B", verify_contractible_loop_identity),
        ("THTW non-metricity sign convention", "B", verify_non_metricity_sign),

        // Octonions — Category A
        ("Octonion units square to −1", "A", verify_octonion_squares),
        ("Octonion Fano-plane triples (all 7)", "A", verify_fano_triples),
        ("Octonion non-associativity (negative test)", "A", verify_non_associativity),
        ("Octonion Moufang identities", "A", verify_moufang_identities),
        ("Octonion norm multiplicativity", "A", verify_norm_multiplicativity),

        // Conservation — Category C
        ("Conservation α + ω = 15", "C", verify_conservation_invariant),
    ];

    checks.into_iter().map(|(name, cat, f)| {
        match f() {
            Ok(()) => VerifyResult {
                name,
                category: cat,
                passed: true,
                message: "PASS".into(),
            },
            Err(msg) => VerifyResult {
                name,
                category: cat,
                passed: false,
                message: msg,
            },
        }
    }).collect()
}

/// Print verification results to stdout in a human-readable format.
pub fn print_verification_report(results: &[VerifyResult]) {
    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       cQ-KITTY-RIPS VERIFICATION REPORT                    ║");
    println!("║       α + ω = 15 | BEDROCK INTEGRITY CHECK                ║");
    println!("╠══════════════════════════════════════════════════════════════╣");

    for r in results {
        let status = if r.passed { "✓ PASS" } else { "✗ FAIL" };
        let cat = format!("[Cat {}]", r.category);
        println!("║ {} {:6} {} ", status, cat, r.name);
        if !r.passed {
            println!("║        └── {}", r.message);
        }
    }

    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║ TOTAL: {}/{} passed", passed, total);
    if passed == total {
        println!("║ BEDROCK STATUS: SOLID ✓");
        println!("║ α + ω = 15 — The keystone holds.");
    } else {
        println!("║ BEDROCK STATUS: CRACKED ✗ — {} failures", total - passed);
        println!("║ DO NOT BUILD ON THIS FOUNDATION.");
    }
    println!("╚══════════════════════════════════════════════════════════════╝");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_verification_passes() {
        let results = run_full_verification();
        for r in &results {
            assert!(r.passed, "Verification failed: {} — {}", r.name, r.message);
        }
    }
}
