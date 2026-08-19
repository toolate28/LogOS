//! reson8-activator — Skill activation, routing, and composition
//!
//! Live surface: [`ops_caps`] (TUI kit probe). `router` / `pipeline` /
//! `awesome_skill` stay on-disk orphans until restored with full types.
pub mod router { /* Intent-to-skill routing */ }
pub mod compose { /* Skill chain composition */ }
pub mod registry { /* Skill registry management */ }

pub mod ops_caps;
pub use ops_caps::{probe_ops_caps, CapProbe, OpsCap};
