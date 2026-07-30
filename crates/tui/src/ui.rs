//! TUI rendering — translates [`App`] state into Ratatui widgets.
//!
//! Layout (vertical: body + toast strip; body horizontal):
//!  [0] Providers | [1] Tasks | [2] Braid+ATOM | [3] Logs | [4] Formal(LSP) | [5] Tests
//! Overlay: urgent popup modal + notification toasts.
//!
//! Keybindings:
//!  [Tab] focus  [p] paper  [n] publish  [t] tests  [u] urgent  [q] quit
//!  Formal pane: Lean/Agda diagnostics; amber = SlowStep/sorry; B placeholders never green.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, FocusPanel, NotifLevel, TestStatus};
use crate::lsp::DiagnosticSeverityUi;
use reson8_forge_core::task::TaskPhase;

pub fn draw(frame: &mut Frame, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(frame.area());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(22),
            Constraint::Percentage(16),
            Constraint::Percentage(18),
            Constraint::Percentage(16),
        ])
        .split(outer[0]);

    draw_providers(frame, app, chunks[0]);
    draw_tasks(frame, app, chunks[1]);
    draw_braid(frame, app, chunks[2]);
    draw_logs(frame, app, chunks[3]);
    draw_formal(frame, app, chunks[4]);
    draw_tests(frame, app, chunks[5]);
    draw_toast_strip(frame, app, outer[1]);

    // Modal last so it paints on top.
    if app.popup.is_some() {
        draw_popup(frame, app);
    }
}

// ---------------------------------------------------------------------------
// Formal / LSP diagnostics (eye of the needle)
// ---------------------------------------------------------------------------

fn draw_formal(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let lean = if app.lean_lsp_ok { "lean●" } else { "lean○" };
    let als = if app.als_lsp_ok { "als●" } else { "als○" };
    // Never paint "live" green for placeholders-only state.
    let status_color = if app.lean_lsp_ok || app.als_lsp_ok {
        Color::Cyan
    } else {
        Color::LightYellow
    };

    let items: Vec<ListItem> = app
        .diagnostics
        .iter()
        .take(area.height.saturating_sub(2) as usize)
        .map(|d| {
            let (icon, color) = match d.severity {
                DiagnosticSeverityUi::Error => ("E", Color::Red),
                DiagnosticSeverityUi::Warning => ("W", Color::Yellow),
                DiagnosticSeverityUi::Info => ("I", Color::Gray),
                DiagnosticSeverityUi::Hint => ("H", Color::DarkGray),
                // Amber SlowStep / JFA open obligation (sorry)
                DiagnosticSeverityUi::SlowStep => ("S", Color::LightYellow),
                DiagnosticSeverityUi::PlaceholderB => ("B", Color::LightYellow),
            };
            let loc = if d.line > 0 {
                format!("{}:{}", d.path, d.line)
            } else {
                d.path.clone()
            };
            let text = format!("[{icon}] {} {} — {}", d.server, loc, d.message);
            ListItem::new(Line::from(Span::styled(
                text,
                Style::default().fg(color),
            )))
        })
        .collect();

    let title = Line::from(vec![
        Span::raw(" Formal "),
        Span::styled(
            format!("{lean} {als} "),
            Style::default().fg(status_color).add_modifier(Modifier::BOLD),
        ),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Formal));

    frame.render_widget(List::new(items).block(block), area);
}

// ---------------------------------------------------------------------------
// Provider panel
// ---------------------------------------------------------------------------

fn draw_providers(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let conn_label = if app.bridge.connected { "◉ live" } else { "○ ---" };
    let conn_color = if app.bridge.connected { Color::Green } else { Color::DarkGray };

    let items: Vec<ListItem> = app
        .providers
        .iter()
        .map(|p| {
            let icon = if p.healthy { "●" } else { "○" };
            let color = if p.healthy { Color::Green } else { Color::DarkGray };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{icon} "), Style::default().fg(color)),
                Span::raw(p.label),
            ]))
        })
        .collect();

    let title = Line::from(vec![
        Span::raw(" Providers "),
        Span::styled(conn_label, Style::default().fg(conn_color).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Providers));

    frame.render_widget(List::new(items).block(block), area);
}

// ---------------------------------------------------------------------------
// Task panel
// ---------------------------------------------------------------------------

