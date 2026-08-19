//! TUI rendering — translates [`App`] state into Ratatui widgets.
//!
//! Layout is **workflow-selectable** via [`crate::layout_presets::LayoutKind`]:
//!  ops · formal · agent · monitor · quantum · minimal
//!  Keys: [l] next · [L] prev · [1–6] jump · env RESON8_LAYOUT
//!
//! Overlay: urgent popup modal + notification toasts.
//!
//! Keybindings:
//!  [Tab] focus  [?] help  [p] paper  [n] publish  [t] tests  [u] urgent  [q] quit
//!  [N] net panel  [R] net probe  [M] net menu (privacy/gaming/stop)
//!  Formal pane: Lean/Agda diagnostics; amber = SlowStep/sorry; B placeholders never green.
//!  Agent terminal: host split (WT) + logos-tui + claude|grok — see help overlay.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, FocusPanel, NotifLevel, TestStatus};
use crate::lsp::DiagnosticSeverityUi;
use crate::phase_evolution::PHASE_EVOLUTION_TABLE;
use reson8_forge_core::task::TaskPhase;

pub fn draw(frame: &mut Frame, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(frame.area());

    draw_layout_body(frame, app, outer[0]);
    draw_toast_strip(frame, app, outer[1]);

    // Modal last so it paints on top.
    if app.popup.is_some() {
        draw_popup(frame, app);
    }
    if app.help_open {
        draw_help_overlay(frame, app);
    }
}

/// Render the active workflow layout (1–2 rows of weighted panels).
fn draw_layout_body(frame: &mut Frame, app: &App, area: Rect) {
    let rows = app.layout.rows();
    if rows.is_empty() {
        return;
    }

    let row_constraints: Vec<Constraint> = if rows.len() == 1 {
        vec![Constraint::Percentage(100)]
    } else {
        // Two-row presets: top primary (58%), bottom support (42%).
        let top = 58u16;
        let rest = 42u16 / (rows.len() as u16 - 1).max(1);
        let mut c: Vec<Constraint> = Vec::with_capacity(rows.len());
        c.push(Constraint::Percentage(top));
        for _ in 1..rows.len() {
            c.push(Constraint::Percentage(rest));
        }
        // Fix remainder drift on last row (compute used before mut borrow).
        let used: u16 = c[..c.len().saturating_sub(1)]
            .iter()
            .map(|x| match x {
                Constraint::Percentage(v) => *v,
                _ => 0,
            })
            .sum();
        if let Some(Constraint::Percentage(p)) = c.last_mut() {
            *p = 100u16.saturating_sub(used);
        }
        c
    };

    let row_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(area);

    for (ri, row) in rows.iter().enumerate() {
        if ri >= row_areas.len() {
            break;
        }
        let total: u32 = row.iter().map(|(_, w)| u32::from(*w)).sum::<u32>().max(1);
        let mut constraints: Vec<Constraint> = row
            .iter()
            .map(|(_, w)| {
                let pct = (u32::from(*w) * 100 / total) as u16;
                Constraint::Percentage(pct.max(1))
            })
            .collect();
        // Normalize last column so percentages sum to 100.
        let used_cols: u16 = constraints[..constraints.len().saturating_sub(1)]
            .iter()
            .map(|c| match c {
                Constraint::Percentage(v) => *v,
                _ => 0,
            })
            .sum();
        if let Some(Constraint::Percentage(last)) = constraints.last_mut() {
            *last = 100u16.saturating_sub(used_cols).max(1);
        }

        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(row_areas[ri]);

        for (ci, (panel, _)) in row.iter().enumerate() {
            if ci >= cols.len() {
                break;
            }
            draw_panel(frame, app, *panel, cols[ci]);
        }
    }
}

