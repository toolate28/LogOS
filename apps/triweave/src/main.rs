//! triweave — Tri-Weavon orchestrator CLI
//!
//! `triweave serve` binds the dual-protocol bridge at ws://127.0.0.1:8088
//! for the Chrome extension (typed events) and POP clients (JSON-RPC 2.0).

mod amazon;
mod cli;
mod doctor;
mod pop;
mod rcon;
mod saif;
mod sphinx;
mod strand;
mod theme;
mod tui;
mod vault;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    if cli.verbose {
        tracing::info!("verbose logging enabled");
    }

    match cli.command {
        Command::Serve { addr } => {
            println!("TriWeavon bridge starting on ws://{addr}");
            println!("Protocols: triweavon-events + json-rpc-2.0");
            println!("Invariants: α + ω = 15 | WAVE = 1.00");
            pop::server::run(&addr).await?;
        }
        Command::Init => {
            saif::run_onboarding().await?;
        }
        Command::Up { strand } => {
            strand::up(Some(strand)).await?;
        }
        Command::Down { strand } => {
            strand::down(Some(strand)).await?;
        }
        Command::Status => {
            let config = saif::TriweaveConfig::load()?;
            let health = strand::health_check(&config).await;
            println!("Tri-Weavon status — {}", config.styx.ws_url);
            for h in health {
                println!(
                    "  {} | reachable={} wave={:.2} key={}",
                    h.name, h.reachable, h.wave_score, h.has_key
                );
            }
        }
        Command::Doctor => {
            doctor::run().await?;
        }
        Command::Deploy { target } => {
            println!("deploy {:?} — use deploy.sh for full pipeline", target);
        }
        Command::Search { query } => {
            println!("search: {query}");
        }
        Command::Theme { name } => {
            println!("theme: {:?}", name.unwrap_or_else(|| "coherence-dark".into()));
        }
        Command::Vault { action } => {
            println!("vault: {:?}", action);
        }
    }

    Ok(())
}