//! # barcode-tui
//!
//! Terminal-native persistent-homology barcode viewer — the first Rust TUI
//! to render live Vietoris–Rips H0 barcodes under a swept ε filtration.
//!
//! See [`BARCODE-TUI-FIXED-POINT.md`](../../../BARCODE-TUI-FIXED-POINT.md)
//! for the constitutional specification and per-strand pickup points.
//!
//! ## Universal Invariant
//!
//! Every render tick computes `reson8_core::coherence_functional(...)` and
//! must satisfy α + ω = 15 ± 0.3. Any breach flashes the status bar magenta
//! and rejects the tick.
//!
//! ## Public surface (stable — do not break without a new ATOM entry)
//!
//! ```rust,ignore
//! use barcode_tui::{AppState, Barcode, Cloud, compute_h0, edges, generate};
//! ```

pub const CRATE_VERSION: &str = "0.1.0";
pub const FIXED_POINT_ATOM: &str = "ATOM-BARCODE-TUI-FIXED-POINT-20260418";

pub mod app;
pub mod ph;
pub mod point_cloud;
pub mod render;
pub mod vr;

pub use app::AppState;
pub use ph::{compute_h0, Barcode};
pub use point_cloud::{generate, Cloud};
pub use render::{draw_barcodes, draw_cloud, draw_status};
pub use vr::{edges, Edge};
