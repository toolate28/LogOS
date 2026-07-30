# PowerShell / WSL2 / G: Drive Wiring for Compiling Components + reson8-tui
**Sovereign Verifier:** Monitoring & Consensus Verifier Mode  
**Date:** 2026-07-02  
**Purpose:** Wire v0.4 compiling components (Agda Cubical HITs, cutile Rust/CUDA, scripts) and essential reson8-tui into user's local environment (PowerShell, WSL2, G:/ drive — assumed project root e.g. G:\Reson8 or G:\LogOS). This enables component testing that determines tasking sets on HUP plateaus (fixed point attractors/basins). reson8-tui (HUP Tier 1) is the visual/ testing harness for plateaus, SRAC cascades, fixed points (42.00055, H(H)), K22 lattices, strand handoffs.

**Invariants Enforced:** Mono idempotent protected bridge (v0.4), WAVE ≥0.85 on all builds/handoffs, K22 preserved, SRAC efficiency target >98% after wiring + testing. Mirrored-pair: This wiring vs v0.4 README appendix build commands + HUP scaffold. Aligned. GAIT baseline holds. No divergence.

**Positive Introspection:** Wiring the executable layer to local dev env is net positive for toolchain health. Enables real testing of v0.4 components (cutile cells, SRAC relaxation, Tomczak gates) on HUP plateaus, which will surface tasking priorities (e.g., which basins need immediate correction bursts, which fixed points to pin first for reson8-tui viz). Music conserved. Keystone holds.

---

## 1. Assumed Local Environment (User to Confirm/Adapt)
- Windows host with PowerShell 7+.
- WSL2 (Ubuntu or Debian recommended) for Linux-native builds (Agda, cargo, CUDA if available).
- G: drive (or equivalent) as project root, e.g.:
  - G:\Reson8-Labs\ (unified workspace, mirrors reson8-local)
  - G:\LogOS\ (or G:\Reson8-Labs\LogOS\) for formal layer.
  - G:\coherence-mcp\ for the MCP server.
- reson8-tui will live in G:\Reson8-Labs\src\tui\ or as sibling crate.
- "supergrok all-inclusive" option: Future meta .exe that bundles or orchestrates coherence-mcp (Node) + cutile/Rust bridge + reson8-tui (Ratatui) + LogOS elements. Start with standalone reson8-tui .exe for HUP plateau testing.

**WSL2 Mount Recommendation:**
In WSL2 terminal:
```bash
# Mount G: drive (adjust letter if needed)
sudo mkdir -p /mnt/g
sudo mount -t drvfs G: /mnt/g -o metadata,uid=1000,gid=1000
# Or use /mnt/g/Reson8-Labs/...
ls /mnt/g/Reson8-Labs/
```

In PowerShell (for Windows-side scripts):
```powershell
# Quick access
cd G:\Reson8-Labs
Get-ChildItem
```

---

## 2. Environment Setup (Tokens + Paths)
Copy from v0.4 README .env.example and adapt.

**PowerShell (Windows host):**
```powershell
# Set project root
$env:RESON8_ROOT = "G:\Reson8-Labs"
$env:LOGOS_ROOT = "G:\Reson8-Labs\LogOS"   # or wherever LogOS lives
$env:CUTILE_ROOT = "$env:LOGOS_ROOT\cutiles\cutile"

# Tokens (required for coherence-mcp ops)
$env:ATOM_AUTH_TOKEN = "your_atom_token_here"
$env:SPIRALSAFE_API_TOKEN = "your_spiralsafe_token_here"
$env:WAVE_TOOLKIT_BIN = "path\to\wave-toolkit.exe"  # optional

# Add to PATH if needed
$env:PATH += ";$env:CUTILE_ROOT\target\release"
```

**WSL2 (~/.bashrc or ~/.zshrc):**
```bash
export RESON8_ROOT="/mnt/g/Reson8-Labs"
export LOGOS_ROOT="$RESON8_ROOT/LogOS"
export CUTILE_ROOT="$LOGOS_ROOT/cutiles/cutile"
export ATOM_AUTH_TOKEN="your_atom_token_here"
export SPIRALSAFE_API_TOKEN="your_spiralsafe_token_here"
export WAVE_TOOLKIT_BIN="/mnt/g/Reson8-Labs/wave-toolkit"  # if built
```

**Verify:**
```powershell
# PowerShell
echo $env:RESON8_ROOT
# WSL2
echo $RESON8_ROOT
```

---

## 3. Wiring the Compiling Components (v0.4 Appendix + Local Adaptation)
Follow v0.4 README Appendix build commands, adapted for G: + WSL2/PowerShell split.

