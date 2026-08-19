//! CTQW Monitor — Eye of LogOS (Norton–Sakuma wavepacket diagnostics).
//!
//! Theme: Midnight Navy Void · Gold Leaf · Cyan Citation · Purple Vector
//! Run: `cargo run -p reson8-tui --example ctqw_monitor`
//!
//! Keys: q quit · space pause · a toggle 3A/3C · ←/→ step t · r reset

use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use cutile::griess::{
    get_axial_constants, simulate_ctqw, AxialAlgebra, NortonSakumaKind, DELTA_3A, DELTA_3C,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph, Sparkline, Wrap};
use ratatui::{DefaultTerminal, Frame};

// Midnight Navy Void / Gold Leaf / Cyan / Purple
const NAVY: Color = Color::Rgb(0x06, 0x0a, 0x16);
const GOLD: Color = Color::Rgb(0xff, 0xd2, 0x50);
const CYAN: Color = Color::Rgb(0x00, 0xc8, 0xff);
const PURPLE: Color = Color::Rgb(0x8c, 0x3c, 0xdc);
const DIM: Color = Color::Rgb(0x4a, 0x55, 0x6a);

struct Monitor {
    algebra: NortonSakumaKind,
    t: f64,
    dt: f64,
    paused: bool,
    frame: u64,
    history_3c: Vec<u64>,
    history_3a: Vec<u64>,
    l2_history: Vec<u64>,
    quit: bool,
}

impl Monitor {
    fn new() -> Self {
        Self {
            algebra: NortonSakumaKind::ThreeC,
            t: 0.0,
            dt: 0.05,
            paused: false,
            frame: 0,
            history_3c: Vec::with_capacity(64),
            history_3a: Vec::with_capacity(64),
            l2_history: Vec::with_capacity(64),
            quit: false,
        }
    }

    fn tick(&mut self) {
        if self.paused {
            return;
        }
        self.t += self.dt;
        self.frame = self.frame.wrapping_add(1);
        self.sample();
    }

    fn sample(&mut self) {
        let push = |hist: &mut Vec<u64>, v: f64| {
            let scaled = (v * 1000.0).clamp(0.0, 1000.0) as u64;
            hist.push(scaled);
            if hist.len() > 48 {
                hist.remove(0);
            }
        };

        if let Ok(c) = simulate_ctqw("3C", self.t, None) {
            push(&mut self.history_3c, c.value.peak_probability);
            let l2_err = (c.value.l2_mass - 1.0).abs();
            push(&mut self.l2_history, 1.0 - l2_err.min(1.0));
        }
        if let Ok(c) = simulate_ctqw("3A", self.t, None) {
            push(&mut self.history_3a, c.value.peak_probability);
        }
    }

    fn toggle_algebra(&mut self) {
        self.algebra = match self.algebra {
            NortonSakumaKind::ThreeC => NortonSakumaKind::ThreeA,
            NortonSakumaKind::ThreeA => NortonSakumaKind::ThreeC,
        };
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut mon = Monitor::new();
    mon.sample();
    let result = run(&mut terminal, &mut mon);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal, mon: &mut Monitor) -> io::Result<()> {
    let tick = Duration::from_millis(50);
    let mut last = Instant::now();

    while !mon.quit {
        terminal.draw(|f| draw(f, mon))?;

        let timeout = tick.saturating_sub(last.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => mon.quit = true,
                        KeyCode::Char(' ') => mon.paused = !mon.paused,
                        KeyCode::Char('a') | KeyCode::Char('A') => mon.toggle_algebra(),
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            mon.t = 0.0;
                            mon.frame = 0;
                            mon.history_3c.clear();
                            mon.history_3a.clear();
                            mon.l2_history.clear();
                            mon.sample();
                        }
                        KeyCode::Left => {
                            mon.t = (mon.t - mon.dt).max(0.0);
                            mon.sample();
                        }
                        KeyCode::Right => {
                            mon.t += mon.dt;
                            mon.sample();
                        }
                        _ => {}
                    }
                }
            }
        }
        if last.elapsed() >= tick {
            mon.tick();
            last = Instant::now();
        }
    }
    Ok(())
}

fn draw(f: &mut Frame, mon: &Monitor) {
    let area = f.area();
    // solid navy background via full-frame block
    let bg = Block::default().style(Style::default().bg(NAVY));
    f.render_widget(bg, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(8),    // main
            Constraint::Length(3), // trail bar
        ])
        .split(area);

    draw_header(f, rows[0], mon);
    draw_main(f, rows[1], mon);
    draw_trail(f, rows[2]);
}

fn draw_header(f: &mut Frame, area: Rect, mon: &Monitor) {
    let status = format!(
        " NOVIKOV: unrepaired · Residual-zero: observe only · CTQW Frame {:02}: ACTIVE · Betti= · Hexaflake=gold-leaf-recursive · alg={} t={:.2}",
        mon.frame % 100,
        mon.algebra.as_str(),
        mon.t
    );
    let p = Paragraph::new(Line::from(vec![
        Span::styled(" Eye of LogOS ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
        Span::styled(status, Style::default().fg(CYAN)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(GOLD))
            .style(Style::default().bg(NAVY)),
    );
    f.render_widget(p, area);
}

fn draw_main(f: &mut Frame, area: Rect, mon: &Monitor) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(35),
            Constraint::Percentage(25),
        ])
        .split(area);

    draw_griess(f, cols[0], mon);
    draw_wavepacket(f, cols[1], mon);
    draw_gaps(f, cols[2], mon);
}