fn draw_tasks(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|t| {
            let color = match t.phase {
                TaskPhase::Pending     => Color::DarkGray,
                TaskPhase::Initialized => Color::Yellow,
                TaskPhase::Executing   => Color::Cyan,
                TaskPhase::Validated   => Color::Blue,
                TaskPhase::Completed   => Color::Green,
                TaskPhase::Failed      => Color::Red,
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:?} ", t.phase), Style::default().fg(color)),
                Span::raw(format!("[{}] {}", t.id, t.kind)),
            ]))
        })
        .collect();

    let block = Block::default()
        .title(" Tasks ")
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Tasks));

    frame.render_widget(List::new(items).block(block), area);
}

// ---------------------------------------------------------------------------
// Braid + Bridge + ATOM Trail panel
// ---------------------------------------------------------------------------

fn draw_braid(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let b = &app.bridge;

    // ── Conservation law ──────────────────────────────────────────────────────
    let cons_color = if b.conservation_valid { Color::Green } else { Color::Red };
    let cons_label = if b.conservation_valid { "✓" } else { "✗" };

    // ── WAVE bar (18 chars) ───────────────────────────────────────────────────
    let wave_bar   = ascii_bar(b.wave_score, 18);
    let wave_color = score_color(b.wave_score);

    // ── Pipeline bar ─────────────────────────────────────────────────────────
    let pipe_bar   = ascii_bar(b.pipeline_percent as f64 / 100.0, 18);
    let pipe_color = if b.pipeline_aborted {
        Color::Red
    } else if b.pipeline_percent == 100 {
        Color::Green
    } else {
        Color::Cyan
    };

    // ── Hardware gauges ───────────────────────────────────────────────────────
    let cpu_bar   = ascii_bar(b.cpu_temp / 100.0, 10);
    let gpu_bar   = ascii_bar(b.gpu_temp / 100.0, 10);
    let cpu_color = temp_color(b.cpu_temp);
    let gpu_color = temp_color(b.gpu_temp);

    // ── Live vs. static braid values ─────────────────────────────────────────
    // When bridge is live and coherence data has arrived, prefer real α/ω/Φ.
    let (alpha_disp, omega_disp, phi_disp) = if b.connected && b.wave_score > 0.0 {
        (b.alpha as f64, b.omega as f64, b.wave_score)
    } else {
        (app.braid.alpha, app.braid.omega, app.braid.phi)
    };

    let mut lines: Vec<Line> = vec![
        // ── Braid status ──────────────────────────────────────────────────────
        Line::from(vec![
            Span::styled("STATUS: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                app.braid.status,
                Style::default()
                    .fg(if !b.connected || b.conservation_valid {
                        Color::Green
                    } else {
                        Color::Red
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("α ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{:.0}", alpha_disp), Style::default().fg(Color::Cyan)),
            Span::styled("  ω ", Style::default().fg(Color::Magenta)),
            Span::styled(format!("{:.0}", omega_disp), Style::default().fg(Color::Magenta)),
            Span::styled("  Φ ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("{:.3}", phi_disp), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        // ── Connection ────────────────────────────────────────────────────────
        Line::from(vec![
            Span::styled("CONN  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if b.connected { "Connected" } else { "Reconnecting…" },
                Style::default().fg(if b.connected { Color::Green } else { Color::Red }),
            ),
        ]),
        // ── Live WAVE score ───────────────────────────────────────────────────
        Line::from(vec![
            Span::styled("WAVE  ", Style::default().fg(Color::DarkGray)),
            Span::styled(wave_bar, Style::default().fg(wave_color)),
            Span::styled(
                format!(" {:.3}", b.wave_score),
                Style::default().fg(wave_color).add_modifier(Modifier::BOLD),
            ),
        ]),
        // ── Conservation law ──────────────────────────────────────────────────
        Line::from(vec![
            Span::styled(
                format!("α{}+ω{}={} ", b.alpha, b.omega, b.alpha.saturating_add(b.omega)),
                Style::default().fg(cons_color),
            ),
            Span::styled(cons_label, Style::default().fg(cons_color).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];

    // ── Hardware (only when bridge is live and data has arrived) ──────────────
    if b.connected && b.cpu_temp > 0.0 {
        lines.push(Line::from(vec![
            Span::styled("CPU   ", Style::default().fg(Color::DarkGray)),
            Span::styled(cpu_bar, Style::default().fg(cpu_color)),
            Span::styled(format!(" {:.0}°C", b.cpu_temp), Style::default().fg(cpu_color)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("GPU   ", Style::default().fg(Color::DarkGray)),
            Span::styled(gpu_bar, Style::default().fg(gpu_color)),
            Span::styled(format!(" {:.0}°C", b.gpu_temp), Style::default().fg(gpu_color)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("PWR   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.0}W  {}", b.power_draw, &b.health),
                Style::default().fg(Color::Yellow),
            ),
        ]));
        lines.push(Line::from(""));
    }

    // ── Pipeline section (only when a pipeline has been triggered) ────────────
    if !b.pipeline_name.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("PIPE  ", Style::default().fg(Color::DarkGray)),
            Span::raw(&b.pipeline_name),
        ]));
        if !b.pipeline_step.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("STEP  ", Style::default().fg(Color::DarkGray)),
                Span::raw(&b.pipeline_step),
            ]));
        }
        lines.push(Line::from(vec![
            Span::styled(pipe_bar, Style::default().fg(pipe_color)),
            Span::styled(format!(" {}%", b.pipeline_percent), Style::default().fg(pipe_color)),
        ]));
        lines.push(Line::from(""));
    }

    // ── Etch-a-sketch / density field (Miura-shaded wave) ───────────────────
    lines.extend(etch_wave_field(b.wave_score, b.connected));
    lines.push(Line::from(""));

    // ── ATOM trail (newest first, up to 8 entries) ────────────────────────────
    // ATOM: Render ATOM trail in Braid panel | Coherence: 96
    if !b.atom_trail.is_empty() {
        lines.push(Line::from(Span::styled(
            "─── ATOM TRAIL ───────────────",
            Style::default().fg(Color::DarkGray),
        )));
        for entry in b.atom_trail.iter().take(8) {
            lines.push(Line::from(Span::styled(
                format!("  {entry}"),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    // ── Panel title with full keybinding hint ─────────────────────────────────
    let title = Line::from(vec![
        Span::raw(" Braid "),
        Span::styled("[t]", Style::default().fg(Color::Green)),
        Span::raw("tests "),
        Span::styled("[u]", Style::default().fg(Color::Yellow)),
        Span::raw("urgent "),
        Span::styled("[p]", Style::default().fg(Color::Cyan)),
        Span::raw("paper "),
        Span::styled("[q]", Style::default().fg(Color::Yellow)),
        Span::raw("quit "),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Braid));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

// ---------------------------------------------------------------------------
// Log panel  (newest first, last 100 entries)
// ---------------------------------------------------------------------------

fn draw_logs(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let entries = app.log_sink.entries();
    let log_lines: Vec<Line> = entries
        .iter()
        .rev()
        .take(100)
        .map(|e| {
            let level_color = match e.level {
                reson8_forge_core::protocol::LogLevel::Trace => Color::DarkGray,
                reson8_forge_core::protocol::LogLevel::Debug => Color::Gray,
                reson8_forge_core::protocol::LogLevel::Info  => Color::Cyan,
                reson8_forge_core::protocol::LogLevel::Warn  => Color::Yellow,
                reson8_forge_core::protocol::LogLevel::Error => Color::Red,
            };
            Line::from(vec![
                Span::styled(
                    format!("{} ", e.level),
                    Style::default().fg(level_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("[{}] ", e.source), Style::default().fg(Color::DarkGray)),
                Span::raw(&e.message),
            ])
        })
        .collect();

    let block = Block::default()
        .title(" Logs ")
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Logs));

    frame.render_widget(Paragraph::new(log_lines).block(block), area);
}

// ---------------------------------------------------------------------------
// Tests panel (mapped harness)
// ---------------------------------------------------------------------------

fn draw_tests(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .tests
        .iter()
        .map(|t| {
            let (mark, color) = match t.status {
                TestStatus::Pending => ("·", Color::DarkGray),
                TestStatus::Running => ("…", Color::Yellow),
                TestStatus::Pass => ("✓", Color::Green),
                TestStatus::Fail => ("✗", Color::Red),
                TestStatus::Skip => ("–", Color::Magenta),
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{mark} "), Style::default().fg(color)),
                Span::styled(format!("{} ", t.id), Style::default().fg(Color::DarkGray)),
                Span::raw(format!("{} [{}]", t.name, t.crate_name)),
            ]))
        })
        .collect();

    let block = Block::default()
        .title(" Tests [t] ")
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Tests));

    frame.render_widget(List::new(items).block(block), area);
}

// ---------------------------------------------------------------------------
// Notification toast strip
// ---------------------------------------------------------------------------

fn draw_toast_strip(frame: &mut Frame, app: &App, area: Rect) {
    let line = if let Some(n) = app.notifications.front() {
        let color = match n.level {
            NotifLevel::Info => Color::Cyan,
            NotifLevel::Warn => Color::Yellow,
            NotifLevel::Critical => Color::Red,
            NotifLevel::Success => Color::Green,
        };
        Line::from(vec![
            Span::styled(" NOTIFY ", Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::styled(format!("[{}] ", n.title), Style::default().fg(Color::White)),
            Span::raw(&n.body),
            Span::styled(
                format!("  (+{} more)", app.notifications.len().saturating_sub(1)),
                Style::default().fg(Color::DarkGray),
            ),
        ])
    } else {
        Line::from(Span::styled(
            " [Tab] focus  [t] tests  [u] urgent  [p] paper  [n] publish  [q] quit  | α+ω=15 ",
            Style::default().fg(Color::DarkGray),
        ))
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    frame.render_widget(Paragraph::new(line).block(block), area);
}

// ---------------------------------------------------------------------------
// Urgent popup modal (centered overlay)
// ---------------------------------------------------------------------------

fn draw_popup(frame: &mut Frame, app: &App) {
    let Some(popup) = app.popup.as_ref() else {
        return;
    };

    let area = centered_rect(60, 40, frame.area());
    frame.render_widget(Clear, area);

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!(" ⚠ {}", popup.title),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::raw(format!(" {}", popup.question))),
        Line::from(""),
        Line::from(Span::styled(
            " Options (↑/↓ or j/k, Enter confirm, Esc cancel):",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
    ];

    for (i, opt) in popup.options.iter().enumerate() {
        let selected = i == popup.selected;
        lines.push(Line::from(Span::styled(
            format!("  {} {}", if selected { "▶" } else { " " }, opt),
            if selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            },
        )));
    }

    let block = Block::default()
        .title(" URGENT GATE ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(
        Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
        area,
    );
}

/// Center a rect as a percentage of the parent.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn border_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

/// Multi-level density bar (etch-a-sketch / ASCII shader ramp).
/// Levels: space ░ ▒ ▓ █ — smooths WAVE without binary fill cliffs (Reidemeister-stable).
fn ascii_bar(fraction: f64, width: usize) -> String {
    const RAMP: &[char] = &[' ', '░', '▒', '▓', '█'];
    let clamped = fraction.clamp(0.0, 1.0);
    let mut out = String::with_capacity(width);
    for i in 0..width {
        // Per-cell fractional coverage with sub-cell dither
        let edge = (i as f64 + 0.5) / width as f64;
        let local = ((clamped - edge) * width as f64 + 0.5).clamp(0.0, 1.0);
        let idx = (local * (RAMP.len() - 1) as f64).round() as usize;
        out.push(RAMP[idx.min(RAMP.len() - 1)]);
    }
    out
}

/// Etch-a-sketch density field — 4-line Miura-ori valley/mountain WAVE surface.
/// Mapped from wave_score; live bridge deepens ink (█), offline uses lighter graphite.
fn etch_wave_field(wave: f64, live: bool) -> Vec<Line<'static>> {
    let ink = if live {
        [Color::Cyan, Color::Magenta, Color::Yellow, Color::Green]
    } else {
        [Color::DarkGray, Color::Gray, Color::DarkGray, Color::Gray]
    };
    let amp = wave.clamp(0.0, 1.0);
    // Four scanlines of a standing wave with Miura alternation
    let patterns: [[f64; 12]; 4] = [
        [0.2, 0.5, 0.9, 0.5, 0.2, 0.1, 0.2, 0.5, 0.9, 0.5, 0.2, 0.1],
        [0.1, 0.3, 0.6, 0.9, 0.6, 0.3, 0.1, 0.3, 0.6, 0.9, 0.6, 0.3],
        [0.3, 0.6, 0.9, 0.6, 0.3, 0.1, 0.3, 0.6, 0.9, 0.6, 0.3, 0.1],
        [0.5, 0.2, 0.1, 0.2, 0.5, 0.9, 0.5, 0.2, 0.1, 0.2, 0.5, 0.9],
    ];
    patterns
        .iter()
        .enumerate()
        .map(|(row, cells)| {
            let s: String = cells
                .iter()
                .map(|c| {
                    let v = (c * amp * 4.0).clamp(0.0, 4.0) as usize;
                    [' ', '░', '▒', '▓', '█'][v.min(4)]
                })
                .collect();
            // Owned String → 'static Span content (ratatui Cow)
            Line::from(Span::styled(
                s,
                Style::default().fg(ink[row % ink.len()]),
            ))
        })
        .collect()
}

/// Color a WAVE score: green ≥ 0.90, yellow ≥ 0.70, red otherwise.
fn score_color(score: f64) -> Color {
    if score >= 0.90 {
        Color::Green
    } else if score >= 0.70 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Color a temperature reading: green < 60, yellow < 80, red ≥ 80.
fn temp_color(temp: f64) -> Color {
    if temp >= 80.0 {
        Color::Red
    } else if temp >= 60.0 {
        Color::Yellow
    } else {
        Color::Green
    }
}
