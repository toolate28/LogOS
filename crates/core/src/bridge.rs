//! WebSocket bridge — bidirectional conduit between TUI and coherence-mcp.
//!
//! The bridge connects to `ws://127.0.0.1:8088` and exchanges typed events:
//! - **Inbound** (from coherence-mcp): Telemetry, Coherence, PipelineProgress, AtomEvent
//! - **Outbound** (to coherence-mcp): Commands (trigger pipeline, store context, etc.)

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tracing::{info, warn};

// ---------------------------------------------------------------------------
// Inbound event types (coherence-mcp → TUI)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeEvent {
    Connected,
    Disconnected,
    Telemetry(TelemetryPayload),
    Coherence(CoherencePayload),
    PipelineProgress(PipelinePayload),
    AtomEvent(AtomPayload),
    ExecuteMcpTool { tool_name: String, args: serde_json::Value, req_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPayload {
    pub sensors: SensorData,
    pub health: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub cpu_temp: f64,
    pub gpu_temp: f64,
    pub power_draw: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherencePayload {
    pub pipeline_id: String,
    pub step_id: String,
    pub wave_score: f64,
    pub components: CoherenceComponents,
    pub conservation: crate::protocol::ConservationState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceComponents {
    pub lexical_diversity: f64,
    pub curl: f64,
    pub divergence: f64,
    pub potential: f64,
    pub entropy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelinePayload {
    pub pipeline_name: String,
    pub current_step: String,
    pub completed_steps: u8,
    pub total_steps: u8,
    pub percent: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomPayload {
    pub atom_type: String,
    pub gate: String,
    pub description: String,
}

// ---------------------------------------------------------------------------
// Outbound command types (TUI → coherence-mcp)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeCommand {
    TriggerPipeline { name: String, params: serde_json::Value },
    StoreContext { key: String, value: serde_json::Value },
    RequestCoherence { pipeline_id: String },
    McpToolResult { req_id: String, result: serde_json::Value },
    Ping,
}

// ---------------------------------------------------------------------------
// BridgeHandle — the async interface the TUI holds
// ---------------------------------------------------------------------------

pub struct BridgeHandle {
    /// Receive inbound events from coherence-mcp.
    pub rx: mpsc::Receiver<BridgeEvent>,
    /// Send outbound commands to coherence-mcp.
    pub cmd: mpsc::Sender<BridgeCommand>,
}

impl BridgeHandle {
    /// Spawn the WebSocket bridge task. Returns immediately with the handle.
    pub fn spawn(url: &str) -> Self {
        let (event_tx, event_rx) = mpsc::channel::<BridgeEvent>(256);
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<BridgeCommand>(256);
        let url = url.to_string();

        tokio::spawn(async move {
            loop {
                match connect_async(&url).await {
                    Ok((ws_stream, _)) => {
                        let _ = event_tx.send(BridgeEvent::Connected).await;
                        info!("Bridge connected to {}", url);

                        let (mut write, mut read) = ws_stream.split();

                        loop {
                            tokio::select! {
                                // Inbound: parse JSON from coherence-mcp
                                msg = read.next() => {
                                    match msg {
                                        Some(Ok(msg)) => {
                                            if let Ok(text) = msg.to_text() {
                                                if let Ok(ev) = serde_json::from_str::<BridgeEvent>(text) {
                                                    let _ = event_tx.send(ev).await;
                                                }
                                            }
                                        }
                                        _ => {
                                            let _ = event_tx.send(BridgeEvent::Disconnected).await;
                                            break;
                                        }
                                    }
                                }
                                // Outbound: serialize command → JSON
                                cmd = cmd_rx.recv() => {
                                    if let Some(cmd) = cmd {
                                        if let Ok(json) = serde_json::to_string(&cmd) {
                                            let _ = write.send(
                                                tokio_tungstenite::tungstenite::Message::Text(json.into())
                                            ).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Bridge connection failed: {} — retrying in 3s", e);
                        let _ = event_tx.send(BridgeEvent::Disconnected).await;
                    }
                }
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
        });

        Self { rx: event_rx, cmd: cmd_tx }
    }
}
