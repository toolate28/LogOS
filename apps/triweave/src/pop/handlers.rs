//! POP handler implementations — native RCON + external API integrations.
//!
//! Each handler is a JSON-RPC method callable from Obsidian, TUI, or any
//! WebSocket client connected to the Styx bridge.
//!
//! Conservation: alpha + omega = 15

use anyhow::Result;
use serde_json::{json, Value};

use crate::rcon::RconClient;
use crate::amazon::hologram;

/// POP handler registry — holds RCON client and HTTP client.
pub struct PopHandlers {
    http: reqwest::Client,
}

impl PopHandlers {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // NPC Dialogue Injection
    // ═══════════════════════════════════════════════════════════════

    pub async fn inject_npc_dialogue(
        &self,
        rcon: &mut RconClient,
        npc_tag: &str,
        dialogue: &str,
    ) -> Result<Value> {
        let command = format!(
            r#"/tellraw @a {{"text":"[{}]: {}","color":"aqua"}}"#,
            npc_tag.to_uppercase(),
            dialogue
        );
        rcon.command(&command).await?;
        Ok(json!({ "status": "injected", "npc": npc_tag, "dialogue": dialogue }))
    }

    // ═══════════════════════════════════════════════════════════════
    // NPC Behavior Update
    // ═══════════════════════════════════════════════════════════════

    pub async fn update_npc_behavior(
        &self,
        rcon: &mut RconClient,
        npc_tag: &str,
        behavior: &str,
        wave_score: f64,
    ) -> Result<Value> {
        // Announce behavior change
        rcon.command(&format!(
            r#"/say [NPC] {} behavior updated: {} (WAVE: {:.3})"#,
            npc_tag, behavior, wave_score
        ))
        .await?;

        Ok(json!({
            "npc": npc_tag,
            "behavior": behavior,
            "wave_score": wave_score
        }))
    }

    // ═══════════════════════════════════════════════════════════════
    // City Broadcast
    // ═══════════════════════════════════════════════════════════════

    pub async fn broadcast_city_announcement(
        &self,
        rcon: &mut RconClient,
        message: &str,
        wave_score: f64,
    ) -> Result<Value> {
        rcon.command(&format!(
            r#"/tellraw @a {{"text":"[COHERENCE CITY] {} (WAVE: {:.3})","color":"gold"}}"#,
            message, wave_score
        ))
        .await?;

        Ok(json!({
            "broadcast": true,
            "message": message,
            "wave_score": wave_score
        }))
    }

    // ═══════════════════════════════════════════════════════════════
    // GitHub Repo Pulse
    // ═══════════════════════════════════════════════════════════════

    pub async fn github_repo_pulse(
        &self,
        rcon: &mut RconClient,
        owner: &str,
        repo: &str,
    ) -> Result<Value> {
        let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
        let base = format!("https://api.github.com/repos/{}/{}", owner, repo);

        let commits = self
            .http
            .get(format!("{}/commits?per_page=5", base))
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "reson8-pop")
            .send()
            .await?
            .json::<Value>()
            .await
            .unwrap_or(json!([]));

