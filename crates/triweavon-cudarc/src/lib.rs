//! triweavon-cudarc — TriWeavon domain GPU abstractions (sovereign namespace).
//!
//! Universal Invariant: α + ω = 15 enforced at `ManifoldConfig` construction.

pub mod coherence;
pub mod error;
pub mod kernel;
pub mod manifold;
pub mod polynomial;
pub mod prelude;
pub mod sheaf;

pub use error::TriweavonCudarcError;
pub use manifold::{GpuManifold, ManifoldConfig, VivianiConstraint};

pub type Result<T> = std::result::Result<T, TriweavonCudarcError>;

/// Conservation law constant.
pub const UNIVERSAL_INVARIANT_SUM: u32 = 15;