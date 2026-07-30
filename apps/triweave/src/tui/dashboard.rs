//! TUI Dashboard — live strand status, WAVE scores, ATOM trail.
//!
//! Phase 3: Real-time WebSocket bridge events from Styx (ws://127.0.0.1:8088).
//! Reuses BridgeHandle from reson8-forge-core for bidirectional comms.
//!
//! Layout:
//!  ╔═STRANDS══╗ ╔═WAVE════════╗
//!  ║ ● Claude  ║ ║ ▁▂▃▅▇█  Φ  ║
//!  ║ ● Grok    ║ ║ α+ω=15 ✓   ║
//!  ║ ● Gemini  ║ ╚════════════╝
//!  ╚══════════╝ ╔═ATOM TRAIL══╗
//!  ╔═FORGE════╗ ║ KENL→AWI→…  ║
//!  ║ GPU 42°C ║ ╚════════════╝
//!  ╚══════════╝
//!
//! Conservation: α + ω = 15

use std::collections::VecDeque;
use std::io;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline};

use reson8_forge_core::bridge::{BridgeEvent, BridgeHandle};

use crate::saif::TriweaveConfig;
use crate::strand;
use crate::theme::Theme;

// ─── App State ────────────────────────────────────────────────────

struct DashboardApp {
    running: bool,
    config: TriweaveConfig,
    theme: Theme,
    health: Vec<strand::StrandHealth>,

    // Live bridge state
    bridge_connected: bool,
    wave_score: f64,
    wave_history: VecDeque<u64>,
    alpha: u8,
    omega: u8,
    conservation_valid: bool,

    // Pipeline
    pipeline_name: String,
    pipeline_step: String,
    pipeline_percent: u8,
    pipeline_active: bool,

    // Telemetry
    cpu_temp: f64,
    gpu_temp: f64,
    power_draw: f64,
    cpu_history: VecDeque<u64>,
    gpu_history: VecDeque<u64>,
    health_label: String,

    // ATOM trail (newest first)
    atom_trail: VecDeque<String>,

    // Event log
    log: VecDeque<String>,
}

impl DashboardApp {
    fn new(config: TriweaveConfig, theme: Theme, health: Vec<strand::StrandHealth>) -> Self {
        let avg_wave = if health.is_empty() {
            0.0
        } else {
            health.iter().map(|s| s.wave_score).sum::<f64>() / health.len() as f64
        };

        let mut wave_history = VecDeque::with_capacity(60);
        wave_history.push_back((avg_wave * 100.0) as u64);

        Self {
            running: true,
            config,
            theme,
            health,
            bridge_connected: false,
            wave_score: avg_wave,
            wave_history,
            alpha: 8,
            omega: 7,
            conservation_valid: true,
            pipeline_name: String::new(),
            pipeline_step: String::new(),
            pipeline_percent: 0,
            pipeline_active: false,
            cpu_temp: 0.0,
            gpu_temp: 0.0,
            power_draw: 0.0,
            cpu_history: VecDeque::with_capacity(60),
            gpu_history: VecDeque::with_capacity(60),
            health_label: "---".to_string(),
            atom_trail: VecDeque::with_capacity(100),
            log: VecDeque::with_capacity(200),
        }
    }

    fn push_log(&mut self, msg: String) {
        if self.log.len() >= 200 {
            self.log.pop_back();
        }
        self.log.push_front(msg);
    }

    fn push_history(buf: &mut VecDeque<u64>, val: f64) {
        if buf.len() == buf.capacity() {
            buf.pop_front();
        }
        buf.push_back(val.max(0.0) as u64);
    }

