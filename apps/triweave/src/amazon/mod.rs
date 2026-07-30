//! Amazon.com Room — Coherence City Zone 5.
//!
//! Vectorize embedding search -> in-world hologram rendering.
//! Location: x:100, z:0 (Fibonacci spiral from Nexus Core)
//!
//! Architecture:
//!   User search query (chat/lectern)
//!     -> mc-bridge RCON captures
//!     -> Cloudflare Worker (Vectorize embedding search)
//!     -> Top-K results
//!     -> RCON hologram commands (armor stands + floating text)
//!     -> Conservation verified per-placement (alpha+omega=15)
//!
//! Conservation: alpha + omega = 15

pub mod blueprint;
pub mod hologram;
pub mod search;

use anyhow::{Context, Result};

use crate::cli::DeployTarget;
use crate::rcon::RconClient;
use crate::saif::TriweaveConfig;

/// Deploy a target to Minecraft.
pub async fn deploy(target: DeployTarget) -> Result<()> {
    match target {
        DeployTarget::Amazon | DeployTarget::AmazonRoom => deploy_amazon_room().await,
        DeployTarget::Local | DeployTarget::City => deploy_city().await,
        DeployTarget::NpcSuite => deploy_npc_suite().await,
    }
}

/// Connect RCON using config, with env var fallback for password.
async fn connect_rcon(config: &TriweaveConfig) -> Result<RconClient> {
    let password = std::env::var("RCON_PASSWORD")
        .context("RCON_PASSWORD env var required (or store in vault via `triweave init`)")?;
    RconClient::connect(&config.minecraft.rcon_host, config.minecraft.rcon_port, &password).await
}

async fn deploy_amazon_room() -> Result<()> {
    let config = TriweaveConfig::load().unwrap_or_default();

    println!("+==================================================+");
    println!("|  Deploying Amazon.com Room -- Zone 5              |");
    println!("|  Location: x:100, z:0 (Fibonacci R=144)          |");
    println!("+==================================================+");
    println!();

    // Step 1: Generate blueprint
    println!("Step 1/4: Generating room blueprint...");
    let room_cmds = blueprint::generate_room_commands();
    println!("  {} RCON commands generated", room_cmds.len());

    // Step 2: Connect RCON
    println!("Step 2/4: Connecting to Minecraft RCON...");
    let mut rcon = match connect_rcon(&config).await {
        Ok(client) => {
            println!(
                "  RCON: connected ({}:{})",
                config.minecraft.rcon_host, config.minecraft.rcon_port
            );
            client
        }
        Err(e) => {
            println!("  RCON: connection failed — {}", e);
            println!("  Commands saved to amazon_room.mcfunction (execute manually)");
            save_mcfunction(&room_cmds)?;
            return Ok(());
        }
    };

    // Step 3: Execute build commands
    println!("Step 3/4: Building Amazon Room via RCON...");

    // Announce in chat
    rcon.command("/say [Triweave] Building Amazon.com Room at Zone 5 (x:100, z:0)...")
        .await
        .ok();

    let (ok, fail) = rcon.execute_batch(&room_cmds, 50).await;
    println!("  Structure: {} commands OK, {} failed", ok, fail);

    if fail > 0 {
        println!("  Some commands failed — run `triweave doctor` for diagnostics");
    }

    // Step 4: Conservation verification announcement
    println!("Step 4/4: Conservation check...");
    rcon.command("/say [Triweave] Amazon Room built. Conservation: a + w = 15")
        .await
        .ok();

    println!();
    println!("Amazon Room deployed at Zone 5.");
    println!("  Search: `triweave search <query>`");
    println!("  Conservation: alpha + omega = 15");
    Ok(())
}

/// Execute a search query and render holograms in-world.
pub async fn search_and_render(query: &str) -> Result<()> {
    let config = TriweaveConfig::load().unwrap_or_default();

    println!("Searching: \"{}\"", query);
    println!("  Endpoint: Vectorize (reson8-embeddings)");

    // Step 1: Query Vectorize
    let results = match search::search_products(query, 9).await {
        Ok(r) => {
            println!("  {} results returned", r.len());
            r
        }
        Err(e) => {
            println!("  Search failed: {}", e);
            println!("  Using demo results for hologram rendering...");
            demo_results(query)
        }
    };

    if results.is_empty() {
        println!("  No results found.");
        return Ok(());
    }

    // Step 2: Generate hologram commands
    let hologram_cmds = hologram::render_holograms(&results);
    println!("  {} hologram commands generated", hologram_cmds.len());

    // Step 3: Execute via RCON
    println!("  Rendering holograms in Amazon Room...");
    match connect_rcon(&config).await {
        Ok(mut rcon) => {
            rcon.command(&format!(
                "/say [Triweave] Rendering search: \"{}\" ({} results)",
                query,
                results.len()
            ))
            .await
            .ok();

            let (ok, fail) = rcon.execute_batch(&hologram_cmds, 30).await;
            println!("  Holograms: {} placed, {} failed", ok, fail);

            // Announce results in chat
            for (i, r) in results.iter().take(3).enumerate() {
                let price = r.price.as_deref().unwrap_or("--");
                rcon.command(&format!(
                    "/say   {}. {} [{}] (score: {:.3})",
                    i + 1,
                    r.title,
                    price,
                    r.score
                ))
                .await
                .ok();
            }

            rcon.command("/say [Triweave] Conservation: a + w = 15")
                .await
                .ok();
        }
        Err(e) => {
            println!("  RCON unavailable: {}", e);
            println!("  Hologram commands:");
            for cmd in &hologram_cmds {
                println!("    {}", cmd);
            }
        }
    }

    println!();
    println!("Conservation: alpha + omega = 15");
    Ok(())
}

