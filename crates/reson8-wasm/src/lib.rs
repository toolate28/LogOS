//! reson8-wasm — WebAssembly edge bindings for Cloudflare Workers
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn enforce_invariant(alpha: f64, omega: f64) -> JsValue {
    let total = alpha + omega;
    let passed = (total - 15.0).abs() <= 0.3;
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "status": if passed { "PASSED" } else { "REJECTED" },
        "total": total,
        "deviation": (total - 15.0).abs()
    })).unwrap_or(JsValue::NULL)
}
