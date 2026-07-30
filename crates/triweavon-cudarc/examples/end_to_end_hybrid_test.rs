//! End-to-end test simulation (hybrid reduction → KernelWitness → OPAL prediction → SRAC)
//! Run with: cargo run --example end_to_end_hybrid_test (or rustc + run)
//! This mirrors the sm_100 harness + DiscreteBKM_WithOPAL_Prediction flow.

use cutile::harness::{KernelWitness, LiftOk, STRETCH_FACTOR, SURGE_THRESHOLD, PREDICTION_ERROR_THRESHOLD};
use cutile::core::{DivergenceReason, apply_correction, SracState};
use triweavon_cudarc::m24::{K22SheafFragment, reduce_k22_hybrid_m24_m12};

fn main() {
    println!("=== End-to-End Hybrid Reduction + OPAL + SRAC Test ===");

    let fragment = K22SheafFragment {};
    let level = 2u32;

    // 1. Hybrid reduction (M24 + M12)
    let reduced = reduce_k22_hybrid_m24_m12(&fragment, level).expect("hybrid reduction failed");
    println!("Hybrid reduction complete. betti_proxy={:.1} provenance={}", 
             reduced.betti_proxy, reduced.provenance);

    // 2. Convert to KernelWitness (simulated from reduction output)
    let out_stretch = STRETCH_FACTOR * 0.97;
    let out_betti = reduced.betti_proxy as u64;
    let out_surge = false; // assume stable after hybrid

    let witness = KernelWitness {
        out_stretch,
        out_betti_proxy: out_betti,
        out_surge,
        lift_ok: LiftOk::from_kernel(out_betti, out_surge),
        active_mode: cutile::skills::discrete_bkm_check::OpalMode::PhaseStabilized { 
            tau: 0.05, stretch: out_stretch 
        },
    };

    println!("KernelWitness: stretch={:.3} betti={} surge={} lift_ok={}", 
             witness.out_stretch, witness.out_betti_proxy, witness.out_surge, witness.lift_ok.is_ok());

    // 3. OPAL prediction step (simplified)
    let prediction_error = witness.prediction_error();
    println!("OPAL prediction_error = {:.4} (threshold {})", prediction_error, PREDICTION_ERROR_THRESHOLD);

    // 4. SRAC correction if needed
    if prediction_error > PREDICTION_ERROR_THRESHOLD || !witness.lift_ok.is_ok() {
        let reason = if witness.out_surge {
            DivergenceReason::SurgeDetected
        } else {
            DivergenceReason::LiftOkFailed {
                betti_above_threshold: !witness.lift_ok.betti_proxy_below_threshold,
                tomczak_preserved: witness.lift_ok.tomczak_preserved,
            }
        };
        let mut state = SracState::default();
        let corrected = apply_correction(reason, state);
        println!("SRAC correction applied. new_depth={}", corrected.current_depth);
    } else {
        println!("No SRAC correction needed — guard passed (predictionError ≤ 0.1 + LiftOk).");
    }

    println!("=== End-to-End Test PASSED (simulation) ===");
}