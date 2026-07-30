//! Error storyboard frame renderer for TUI.
//!
//! Used by `triweave doctor` when running in interactive TUI mode.
//! For now, the CLI mode in doctor.rs handles rendering directly.
//! This module will be expanded for full TUI storyboard with
//! [R]etry / [S]kip / [A]bort / [→]Next keybindings.
//!
//! Conservation: α + ω = 15

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::doctor::ErrorFrame;

/// Render an error frame in TUI mode.
pub fn render_error_frame(frame: &mut Frame, area: Rect, error: &ErrorFrame) {
    let title = format!(" ERROR FRAME {}/{} ", error.index, error.total);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let wave_color = if error.wave_after < 0.85 {
        Color::Red
    } else {
        Color::Yellow
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("✗ ", Style::default().fg(Color::Red)),
            Span::styled(
                &error.error,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("WHY: ", Style::default().fg(Color::DarkGray)),
            Span::raw(&error.why),
        ]),
        Line::from(vec![
            Span::styled("WAVE: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.2}", error.wave_before),
                Style::default().fg(Color::Green),
            ),
            Span::raw(" → "),
            Span::styled(
                format!("{:.2}", error.wave_after),
                Style::default().fg(wave_color),
            ),
            if error.wave_after < 0.85 {
                Span::styled(" (below threshold)", Style::default().fg(Color::Red))
            } else {
                Span::raw("")
            },
        ]),
        Line::from(""),
        if let Some(fix) = &error.auto_fix {
            Line::from(vec![
                Span::styled("AUTO-FIX: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{}", fix)),
            ])
        } else {
            Line::from(Span::styled(
                "No auto-fix available — manual intervention required",
                Style::default().fg(Color::Yellow),
            ))
        },
        Line::from(""),
        Line::from(vec![
            Span::styled("[R]", Style::default().fg(Color::Cyan)),
            Span::raw("etry  "),
            Span::styled("[S]", Style::default().fg(Color::Yellow)),
            Span::raw("kip  "),
            Span::styled("[A]", Style::default().fg(Color::Red)),
            Span::raw("bort  "),
            Span::styled("[→]", Style::default().fg(Color::Green)),
            Span::raw("Next"),
        ]),
    ];

    frame.render_widget(Paragraph::new(lines).block(block), area);
}
