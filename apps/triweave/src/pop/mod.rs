//! POP — Plugin Orchestration Protocol.
//!
//! JSON-RPC 2.0 over WebSocket (ws://127.0.0.1:8088).
//! Five-phase pipeline model: Discovery -> Orchestration -> Progress -> Coherence -> Rollback.
//!
//! Bridges Obsidian, RCON, Cloudflare, GitHub, X API, Google Maps,
//! and live market data into a unified orchestration surface.
//!
//! Conservation: alpha + omega = 15

pub mod adapter;
pub mod handlers;
pub mod server;
