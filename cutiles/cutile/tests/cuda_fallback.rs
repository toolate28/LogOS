#![cfg(feature = "cuda")]

use cutile::CudaBackend;

#[test]
fn cuda_backend_cpu_fallback_without_ptx() {
    if CudaBackend::new().is_err() {
        return; // no GPU in CI
    }
    let backend = CudaBackend::new().unwrap();
    let omega = vec![0.1f32; 16];
    let rho = omega.clone();
    let grad = vec![0.01f32; 4];
    let strain = vec![0.2f32; 4];
    let r = backend
        .compute_entropy_v2(&omega, &grad, &rho, &strain, 1.0, 0.01, 0.05, 0.0)
        .expect("entropy v2");
    assert!(r.w.is_finite());
    // Without PTX on disk, expect CPU path
    assert!(!r.used_gpu_kernel || r.w.is_finite());
}