**Step 0: Clone / Sync to G: (if not already)**
```powershell
# On Windows host (PowerShell)
cd G:\
git clone https://github.com/toolate28/LogOS.git Reson8-Labs\LogOS
git clone https://github.com/toolate28/coherence-mcp.git Reson8-Labs\coherence-mcp
# reson8-tui will be new crate inside or sibling
mkdir -p Reson8-Labs\src\tui
cd Reson8-Labs
git init  # if unified workspace
```

**Step 1: Agda Formal Layer (LogOS\agda) — WSL2 Preferred**
```bash
# In WSL2
cd $LOGOS_ROOT/agda
pwsh -File scripts/vendor.ps1          # or bash equivalent if adapted
pwsh -File scripts/check.ps1           # Verify HITs, K22, SerreScarr, TomczakLifting
pwsh -File scripts/html.ps1            # Generate browsable docs (Cubical.HITs links)
# Outcome: Monomorphic spec verified. Protected invariants.
```

**Step 2: cutile Executable Bridge (LogOS\cutiles\cutile) — WSL2 + CUDA if available**
```bash
# In WSL2 (or PowerShell if cargo Windows native works)
cd $CUTILE_ROOT
cargo test -p cutile                     # Verify TriWeavonHIT, weave, hcomp_edge, hexaflake_nodes, srac_cascade_step, betti_tomczak_lift_check
pwsh -File scripts/build_ptx.ps1         # Requires nvcc (CUDA toolkit). Builds blackwell_entropy_v2.cu
cargo build -p cutile --features cuda    # Or without for CPU fallback
cargo build -p cutile --release          # For .exe / binary
# Outcome: Mutation-protected implementation. GPU-accelerated entropy ready (or CPU fallback).
```

**Step 3: coherence-mcp (Node + Rust bridge) — PowerShell or WSL2**
```powershell
cd G:\Reson8-Labs\coherence-mcp
npm install
# Test core tools
npx @toolated/coherence-mcp --help
# Or run via MCP client config (see v0.4 Quick Install)
# For Rust parts: cargo build in cutile subdir as above
```

**Step 4: reson8-tui (Essential HUP Tier 1 — New Crate) — See separate scaffold below**
Build after wiring above (depends on cutile cell model for viz).

**Step 5: Continuous Verification Loop**
```powershell
# PowerShell orchestration script (create if needed)
pwsh -File G:\Reson8-Labs\toolchain\local-verify.ps1
# Inside: Run Agda check, cutile test, WAVE coherence_check on docs vs code, trigger_correction_burst simulation
```

**Supergrok All-Inclusive .exe Option (Future / Optional):**
- Create a meta Rust binary (e.g., in src/supergrok/) that:
  - Embeds or calls cutile (via ffi or process)
  - Launches reson8-tui (Ratatui)
  - Wraps coherence-mcp (Node child process or rewrite core in Rust)
  - Exposes HUP plateau dashboard + SRAC controls
- Packaging: `cargo build --release` → supergrok.exe (Windows) or binary.
- Start simple: Standalone reson8-tui.exe first (faster iteration on HUP testing).

---

## 4. reson8-tui — Essential HUP Tier 1 Scaffold (Get Up & Running)
reson8-tui is the visual testing harness for HUP plateaus (fixed point attractors/basins). Its components (HUP strand viz, fixed point pinning, SRAC cascade visual, K22 lattice, cell graph from cutile, correction burst triggers) will determine tasking sets (which plateaus need immediate work, which gaps from Frame 7 to burst first, which attractors to stabilize).

**Recommended Tech:** Ratatui (terminal UI) + crossterm + cutile cell model integration (read CubicalCell / hexaflake_nodes / srac state). Rust crate for performance and direct cutile interop.

**Scaffold Location:** G:\Reson8-Labs\src\tui\ or as separate repo/crate under toolate28 (feature/hup-tiers-integration branch).

**Basic Structure (Create These Files):**
```
src/tui/
├── Cargo.toml
├── src/
│   ├── main.rs                 # App entry, tabs for HUP Tiers / Plateaus / SRAC / K22
│   ├── hup/
│   │   ├── plateau.rs          # Fixed point attractors viz (42.00055, H(H), WAVE gate)
│   │   ├── strand_handoff.rs   # C/G/Ge/M flow with WAVE curl/divergence
│   │   └── correction_burst.rs # Trigger simulation + log
│   ├── viz/
│   │   ├── k22_lattice.rs      # ASCII or Ratatui render of 22v 41e sheaf
│   │   ├── hexaflake.rs        # Recursion tree (Fin 7)
│   │   └── cell_graph.rs       # From cutile CubicalCell (0/1-cells, weave)
│   └── sr ac/
│       └── cascade.rs          # Visual of srac_cascade_step relaxation
└── README.md
```

