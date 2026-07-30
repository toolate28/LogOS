//! cutile — Cubical Tiling & HIT Execution Bridge
//!
//! Provides executable counterparts to the Agda/Lean TriWeavon formalization,
//! along with portable GPU backends for numerical computation.

pub mod backend;
pub mod clamping;
pub mod core;
pub mod error;
pub mod existence_cert;
pub mod harness;
pub mod hit;
pub mod l39_harmony;
pub mod mog_bridge;
pub mod traits;
pub mod viz;

/// Frozen v0.1 gate API surface (`todo!()` bodies). Enable with `--features schema_freeze_v0_1`.
#[cfg(feature = "schema_freeze_v0_1")]
pub mod gate_api;

pub use backend::{
    Backend, CpuBackend, CudaBackend, CudaEntropyResult, DefaultTiler, EntropyParams, EntropyResult,
    WgpuBackend,
};
pub use core::C12HexaflakeTiler;
pub use clamping::{
    apply_clamping, duration_clamp_config, intensity_clamp_config, priority_clamp_config,
    resolve_clamped_parameter, resolve_intensity, ClampConfig, ClampResult, Clampable,
    DURATION_DEFAULT, DURATION_MAX, DURATION_MIN, INTENSITY_MAX, INTENSITY_MIN, PRIORITY_DEFAULT,
    PRIORITY_MAX, PRIORITY_MIN,
};
pub use core::entropy::DefaultSurgeDetector;
pub use core::srac::{betti_tomczak_lift_check, srac_correct_if_needed};
pub use core::srac_strategies::{apply_correction, DivergenceReason, SracState};
pub use core::entropy::betti_proxy;
pub use core::{compute_entropy_diagnostic, hexaflake_nodes, srac_cascade_step, SRACorrection};
pub use error::CutileError;
pub use existence_cert::{ExistenceCertificate, TomczakGateWitness};
pub use l39_harmony::{
    betti_proxy_with_l39_harmony, compute_bias_scale, compute_harmonic_benefit,
    detect_l39_harmony_pair, emit_biased_existence_certificate, harmonic_auxiliary_loss,
    l39_harmony_divergence, map_tokens_to_lattice, L39HarmonyObservation,
};
pub use harness::{
    MehlerLevinHarness, MehlerLevinResult, CERTIFIED_ERROR_TOL, FAST_PATH_ERROR_TYPICAL,
    N_LEVIN_NODES,
};
pub use hit::{CubicalHIT, HComp, TriWeavonHIT};
pub use mog_bridge::{
    bit_on, bit_val, gf_add_n, gf_dot6, gf_mul_n, golay_mask_ok_n, is_hexacodeword_n,
    mask_col_count_n, mask_col_score_n, mask_of_indices, mask_top_count_n, mask_weight_n,
    KEYSTONE_ALPHA_PEAK, KEYSTONE_OMEGA_PEAK, KEYSTONE_SUM,
};
pub use traits::{ManifoldCompute, SurgeDetector, TilingStrategy};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");