use cutile::{Backend, EntropyParams};

fn sample_params(n: usize) -> EntropyParams {
    EntropyParams {
        omega_tilde: vec![0.1; n],
        d_perp_rho_sq: vec![0.01, 0.2, 0.01, 0.3],
        rho: vec![0.05; n],
        strain_norms: vec![0.2; 4],
        tau: 1.0,
        nu: 0.01,
        surge_threshold: 0.05,
        prev_w_avg: 0.0,
    }
}

#[test]
fn backend_cpu_entropy_v2() {
    let backend = Backend::cpu();
    let r = backend
        .compute_entropy_v2(&sample_params(32))
        .expect("cpu entropy");
    assert!(r.w.is_finite());
    assert!(!r.used_gpu_kernel);
}

#[test]
fn backend_batch_matches_single() {
    let backend = Backend::cpu();
    let p = sample_params(16);
    let single = backend.compute_entropy_v2(&p).unwrap();
    let batch = backend.compute_entropy_batch(&[p.clone(), p]).unwrap();
    assert_eq!(batch.len(), 2);
    assert_eq!(batch[0].w, single.w);
    assert_eq!(batch[1].w, single.w);
}

#[cfg(not(feature = "wgpu-backend"))]
#[test]
fn backend_auto_selects_cpu_without_gpu_features() {
    let backend = Backend::auto();
    assert_eq!(backend.name(), "cpu");
}

#[cfg(feature = "wgpu-backend")]
#[test]
fn backend_auto_selects_wgpu_when_feature_enabled() {
    let backend = Backend::auto();
    assert_eq!(backend.name(), "wgpu");
}

#[cfg(feature = "wgpu-backend")]
#[test]
fn wgpu_entropy_v2_when_available() {
    let backend = match Backend::try_wgpu() {
        Ok(b) => b,
        Err(_) => return,
    };
    let r = backend
        .compute_entropy_v2(&sample_params(64))
        .expect("wgpu entropy");
    assert!(r.w.is_finite());
    assert!(r.used_gpu_kernel);
}