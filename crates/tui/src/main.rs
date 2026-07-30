//! reson8-forge (RESON8-TUI) — real-time TUI for multi-AI pipeline orchestration.
//!
//! Tagline: *Where the last thing you've done becomes the first thing you need*
//!
//! Phase evolution: quantum-redstone gates × SPHINX (KENL→AWI→ATOM→SAIF)
//! mapped onto TaskPhase, orchestrated as Tokio/Ratatui sequences.
//!
//! Conservation law: alpha + omega = 15 on every pipeline event.
//!
//! Keybindings:
//! - `Tab`  — cycle panel focus
//! - `s`    — start full-spiral sequence (phase evolution)
//! - `g`    — gate tick (advance one QR / SPHINX step)
//! - `p`    — paper-draft sequence + superskill pipeline
//! - `n`    — idea-to-publish sequence + superskill pipeline
//! - `t`    — run test map harness (phase-gated)
//! - `u`    — open urgent popup (question gate)
//! - `f`    — focus Formal (LSP / B placeholders) pane
//! - `q`    — quit
//! Popup: ↑/↓/j/k select, Enter confirm, Esc dismiss
//!
//! LSP: decoupled stdout reader → LspEvent mpsc → Formal pane (amber SlowStep).

mod app;
mod lsp;
mod phase_evolution;
mod ui;

use std::io;
use std::path::PathBuf;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use tokio::sync::mpsc;

use reson8_forge_core::bridge::BridgeHandle;
use reson8_forge_core::superskill::{SuperskillEngine, SuperskillEvent};

