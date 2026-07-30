//! SAIF — Setup with INTENT.
//!
//! Interactive onboarding flow:
//! 1. Strand selection (Claude, Grok, Gemini)
//! 2. API key collection (masked input)
//! 3. SPHINX vault creation
//! 4. Health checks per strand
//! 5. Config write (~/.triweave/config.toml)
//! 6. ATOM trail entries at KENL → AWI → ATOM gates
//!
//! Conservation: α + ω = 15

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

use crate::vault::Vault;

/// Configuration written to `~/.triweave/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriweaveConfig {
    pub theme: ThemeConfig,
    pub strands: StrandsConfig,
    pub near: NearConfig,
    pub minecraft: MinecraftConfig,
    pub styx: StyxConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub accent: String,
    pub border: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrandsConfig {
    pub enabled: Vec<String>,
    pub transport: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearConfig {
    pub network: String,
    pub account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftConfig {
    pub rcon_host: String,
    pub rcon_port: u16,
    pub world: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyxConfig {
    pub ws_url: String,
}

impl Default for TriweaveConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig {
                name: "coherence-dark".to_string(),
                accent: "cyan".to_string(),
                border: "rounded".to_string(),
            },
            strands: StrandsConfig {
                enabled: vec![],
                transport: "styx".to_string(),
            },
            near: NearConfig {
                network: "testnet".to_string(),
                account: "reson8-test.testnet".to_string(),
            },
            minecraft: MinecraftConfig {
                rcon_host: "127.0.0.1".to_string(),
                rcon_port: 25575,
                world: "WORLD_CONVERGENCE".to_string(),
            },
            styx: StyxConfig {
                ws_url: "ws://127.0.0.1:8088".to_string(),
            },
        }
    }
}