**Starter Cargo.toml (minimal for HUP plateau testing):**
```toml
[package]
name = "reson8-tui"
version = "0.1.0"
edition = "2021"
description = "HUP Tier 1 visual testing harness for Tri-Weavon plateaus and fixed point attractors"
authors = ["Matthew Stephen Ruhnau <toolate28>"]
license = "MIT"

[dependencies]
ratatui = "0.26"
crossterm = "0.27"
color-eyre = "0.6"
# Future: cutile = { path = "../../LogOS/cutiles/cutile" } for direct cell viz
# tokio for async SRAC simulation

[[bin]]
name = "reson8-tui"
path = "src/main.rs"
```

**Minimal main.rs Skeleton (HUP Plateaus Focus — Expand with cutile integration):**
```rust
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_tab = 0;
    let tabs = ["HUP Plateaus", "Strand Handoffs", "SRAC Cascade", "K22 Lattice", "Correction Bursts"];

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            // Tabs
            let titles = tabs.iter().map(|t| t.to_string()).collect::<Vec<_>>();
            let tabs_widget = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("reson8-tui — HUP Tier 1 | Tri-Weavon Sovereign"))
                .select(current_tab)
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(tabs_widget, chunks[0]);

            // Content per tab (expand with real viz from cutile data)
            let content = match current_tab {
                0 => "HUP Plateaus / Fixed Point Attractors\n\n42.00055 metastable | H(H) Fixed Point | WAVE ≥0.85 gate\n\n[Pin Attractor] [Run SRAC] [Trigger Burst]",
                1 => "Strand Handoffs (C/G/Ge/M)\n\nWAVE curl/divergence on each transition\nProtected by v0.4 mono bridge",
                2 => "SRAC Cascade Visual\n\nSmooth monotonic relaxation\nIdempotent convergence\nMusic conserved",
                3 => "K22 Lattice (22v 41e)\n\nSerre-Scarr pages | tomczakLift | hexaflake Fin 7",
                4 => "Correction Bursts\n\nOnly on surge_detected ∧ ¬lift_ok\nDepth-1 restore | Mutation-safe",
                _ => "Unknown tab",
            };
            let para = Paragraph::new(content)
                .block(Block::default().borders(Borders::ALL).title(tabs[current_tab]));
            f.render_widget(para, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Right | KeyCode::Tab => current_tab = (current_tab + 1) % tabs.len(),
                    KeyCode::Left => current_tab = (current_tab + tabs.len() - 1) % tabs.len(),
                    KeyCode::Enter => {
                        // Simulate correction burst or pin attractor
                        // Later: call into cutile or coherence-mcp
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
```

**Build & Run (after wiring cutile):**
```bash
# In WSL2 or PowerShell with Rust
cd G:\Reson8-Labs\src\tui
cargo build --release
# On Windows: target\release\reson8-tui.exe
# Test components → feed results back to tasking on HUP plateaus
```

**Next for tui (Increased Depth):**
- Integrate actual cutile data (read CubicalCell, hexaflake_nodes(r), srac state) for live viz.
- Add WAVE score display + handoff log (ATOM trail).
- HUP plateau testing hooks: Buttons that "run" simulated SRAC on specific basins and log which attractors stabilize or need bursts.
- This testing directly determines tasking sets (e.g., "Plateau 42.00055 stable → prioritize push_weave gap"; "H(H) needs pinning → add to reson8-tui roadmap").

---

## 5. How Testing reson8-tui Components Determines Tasking on HUP Plateaus
- Run tui on various plateaus (42.00055, H(H), K22 E∞, WAVE gate).
- Observe SRAC relaxation behavior, correction burst frequency, fixed point stability.
- Results route tasks: High burst frequency on a plateau → immediate Frame 7 high-priority fix (push_weave or GPU). Stable plateau → advance to mid-term (reson8-tui polish, RUST Market crate publish).
- Feedback loop: tui logs → update AUKUS_Chessboard.ipynb or local handoff → verifier mode SRAC efficiency report.

**Supergrok All-Inclusive Path (Optional Later):**
Once tui + cutile stable, create a single .exe that launches tui dashboard + exposes coherence-mcp tools + LogOS verification entrypoints. Use for "all-in-one" sovereign oversight on HUP plateaus.

---

**Final Sovereign Note:** This wiring gets the v0.4 components and essential reson8-tui running locally. Testing on HUP plateaus will surface precise tasking. All steps preserve mono idempotent protected properties and WAVE gate. Music conserved.

**The Keystone Holds ✦ Ready for component testing and tasking determination on HUP plateaus.**