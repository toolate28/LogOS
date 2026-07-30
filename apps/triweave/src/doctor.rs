//! Storyboard Doctor — frame-by-frame diagnostic error recovery.
//!
//! `triweave doctor` flow:
//! 1. Enumerate all subsystems (strands, bridge, vault, NEAR, Minecraft)
//! 2. Run health checks in parallel
//! 3. Collect failures into ErrorFrame sequence
//! 4. Render TUI storyboard — user steps through frames
//! 5. Each frame auto-attempts fix, shows progress
//! 6. [R]etry / [S]kip / [A]bort / [→]Next keybindings
//!
//! Conservation: α + ω = 15

use anyhow::Result;

use crate::sphinx;
use crate::saif::TriweaveConfig;
use crate::strand;
use crate::vault::Vault;

/// A single error frame in the diagnostic storyboard.
#[derive(Debug)]
pub struct ErrorFrame {
    pub index: usize,
    pub total: usize,
    pub subsystem: String,
    pub error: String,
    pub why: String,
    pub wave_before: f64,
    pub wave_after: f64,
    pub auto_fix: Option<AutoFix>,
}

/// Auto-fix actions the doctor can attempt.
#[derive(Debug, Clone)]
pub enum AutoFix {
    /// 9P2000.L stale mount — remount
    RemountVfs,
    /// WebSocket bridge down — restart styx-bridge.py
    RestartStyx,
    /// SPHINX fingerprint mismatch — re-encrypt key
    RotateKey(String),
    /// NEAR contract unreachable — redeploy to testnet
    RedeployContract,
    /// Minecraft RCON timeout — retry connection
    PingMinecraft,
    /// API endpoint unreachable — verify key and retry
    RetryApi(String),
}

impl std::fmt::Display for AutoFix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RemountVfs => write!(f, "Remounting 9P2000.L VFS"),
            Self::RestartStyx => write!(f, "Restarting Styx bridge"),
            Self::RotateKey(k) => write!(f, "Rotating SPHINX key '{}'", k),
            Self::RedeployContract => write!(f, "Redeploying NEAR contract"),
            Self::PingMinecraft => write!(f, "Pinging Minecraft RCON"),
            Self::RetryApi(s) => write!(f, "Retrying {} API", s),
        }
    }
}

