//! reson8-forge (RESON8-TUI) — real-time TUI for multi-AI pipeline orchestration.
//!
//! Tagline: *Where the last thing you've done becomes the first thing you need*
//!
//! Phase evolution: quantum-redstone gates × SPHINX (KENL→AWI→ATOM→SAIF)
//! mapped onto TaskPhase, orchestrated as Tokio/Ratatui sequences.
//!
//! Conservation law: alpha + omega = 15 on every pipeline event (Category C).
//! Residual R = max(0, 15−α−ω) observed in Braid (Tier 1) — not deploy-green.
//!
//! QDI laws (ATOM-GROK-TUI-QR-META-20260806 + drain audit 20260807):
//! - H drain is **bounded** (`DRAIN_BUDGET`) — never spin past draw.
//! - `engine.handle` runs in **ε only**, budgeted (`ENGINE_HANDLE_BUDGET`), named ack.
//!
//! Event loop: H drain → RS-NOR latch → ε draw → ε effects → H key poll

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use tokio::sync::mpsc;

use reson8_forge_core::bridge::BridgeHandle;
use reson8_forge_core::superskill::{SuperskillEngine, SuperskillEvent};

use reson8_tui::lsp::{self, LspEvent};
use reson8_tui::phase_evolution;
use reson8_tui::qr_meta::{
    self, active_latch, drain_dust, intent_from_key, CircuitIntent, Effect, LatchLayer,
    DEFAULT_POLL_MS, DRAIN_BUDGET, ENGINE_HANDLE_BUDGET,
};
use reson8_tui::{app, ui};