    fn handle_bridge_event(&mut self, ev: BridgeEvent) {
        match ev {
            BridgeEvent::Connected => {
                self.bridge_connected = true;
                self.push_log("bridge: connected".to_string());
            }
            BridgeEvent::Disconnected => {
                self.bridge_connected = false;
                self.push_log("bridge: disconnected — retrying...".to_string());
            }
            BridgeEvent::Telemetry(p) => {
                self.cpu_temp = p.sensors.cpu_temp;
                self.gpu_temp = p.sensors.gpu_temp;
                self.power_draw = p.sensors.power_draw;
                self.health_label = p.health.clone();
                Self::push_history(&mut self.cpu_history, p.sensors.cpu_temp);
                Self::push_history(&mut self.gpu_history, p.sensors.gpu_temp);
            }
            BridgeEvent::Coherence(p) => {
                self.wave_score = p.wave_score;
                self.alpha = p.conservation.alpha;
                self.omega = p.conservation.omega;
                self.conservation_valid =
                    p.conservation.valid && p.conservation.alpha + p.conservation.omega == 15;
                Self::push_history(&mut self.wave_history, p.wave_score * 100.0);
                self.push_log(format!(
                    "wave: {:.3} a{}+w{}={}",
                    p.wave_score,
                    p.conservation.alpha,
                    p.conservation.omega,
                    p.conservation.alpha + p.conservation.omega,
                ));
            }
            BridgeEvent::PipelineProgress(p) => {
                self.pipeline_name = p.pipeline_name.clone();
                self.pipeline_step = p.current_step.clone();
                self.pipeline_percent = p.percent;
                self.pipeline_active = p.completed_steps < p.total_steps;
                self.push_log(format!(
                    "pipeline: {} {}% ({})",
                    p.pipeline_name, p.percent, p.current_step
                ));
            }
            BridgeEvent::AtomEvent(p) => {
                let entry = format!("{} {} -> {}", p.atom_type, p.gate, p.description);
                if self.atom_trail.len() >= 100 {
                    self.atom_trail.pop_back();
                }
                self.atom_trail.push_front(entry.clone());
                self.push_log(format!("atom: {}", entry));
            }
            BridgeEvent::ExecuteMcpTool { tool_name, .. } => {
                self.push_log(format!("mcp: execute {}", tool_name));
            }
        }
    }
}

// ─── Run ──────────────────────────────────────────────────────────

/// Run the TUI dashboard with live WebSocket events.
pub async fn run() -> Result<()> {
    let config = TriweaveConfig::load().unwrap_or_default();
    let theme = Theme::from_config(&config.theme);
    let health = strand::health_check(&config).await;

    // Spawn bridge handle for live events
    let mut bridge = BridgeHandle::spawn(&config.styx.ws_url);

    let mut app = DashboardApp::new(config, theme, health);
    app.push_log("TRIWEAVE v0.1.0 dashboard started".to_string());
    app.push_log(format!("Styx bridge: {}", app.config.styx.ws_url));

    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    while app.running {
        terminal.draw(|frame| draw(frame, &app))?;

        // Drain all pending bridge events (non-blocking)
        while let Ok(ev) = bridge.rx.try_recv() {
            app.handle_bridge_event(ev);
        }

        // Poll terminal events with 50ms timeout for smooth updates
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                        KeyCode::Char('r') => {
                            // Refresh health checks
                            app.push_log("Refreshing strand health...".to_string());
                            app.health = strand::health_check(&app.config).await;
                            app.push_log("Health refresh complete.".to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

// ─── Drawing ──────────────────────────────────────────────────────

fn draw(frame: &mut Frame, app: &DashboardApp) {
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(frame.area());

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Strands
            Constraint::Length(8),  // Forge
            Constraint::Min(3),    // Log
        ])
        .split(main[0]);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // WAVE + sparkline
            Constraint::Length(7),  // Pipeline
            Constraint::Min(5),    // ATOM trail
        ])
        .split(main[1]);

    draw_strands(frame, app, left[0]);
    draw_forge(frame, app, left[1]);
    draw_log(frame, app, left[2]);
    draw_wave(frame, app, right[0]);
    draw_pipeline(frame, app, right[1]);
    draw_atom_trail(frame, app, right[2]);
}