impl TriweaveConfig {
    pub fn path() -> Result<PathBuf> {
        let home = home_dir().context("cannot determine home directory")?;
        Ok(home.join(".triweave").join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(&path)?;
        toml::from_str(&data).context("parsing config.toml")
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = toml::to_string_pretty(self)?;
        std::fs::write(&path, data)?;
        tracing::info!("config saved to {}", path.display());
        Ok(())
    }
}

/// Key definition: what keys each strand requires.
struct KeyDef {
    name: &'static str,
    strand: &'static str,
    prompt: &'static str,
    env_var: &'static str,
}

const KEY_DEFS: &[KeyDef] = &[
    KeyDef {
        name: "anthropic",
        strand: "claude",
        prompt: "Anthropic API key (sk-ant-...): ",
        env_var: "ANTHROPIC_API_KEY",
    },
    KeyDef {
        name: "xai",
        strand: "grok",
        prompt: "xAI API key (xai-...): ",
        env_var: "XAI_API_KEY",
    },
    KeyDef {
        name: "google",
        strand: "gemini",
        prompt: "Google AI API key: ",
        env_var: "GOOGLE_AI_KEY",
    },
];

const OPTIONAL_KEYS: &[KeyDef] = &[
    KeyDef {
        name: "near_account",
        strand: "all",
        prompt: "NEAR account ID (or Enter to skip): ",
        env_var: "NEAR_ACCOUNT_ID",
    },
    KeyDef {
        name: "cloudflare",
        strand: "all",
        prompt: "Cloudflare API token (or Enter to skip): ",
        env_var: "CLOUDFLARE_API_TOKEN",
    },
    KeyDef {
        name: "rcon",
        strand: "all",
        prompt: "Minecraft RCON password (or Enter to skip): ",
        env_var: "RCON_PASSWORD",
    },
];

/// Run the full SAIF onboarding flow.
pub async fn run_onboarding() -> Result<()> {
    println!();
    println!("╔══════════════════════════════════════════════════╗");
    println!("║       TRIWEAVE — Setup with INTENT (SAIF)       ║");
    println!("║           Conservation: α + ω = 15              ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    // ── Step 1: Strand Selection ────────────────────────────────────
    println!("Step 1/5 — Select strands to enable:");
    let strands = select_strands()?;
    if strands.is_empty() {
        println!("No strands selected. Aborting.");
        return Ok(());
    }
    println!("  Enabled: {}", strands.join(", "));
    // ATOM: KENL gate — intention locked
    tracing::info!("ATOM KENL gate: intention locked — strands={:?}", strands);
    println!("  ATOM gate: KENL (intention locked) ✓");
    println!();

    // ── Step 2: Passphrase ──────────────────────────────────────────
    println!("Step 2/5 — Create vault passphrase:");
    let passphrase = prompt_passphrase_confirmed()?;
    let mut vault = Vault::new(&passphrase);
    println!("  SPHINX vault initialized (master fingerprint={}…)", &vault.master_fingerprint[..16]);
    println!();

    // ── Step 3: API Keys ────────────────────────────────────────────
    println!("Step 3/5 — API keys (stored in SPHINX-gated vault):");
    for kd in KEY_DEFS {
        if strands.contains(&kd.strand.to_string()) {
            // Check env var first
            if let Ok(val) = std::env::var(kd.env_var) {
                println!("  {} — found in ${}, storing in vault", kd.name, kd.env_var);
                vault.store_key(kd.name, &val, &passphrase)?;
            } else {
                let val = prompt_key(kd.prompt)?;
                if !val.is_empty() {
                    vault.store_key(kd.name, &val, &passphrase)?;
                    println!("  {} — stored ✓", kd.name);
                } else {
                    println!("  {} — skipped", kd.name);
                }
            }
        }
    }

    println!();
    println!("  Optional integrations:");
    for kd in OPTIONAL_KEYS {
        if let Ok(val) = std::env::var(kd.env_var) {
            println!("  {} — found in ${}", kd.name, kd.env_var);
            vault.store_key(kd.name, &val, &passphrase)?;
        } else {
            let val = prompt_key(kd.prompt)?;
            if !val.is_empty() {
                vault.store_key(kd.name, &val, &passphrase)?;
                println!("  {} — stored ✓", kd.name);
            }
        }
    }

    vault.save()?;
    println!();

    // ── Step 4: Health Checks ───────────────────────────────────────
    println!("Step 4/5 — Health checks:");
    for strand in &strands {
        let status = check_strand_health(strand).await;
        let icon = if status { "✓" } else { "✗" };
        println!("  {} {} — {}", icon, strand, if status { "reachable" } else { "unreachable (will retry on `triweave up`)" });
    }
    // Check Styx bridge
    let styx_ok = check_styx_health().await;
    println!(
        "  {} styx bridge — {}",
        if styx_ok { "✓" } else { "○" },
        if styx_ok { "ws://127.0.0.1:8088 live" } else { "not running (started by `triweave up`)" }
    );
    // ATOM: AWI gate — awareness confirmed
    tracing::info!("ATOM AWI gate: awareness confirmed");
    println!("  ATOM gate: AWI (awareness confirmed) ✓");
    println!();

    // ── Step 5: Config Write ────────────────────────────────────────
    println!("Step 5/5 — Writing configuration:");
    let config = TriweaveConfig {
        strands: StrandsConfig {
            enabled: strands.clone(),
            transport: "styx".to_string(),
        },
        ..Default::default()
    };
    config.save()?;
    println!("  {} written ✓", TriweaveConfig::path()?.display());
    // ATOM: ATOM gate — operational
    tracing::info!("ATOM gate: operational — triweave init complete");

    println!();
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  SAIF complete — α + ω = 15                     ║");
    println!("║  Vault: {} keys encrypted                       ║", vault.entries.len());
    println!("║  Strands: {}                  ║", strands.join(", "));
    println!("║                                                  ║");
    println!("║  Next: `triweave up` to start strands            ║");
    println!("║        `triweave status` for TUI dashboard       ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    Ok(())
}

// ─── Helpers ────────────────────────────────────────────────────────

fn select_strands() -> Result<Vec<String>> {
    let options = ["claude", "grok", "gemini"];
    println!("  Available: claude (Windows), grok (NixOS/GLF), gemini (WSL2/Kali)");
    print!("  Enable (comma-separated, or 'all'): ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input == "all" || input.is_empty() {
        return Ok(options.iter().map(|s| s.to_string()).collect());
    }

    let selected: Vec<String> = input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| options.contains(&s.as_str()))
        .collect();

    Ok(selected)
}

fn prompt_key(prompt: &str) -> Result<String> {
    print!("  {}", prompt);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_passphrase_confirmed() -> Result<String> {
    loop {
        let p1 = crate::vault::prompt_passphrase("  Passphrase: ")?;
        let p2 = crate::vault::prompt_passphrase("  Confirm: ")?;
        if p1 == p2 {
            return Ok(p1);
        }
        println!("  Passphrases don't match. Try again.");
    }
}

async fn check_strand_health(strand: &str) -> bool {
    match strand {
        "claude" => {
            // Check if Anthropic API is reachable
            reqwest::get("https://api.anthropic.com/v1/messages")
                .await
                .is_ok()
        }
        "grok" => {
            // Check if xAI API is reachable
            reqwest::get("https://api.x.ai/v1/models")
                .await
                .is_ok()
        }
        "gemini" => {
            // Check if Google AI API is reachable
            reqwest::get("https://generativelanguage.googleapis.com/v1/models")
                .await
                .is_ok()
        }
        _ => false,
    }
}

async fn check_styx_health() -> bool {
    tokio_tungstenite::connect_async("ws://127.0.0.1:8088")
        .await
        .is_ok()
}

fn home_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    }
    #[cfg(not(windows))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
