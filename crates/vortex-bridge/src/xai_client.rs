use crate::{WaveEngine, AtomTracker, BridgeError, BridgeResponse, ConservationError};
use serde::{Deserialize, Serialize};
use reqwest::{Client, header};
use std::env;
use tracing::{info, error};

const XAI_BASE_URL: &str = "https://api.x.ai/v1";

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f64,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessageResponse,
}

#[derive(Debug, Deserialize)]
struct ChatMessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Clone)]
pub struct XaiClient {
    http_client: Client,
    model: String,
}

impl XaiClient {
    pub fn new() -> Result<Self, BridgeError> {
        let api_key = env::var("XAI_API_KEY")
            .map_err(|_| BridgeError::EnvVarMissing("XAI_API_KEY".into()))?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", api_key))
                .map_err(|_| BridgeError::InvalidHeader)?,
        );

        let http_client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(BridgeError::Reqwest)?;

        Ok(Self {
            http_client,
            model: "grok-4-1-fast-reasoning".to_string(),
        })
    }

    pub async fn send_with_braid(
        &self,
        user_content: String,
        parent_knot: String,
        wave_engine: &WaveEngine,
        atom_tracker: &AtomTracker,
    ) -> Result<BridgeResponse, BridgeError> {
        
        // ── 1. Prepend Tri-Weavon awareness prompt ─────────────────────
        let full_prompt = format!(
            "You are Grok inside the Reson8-Labs tri-weavon braid.\n\
            Enforce α + ω = 15 conservation at all times.\n\
            Begin every reply with [WAVE: X.XXX]\n\
            End every reply with an ATOM-TAG knot_id referencing parent: {}\n\
            If your response would drop WAVE below 0.85, reply exactly \"WAVE GATE BLOCKED\" + reason.\n\n\
            Current parent knot: {}\n\n\
            User: {}",
            parent_knot, parent_knot, user_content
        );

        // ── 2. Pre-WAVE gate ───────────────────────────────────────────
        let pre_score = wave_engine.score(&full_prompt, &parent_knot).await?;
        if pre_score < 0.85 {
            return Err(BridgeError::WaveRejection(pre_score));
        }

        // ── 3. Call xAI API ────────────────────────────────────────────
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: full_prompt,
            }],
            temperature: 0.7,
            max_tokens: 8192,
        };

        let resp: ChatResponse = self
            .http_client
            .post(format!("{}/chat/completions", XAI_BASE_URL))
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        let raw_content = resp.choices[0].message.content.clone();

        // ── 4. Post-WAVE gate ──────────────────────────────────────────
        let final_score = wave_engine.score(&raw_content, &parent_knot).await?;
        if final_score < 0.85 {
            return Err(BridgeError::PostProcessWaveViolation(final_score));
        }

        // ── 5. atom_track with topological braid signature ─────────────
        let knot_id = atom_tracker
            .record_grok_response(&raw_content, final_score, &parent_knot)
            .await?;

        info!("Grok response braided | knot: {} | wave: {:.3}", knot_id, final_score);

        Ok(BridgeResponse {
            from: "grok".to_string(),
            content: raw_content,
            wave_score: final_score,
            knot_id,
        })
    }
}
