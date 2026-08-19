//! Lattice layer probe — apps · cutiles · crates · kernels · ops + sibling interweave.
//!
//! Filesystem presence only. Never a deploy gate. Category B observation.
//! Key `A` re-probes (CircuitIntent::RefreshLattice). Shell twin: `logos-activate`.

use std::env;
use std::path::{Path, PathBuf};

/// One load-bearing tree the operator asked to activate through TUI / shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatticeLayer {
    Apps,
    Cutiles,
    Crates,
    Kernels,
    Ops,
}

impl LatticeLayer {
    pub const ALL: [Self; 5] = [
        Self::Apps,
        Self::Cutiles,
        Self::Crates,
        Self::Kernels,
        Self::Ops,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Apps => "apps",
            Self::Cutiles => "cutiles",
            Self::Crates => "crates",
            Self::Kernels => "kernels",
            Self::Ops => "ops",
        }
    }

    fn rel(self) -> &'static str {
        match self {
            Self::Apps => "apps",
            Self::Cutiles => "cutiles/cutile",
            Self::Crates => "crates",
            Self::Kernels => "kernels",
            Self::Ops => "ops",
        }
    }

    fn marker(self) -> &'static str {
        match self {
            Self::Apps => "triweave/Cargo.toml",
            Self::Cutiles => "Cargo.toml",
            Self::Crates => "tui/Cargo.toml",
            Self::Kernels => "fundamental_r_matrix.cu",
            Self::Ops => "command-surface.json",
        }
    }
}

/// Sibling / in-tree interweave (thin — no fat copy of foreign trees).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interweave {
    CoherenceMcp,
    SpiralSafe,
    QuantumRedstone,
    HopeNpc,
}

impl Interweave {
    pub const ALL: [Self; 4] = [
        Self::CoherenceMcp,
        Self::SpiralSafe,
        Self::QuantumRedstone,
        Self::HopeNpc,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::CoherenceMcp => "coherence-mcp",
            Self::SpiralSafe => "spiral-safe",
            Self::QuantumRedstone => "quantum-redstone",
            Self::HopeNpc => "hope-npc",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerStatus {
    pub layer: LatticeLayer,
    pub path: PathBuf,
    pub present: bool,
}

#[derive(Debug, Clone)]
pub struct InterweaveStatus {
    pub kind: Interweave,
    pub path: Option<PathBuf>,
    pub present: bool,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct LatticeSnapshot {
    pub root: PathBuf,
    pub layers: Vec<LayerStatus>,
    pub interweave: Vec<InterweaveStatus>,
}

impl LatticeSnapshot {
    pub fn probe() -> Self {
        Self::probe_from(resolve_root())
    }

    pub fn probe_from(root: PathBuf) -> Self {
        let layers = LatticeLayer::ALL
            .iter()
            .copied()
            .map(|layer| {
                let path = root.join(layer.rel().replace('/', std::path::MAIN_SEPARATOR_STR));
                let marker = path.join(layer.marker().replace('/', std::path::MAIN_SEPARATOR_STR));
                LayerStatus {
                    layer,
                    present: path.is_dir() && marker.exists(),
                    path,
                }
            })
            .collect();

        let parent = root.parent().unwrap_or(root.as_path());
        let interweave = Interweave::ALL
            .iter()
            .copied()
            .map(|kind| probe_interweave(kind, &root, parent))
            .collect();

        Self {
            root,
            layers,
            interweave,
        }
    }

    pub fn ready_count(&self) -> usize {
        self.layers.iter().filter(|l| l.present).count()
    }

    pub fn all_ready(&self) -> bool {
        self.ready_count() == LatticeLayer::ALL.len()
    }

    /// Compact status-bar glyph: `lat 5/5` or `lat 3/5`.
    pub fn short(&self) -> String {
        format!("lat {}/{}", self.ready_count(), LatticeLayer::ALL.len())
    }

    pub fn missing_ids(&self) -> Vec<&'static str> {
        self.layers
            .iter()
            .filter(|l| !l.present)
            .map(|l| l.layer.id())
            .collect()
    }

    pub fn interweave_short(&self) -> String {
        self.interweave
            .iter()
            .map(|i| {
                format!(
                    "{}{}",
                    i.kind.id(),
                    if i.present { "●" } else { "○" }
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn resolve_root() -> PathBuf {
    if let Ok(p) = env::var("LOGOS_ROOT") {
        let pb = PathBuf::from(p);
        if pb.is_dir() {
            return pb;
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn first_existing(cands: &[PathBuf]) -> Option<PathBuf> {
    cands.iter().find(|p| p.exists()).cloned()
}

fn probe_interweave(kind: Interweave, root: &Path, parent: &Path) -> InterweaveStatus {
    let (cands, detail_ok, detail_miss): (Vec<PathBuf>, &str, &str) = match kind {
        Interweave::CoherenceMcp => (
            vec![
                parent.join("coherence-mcp").join("build").join("index.js"),
                root.join("coherence-mcp").join("build").join("index.js"),
                parent.join("coherence-mcp"),
                root.join("coherence-mcp"),
            ],
            "MCP server / site tree",
            "set COHERENCE_MCP_ROOT · logos-mcp",
        ),
        Interweave::SpiralSafe => (
            vec![
                root.join("crates").join("spiral-safe").join("Cargo.toml"),
                parent.join("SpiralSafe").join("README.md"),
                parent.join("Spiralsafe").join("README.md"),
            ],
            "in-tree crate and/or sibling guardian",
            "crates/spiral-safe or sibling SpiralSafe",
        ),
        Interweave::QuantumRedstone => (
            vec![
                parent
                    .join("HOPE-AI-NPC-SUITE")
                    .join("quantum-redstone"),
                parent.join("quantum-redstone"),
                root.join("docs")
                    .join("architecture")
                    .join("TUI-QR-METAPROGRAMMING.md"),
            ],
            "QR gates live in TUI phase_evolution · sibling datapack",
            "clone toolate28/quantum-redstone or HOPE nest",
        ),
        Interweave::HopeNpc => (
            vec![
                parent.join("HOPE-AI-NPC-SUITE").join("README.md"),
                parent.join("HOPE-AI-NPC-SUITE").join("ClaudeNPC"),
            ],
            "Minecraft NPC suite (mc-bridge / QR circuits)",
            "sibling HOPE-AI-NPC-SUITE",
        ),
    };

    if let Some(path) = first_existing(&cands) {
        InterweaveStatus {
            kind,
            present: true,
            path: Some(path),
            detail: detail_ok.into(),
        }
    } else {
        InterweaveStatus {
            kind,
            present: false,
            path: None,
            detail: detail_miss.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_layers_named() {
        assert_eq!(LatticeLayer::ALL.len(), 5);
        assert_eq!(LatticeLayer::Apps.id(), "apps");
        assert_eq!(LatticeLayer::Ops.id(), "ops");
    }

    #[test]
    fn probe_this_workspace() {
        let snap = LatticeSnapshot::probe();
        assert!(
            snap.ready_count() >= 3,
            "expected workspace layers under {:?}, got {}/5 missing {:?}",
            snap.root,
            snap.ready_count(),
            snap.missing_ids()
        );
        assert!(snap.short().starts_with("lat "));
    }

    #[test]
    fn missing_root_is_honest() {
        let snap = LatticeSnapshot::probe_from(PathBuf::from("/no/such/logos-root-xyz"));
        assert!(!snap.all_ready());
        assert_eq!(snap.ready_count(), 0);
        assert_eq!(snap.missing_ids().len(), 5);
    }
}