fn draw_strands(frame: &mut Frame, app: &DashboardApp, area: Rect) {
    let t = &app.theme;
    let conn_icon = if app.bridge_connected { "live" } else { "---" };
    let conn_style = if app.bridge_connected { t.strand_ok } else { t.strand_down };

    let mut lines: Vec<Line> = app
        .health
        .iter()
        .map(|s| {
            let icon = if s.reachable { "●" } else { "○" };
            let status_style = if s.reachable { t.strand_ok } else { t.strand_down };
            let wave_style = if s.wave_score >= 0.85 { t.wave_high } else { t.wave_low };
            let key_flag = if s.has_key { "" } else { " (no key)" };
            Line::from(vec![
                Span::styled(format!(" {} ", icon), status_style),
                Span::styled(format!("{:<8}", s.name), t.text),
                Span::styled(format!("{:.3}", s.wave_score), wave_style),
                Span::styled(key_flag, t.strand_no_key),
            ])
        })
        .collect();

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(" Bridge: ", t.forge_label),
        Span::styled(conn_icon, conn_style),
    ]));

    let block = Block::default()
        .title(" Strands ")
        .borders(Borders::ALL)
        .border_type(t.border_type.to_ratatui())
        .border_style(t.border_focused);

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_forge(frame: &mut Frame, app: &DashboardApp, area: Rect) {
    let t = &app.theme;
    let block = Block::default()
        .title(" Forge ")
        .borders(Borders::ALL)
        .border_type(t.border_type.to_ratatui())
        .border_style(t.border);

    let cpu_bar = temp_bar(app.cpu_temp);
    let gpu_bar = temp_bar(app.gpu_temp);

    let lines = vec![
        Line::from(vec![
            Span::styled("Transport: ", t.forge_label),
            Span::styled(&*app.config.strands.transport, t.forge_value),
        ]),
        Line::from(vec![
            Span::styled("Styx:      ", t.forge_label),
            Span::styled(&*app.config.styx.ws_url, t.forge_value),
        ]),
        Line::from(vec![
            Span::styled("NEAR:      ", t.forge_label),
            Span::styled(&*app.config.near.network, t.forge_value),
        ]),
        Line::from(vec![
            Span::styled("CPU: ", t.forge_label),
            Span::styled(format!("{:.0}C ", app.cpu_temp), temp_style(app.cpu_temp, t)),
            Span::styled(cpu_bar, temp_style(app.cpu_temp, t)),
        ]),
        Line::from(vec![
            Span::styled("GPU: ", t.forge_label),
            Span::styled(format!("{:.0}C ", app.gpu_temp), temp_style(app.gpu_temp, t)),
            Span::styled(gpu_bar, temp_style(app.gpu_temp, t)),
        ]),
        Line::from(vec![
            Span::styled("Power: ", t.forge_label),
            Span::styled(format!("{:.0}W  ", app.power_draw), t.forge_value),
            Span::styled(&*app.health_label, t.text_dim),
        ]),
    ];

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_wave(frame: &mut Frame, app: &DashboardApp, area: Rect) {
    let t = &app.theme;
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(3)])
        .split(area);

    let cons_icon = if app.conservation_valid { "OK" } else { "!!" };
    let cons_style = if app.conservation_valid { t.conservation_ok } else { t.conservation_fail };
    let wave_style = wave_style(app.wave_score, t);

    let lines = vec![
        Line::from(vec![
            Span::styled(
                format!(" {:.4} ", app.wave_score),
                wave_style.add_modifier(Modifier::BOLD),
            ),
            Span::styled(ascii_bar(app.wave_score, 20), wave_style),
        ]),
        Line::from(vec![
            Span::styled(" a ", t.gate_atom),
            Span::styled(format!("{}", app.alpha), t.text),
            Span::styled("  w ", t.gate_awi),
            Span::styled(format!("{}", app.omega), t.text),
            Span::styled("  S ", t.gate_kenl),
            Span::styled(format!("{} ", app.alpha + app.omega), t.text),
            Span::styled(cons_icon, cons_style),
        ]),
    ];

    let block = Block::default()
        .title(" WAVE -- alpha + omega = 15 ")
        .borders(Borders::ALL)
        .border_type(t.border_type.to_ratatui())
        .border_style(t.conservation_ok);

    frame.render_widget(Paragraph::new(lines).block(block), inner[0]);

    // Sparkline
    let data: Vec<u64> = app.wave_history.iter().copied().collect();
    let spark_block = Block::default()
        .title(" WAVE history ")
        .borders(Borders::ALL)
        .border_type(t.border_type.to_ratatui())
        .border_style(t.border);

    let sparkline = Sparkline::default()
        .block(spark_block)
        .data(&data)
        .max(100)
        .style(t.accent);

    frame.render_widget(sparkline, inner[1]);
}

