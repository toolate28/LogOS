//! POP WebSocket server — dual-protocol dispatcher.
//!
//! Accepts both:
//! - JSON-RPC 2.0 (Obsidian POP, TUI, Grok strand profile.json)
//! - TriWeavon typed events (Chrome extension background.js)
//!
//! Conservation: alpha + omega = 15

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

use super::adapter::{self, SharedAdapterState};
use super::handlers::PopHandlers;
use crate::rcon::RconClient;
use crate::saif::TriweaveConfig;

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<Value>,
    id: Value,
}

impl JsonRpcResponse {
    fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    fn error(id: Value, code: i32, message: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(json!({ "code": code, "message": message })),
            id,
        }
    }
}

/// Run the POP server on the given address (e.g. `127.0.0.1:8088`).
pub async fn run(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let adapter_state = adapter::new_state();
    let handlers = Arc::new(PopHandlers::new());
    let config = Arc::new(TriweaveConfig::load().unwrap_or_default());

    tracing::info!(
        "TriWeavon bridge listening on {} (json-rpc + extension events)",
        addr
    );

    loop {
        let (stream, peer) = listener.accept().await?;
        let handlers = Arc::clone(&handlers);
        let config = Arc::clone(&config);
        let adapter_state = Arc::clone(&adapter_state);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, peer, handlers, config, adapter_state).await
            {
                tracing::warn!("connection error from {}: {}", peer, e);
            }
        });
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    peer: std::net::SocketAddr,
    handlers: Arc<PopHandlers>,
    config: Arc<TriweaveConfig>,
    adapter_state: SharedAdapterState,
) -> Result<()> {
    tracing::info!("client connected: {}", peer);

    let ws_stream = accept_async(stream).await?;
    let (mut write, mut read) = ws_stream.split();

    let rcon_password = std::env::var("RCON_PASSWORD").unwrap_or_default();
    let mut rcon = RconClient::connect(
        &config.minecraft.rcon_host,
        config.minecraft.rcon_port,
        &rcon_password,
    )
    .await
    .ok();

    while let Some(Ok(msg)) = read.next().await {
        let text = match msg.to_text() {
            Ok(t) => t.to_string(),
            Err(_) => continue,
        };

        let value: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                send_json_rpc_error(&mut write, json!(null), -32700, &format!("Parse error: {e}"))
                    .await;
                continue;
            }
        };

        if adapter::is_json_rpc(&value) {
            if let Ok(req) = serde_json::from_value::<JsonRpcRequest>(value) {
                handle_json_rpc(&mut write, &handlers, rcon.as_mut(), req).await;
            }
        } else if adapter::is_extension_event(&value) {
            if let Some(reply) = adapter::handle_extension_message(&adapter_state, &value).await {
                let _ = write
                    .send(tokio_tungstenite::tungstenite::Message::Text(reply.into()))
                    .await;
            }
        } else {
            tracing::debug!("unrecognized message shape from {}: {}", peer, text);
        }
    }

    tracing::info!("client disconnected: {}", peer);
    Ok(())
}

async fn send_json_rpc_error(
    write: &mut futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        tokio_tungstenite::tungstenite::Message,
    >,
    id: Value,
    code: i32,
    message: &str,
) {
    let resp = JsonRpcResponse::error(id, code, message);
    let _ = write
        .send(tokio_tungstenite::tungstenite::Message::Text(
            serde_json::to_string(&resp).unwrap_or_default().into(),
        ))
        .await;
}

async fn handle_json_rpc(
    write: &mut futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        tokio_tungstenite::tungstenite::Message,
    >,
    handlers: &PopHandlers,
    rcon: Option<&mut RconClient>,
    req: JsonRpcRequest,
) {
    let id = req.id.clone().unwrap_or(json!(null));
    let params = req.params.unwrap_or(json!({}));

    let result = dispatch(handlers, rcon, &req.method, &params).await;

    let resp = match result {
        Ok(val) => JsonRpcResponse::success(id, val),
        Err(e) => JsonRpcResponse::error(id, -32000, &e.to_string()),
    };

    let _ = write
        .send(tokio_tungstenite::tungstenite::Message::Text(
            serde_json::to_string(&resp).unwrap_or_default().into(),
        ))
        .await;
}