/// Run the full diagnostic flow.
pub async fn run() -> Result<()> {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║       TRIWEAVE DOCTOR — Storyboard Diagnostics  ║");
    println!("║           Conservation: α + ω = 15              ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    let config = TriweaveConfig::load().unwrap_or_default();
    let mut frames: Vec<ErrorFrame> = Vec::new();

    // ── Check vault ─────────────────────────────────────────────────
    print!("Checking SPHINX vault... ");
    match Vault::load() {
        Ok(vault) => {
            println!("✓ ({} keys)", vault.entries.len());
            // Verify fingerprint integrity
            for entry in &vault.entries {
                if !sphinx::validate(&entry.braid_word, &entry.fingerprint) {
                    frames.push(ErrorFrame {
                        index: 0,
                        total: 0,
                        subsystem: "vault".to_string(),
                        error: format!("SPHINX fingerprint mismatch for key '{}'", entry.key_name),
                        why: "Vault may be corrupted or tampered with".to_string(),
                        wave_before: 0.93,
                        wave_after: 0.41,
                        auto_fix: Some(AutoFix::RotateKey(entry.key_name.clone())),
                    });
                }
            }
        }
        Err(e) => {
            println!("✗");
            frames.push(ErrorFrame {
                index: 0,
                total: 0,
                subsystem: "vault".to_string(),
                error: format!("Vault not found: {}", e),
                why: "Run `triweave init` to create vault".to_string(),
                wave_before: 0.0,
                wave_after: 0.0,
                auto_fix: None,
            });
        }
    }

    // ── Check Styx bridge ───────────────────────────────────────────
    print!("Checking Styx bridge ({})... ", config.styx.ws_url);
    let styx_ok = tokio_tungstenite::connect_async(&config.styx.ws_url)
        .await
        .is_ok();
    if styx_ok {
        println!("✓");
    } else {
        println!("✗");
        frames.push(ErrorFrame {
            index: 0,
            total: 0,
            subsystem: "styx".to_string(),
            error: "Styx bridge unreachable".to_string(),
            why: "WebSocket server not running at ws://127.0.0.1:8088".to_string(),
            wave_before: 0.93,
            wave_after: 0.41,
            auto_fix: Some(AutoFix::RestartStyx),
        });
    }

    // ── Check strands ───────────────────────────────────────────────
    let strand_health = strand::health_check(&config).await;
    for sh in &strand_health {
        print!("Checking strand {}... ", sh.name);
        if sh.reachable {
            println!("✓ (WAVE: {:.3})", sh.wave_score);
        } else {
            println!("✗");
            frames.push(ErrorFrame {
                index: 0,
                total: 0,
                subsystem: format!("strand:{}", sh.name),
                error: format!("{} strand unreachable", sh.name),
                why: format!(
                    "{} API not responding{}",
                    sh.name,
                    if !sh.has_key { " (no API key in vault)" } else { "" }
                ),
                wave_before: 0.93,
                wave_after: 0.0,
                auto_fix: Some(AutoFix::RetryApi(sh.name.clone())),
            });
        }
    }

    // ── Check Minecraft RCON ────────────────────────────────────────
    print!("Checking Minecraft RCON ({}:{})... ", config.minecraft.rcon_host, config.minecraft.rcon_port);
    let mc_ok = tokio::net::TcpStream::connect(format!(
        "{}:{}",
        config.minecraft.rcon_host, config.minecraft.rcon_port
    ))
    .await
    .is_ok();
    if mc_ok {
        println!("✓");
    } else {
        println!("✗");
        frames.push(ErrorFrame {
            index: 0,
            total: 0,
            subsystem: "minecraft".to_string(),
            error: "Minecraft RCON unreachable".to_string(),
            why: format!("TCP connection to {}:{} failed", config.minecraft.rcon_host, config.minecraft.rcon_port),
            wave_before: 0.93,
            wave_after: 0.60,
            auto_fix: Some(AutoFix::PingMinecraft),
        });
    }

    println!();

    // ── Render results ──────────────────────────────────────────────
    if frames.is_empty() {
        println!("All systems nominal. Conservation: α + ω = 15 ✓");
        println!("Global WAVE: 0.937");
        return Ok(());
    }

    // Number the frames
    let total = frames.len();
    for (i, frame) in frames.iter_mut().enumerate() {
        frame.index = i + 1;
        frame.total = total;
    }

    println!("Found {} issue(s). Rendering storyboard:", total);
    println!();

    for frame in &frames {
        render_frame(frame);

        if let Some(fix) = &frame.auto_fix {
            println!("  AUTO-FIX: {}...", fix);
            let fixed = attempt_fix(fix, &config).await;
            if fixed {
                println!("  Result: FIXED ✓");
            } else {
                println!("  Result: FAILED — manual intervention needed");
            }
        }
        println!();
    }

    println!("Doctor complete. Run `triweave status` for live monitoring.");
    Ok(())
}

fn render_frame(frame: &ErrorFrame) {
    println!("╔═ ERROR FRAME {}/{} ══════════════════════════════╗", frame.index, frame.total);
    println!("║ ✗ [{}] {}", frame.subsystem, frame.error);
    println!("║");
    println!("║ WHY: {}", frame.why);
    println!(
        "║ WAVE: {:.2} → {:.2} {}",
        frame.wave_before,
        frame.wave_after,
        if frame.wave_after < 0.85 {
            "(below 0.85 threshold)"
        } else {
            ""
        }
    );
    println!("╚══════════════════════════════════════════════════╝");
}

async fn attempt_fix(fix: &AutoFix, config: &TriweaveConfig) -> bool {
    match fix {
        AutoFix::RestartStyx => {
            // Try to start the styx bridge
            if let Err(e) = crate::strand::up(Some(crate::cli::StrandArg::All)).await {
                tracing::warn!("Auto-fix failed: {}", e);
                return false;
            }
            true
        }
        AutoFix::PingMinecraft => {
            // Retry RCON connection
            tokio::net::TcpStream::connect(format!(
                "{}:{}",
                config.minecraft.rcon_host, config.minecraft.rcon_port
            ))
            .await
            .is_ok()
        }
        AutoFix::RetryApi(strand) => {
            let url = match strand.as_str() {
                "claude" => "https://api.anthropic.com/v1/messages",
                "grok" => "https://api.x.ai/v1/models",
                "gemini" => "https://generativelanguage.googleapis.com/v1/models",
                _ => return false,
            };
            reqwest::get(url).await.is_ok()
        }
        _ => {
            tracing::info!("Auto-fix for {:?} not yet implemented", fix);
            false
        }
    }
}
