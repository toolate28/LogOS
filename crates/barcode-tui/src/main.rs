//! barcode-tui binary entry.
//!
//! Event loop driven by `crossterm` events + a 60Hz tick. All visual state
//! lives in [`barcode_tui::AppState`]. Keys:
//!   - `q`     : quit
//!   - `space` : pause/resume ε sweep
//!   - `r`     : reset (ε = 0)
//!   - `←/→`   : step ε manually (±1/60 of ε_max)
//!   - `1/2/3` : switch cloud (Circle / TwoBlobs / Grid)

use std::io;
use std::time::{Duration, Instant};

use barcode_tui::{AppState, Cloud};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};

#[derive(Parser)]
#[command(name = "barcode-tui")]
#[command(about = "Terminal-native persistent-homology barcodes (first-in-Rust)")]
struct Args {
    /// Which point cloud to seed with.
    #[arg(long, value_enum, default_value_t = Cloud::Circle)]
    cloud: Cloud,

    /// Number of points.
    #[arg(long, default_value_t = 24)]
    points: usize,
}

const TICK: Duration = Duration::from_millis(16); // ≈60 Hz

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut state = AppState::new(args.cloud, args.points);

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &mut state);
    ratatui::restore();
    result
}

fn run<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    state: &mut AppState,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    while !state.should_quit {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40), // cloud
                    Constraint::Percentage(55), // barcodes
                    Constraint::Length(1),      // status
                ])
                .split(size);

            barcode_tui::draw_cloud(f, chunks[0], state);
            barcode_tui::draw_barcodes(f, chunks[1], state);
            barcode_tui::draw_status(f, chunks[2], state);
        })?;

        let timeout = TICK
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => state.should_quit = true,
                        KeyCode::Char(' ') => state.paused = !state.paused,
                        KeyCode::Char('r') => state.reset(),
                        KeyCode::Left => state.step_eps(-1.0 / 60.0),
                        KeyCode::Right => state.step_eps(1.0 / 60.0),
                        KeyCode::Char('1') => state.switch_cloud(Cloud::Circle),
                        KeyCode::Char('2') => state.switch_cloud(Cloud::TwoBlobs),
                        KeyCode::Char('3') => state.switch_cloud(Cloud::Grid),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= TICK {
            state.tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
