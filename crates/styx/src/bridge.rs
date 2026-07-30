//! Core Styx bridge — WebSocket server with SPHINX gating.

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tracing::{error, info, warn};

/// Default bind address for the Styx bridge.
pub const DEFAULT_ADDR: &str = "127.0.0.1:8088";

/// Inbound request envelope.
#[derive(Debug, Deserialize)]
pub struct StyxRequest {
    pub id: Option<String>,
    pub braid_word: Option<Vec<i32>>,
    pub jones_fingerprint: Option<String>,
    pub payload: Option<serde_json::Value>,
}

/// Outbound response envelope.
#[derive(Debug, Serialize)]
pub struct StyxResponse {
    pub id: Option<String>,
    #[serde(flatten)]
    pub body: StyxResponseBody,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum StyxResponseBody {
    Pass {
        result: StyxPassResult,
    },
    Reject {
        error: &'static str,
        reason: &'static str,
        expected: String,
    },
    Error {
        error: String,
    },
}

#[derive(Debug, Serialize)]
pub struct StyxPassResult {
    pub status: &'static str,
    pub jones_fingerprint: String,
    pub gate: &'static str,
}

/// Run the Styx bridge on the given address.
pub async fn serve(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("[Styx Bridge] SPHINX active on ws://{addr}");

    loop {
        let (stream, peer) = listener.accept().await?;
        info!("[Styx] Connection from {peer}");

        tokio::spawn(async move {
            let ws = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    error!("[Styx] Handshake failed: {e}");
                    return;
                }
            };

            let (mut tx, mut rx) = ws.split();

            while let Some(msg) = rx.next().await {
                let msg = match msg {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("[Styx] Read error: {e}");
                        break;
                    }
                };

                if !msg.is_text() {
                    continue;
                }

                let text = msg.into_text().unwrap_or_default();
                let response = handle_message(&text);

                let out = serde_json::to_string(&response).unwrap_or_default();
                if tx
                    .send(tokio_tungstenite::tungstenite::Message::Text(out.into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });
    }
}

/// Process a single inbound message through SPHINX gating.
fn handle_message(text: &str) -> StyxResponse {
    let req: StyxRequest = match serde_json::from_str(text) {
        Ok(r) => r,
        Err(e) => {
            return StyxResponse {
                id: None,
                body: StyxResponseBody::Error {
                    error: format!("invalid_json: {e}"),
                },
            };
        }
    };

    // Derive braid word from payload or use explicit
    let braid_word = req.braid_word.unwrap_or_else(|| {
        let payload_str = req
            .payload
            .as_ref()
            .map(|p| serde_json::to_string(p).unwrap_or_default())
            .unwrap_or_default();
        sphinx::payload_to_braid_word(&payload_str)
    });

    let fp = sphinx::compute_fingerprint(&braid_word, 3);

    // SPHINX gate: verify provided fingerprint matches computed
    match &req.jones_fingerprint {
        Some(provided) if *provided == fp.fingerprint => StyxResponse {
            id: req.id,
            body: StyxResponseBody::Pass {
                result: StyxPassResult {
                    status: "sphinx_pass",
                    jones_fingerprint: format!("{}...", &fp.fingerprint[..16]),
                    gate: "JONES_LOCK_CONFIRMED",
                },
            },
        },
        Some(_) => StyxResponse {
            id: req.id,
            body: StyxResponseBody::Reject {
                error: "SPHINX_REJECT",
                reason: "jones_polynomial_mismatch",
                expected: format!("{}...", &fp.fingerprint[..16]),
            },
        },
        None => {
            // No fingerprint provided — return the computed one for handshake
            StyxResponse {
                id: req.id,
                body: StyxResponseBody::Pass {
                    result: StyxPassResult {
                        status: "sphinx_handshake",
                        jones_fingerprint: fp.fingerprint,
                        gate: "FINGERPRINT_ISSUED",
                    },
                },
            }
        }
    }
}
