use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use serde_json::json;
use tracing::{info, error, warn};

use crate::models::{JsonRpcRequest, JsonRpcResponse, ExecutePipelineParams, OutgoingMessage, StepCompleteParams, StepCompleteOutput};
use crate::router;
use crate::{BridgeCommand, WaveEngine, AtomTracker};

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8088";
    let listener = TcpListener::bind(&addr).await?;
    info!("🌀 VORTEX BRIDGE ACTIVE: Listening on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().unwrap_or(SocketAddr::from(([0, 0, 0, 0], 0)));
        tokio::spawn(handle_connection(peer, stream));
    }

    Ok(())
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during websocket handshake with {}: {}", peer, e);
            return;
        }
    };
    
    info!("New Obsidian POP connection established: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Setup dummy WaveEngine & AtomTracker for scaffolding
    let wave_engine = WaveEngine;
    let atom_tracker = AtomTracker;

    while let Some(msg_result) = ws_receiver.next().await {
        let msg = match msg_result {
            Ok(m) => m,
            Err(e) => {
                error!("Error receiving message from {}: {}", peer, e);
                break;
            }
        };

        if let Message::Text(text) = msg {
            info!("Received payload from {}: {}", peer, text);
            // Parse JSON-RPC Payload
            let req: JsonRpcRequest = match serde_json::from_str(&text) {
                Ok(r) => r,
                Err(e) => {
                    warn!("Invalid JSON-RPC payload from {}: {}. Payload: {}", peer, e, text);
                    continue;
                }
            };
            
            match req.method.as_str() {
                "GET_PLUGIN_MANIFEST" | "DISCOVERY" => {
                    info!("Discovery phase initiated.");
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".into(),
                        id: req.id.clone(),
                        result: Some(json!({ "status": "CONNECTED", "strand": "vortex-bridge (Rust)", "version": "0.1.0" })),
                        error: None,
                    };
                    let _ = ws_sender.send(Message::Text(serde_json::to_string(&resp).unwrap())).await;
                }
                "EXECUTE_PIPELINE" => {
                    info!("Orchestrating pipeline execution. Triggering WAVE gate.");
                    // Extract the params we expect from the generic payload
                    // If parsing specific struct fails on standard execute, we fallback to a generic raw content for routing
                    let cmd_content = if let Some(p) = &req.params {
                         if let Ok(params) = serde_json::from_value::<ExecutePipelineParams>(p.clone()) {
                             serde_json::to_string(&params.steps).unwrap_or_else(|_| "[]".to_string())
                         } else {
                             p.to_string()
                         }
                    } else {
                        "".to_string()
                    };

                    let cmd = BridgeCommand {
                        content: cmd_content,
                        parent_knot: req.atom_token.clone().unwrap_or_else(|| "ROOT-OBSIDIAN".to_string()),
                    };
                    
                    // --- 4π Double-Cover ATOM Initialization ---
                    info!("⚛️ Engaging ATOM Tracker (4π Double-Cover Ledger) for execution intent");
                    let _tracked = atom_tracker.record_grok_response(
                        &format!("INTENT: EXECUTE_PIPELINE (Pre-Braid Initialization)"),
                        1.0, 
                        &cmd.parent_knot,
                    ).await;
                    
                    // Route to Grok via router layer
                    match router::route_and_braid("grok", cmd, &wave_engine, &atom_tracker).await {
                        Ok(braid_event) => {
                            // Send STEP_COMPLETE back down
                            let out_msg = OutgoingMessage::StepComplete(StepCompleteParams {
                                output: StepCompleteOutput {
                                    content: braid_event.content,
                                    wave_score: Some(braid_event.wave_score),
                                },
                                knot_id: braid_event.knot_id,
                            });
                            let out_json = serde_json::to_string(&out_msg).unwrap();
                            let _ = ws_sender.send(Message::Text(out_json)).await;
                        }
                        Err(e) => {
                            error!("Braid error: {:?}", e);
                            // Return PIPELINE_FAILED optionally
                        }
                    }
                }
                _ => {
                    warn!("Unknown POP method: {}", req.method);
                }
            }
        }
    }
    
    info!("POP connection closed: {}", peer);
}
