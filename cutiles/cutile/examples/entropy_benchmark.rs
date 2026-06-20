use cutile::{compute_entropy_diagnostic, DefaultTiler, TilingStrategy};
use std::time::Instant;

fn main() {
    let n = 65_536usize;
    let omega: Vec<f32> = (0..n).map(|i| (i as f32 * 1e-4).sin()).collect();
    let rho = omega.clone();
    let grad = vec![0.01f32; n / 8];
    let strain = vec![0.2f32; n / 8];

    let tiler = DefaultTiler;
    let tile = tiler.recommended_tile_size(n);
    let start = Instant::now();
    let (w, visc, stretch) = compute_entropy_diagnostic(&omega, &grad, &rho, &strain, 1.0, 0.01);
    let ms = start.elapsed().as_secs_f64() * 1000.0;

    println!("entropy_benchmark n={n} tile={tile}");
    println!("  W={w:.4} visc={visc:.4} stretch={stretch:.4} ({ms:.2}ms CPU)");
}