fn draw_pipeline(frame: &mut Frame, app: &DashboardApp, area: Rect) {
    let t = &app.theme;
    let border = if app.pipeline_active { t.pipeline_active } else { t.border };
    let block = Block::default()
        .title(" Pipeline ")
        .borders(Borders::ALL)
        .border_type(t.border_type.to_ratatui())
        .border_style(border);

    let lines = if app.pipeline_active {
        let bar = progress_bar(app.pipeline_percent, 30);
        vec![
            Line::from(vec![
                Span::styled(" ", t.pipeline_active),
                Span::styled(&*app.pipeline_name, t.text),
            ]),
            Line::from(vec![
                Span::raw(" "),
                Span::styled(bar, t.pipeline_bar),
                Span::styled(format!(" {}%", app.pipeline_percent), t.text),
            ]),
            Line::from(vec![
                Span::styled(" Step: ", t.forge_label),
                Span::styled(&*app.pipeline_step, t.text),
            ]),
        ]
    } else {
        vec![
            Line::from(Span::styled(" No active pipeline", t.pipeline_idle)),
            Line::from(""),
            Line::from(Span::styled(
                " Trigger via Styx bridge or coherence-mcp",
                t.text_dim,
            )),
        ]
    };

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_atom_trail(frame: &mut Frame, app: &DashboardApp, area: Rect) {
    let t = &app.theme;
    let block = Block::default()
        .title(" ATOM Trail ")
        .borders(Borders::ALL)
        .border_type(t.border_type.to_ratatui())
        .border_style(t.border);

    let max_lines = area.height.saturating_sub(2) as usize;

    let mut lines: Vec<Line> = if app.atom_trail.is_empty() {
        vec![
            Line::from(vec![
                Span::styled("KENL", t.gate_kenl),
                Span::styled(" -> ", t.text_dim),
                Span::styled("AWI", t.gate_awi),
                Span::styled(" -> ", t.text_dim),
                Span::styled("ATOM", t.gate_atom),
                Span::styled(" -> ", t.text_dim),
                Span::styled("SAIF", t.gate_saif),
            ]),
            Line::from(""),
            Line::from(Span::styled("  Waiting for bridge events...", t.text_dim)),
        ]
    } else {
        app.atom_trail
            .iter()
            .take(max_lines)
            .map(|entry| {
                let style = if entry.contains("SAIF") {
                    t.gate_saif
                } else if entry.contains("ATOM") {
                    t.gate_atom
                } else if entry.contains("AWI") {
                    t.gate_awi
                } else if entry.contains("KENL") {
                    t.gate_kenl
                } else {
                    t.text_dim
                };
                Line::from(Span::styled(format!(" {}", entry), style))
            })
            .collect()
    };

    lines.truncate(max_lines);
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_log(frame: &mut Frame, app: &DashboardApp, area: Rect) {
    let t = &app.theme;
    let block = Block::default()
        .title(" [q]uit [r]efresh ")
        .borders(Borders::ALL)
        .border_type(t.border_type.to_ratatui())
        .border_style(t.border);

    let max_lines = area.height.saturating_sub(2) as usize;
    let lines: Vec<Line> = app
        .log
        .iter()
        .take(max_lines)
        .map(|entry| {
            let style = if entry.starts_with("wave:") {
                t.log_info
            } else if entry.starts_with("atom:") {
                t.gate_atom
            } else if entry.contains("disconnect") || entry.contains("error") {
                t.log_error
            } else {
                t.text_dim
            };
            Line::from(Span::styled(format!(" {}", entry), style))
        })
        .collect();

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

// ─── Helpers ──────────────────────────────────────────────────────

fn ascii_bar(fraction: f64, width: usize) -> String {
    let clamped = fraction.clamp(0.0, 1.0);
    let filled = (clamped * width as f64).round() as usize;
    format!(
        "{}{}",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(width - filled)
    )
}

fn progress_bar(percent: u8, width: usize) -> String {
    let filled = (percent as usize * width) / 100;
    format!(
        "[{}{}]",
        "#".repeat(filled),
        "-".repeat(width.saturating_sub(filled))
    )
}

fn temp_bar(temp: f64) -> String {
    let blocks = ((temp / 100.0).clamp(0.0, 1.0) * 10.0) as usize;
    "\u{2588}".repeat(blocks)
}

fn temp_style(temp: f64, t: &Theme) -> Style {
    if temp > 80.0 {
        t.temp_hot
    } else if temp > 60.0 {
        t.temp_warm
    } else {
        t.temp_cool
    }
}

fn wave_style(score: f64, t: &Theme) -> Style {
    if score >= 0.85 {
        t.wave_high
    } else if score >= 0.70 {
        t.wave_mid
    } else {
        t.wave_low
    }
}
