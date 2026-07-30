//! Ratatui draw functions. Three primitives exported for forge-cockpit reuse:
//! `draw_cloud`, `draw_barcodes`, `draw_status`.
//!
//! - `draw_status` is implemented (α-rail, Reason strand).
//! - `draw_cloud` and `draw_barcodes` are `TODO(gemini)` — they want a
//!   proper braille canvas, which is Scale/Multimodal-shaped work.

use crate::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use reson8_core::{enforce_invariant, InvariantStatus};

/// Draw the point cloud pane with ε-balls around the current cursor.
///
/// TODO(gemini): implement — braille canvas. Use
/// `ratatui::widgets::canvas::Canvas` with `symbols::Marker::Braille`.
/// Draw one dot per point; draw a circle of radius `state.epsilon` around
/// each. Until then, render a placeholder summary.
pub fn draw_cloud(f: &mut Frame, area: Rect, state: &AppState) {
    let text = vec![
        Line::from(format!(" Cloud: {:?}", state.cloud)),
        Line::from(format!(" Points: {}", state.n_points)),
        Line::from(format!(" ε:  {:.4}  /  ε_max: {:.4}", state.epsilon, state.eps_max)),
        Line::from(format!(" Alive bars: {}/{}", state.alive_bars(), state.barcodes.len())),
        Line::from(""),
        Line::from(Span::styled(
            " [braille scatter — TODO(gemini)] ",
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
        )),
    ];
    f.render_widget(Paragraph::new(text), area);
}

/// Draw the barcode pane — one horizontal bar per H0 interval.
///
/// TODO(gemini): implement proper braille barcode widget with ε cursor
/// overlay. Placeholder below lists intervals as text.
pub fn draw_barcodes(f: &mut Frame, area: Rect, state: &AppState) {
    let mut lines = Vec::with_capacity(state.barcodes.len().min(area.height as usize));
    for (i, b) in state.barcodes.iter().take(area.height as usize).enumerate() {
        let alive = b.birth <= state.epsilon && state.epsilon < b.death;
        let glyph = if alive { "█" } else { "░" };
        lines.push(Line::from(Span::styled(
            format!(
                " H{:01} [{:>2}] birth={:.3}  death={:.3}  {}",
                b.dim, i, b.birth, b.death, glyph
            ),
            if alive {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        )));
    }
    f.render_widget(Paragraph::new(lines), area);
}

/// Draw the status line. Enforces the Universal Invariant on every render:
/// if α + ω ≠ 15 ± tolerance, flashes magenta.
pub fn draw_status(f: &mut Frame, area: Rect, state: &AppState) {
    let inv = enforce_invariant(state.alpha, state.omega);
    let (sum_glyph, style) = match inv.status {
        InvariantStatus::Passed => ("✓", Style::default().fg(Color::Yellow)),
        InvariantStatus::Rejected => (
            "✗",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
    };

    let void_class = if state.wave_score >= 0.90 {
        "V0"
    } else if state.wave_score >= 0.75 {
        "V1"
    } else if state.wave_score >= 0.50 {
        "V2"
    } else {
        "V3"
    };

    let line = Line::from(vec![
        Span::styled(" WAVE=", Style::default().fg(Color::Cyan)),
        Span::raw(format!("{:.3}", state.wave_score)),
        Span::raw("  │  "),
        Span::styled("α=", style),
        Span::raw(format!("{:.0}", state.alpha)),
        Span::raw("  "),
        Span::styled("ω=", style),
        Span::raw(format!("{:.0}", state.omega)),
        Span::raw("  SUM="),
        Span::raw(format!("{:.0}", inv.total)),
        Span::raw("  "),
        Span::styled(sum_glyph, style),
        Span::raw("  │  VOID="),
        Span::styled(void_class, Style::default().fg(Color::Green)),
        Span::raw("  │  q=quit  space=pause  1/2/3=cloud"),
    ]);
    f.render_widget(Paragraph::new(line), area);
}
