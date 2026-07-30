//! reson8-forge-core — compatibility crate for TUI and triweave.
//!
//! Forge modules live in `reson8-core`; this crate preserves the
//! `reson8_forge_core::` import path expected by reson8-tui.

pub use reson8_core::adapter;
pub use reson8_core::agent;
pub use reson8_core::bridge;
pub use reson8_core::bus;
pub use reson8_core::capability;
pub use reson8_core::logic;
pub use reson8_core::memory;
pub use reson8_core::orchestrator;
pub use reson8_core::protocol;
pub use reson8_core::superskill;
pub use reson8_core::task;