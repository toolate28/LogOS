//! Dual-protocol adapter: JSON-RPC 2.0 vs Chrome-extension events.
//!
//! Category **C** telemetry. Conservation constants are labels, not gates.

use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

/// Viviani-peak labels (Category C).
pub const ALPHA: u8 = 7;
pub const OMEGA: u8 = 8;

pub struct AdapterState {
    pub connected: bool,
}

pub type SharedAdapterState = Arc<Mutex<AdapterState>>;

pub fn new_state() -> SharedAdapterState {
    Arc::new(Mutex::new(AdapterState { connected: false }))
}

pub fn is_json_rpc(value: &Value) -> bool {
    value.get("jsonrpc").and_then(|v| v.as_str()) == Some("2.0") && value.get("method").is_some()
}

pub fn is_extension_event(value: &Value) -> bool {
    if is_json_rpc(value) {
        return false;
    }
    value.get("type").is_some()
        || value.get("event").is_some()
        || value.get("kind").and_then(|v| v.as_str()) == Some("triweavon-event")
}

pub async fn handle_extension_message(
    state: &SharedAdapterState,
    value: &Value,
) -> Option<String> {
    if let Ok(mut s) = state.lock() {
        s.connected = true;
    }
    let typ = value
        .get("type")
        .and_then(|v| v.as_str())
        .or_else(|| value.get("event").and_then(|v| v.as_str()))
        .unwrap_or("unknown");
    Some(
        json!({
            "protocol": "triweavon-events",
            "type": "ack",
            "ack": typ,
            "ok": true,
            "conservation": {
                "alpha": ALPHA,
                "omega": OMEGA,
                "sum": ALPHA + OMEGA,
                "category": "C"
            }
        })
        .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_json_rpc() {
        let v = json!({"jsonrpc":"2.0","method":"GET_PLUGIN_MANIFEST","id":1});
        assert!(is_json_rpc(&v));
        assert!(!is_extension_event(&v));
    }

    #[test]
    fn classifies_extension_event() {
        let v = json!({"type":"ping","kind":"triweavon-event"});
        assert!(!is_json_rpc(&v));
        assert!(is_extension_event(&v));
    }
}
