//! TQEC visualisation layer — reson8Labs cascade palette + braid/syndrome prototypes.

pub mod braid;
pub mod palette;
pub mod tqec_syndrome;

pub use braid::BraidLayout;
pub use palette::{entropy_to_hup, surge_color, tqec, Rgb};
pub use tqec_syndrome::{render_syndrome_field_cpu, SyndromeField};