//! GpuManifold skeleton — TriWeavon domain types on GPU

use crate::m24::{reduce_k22_with_m24, M24ReductionConfig, ReducedK22Fragment, K22SheafFragment};

#[cfg(feature = "cuda")]
use cudarc::driver::CudaDevice;

pub struct ManifoldConfig {
    pub alpha_omega: u32,
    pub wave_target: f32,
}

pub struct GpuManifold {
    #[allow(dead_code)]
    #[cfg(feature = "cuda")]
    device: Option<CudaDevice>,
    config: ManifoldConfig,
}

impl GpuManifold {
    pub fn new(config: ManifoldConfig) -> crate::Result<Self> {
        Ok(Self {
            #[cfg(feature = "cuda")]
            device: Some(
                CudaDevice::new(0)
                    .map_err(|e| crate::TriweavonCudarcError::Cuda(e.to_string()))?,
            ),
            config,
        })
    }

    pub fn reduce_k22_m24(&self, fragment: &K22SheafFragment, level: u32) -> crate::Result<ReducedK22Fragment> {
        let cfg = M24ReductionConfig {
            reduction_level: level,
            preserve_tomczak: true,
            enable_srac: true,
            use_golay_vectors: true,
        };
        reduce_k22_with_m24(fragment, &cfg).map_err(Into::into)
    }
}

pub struct VivianiConstraint {
    pub target: f32,
}

impl Default for VivianiConstraint {
    fn default() -> Self {
        Self { target: 0.9998 }
    }
}