#[tokio::main]
async fn main() -> io::Result<()> {
    // ── Help flag ────────────────────────────────────────────────────────────
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        println!("RESON8-TUI (reson8-forge) — multi-AI pipeline + phase evolution");
        println!();
        println!("  \"{}\"", phase_evolution::TAGLINE);
        println!();
        println!("USAGE: reson8-forge [OPTIONS]");
        println!();
        println!("ENVIRONMENT:");
        println!("  FORGE_WS_URL     WebSocket URL (default: ws://127.0.0.1:8088)");
        println!("  RESON8_LAYOUT    Workflow layout (ops|formal|agent|monitor|quantum|minimal|codes|hitl)");
        println!("  FORGE_LAYOUT     Alias for RESON8_LAYOUT");
        println!();
        println!("KEYBINDINGS:");
        println!("  Tab     cycle panel focus (within layout)");
        println!("  l / L   next / previous workflow layout");
        println!("  1-8     jump: 1 ops  2 formal  3 agent  4 monitor  5 quantum  6 minimal  7 codes  8 hitl");
        println!("  ? / h   agent-terminal + panel help (in-frame)");
        println!("  s       full-spiral sequence (KENL→AWI→ATOM→SAIF)");
        println!("  g       gate tick (one quantum-redstone / SPHINX step)");
        println!("  p       paper-draft sequence + pipeline");
        println!("  n       idea-to-publish sequence + pipeline");
        println!("  t       mapped test harness (phase-gated)");
        println!("  u       HITL gate (request → escalate → approve) on first_need ⚑");
        println!("  o / O   focus / reload SAIF human-action queue");
        println!("  G / V   git observe refresh / fetch --prune (no commit, no push)");
        println!("  f       focus Formal (auto-switches layout if needed)");
        println!("  c       focus Codes lab (Hexacode · Golay G24 · Reed–Muller · SC-LDPC)");
        println!("  d / D   codes family demo / multi-family decode battery");
        println!("  y / e   cycle code family / bump inject-error weight");
        println!("  [ ]     RM r or SC w −/+   {{ }}  RM m or SC L −/+");
        println!("  N       focus Net proxy stack panel");
        println!("  R       refresh net port probe");
        println!("  M       net menu: privacy / gaming clearnet / stop / install");
        println!("  q       quit");
        println!();
        println!("WORKFLOW LAYOUTS:");
        println!("  ops       full dashboard (default) — includes Net panel");
        println!("  formal    Lean/Agda eye — formal top, support row bottom");
        println!("  agent     providers · logs · braid · formal");
        println!("  monitor   braid · net · logs · tests");
        println!("  quantum   QR×SPHINX phases + braid/formal/logs");
        println!("  minimal   braid + logs");
        println!("  codes     Hexacode · Golay G24 · Reed–Muller · SC-LDPC lab");
        println!("  hitl      SAIF ⚑ queue — request · escalate · approve (receipt only)");
        println!();
        println!("CODES LAB (runtime Category B; Lean HexacodeGolay construction A):");
        println!("  Hexacode  [6,3,4]_4 MDS · t=1 symbol · syndrome/NN");
        println!("  Golay G24 [24,12,8] · t=3 · MOG + hex scores + R_parity · NN");
        println!("  RM(r,m)   n=2^m · dual RM(m-r-1,m) · FHT for RM(1,m)");
        println!("  SC-LDPC   chain L · couple w · window W · lift Z · threshold sat.");
        println!("  note      G24 is NOT RM and NOT SC-LDPC");
        println!();
        println!("QDI / residual (Track A — observe only):");
        println!("  drain budget {}/source · engine handle budget {}/frame", DRAIN_BUDGET, ENGINE_HANDLE_BUDGET);
        println!("  R=max(0,15-α-ω) on Braid · ε={} · Category C · not cert.pass", qr_meta::RESIDUAL_EPS);
        println!();
        println!("NET PROXY (ops/net/LogOS.NetProxy.ps1):");
        println!("  privacy  Tor:9050 Privoxy:8118 i2pd:4444 DNSCrypt:5353");
        println!("  gaming   clearnet high-speed — no Tor/I2P");
        println!();
        println!("FORMAL / LSP:");
        println!("  Lean: lake serve | lean --server  (LOGOS_ROOT or cwd)");
        println!("  Agda: als  [CATEGORY B until installed]");
        println!("  Diagnostics: amber SlowStep for sorry; B stubs never green");
        println!();
        println!("Part of the Reson8-Labs ecosystem — https://reson8labs.ai");
        println!("Conservation law: alpha + omega = 15 (Category C label)");
        return Ok(());
    }

    // ── Terminal setup ────────────────────────────────────────────────────────
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut app = app::App::new();

    // ── Bridge — configurable via FORGE_WS_URL env var ───────────────────────
    let ws_url = std::env::var("FORGE_WS_URL")
        .unwrap_or_else(|_| "ws://127.0.0.1:8088".to_string());
    let mut bridge = BridgeHandle::spawn(&ws_url);

    // ── Superskill engine — enforces alpha+omega=15 invariant ────────────────
    let (ss_ev_tx, mut ss_ev_rx) = mpsc::channel::<SuperskillEvent>(256);
    let mut engine = SuperskillEngine::new(bridge.cmd.clone(), ss_ev_tx);

    // ── LSP bus (decoupled) — Formal pane is the eye of the needle ───────────
    let (lsp_tx, mut lsp_rx) = mpsc::channel::<LspEvent>(256);
    let logos_root = std::env::var("LOGOS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    let _lean_conn = lsp::try_attach_lean(&logos_root, lsp_tx.clone()).await;
    let _als_conn = lsp::try_attach_agda(lsp_tx.clone()).await;

    // ── Main event loop (QR circuit: H → CNOT → RS-NOR → ε) ─────────────────
    while app.running {
        // ── H · HadamardDrain (KENL) — superpose ready dust, never block ─────
        // Bridge: App latch only. Engine fan-out deferred to ε (named ack).
        let mut drained = 0usize;
        for _ in 0..DRAIN_BUDGET {
            match bridge.rx.try_recv() {
                Ok(ev) => {
                    app.handle_bridge_event(&ev);
                    app.enqueue_engine_bridge(ev);
                    drained += 1;
                }
                Err(_) => break,
            }
        }
        drained += drain_dust! {
            ss_ev_rx => |ev| app.handle_superskill_event(ev),
            lsp_rx => |ev| app.handle_lsp_event(ev),
        };
        app.bridge.last_drain_count = drained;

        // ── RS-NOR · latch housekeeping (notif TTL + residual C) ─────────────
        app.tick_notifications();

        // ── ε · EpsilonMeasure — projective draw (read-only widgets) ─────────
        terminal.draw(|frame| ui::draw(frame, &app))?;

        // ── ε · post-draw effects: budgeted engine.handle (ack: engine wire) ─
        // ATOM: isochronic fork closed — bounded + after draw, not in H.
        for _ in 0..ENGINE_HANDLE_BUDGET {
            let Some(ev) = app.engine_pending.pop_front() else {
                break;
            };
            engine.handle(ev).await;
        }
        app.bridge.engine_queue_depth = app.engine_pending.len();

        // ── H · key dust (QDI poll budget) ───────────────────────────────────
        if event::poll(Duration::from_millis(DEFAULT_POLL_MS))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                let Some(intent) = intent_from_key(key.code) else {
                    continue;
                };

                // ── RS-NOR · overlay priority mask ───────────────────────────
                let layer = active_latch(app.popup_blocking(), app.help_open);
                let allowed = match layer {
                    LatchLayer::BlockingPopup => matches!(
                        intent,
                        CircuitIntent::Quit
                            | CircuitIntent::PopupConfirm
                            | CircuitIntent::PopupDismiss
                            | CircuitIntent::PopupUp
                            | CircuitIntent::PopupDown
                    ),
                    LatchLayer::HelpOverlay => {
                        matches!(
                            intent,
                            CircuitIntent::Quit
                                | CircuitIntent::ToggleHelp
                                | CircuitIntent::PopupDismiss
                        )
                    }
                    LatchLayer::Normal => !matches!(
                        intent,
                        CircuitIntent::PopupConfirm | CircuitIntent::PopupDismiss
                    ),
                };
                if !allowed {
                    continue;
                }

                if layer == LatchLayer::HelpOverlay {
                    match intent {
                        CircuitIntent::Quit => {
                            app.quit();
                        }
                        CircuitIntent::ToggleHelp | CircuitIntent::PopupDismiss => {
                            app.help_open = false;
                        }
                        _ => {}
                    }
                    continue;
                }

                // ── CNOT · entangle intent with App ──────────────────────────
                let effect = app.apply_intent(intent);

                // ── ε · post-latch effects (network / engine trigger) ────────
                match effect {
                    Effect::Side { name } => {
                        let _ = engine.trigger(name, serde_json::json!({})).await;
                        if matches!(name, "paper-draft" | "idea-to-publish") {
                            for _ in 0..4 {
                                app.gate_tick();
                            }
                        }
                    }
                    Effect::Net { action } => {
                        app.push_notification(
                            app::NotifLevel::Info,
                            "net",
                            format!("running {}…", action.id()),
                        );
                        // Blocking controller call is intentional (HITL-confirmed).
                        let summary = app.net.run_action(action);
                        app.push_notification(app::NotifLevel::Success, "net", summary);
                        app.focus_panel(app::FocusPanel::Net);
                    }
                    Effect::EngineBridge { event } => {
                        // Direct ε path if a reducer ever emits one.
                        engine.handle(event).await;
                    }
                    Effect::Hitl { decision, action_id } => {
                        let root = logos_root.clone();
                        let summary = app.actions.write_receipt(&root, decision, &action_id);
                        if let Some(item) = app.actions.items.iter().find(|a| a.id == action_id) {
                            if !item.reply.is_empty() {
                                app.log_sink.emit(&reson8_forge_core::protocol::LogEntry::new(
                                    reson8_forge_core::protocol::LogLevel::Info,
                                    "hitl-reply",
                                    item.reply.clone(),
                                ));
                            }
                        }
                        app.push_notification(
                            app::NotifLevel::Success,
                            "hitl",
                            summary,
                        );
                        app.focus_panel(app::FocusPanel::Actions);
                    }
                    Effect::Git { action } => {
                        let summary = match action {
                            reson8_tui::git_lab::GitAction::Refresh => {
                                app.refresh_git();
                                app.git.reconcile_note()
                            }
                            reson8_tui::git_lab::GitAction::Fetch => app.git.fetch(),
                        };
                        app.push_notification(app::NotifLevel::Info, "git", summary);
                        app.focus_panel(app::FocusPanel::Git);
                    }
                    Effect::Notify { .. } | Effect::None => {}
                }
            }
        }
    }

    // ── Cleanup ──────────────────────────────────────────────────────────────
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