fn draw_panel(frame: &mut Frame, app: &App, panel: FocusPanel, area: Rect) {
    match panel {
        FocusPanel::Providers => draw_providers(frame, app, area),
        FocusPanel::Tasks => draw_tasks(frame, app, area),
        FocusPanel::Braid => draw_braid(frame, app, area),
        FocusPanel::Phases => draw_phases(frame, app, area),
        FocusPanel::Logs => draw_logs(frame, app, area),
        FocusPanel::Formal => draw_formal(frame, app, area),
        FocusPanel::Tests => draw_tests(frame, app, area),
        FocusPanel::Net => draw_net(frame, app, area),
        FocusPanel::Actions => draw_actions(frame, app, area),
        FocusPanel::Git => draw_git(frame, app, area),
        FocusPanel::Codes => draw_codes(frame, app, area),
    }
}

// ---------------------------------------------------------------------------
// Classical codes lab — Hexacode · Golay G24 · Reed–Muller
// ---------------------------------------------------------------------------

fn draw_codes(frame: &mut Frame, app: &App, area: Rect) {
    let lab = &app.codes;
    let status_color = if lab.last_ok {
        Color::Green
    } else {
        Color::LightYellow
    };

    let mut lines: Vec<Line> = Vec::new();
    for h in lab.header_lines() {
        lines.push(Line::from(Span::styled(
            h,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
    }
    lines.push(Line::from(Span::styled(
        "d demo · D battery · y family · e t± · [ ] r|w · { } m|L · 7 layout",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "── log ──",
        Style::default().fg(Color::DarkGray),
    )));

    let budget = area.height.saturating_sub(8) as usize;
    for line in lab.lines.iter().take(budget.max(4)) {
        lines.push(Line::from(Span::styled(
            truncate_ui(line, (area.width.saturating_sub(4)) as usize),
            Style::default().fg(Color::Gray),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("last ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if lab.last_ok { "OK" } else { "FAIL/DETECT" },
            Style::default().fg(status_color).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            "Hex·G24·RM·SC-LDPC · G24≠RM≠SC · sat: BP→MAP",
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    let title = Line::from(vec![
        Span::raw(" Codes "),
        Span::styled("[c]", Style::default().fg(Color::Cyan)),
        Span::raw(" "),
        Span::styled(
            lab.family.label(),
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Codes));

    frame.render_widget(
        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true }),
        area,
    );
}

// ---------------------------------------------------------------------------
// Net proxy stack (Tor / i2pd / Privoxy / DNSCrypt / gaming clearnet)
// ---------------------------------------------------------------------------

fn draw_net(frame: &mut Frame, app: &App, area: Rect) {
    let net = &app.net;
    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("lane ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                net.active_lane.clone(),
                Style::default()
                    .fg(match net.active_lane.as_str() {
                        "gaming" => Color::Green,
                        "privacy" => Color::Magenta,
                        "idle" => Color::DarkGray,
                        _ => Color::Yellow,
                    })
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                if net.script_present { "ctl●" } else { "ctl○" },
                Style::default().fg(if net.script_present {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ]),
        Line::from(Span::styled(
            "gaming=clearnet · privacy=Tor/i2pd  [B probe]",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
    ];

    for s in &net.services {
        let mark = if s.open { "◉" } else { "○" };
        let color = if s.open { Color::Green } else { Color::DarkGray };
        lines.push(Line::from(vec![
            Span::styled(format!("{mark} "), Style::default().fg(color)),
            Span::styled(
                format!("{:<12}", s.id),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(format!(":{} ", s.port), Style::default().fg(Color::DarkGray)),
            Span::styled(s.lane, Style::default().fg(Color::Gray)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        truncate_ui(&net.last_message, 56),
        Style::default().fg(Color::Yellow),
    )));
    lines.push(Line::from(Span::styled(
        "[N] focus [R] probe [M] menu",
        Style::default().fg(Color::DarkGray),
    )));

    let title = Line::from(vec![
        Span::raw(" Net "),
        Span::styled("[M]", Style::default().fg(Color::Magenta)),
        Span::raw("menu "),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Net));

    frame.render_widget(
        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true }),
        area,
    );
}

fn truncate_ui(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

// ---------------------------------------------------------------------------
// Phases — quantum-redstone × SPHINX evolution table
// ---------------------------------------------------------------------------

fn draw_phases(frame: &mut Frame, app: &App, area: Rect) {
    let seq = &app.sequence;
    let active = if seq.active { "ACTIVE" } else { "idle" };
    let kind = seq
        .kind
        .map(|k| k.label())
        .unwrap_or("—");
    let cons = if seq.conservation_valid() {
        "α+ω=15 ✓"
    } else {
        "α+ω≠15 ✗"
    };
    let cons_color = if seq.conservation_valid() {
        Color::Green
    } else {
        Color::Red
    };

    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("SEQ  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{kind} [{active}]"),
                Style::default()
                    .fg(if seq.active {
                        Color::Cyan
                    } else {
                        Color::DarkGray
                    })
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(cons, Style::default().fg(cons_color)),
        ]),
        Line::from(vec![
            Span::styled("need ", Style::default().fg(Color::DarkGray)),
            Span::raw(&seq.first_need),
        ]),
        Line::from(vec![
            Span::styled("last ", Style::default().fg(Color::DarkGray)),
            Span::raw(if seq.last_done.is_empty() {
                "—"
            } else {
                seq.last_done.as_str()
            }),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            " step  SPHINX × QR          phase              role",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    for row in PHASE_EVOLUTION_TABLE {
        let current = row.sphinx == seq.gate;
        let marker = if current { "▶" } else { " " };
        let style = if current {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        lines.push(Line::from(Span::styled(
            format!(
                " {marker} {}  {:4} × {:6}   {:?}→{:?}   {}",
                row.step,
                row.sphinx.label(),
                row.redstone.short(),
                row.from_phase,
                row.to_phase,
                row.role
            ),
            style,
        )));
    }

    if !seq.history.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "── history ──",
            Style::default().fg(Color::DarkGray),
        )));
        for h in seq.history.iter().rev().take(4) {
            lines.push(Line::from(Span::styled(
                format!("  {h}"),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let title = Line::from(vec![
        Span::raw(" Phases "),
        Span::styled("[s]", Style::default().fg(Color::Green)),
        Span::raw("spiral "),
        Span::styled("[g]", Style::default().fg(Color::Yellow)),
        Span::raw("gate "),
        Span::styled(
            format!(" layout:{} ", app.layout.id()),
            Style::default().fg(Color::Magenta),
        ),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Phases));

    frame.render_widget(Paragraph::new(lines).block(block), area);
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
// HITL / SAIF human-action board
// ---------------------------------------------------------------------------

fn draw_actions(frame: &mut Frame, app: &App, area: Rect) {
    use crate::human_actions::SessionStatus;

    let q = &app.actions;
    let need_id = q.first_need().map(|a| a.id.as_str());
    let budget = area.height.saturating_sub(6) as usize;

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        format!(
            "{} · {} · {} · seq {}",
            q.short(),
            q.loaded_from,
            q.constraint,
            q.seq
        ),
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        "j/k select · u gate · O reload · 8 layout · approve = receipt only",
        Style::default().fg(Color::DarkGray),
    )));
    if let Some(need) = q.first_need() {
        lines.push(Line::from(vec![
            Span::styled(" first_need ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(need.summary_line(), Style::default().fg(Color::White)),
        ]));
    } else {
        lines.push(Line::from(Span::styled(
            " first_need — none (queue clear this session)",
            Style::default().fg(Color::Green),
        )));
    }
    lines.push(Line::from(""));

    let start = q.selected.saturating_sub(budget.saturating_sub(1) / 2);
    for (i, item) in q.items.iter().enumerate().skip(start).take(budget.max(3)) {
        let selected = i == q.selected;
        let is_need = need_id == Some(item.id.as_str());
        let status_color = match item.status {
            SessionStatus::Open | SessionStatus::Requested => Color::Yellow,
            SessionStatus::Escalated => Color::Cyan,
            SessionStatus::Approved => Color::Green,
            SessionStatus::Deferred => Color::DarkGray,
            SessionStatus::Denied => Color::Red,
        };
        let marker = if selected { "▶" } else if is_need { "→" } else { " " };
        let fg = if selected { Color::Cyan } else { Color::Gray };
        lines.push(Line::from(vec![
            Span::styled(format!("{marker} "), Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{}{} ", item.who.glyph(), item.priority.as_str()),
                Style::default().fg(status_color),
            ),
            Span::styled(
                format!("{:<4}", item.id),
                Style::default().fg(fg).add_modifier(if selected {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
            ),
            Span::styled(
                format!(" {:<9} ", item.status.as_str()),
                Style::default().fg(status_color),
            ),
            Span::styled(truncate_ui(&item.title, 36), Style::default().fg(fg)),
        ]));
    }

    if !app.kit.is_empty() {
        lines.push(Line::from(Span::styled(
            "── kit (activator) ──",
            Style::default().fg(Color::DarkGray),
        )));
        for cap in app.kit.iter().take(6) {
            let color = if cap.present { Color::Green } else { Color::DarkGray };
            lines.push(Line::from(Span::styled(
                truncate_ui(&cap.line(), 52),
                Style::default().fg(color),
            )));
        }
    }

    if let Some(err) = &q.last_error {
        lines.push(Line::from(Span::styled(
            truncate_ui(err, 48),
            Style::default().fg(Color::Red),
        )));
    }

    let title = Line::from(vec![
        Span::raw(" HITL "),
        Span::styled("[o]", Style::default().fg(Color::Cyan)),
        Span::raw(" "),
        Span::styled(q.short(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Actions));

    frame.render_widget(
        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false }),
        area,
    );
}

// ---------------------------------------------------------------------------
// Observe-only git (guitar graph)
// ---------------------------------------------------------------------------

fn draw_git(frame: &mut Frame, app: &App, area: Rect) {
    let g = &app.git;
    let mut lines: Vec<Line> = Vec::new();
    for h in g.header_lines() {
        lines.push(Line::from(Span::styled(
            truncate_ui(&h, (area.width.saturating_sub(4)) as usize),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));
    }
    lines.push(Line::from(Span::styled(
        "G refresh · V fetch (ε) · observe only — no commit/push",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    let budget = area.height.saturating_sub(8) as usize;
    let files = g.status_files_budgeted(budget.min(6));
    if files.is_empty() {
        lines.push(Line::from(Span::styled(
            "working tree clean (or unprobed)",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (glyph, staged, path) in files {
            let color = match glyph {
                '!' => Color::Red,
                '?' => Color::DarkGray,
                '+' => Color::Green,
                _ => Color::Yellow,
            };
            lines.push(Line::from(vec![
                Span::styled(format!(" {glyph} "), Style::default().fg(color)),
                Span::styled(
                    if staged { "S " } else { "  " },
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(truncate_ui(path, 36), Style::default().fg(Color::Gray)),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "── graph ──",
        Style::default().fg(Color::DarkGray),
    )));
    let graph_budget = area.height.saturating_sub(14) as usize;
    for (i, row) in g.graph.iter().take(graph_budget.max(3)).enumerate() {
        let is_head = i == 0 && !row.is_uncommitted || (i == 1 && g.graph[0].is_uncommitted);
        lines.push(Line::from(Span::styled(
            row.display_line(is_head, (area.width.saturating_sub(4)) as usize),
            Style::default().fg(if row.is_uncommitted {
                Color::Yellow
            } else {
                Color::Gray
            }),
        )));
    }

    let title = Line::from(vec![
        Span::raw(" Git "),
        Span::styled("[G]", Style::default().fg(Color::Cyan)),
        Span::raw(" "),
        Span::styled(
            g.branch.clone(),
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style(app.focus == FocusPanel::Git));

    frame.render_widget(
        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false }),
        area,
    );
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
            Span::styled("  [C]", Style::default().fg(Color::DarkGray)),
        ]),
        // ── Residual R (Tier 1 observe — never deploy-green alone) ──────────
        {
            let r = b.residual_r;
            let claim = b.residual_zero_claim;
            let r_color = if !b.residual_has_sample {
                Color::DarkGray
            } else if claim {
                Color::Yellow // lab claim — amber, not production green
            } else {
                Color::LightRed
            };
            let claim_txt = if !b.residual_has_sample {
                "—"
            } else if claim {
                "R≤ε lab"
            } else {
                "R>ε"
            };
            Line::from(vec![
                Span::styled("R     ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{r:.5}  ε={}", crate::app::App::residual_eps()),
                    Style::default().fg(r_color),
                ),
                Span::raw("  "),
                Span::styled(
                    claim_txt,
                    Style::default().fg(r_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled("  ⚠C", Style::default().fg(Color::DarkGray)),
            ])
        },
        Line::from(vec![
            Span::styled("QDI   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!(
                    "drain={} eng_q={}  ε_hz≈{:.5}[C]",
                    b.last_drain_count,
                    b.engine_queue_depth,
                    crate::qr_meta::EPSILON_HZ_LABEL
                ),
                Style::default().fg(Color::Gray),
            ),
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
                TestStatus::Observed => ("○", Color::Gray),
                TestStatus::DryRun => ("▷", Color::Cyan),
                TestStatus::Running => ("…", Color::Yellow),
                TestStatus::Pass => ("✓", Color::Green),
                TestStatus::Fail => ("✗", Color::Red),
                TestStatus::Skip => ("–", Color::Magenta),
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{mark} "), Style::default().fg(color)),
                Span::styled(format!("{} ", t.id), Style::default().fg(Color::DarkGray)),
                Span::raw(format!("{} [{}] ", t.name, t.crate_name)),
                Span::styled(
                    format!("{}·{}", t.stage.id(), t.category),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
        })
        .collect();

    let block = Block::default()
        .title(" Tests [t] observe→dry-run→exec ")
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
        let br = if app.bridge.connected { "bridge◉" } else { "bridge○ logos-bridge" };
        let lay = app.layout.id();
        let host = app.surface.id();
        let lat = app.lattice.short();
        let hitl = app.actions.short();
        let need = app
            .actions
            .first_need()
            .map(|a| format!("{} {}", a.id, a.title))
            .unwrap_or_else(|| "clear".into());
        Line::from(Span::styled(
            format!(
                " [?]help  [l]{lay}  [1-8]  [o]{hitl} first:{need}  [A]{lat}  [c]codes  [q]  | {host} | {br} "
            ),
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

// ---------------------------------------------------------------------------
// Operator / agent-terminal help (learning overlay — no functionality cut)
// ---------------------------------------------------------------------------

fn draw_help_overlay(frame: &mut Frame, app: &App) {
    let area = centered_rect(78, 72, frame.area());
    frame.render_widget(Clear, area);

    let bridge = if app.bridge.connected {
        "◉ live  ws://127.0.0.1:8088"
    } else {
        "○ down — run: logos-bridge   or  tw up bridge"
    };
    let lean = if app.lean_lsp_ok { "lean●" } else { "lean○" };
    let als = if app.als_lsp_ok { "als●" } else { "als○" };

    let lines: Vec<Line> = vec![
        Line::from(Span::styled(
            " RESON8-TUI · AGENT TERMINAL GUIDE ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "  Quick config exercise — learn the frame before orchestration automates it.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " 1) This window (α dashboard)",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("    Bridge: {bridge}")),
        Line::from(format!(
            "    Host:   {} → default layout {}  ({})",
            app.surface.label(),
            app.surface.default_layout().label(),
            if app.layout_pinned {
                "overridden by RESON8_LAYOUT"
            } else {
                "auto-selected"
            }
        )),
        Line::from(format!(
            "    Formal: {lean} {als}   ·  layout: {} ({}/{})  ·  Tab · f Formal · c Codes",
            app.layout.label(),
            app.layout.index() + 1,
            crate::layout_presets::LayoutKind::ALL.len()
        )),
        Line::from("    Providers show Claude / Grok / Gemini strands (health from bridge telemetry)."),
        Line::from(""),
        Line::from(Span::styled(
            " 2) Workflow layouts  [l] next  [L] prev  [1-8] jump  ·  env RESON8_LAYOUT",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    1 ops       full dashboard (…·braid·net·logs·formal·tests)"),
        Line::from("    2 formal    Lean/Agda eye — formal top, logs+tests+braid bottom"),
        Line::from("    3 agent     strand watch — providers·logs·braid·formal"),
        Line::from("    4 monitor   pipeline WAVE — braid·net·logs·tests"),
        Line::from("    5 quantum   QR×SPHINX phases — phases top, braid·formal·logs"),
        Line::from("    6 minimal   braid + logs only"),
        Line::from("    7 codes     Hexacode · Golay G24 · Reed–Muller · SC-LDPC"),
        Line::from("    8 hitl      SAIF ⚑ queue — request · escalate · approve (receipt only)"),
        Line::from(""),
        Line::from(Span::styled(
            " 2b) Codes lab  [c] panel  [d] demo  [D] battery  [y] family  [e] t±",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Hex t=1 · G24 t=3 MOG · RM FHT(r=1) · SC-LDPC windowed BEC"),
        Line::from("    [ ] RM r or SC w · { } RM m or SC L · A construction · B decoders"),
        Line::from("    G24≠RM≠SC-LDPC · sat: σ_BP→σ_MAP · Lean: K22.HexacodeGolay"),
        Line::from(""),
        Line::from(Span::styled(
            " 2c) Net proxy stack  [N] panel  [R] probe  [M] menu",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    privacy: Tor:9050 Privoxy:8118 i2pd:4444 DNSCrypt:5353"),
        Line::from("    gaming:  clearnet high-speed (no Tor/I2P) — ops/net/LogOS.NetProxy.ps1"),
        Line::from(""),
        Line::from(Span::styled(
            " 2d) Lattice activate  [A] re-probe  ·  logos-activate / logos-lattice",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(format!(
            "    {} · {}",
            app.lattice.short(),
            app.lattice.interweave_short()
        )),
        Line::from("    layers: apps · cutiles · crates · kernels · ops  (presence, not deploy-green)"),
        Line::from("    interweave: coherence-mcp · spiral-safe · quantum-redstone · HOPE-NPC"),
        Line::from(""),
        Line::from(Span::styled(
            " 2e) HITL gate  [o] panel  [O] reload  [u] request/escalate/approve  [8] layout",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(format!(
            "    {} · first_need from {} · ATOM-GROK-TUI-HITL-GATE-20260818",
            app.actions.short(),
            app.actions.loaded_from
        )),
        Line::from("    Approve writes ops/marks/hitl-receipts.jsonl — does not run GCP/sudo"),
        Line::from("    Clock is ATOM constraint order, not wall time. capability ≠ authority"),
        Line::from(""),
        Line::from(Span::styled(
            " 2f) Git observe  [G] refresh  [V] fetch  ·  activator kit on HITL board",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("    {} · no commit/push from TUI", app.git.reconcile_note())),
        Line::from("    Shared with the agent: files + receipts. Not shared: this framebuffer."),
        Line::from(""),
        Line::from(Span::styled(
            " 3) Partition the host terminal (Windows Terminal)",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Alt+Shift+D   split pane  ·  leave this TUI running"),
        Line::from(""),
        Line::from(Span::styled(
            " 4) Agent shell in the new split",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    claude | grok | logos-mcp gauge  (Reason / Monitor strands)"),
        Line::from(""),
        Line::from(Span::styled(
            " 5) In-TUI keys (no external agent required)",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Tab focus · s spiral · g gate · t tests · c codes · o HITL · u gate"),
        Line::from("    p paper · n publish · l/L layout · 8 hitl · ?/h help · q quit"),
        Line::from(""),
        Line::from(Span::styled(
            " Later: FORGE_WS · capability ≠ authority · α+ω=15 (Category C label)",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            " Esc or ? to close ",
            Style::default().fg(Color::Cyan),
        )),
    ];

    let block = Block::default()
        .title(" HELP · LAYOUTS + AGENT + PANELS ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
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