fn draw_griess(f: &mut Frame, area: Rect, mon: &Monitor) {
    let alg = AxialAlgebra::for_kind(mon.algebra);
    let matrix = alg.format_matrix_panel();
    let claim_note = get_axial_constants(mon.algebra.as_str())
        .map(|c| format!("Claim cat={:?} · may_gate={}", c.category(), c.may_gate()))
        .unwrap_or_else(|e| e);
    let text = format!("{matrix}\n\n{claim_note}");
    let p = Paragraph::new(text)
        .style(Style::default().fg(GOLD).bg(NAVY))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(" Griess Matrix ")
                .title_style(Style::default().fg(GOLD).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(PURPLE))
                .style(Style::default().bg(NAVY)),
        );
    f.render_widget(p, area);
}

fn draw_wavepacket(f: &mut Frame, area: Rect, mon: &Monitor) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(area);

    let state = simulate_ctqw(mon.algebra.as_str(), mon.t, None);
    let (probs, l2, ipr, peak) = match &state {
        Ok(c) => (
            c.value.probabilities.clone(),
            c.value.l2_mass,
            c.value.ipr,
            c.value.peak_probability,
        ),
        Err(e) => {
            let p = Paragraph::new(e.as_str()).style(Style::default().fg(Color::Red));
            f.render_widget(p, area);
            return;
        }
    };

    let block = Block::default()
        .title(" CTQW Wavepacket Monitor ")
        .title_style(Style::default().fg(CYAN).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(CYAN))
        .style(Style::default().bg(NAVY));
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(3),
            Constraint::Length(2),
        ])
        .split(ratatui::layout::Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        });

    // Site probability bars
    let mut lines = Vec::new();
    for (i, p) in probs.iter().enumerate() {
        let bar_w = ((p * 20.0) as usize).min(20);
        let bar: String = "█".repeat(bar_w) + &"░".repeat(20 - bar_w);
        lines.push(Line::from(vec![
            Span::styled(format!("s{i} "), Style::default().fg(DIM)),
            Span::styled(bar, Style::default().fg(PURPLE)),
            Span::styled(format!(" {p:.4}"), Style::default().fg(GOLD)),
        ]));
    }
    f.render_widget(Paragraph::new(lines), inner[0]);

    // Peak gauge
    let g = Gauge::default()
        .block(Block::default().title(" peak localisation ").style(Style::default().fg(DIM)))
        .gauge_style(Style::default().fg(GOLD).bg(NAVY))
        .ratio(peak.clamp(0.0, 1.0))
        .label(format!("peak={peak:.4} IPR={ipr:.4}"));
    f.render_widget(g, inner[1]);

    // Sparklines of peak prob history
    let spark_3c = Sparkline::default()
        .block(Block::default().title(" Δ·peak 3C ").border_style(Style::default().fg(CYAN)))
        .style(Style::default().fg(CYAN))
        .data(&mon.history_3c)
        .max(1000);
    f.render_widget(spark_3c, inner[2]);

    let l2_line = Paragraph::new(Line::from(vec![
        Span::styled(" L₂ mass ", Style::default().fg(DIM)),
        Span::styled(
            format!("{l2:.9}"),
            Style::default().fg(if (l2 - 1.0).abs() < 1e-6 {
                GOLD
            } else {
                Color::Red
            }),
        ),
        Span::styled("  ·  tolerance ±1e-6", Style::default().fg(DIM)),
    ]));
    f.render_widget(l2_line, inner[3]);

    let _ = chunks; // layout reserved
}

fn draw_gaps(f: &mut Frame, area: Rect, mon: &Monitor) {
    let gap_3c = DELTA_3C;
    let gap_3a = DELTA_3A;
    let active = mon.algebra.spectral_gap();
    let text = format!(
        "Spectral gaps (canonical)\n\n\
         Δ₃C = 3/32     = {gap_3c:.6}\n\
         Δ₃A = 103/256  = {gap_3a:.6}\n\n\
         active ({}) = {active:.6}\n\n\
         WAVE floor ≥ 0.995\n\
         α + ω = 15 conserved\n\n\
         [a] algebra  [space] pause\n\
         [←/→] step t  [r] reset  [q] quit",
        mon.algebra.as_str()
    );
    let p = Paragraph::new(text)
        .style(Style::default().fg(CYAN).bg(NAVY))
        .block(
            Block::default()
                .title(" Gap Compare ")
                .title_style(Style::default().fg(PURPLE).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(PURPLE))
                .style(Style::default().bg(NAVY)),
        );
    f.render_widget(p, area);
}

fn draw_trail(f: &mut Frame, area: Rect) {
    let p = Paragraph::new(Line::from(vec![
        Span::styled(
            " ATOM-TRAIL · CTQW-HEIGHT1-K1-LOCALISATION-20260807 · α+ω=15 conserved ",
            Style::default().fg(GOLD).bg(NAVY).add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(GOLD))
            .style(Style::default().bg(NAVY)),
    );
    f.render_widget(p, area);
}
