//! TQEC concurrent cascade skeleton — entropy on blocking thread, HIT on main.
//!
//! Run: `cargo run -p cutile --example tqec_concurrent_cascade --features viz-async`

use cutile::{Backend, CubicalHIT, EntropyParams, TriWeavonHIT};
use std::thread;

fn main() {
    let mut hit = TriWeavonHIT::new();
    let a = hit.add_point();
    let b = hit.add_point();
    hit.weave(a, b);

    let backend = Backend::auto();
    let params = EntropyParams {
        omega_tilde: vec![0.1; 32],
        d_perp_rho_sq: vec![0.01, 0.2, 0.01, 0.3],
        rho: vec![0.05; 32],
        strain_norms: vec![0.2; 4],
        tau: 1.0,
        nu: 0.01,
        surge_threshold: 0.05,
        prev_w_avg: 0.0,
    };

    let entropy_handle = thread::spawn(move || backend.compute_entropy_v2(&params));

    println!(
        "TQEC concurrent cascade — hit cells={} weaves={}",
        hit.point_count(),
        hit.weave_edges().len()
    );

    match entropy_handle.join() {
        Ok(Ok(r)) => println!(
            "entropy witness: w={:.4} surge={:.1} betti={:.1} gpu={}",
            r.w, r.surge, r.betti_proxy, r.used_gpu_kernel
        ),
        Ok(Err(e)) => eprintln!("entropy error: {e}"),
        Err(_) => eprintln!("entropy thread panicked"),
    }

    println!("Open `tqec_braid_viz` for full visual cascade.");
}