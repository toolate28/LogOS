//! reson8-tui library surface.
//!
//! Circuit types, lattice probe, and QR metaprogramming live here so
//! `cargo test -p reson8-tui --lib` actually runs. The interactive
//! dashboard is the `reson8-forge` binary (`main.rs`).
//!
//! Lean twins: `lean/TriWeavon/QuantumRedstone.lean`,
//! `lean/TriWeavon/LatticeLayers.lean`.

pub mod app;
pub mod codes;
pub mod git_lab;
pub mod human_actions;
pub mod lattice;
pub mod layout_presets;
pub mod lsp;
pub mod net_proxy;
pub mod phase_evolution;
pub mod qr_meta;
pub mod smoke;
pub mod strands;
pub mod surface;
pub mod ui;