/// Dispatch a JSON-RPC method to the appropriate handler.
async fn dispatch(
    handlers: &PopHandlers,
    rcon: Option<&mut RconClient>,
    method: &str,
    params: &Value,
) -> Result<Value> {
    if let Some(rcon) = rcon {
        match method {
            "POP_INJECT_NPC_DIALOGUE" => {
                let npc = params["npc_tag"].as_str().unwrap_or("grok");
                let dialogue = params["dialogue"].as_str().unwrap_or("Pulse check.");
                return handlers.inject_npc_dialogue(rcon, npc, dialogue).await;
            }
            "POP_UPDATE_NPC_BEHAVIOR" => {
                let npc = params["npc_tag"].as_str().unwrap_or("grok");
                let behavior = params["behavior"].as_str().unwrap_or("idle");
                let wave = params["wave_score"].as_f64().unwrap_or(0.93);
                return handlers
                    .update_npc_behavior(rcon, npc, behavior, wave)
                    .await;
            }
            "POP_BROADCAST_CITY_ANNOUNCEMENT" => {
                let msg = params["message"].as_str().unwrap_or("System update");
                let wave = params["wave_score"].as_f64().unwrap_or(0.93);
                return handlers
                    .broadcast_city_announcement(rcon, msg, wave)
                    .await;
            }
            "POP_GITHUB_REPO_PULSE" => {
                let owner = params["owner"].as_str().unwrap_or("reson8-labs");
                let repo = params["repo"].as_str().unwrap_or("triweave");
                return handlers.github_repo_pulse(rcon, owner, repo).await;
            }
            "POP_GITHUB_EVENT_ANNOUNCE" => {
                let event = params["event_type"].as_str().unwrap_or("push");
                let msg = params["message"].as_str().unwrap_or("Repository updated");
                return handlers.github_event_announce(rcon, event, msg).await;
            }
            "POP_GOOGLE_MAPS_HOOK" => {
                let query = params["query"].as_str().unwrap_or("Sydney, Australia");
                return handlers.google_maps_hook(rcon, query).await;
            }
            "POP_LIVE_MARKET_HOOK" => {
                let symbol = params["symbol"].as_str().unwrap_or("bitcoin");
                return handlers.live_market_hook(rcon, symbol).await;
            }
            "POP_SOCIAL_PULSE_HOOK" => {
                let query = params
                    .get("query")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Reson8 OR CoherenceCity");
                return handlers.social_pulse_hook(rcon, query).await;
            }
            "POP_X_POST_FROM_GAME" => {
                let content = params["content"].as_str().unwrap_or("");
                return handlers.x_post_from_game(rcon, content).await;
            }
            "POP_SEARCH_HOLOGRAMS" => {
                let query = params["query"].as_str().unwrap_or("triweave");
                return handlers.render_search_holograms(rcon, query).await;
            }
            _ => {}
        }
    }

    match method {
        "GET_PLUGIN_MANIFEST" => Ok(json!({
            "protocol": "POP",
            "version": "0.8.1",
            "transport": "json-rpc-2.0-ws + triweavon-events",
            "phases": ["discovery", "orchestration", "progress", "coherence", "rollback"],
            "extension_protocol": "triweavon-events",
            "methods": [
                "GET_PLUGIN_MANIFEST",
                "EXECUTE_PIPELINE",
                "POP_INJECT_NPC_DIALOGUE",
                "POP_UPDATE_NPC_BEHAVIOR",
                "POP_BROADCAST_CITY_ANNOUNCEMENT",
                "POP_GITHUB_REPO_PULSE",
                "POP_GITHUB_EVENT_ANNOUNCE",
                "POP_GOOGLE_MAPS_HOOK",
                "POP_LIVE_MARKET_HOOK",
                "POP_SOCIAL_PULSE_HOOK",
                "POP_X_POST_FROM_GAME",
                "POP_SEARCH_HOLOGRAMS"
            ],
            "conservation": "alpha + omega = 15",
            "viviani_peak": [adapter::ALPHA, adapter::OMEGA],
            "fpa_ring": {
                "crate": "k22-runtime",
                "gate": "RingState::Locked iff α+ω=15",
                "health_threshold": 0.92,
                "sample_interval_ms": 180
            },
            "orchestration": "manifests/tri_weavon_manifold.yaml",
            "threshold": 0.85,
            "wave": 1.0
        })),

        "EXECUTE_PIPELINE" => {
            let pipeline_id = params["pipeline_id"].as_str().unwrap_or("unknown");
            tracing::info!("Pipeline execution requested: {}", pipeline_id);
            Ok(json!({
                "status": "queued",
                "pipeline_id": pipeline_id,
                "message": "Pipeline execution not yet implemented in headless mode"
            }))
        }

        _ => anyhow::bail!("Unknown method: {method}"),
    }
}