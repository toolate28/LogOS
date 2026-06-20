//! cutile — Cubical Tiling & HIT Execution Bridge
//!
//! Provides executable counterparts to the Agda/Lean TriWeavon formalization,
//! along with portable GPU backends for numerical computation.

pub mod backend;
pub mod clamping;
pub mod core;
pub mod error;
pub mod hit;
pub mod traits;
pub mod viz;

pub use backend::{
    Backend, CpuBackend, CudaBackend, CudaEntropyResult, DefaultTiler, EntropyParams, EntropyResult,
    WgpuBackend,
};
pub use clamping::{
    apply_clamping, duration_clamp_config, intensity_clamp_config, priority_clamp_config,
    resolve_clamped_parameter, resolve_intensity, ClampConfig, ClampResult, Clampable,
    DURATION_DEFAULT, DURATION_MAX, DURATION_MIN, INTENSITY_MAX, INTENSITY_MIN, PRIORITY_DEFAULT,
    PRIORITY_MAX, PRIORITY_MIN,
};
pub use core::entropy::DefaultSurgeDetector;
pub use core::srac::{betti_tomczak_lift_check, srac_correct_if_needed};
pub use core::entropy::betti_proxy;
pub use core::{compute_entropy_diagnostic, hexaflake_nodes, srac_cascade_step, SRACorrection};
pub use error::CutileError;
pub use hit::{CubicalHIT, HComp, TriWeavonHIT};
pub use traits::{ManifoldCompute, SurgeDetector, TilingStrategy};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");