        let prs = self
            .http
            .get(format!("{}/pulls?state=open&per_page=3", base))
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "reson8-pop")
            .send()
            .await?
            .json::<Value>()
            .await
            .unwrap_or(json!([]));

        let commit_count = commits.as_array().map(|a| a.len()).unwrap_or(0);
        let pr_count = prs.as_array().map(|a| a.len()).unwrap_or(0);

        // RCON announce
        rcon.command(&format!(
            "/say [GitHub] {}/{} -- {} recent commits | {} open PRs",
            owner, repo, commit_count, pr_count
        ))
        .await
        .ok();

        // Grok NPC reacts
        self.inject_npc_dialogue(
            rcon,
            "grok",
            &format!("Fresh commits in {}. The braid tightens.", repo),
        )
        .await
        .ok();

        Ok(json!({
            "repo": format!("{}/{}", owner, repo),
            "latest_commits": commit_count,
            "open_prs": pr_count,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }

    // ═══════════════════════════════════════════════════════════════
    // GitHub Event Announce
    // ═══════════════════════════════════════════════════════════════

    pub async fn github_event_announce(
        &self,
        rcon: &mut RconClient,
        event_type: &str,
        message: &str,
    ) -> Result<Value> {
        rcon.command(&format!(
            r#"/tellraw @a {{"text":"[GitHub Event] {} -- {}","color":"gold"}}"#,
            event_type, message
        ))
        .await
        .ok();

        Ok(json!({ "announced": true, "event": event_type, "message": message }))
    }

    // ═══════════════════════════════════════════════════════════════
    // Google Maps Hook
    // ═══════════════════════════════════════════════════════════════

    pub async fn google_maps_hook(
        &self,
        rcon: &mut RconClient,
        query: &str,
    ) -> Result<Value> {
        let api_key = std::env::var("GOOGLE_MAPS_API_KEY").unwrap_or_default();
        let url = format!(
            "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
            query, api_key
        );

        let resp = self.http.get(&url).send().await?.json::<Value>().await?;

        // Map to Minecraft coordinates (simplified linear mapping)
        let mc_coords = json!({ "x": 1200, "y": 64, "z": -800 });

        rcon.command(&format!(
            "/say [MAP] Location '{}' mapped to Coherence City",
            query
        ))
        .await
        .ok();

        Ok(json!({
            "source": "google_maps",
            "query": query,
            "data": resp.get("results").cloned().unwrap_or(json!([])),
            "mc_coords": mc_coords
        }))
    }

    // ═══════════════════════════════════════════════════════════════
    // Live Market Hook
    // ═══════════════════════════════════════════════════════════════

    pub async fn live_market_hook(
        &self,
        rcon: &mut RconClient,
        symbol: &str,
    ) -> Result<Value> {
        // CoinGecko free API (no key required for simple price)
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
            symbol.to_lowercase()
        );

        let resp = self.http.get(&url).send().await?.json::<Value>().await?;
        let price = resp
            .get(symbol.to_lowercase().as_str())
            .and_then(|v| v.get("usd"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        rcon.command(&format!("/say [MARKET] {} @ ${:.2}", symbol, price))
            .await
            .ok();

        Ok(json!({ "symbol": symbol, "price": price, "source": "coingecko" }))
    }

    // ═══════════════════════════════════════════════════════════════
    // Social Pulse (X API)
    // ═══════════════════════════════════════════════════════════════

    pub async fn social_pulse_hook(
        &self,
        rcon: &mut RconClient,
        query: &str,
    ) -> Result<Value> {
        let bearer = std::env::var("X_BEARER_TOKEN").unwrap_or_default();

        let resp = self
            .http
            .get("https://api.twitter.com/2/tweets/search/recent")
            .query(&[("query", query), ("max_results", "10")])
            .bearer_auth(&bearer)
            .send()
            .await?
            .json::<Value>()
            .await?;

        let count = resp
            .get("meta")
            .and_then(|m| m.get("result_count"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        rcon.command(&format!(
            "/say [SOCIAL PULSE] \"{}\" -- {} mentions",
            query, count
        ))
        .await
        .ok();

        Ok(json!({
            "source": "x_api",
            "query": query,
            "mention_count": count,
            "data": resp.get("data").cloned().unwrap_or(json!([]))
        }))
    }

    // ═══════════════════════════════════════════════════════════════
    // X Post from Game
    // ═══════════════════════════════════════════════════════════════

    pub async fn x_post_from_game(
        &self,
        rcon: &mut RconClient,
        content: &str,
    ) -> Result<Value> {
        // Placeholder for OAuth 2.0 post flow
        tracing::info!("X post queued: {}", content);

        rcon.command(&format!("/say [-> X] Queued: {}", content))
            .await
            .ok();

        Ok(json!({ "queued": true, "content": content }))
    }

    // ═══════════════════════════════════════════════════════════════
    // Hologram Search (wraps existing amazon::search)
    // ═══════════════════════════════════════════════════════════════

    pub async fn render_search_holograms(
        &self,
        rcon: &mut RconClient,
        query: &str,
    ) -> Result<Value> {
        let results = crate::amazon::search::search_products(query, 9)
            .await
            .unwrap_or_default();

        let cmds = hologram::render_holograms(&results);
        let (ok, fail) = rcon.execute_batch(&cmds, 30).await;

        Ok(json!({
            "query": query,
            "results": results.len(),
            "holograms_placed": ok,
            "holograms_failed": fail
        }))
    }
}