use crate::lsp::LspEvent;

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
        println!("  FORGE_WS_URL    WebSocket URL (default: ws://127.0.0.1:8088)");
        println!();
        println!("KEYBINDINGS:");
        println!("  Tab   cycle panel focus");
        println!("  s     full-spiral sequence (KENL→AWI→ATOM→SAIF)");
        println!("  g     gate tick (one quantum-redstone / SPHINX step)");
        println!("  p     paper-draft sequence + pipeline");
        println!("  n     idea-to-publish sequence + pipeline");
        println!("  t     mapped test harness (phase-gated)");
        println!("  u     urgent popup question");
        println!("  f     focus Formal (LSP diagnostics / B placeholders)");
        println!("  q     quit");
        println!();
        println!("FORMAL / LSP:");
        println!("  Lean: lake serve | lean --server  (LOGOS_ROOT or cwd)");
        println!("  Agda: als  [CATEGORY B until installed]");
        println!("  Diagnostics: amber SlowStep for sorry; B stubs never green");
        println!();
        println!("PHASE EVOLUTION TABLE:");
        println!("  KENL  × Hadamard     Pending → Initialized");
        println!("  AWI   × CNOT         Initialized → Executing");
        println!("  ATOM  × RS-NOR       Executing → Validated");
        println!("  SAIF  × ε-Tetrahedron Validated → Completed → seed KENL");
        println!();
        println!("POPUP (when open):");
        println!("  j/k or arrows  select option");
        println!("  Enter          confirm");
        println!("  Esc            dismiss");
        println!();
        println!("Part of the Reson8-Labs ecosystem — https://reson8labs.ai");
        println!("Conservation law: alpha + omega = 15");
        return Ok(());
    }

    // ── Terminal setup ────────────────────────────────────────────────────────
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut app = app::App::new();

    // ── Bridge — configurable via FORGE_WS_URL env var ───────────────────────
    // ATOM: Spawn WebSocket bridge to ws://127.0.0.1:8088 | Coherence: 97
    let ws_url = std::env::var("FORGE_WS_URL")
        .unwrap_or_else(|_| "ws://127.0.0.1:8088".to_string());
    let mut bridge = BridgeHandle::spawn(&ws_url);

    // ── Superskill engine — enforces alpha+omega=15 invariant ────────────────
    // ATOM: Wire SuperskillEngine with CommandTx clone | Coherence: 96
    let (ss_ev_tx, mut ss_ev_rx) = mpsc::channel::<SuperskillEvent>(256);
    let mut engine = SuperskillEngine::new(bridge.cmd.clone(), ss_ev_tx);

    // ── LSP bus (decoupled) — Formal pane is the eye of the needle ───────────
    let (lsp_tx, mut lsp_rx) = mpsc::channel::<LspEvent>(256);
    let logos_root = std::env::var("LOGOS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    // Hold connections so kill_on_drop keeps servers for process lifetime.
    let _lean_conn = lsp::try_attach_lean(&logos_root, lsp_tx.clone()).await;
    let _als_conn = lsp::try_attach_agda(lsp_tx.clone()).await;

    // ── Main event loop ─────────────────────────────────────────────────────
    while app.running {
        // Drain bridge events to both app state and superskill engine.
        // Both observe the same events; engine acts on Coherence/Pipeline,
        // app acts on all four types (Telemetry, Coherence, Pipeline, Atom).
        while let Ok(ev) = bridge.rx.try_recv() {
            let ev_for_engine = ev.clone();
            app.handle_bridge_event(&ev);
            engine.handle(ev_for_engine).await;
        }

        // Drain superskill events to app state (pipeline start/abort/complete).
        while let Ok(ev) = ss_ev_rx.try_recv() {
            app.handle_superskill_event(ev);
        }

        // Drain LSP / placeholder events → Formal diagnostics pane.
        while let Ok(ev) = lsp_rx.try_recv() {
            app.handle_lsp_event(ev);
        }

        app.tick_notifications();
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if event::poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // Blocking popup captures input first.
                    if app.popup.as_ref().is_some_and(|p| p.blocking) {
                        match key.code {
                            KeyCode::Esc => app.dismiss_popup(),
                            KeyCode::Enter => app.confirm_popup(),
                            KeyCode::Up | KeyCode::Char('k') => app.popup_select_prev(),
                            KeyCode::Down | KeyCode::Char('j') => app.popup_select_next(),
                            KeyCode::Char('q') => app.quit(),
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        // ── Navigation ────────────────────────────────────────
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Tab => app.cycle_focus(),
                        KeyCode::Char('f') => app.focus = app::FocusPanel::Formal,

                        // ── Phase evolution / quantum-redstone sequences ──────
                        KeyCode::Char('s') => {
                            // Full spiral: auto-tick all 4 gates (last→first seed).
                            app.start_sequence(app::SequenceKind::FullSpiral);
                            for _ in 0..4 {
                                app.gate_tick();
                            }
                        }
                        KeyCode::Char('g') => {
                            if !app.sequence.active {
                                app.start_sequence(app::SequenceKind::GateTick);
                            }
                            app.gate_tick();
                        }

                        // ── Test map + urgent popup ───────────────────────────
                        KeyCode::Char('t') => {
                            app.start_sequence(app::SequenceKind::TestMap);
                            app.run_test_map();
                            for _ in 0..4 {
                                app.gate_tick();
                            }
                        }
                        KeyCode::Char('u') => {
                            app.open_urgent(
                                "Director Gate",
                                "Urgent decision required — choose path:",
                                vec![
                                    "proceed".into(),
                                    "defer".into(),
                                    "abort".into(),
                                    "open Gitea migrate".into(),
                                ],
                            );
                        }

                        // ── Pipeline triggers (superskill + phase table) ──────
                        KeyCode::Char('p') => {
                            app.start_sequence(app::SequenceKind::PaperDraft);
                            app.push_notification(
                                app::NotifLevel::Info,
                                "pipeline",
                                "paper-draft + phase spiral",
                            );
                            let _ = engine
                                .trigger("paper-draft", serde_json::json!({}))
                                .await;
                            for _ in 0..4 {
                                app.gate_tick();
                            }
                        }
                        KeyCode::Char('n') => {
                            app.start_sequence(app::SequenceKind::IdeaToPublish);
                            app.push_notification(
                                app::NotifLevel::Info,
                                "pipeline",
                                "idea-to-publish + phase spiral",
                            );
                            let _ = engine
                                .trigger("idea-to-publish", serde_json::json!({}))
                                .await;
                            for _ in 0..4 {
                                app.gate_tick();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // ── Cleanup ──────────────────────────────────────────────────────────────
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