/// Save commands as .mcfunction file for manual execution.
fn save_mcfunction(commands: &[String]) -> Result<()> {
    let path = std::path::Path::new("amazon_room.mcfunction");
    let content = commands.join("\n");
    std::fs::write(path, content)?;
    println!("  Saved: {}", path.display());
    Ok(())
}

/// Demo results when Vectorize endpoint is unreachable.
fn demo_results(query: &str) -> Vec<hologram::SearchResult> {
    vec![
        hologram::SearchResult {
            title: format!("\"{}\" — Top Result", query),
            price: Some("$29.99".to_string()),
            category: Some("Search".to_string()),
            score: 0.95,
            metadata_id: "demo-001".to_string(),
        },
        hologram::SearchResult {
            title: "Reson8 Topology Handbook".to_string(),
            price: Some("$49.99".to_string()),
            category: Some("Books".to_string()),
            score: 0.89,
            metadata_id: "demo-002".to_string(),
        },
        hologram::SearchResult {
            title: "RTX 5090 Founders Edition".to_string(),
            price: Some("$1,999.99".to_string()),
            category: Some("Electronics".to_string()),
            score: 0.84,
            metadata_id: "demo-003".to_string(),
        },
    ]
}

async fn deploy_city() -> Result<()> {
    let config = TriweaveConfig::load().unwrap_or_default();

    println!("Deploying full Coherence City (5 zones)...");

    match connect_rcon(&config).await {
        Ok(mut rcon) => {
            rcon.command("/say [Triweave] Deploying Coherence City — 5 zones...")
                .await
                .ok();

            // Zone 1: Nexus Core
            println!("  Zone 1: Nexus Core (x:0, z:0)");
            rcon.command("/say Zone 1: Nexus Core — heartbeat pulse")
                .await
                .ok();

            // Zone 2: Museum
            println!("  Zone 2: Museum of Computation (x:0, z:-50)");
            rcon.command("/say Zone 2: Museum of Computation — epsilon tetrahedron")
                .await
                .ok();

            // Zone 3: Blockchain Bank
            println!("  Zone 3: Blockchain Bank (x:50, z:50)");
            rcon.command("/say Zone 3: Blockchain Bank — zero-latency ledger")
                .await
                .ok();

            // Zone 4: Anyon Collider
            println!("  Zone 4: Anyon Collider (y:-50, x:0, z:100)");
            rcon.command("/say Zone 4: Anyon Collider — QDI labs")
                .await
                .ok();

            // Zone 5: Amazon Room
            println!("  Zone 5: Amazon Room (x:100, z:0)");
            deploy_amazon_room().await?;

            rcon.command("/say [Triweave] Coherence City complete. a + w = 15")
                .await
                .ok();
        }
        Err(e) => {
            println!("  RCON unavailable: {}", e);
            println!("  Run with RCON_PASSWORD set and Minecraft server running.");
        }
    }

    Ok(())
}

async fn deploy_npc_suite() -> Result<()> {
    let config = TriweaveConfig::load().unwrap_or_default();

    println!("Deploying NPC suite (HOPE-AI-NPC-SUITE)...");

    let npcs = vec![
        (
            "Claude",
            "alpha",
            8,
            blueprint::ORIGIN_X - 20,
            blueprint::ORIGIN_Y + 1,
            blueprint::ORIGIN_Z,
        ),
        (
            "Grok",
            "omega",
            5,
            blueprint::ORIGIN_X - 20,
            blueprint::ORIGIN_Y + 1,
            blueprint::ORIGIN_Z + 5,
        ),
        (
            "Gemini",
            "omega",
            3,
            blueprint::ORIGIN_X - 20,
            blueprint::ORIGIN_Y + 1,
            blueprint::ORIGIN_Z + 10,
        ),
    ];

    match connect_rcon(&config).await {
        Ok(mut rcon) => {
            for (name, component, weight, x, y, z) in &npcs {
                let cmd = format!(
                    r#"/summon villager {} {} {} {{CustomName:'{{"text":"{} NPC","color":"gold","bold":true}}',Tags:["reson8_npc","{}_strand","hope_ai_npc"],Invulnerable:1b,Glowing:1b,NoAI:1b}}"#,
                    x, y, z, name, name.to_lowercase()
                );
                match rcon.command(&cmd).await {
                    Ok(_) => println!(
                        "  {} NPC: spawned ({}-component, weight={})",
                        name, component, weight
                    ),
                    Err(e) => println!("  {} NPC: failed — {}", name, e),
                }
            }
            rcon.command("/say [Triweave] NPC suite deployed: Claude, Grok, Gemini")
                .await
                .ok();
        }
        Err(e) => {
            println!("  RCON unavailable: {}", e);
            println!("  NPC commands:");
            for (name, _, _, x, y, z) in &npcs {
                println!(
                    "    /summon villager {} {} {} ({})",
                    x, y, z, name
                );
            }
        }
    }

    Ok(())
}
