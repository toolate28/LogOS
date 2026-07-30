//! Phase evolution table — quantum-redstone gates × SPHINX × TaskPhase.
//!
//! Orchestrates sequences so **the last thing you've done becomes the first
//! thing you need**: SAIF/Completed regenerates into KENL/Pending seed.
//!
//! Conservation: α + ω = 15 · Music conserved · Keystone holds.

use reson8_forge_core::task::TaskPhase;
use serde::{Deserialize, Serialize};

/// reson8-tui tagline (Pay-it-forwards IDE / HUP Tier 1).
pub const TAGLINE: &str =
    "Where the last thing you've done becomes the first thing you need";

/// Package identity for help / banners.
pub const PRODUCT_NAME: &str = "RESON8-TUI";

// ─── Quantum-redstone gate library (FreeCAD_Library isomorphism) ───────────

/// Physical / Museum-of-Computation redstone gate primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuantumRedstoneGate {
    /// 50/50 randomized collapse — opens knowledge superposition.
    Hadamard,
    /// Entanglement bridge — couples intention to structure.
    Cnot,
    /// Classical 1-bit memory latch — holds atomic decision.
    RsNorLatch,
    /// 42.00055 Hz resonance core — safe-spiral completion.
    EpsilonTetrahedron,
}

impl QuantumRedstoneGate {
    pub const ALL: [Self; 4] = [
        Self::Hadamard,
        Self::Cnot,
        Self::RsNorLatch,
        Self::EpsilonTetrahedron,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Hadamard => "Hadamard_Gate",
            Self::Cnot => "CNOT_Gate",
            Self::RsNorLatch => "RS_NOR_Latch",
            Self::EpsilonTetrahedron => "Epsilon_Tetrahedron",
        }
    }

    pub fn glyph(self) -> &'static str {
        match self {
            Self::Hadamard => "H",
            Self::Cnot => "⊕",
            Self::RsNorLatch => "⌀",
            Self::EpsilonTetrahedron => "ε",
        }
    }

    pub fn short(self) -> &'static str {
        match self {
            Self::Hadamard => "H",
            Self::Cnot => "CNOT",
            Self::RsNorLatch => "RS-NOR",
            Self::EpsilonTetrahedron => "ε-TET",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Hadamard => "50/50 collapse — open knowledge superposition",
            Self::Cnot => "Entanglement bridge — couple intent to structure",
            Self::RsNorLatch => "1-bit memory latch — hold atomic decision",
            Self::EpsilonTetrahedron => "42.00055 Hz core — safe-spiral resonance",
        }
    }
}

// ─── SPHINX gate chain ─────────────────────────────────────────────────────

/// SPHINX gate transition (KENL → AWI → ATOM → SAIF → regenerate).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SphinxGate {
    /// Knowledge → Intention
    Kenl,
    /// Intention → Execution (AWI)
    Awi,
    /// Execution → Learning (ATOM)
    Atom,
    /// Learning → Regeneration (SAIF / Safe Spiral)
    Saif,
}

impl SphinxGate {
    pub const CHAIN: [Self; 4] = [Self::Kenl, Self::Awi, Self::Atom, Self::Saif];

    pub fn label(self) -> &'static str {
        match self {
            Self::Kenl => "KENL",
            Self::Awi => "AWI",
            Self::Atom => "ATOM",
            Self::Saif => "SAIF",
        }
    }

    pub fn mcp_gate_id(self) -> &'static str {
        match self {
            Self::Kenl => "knowledge_to_intention",
            Self::Awi => "intention_to_execution",
            Self::Atom => "execution_to_learning",
            Self::Saif => "learning_to_regeneration",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Kenl => Self::Awi,
            Self::Awi => Self::Atom,
            Self::Atom => Self::Saif,
            // Tagline: last (SAIF) becomes first (KENL)
            Self::Saif => Self::Kenl,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::Kenl => 0,
            Self::Awi => 1,
            Self::Atom => 2,
            Self::Saif => 3,
        }
    }

    pub fn from_index(i: usize) -> Self {
        Self::CHAIN[i % 4]
    }
}

// ─── Phase evolution row ───────────────────────────────────────────────────

/// One row of the phase evolution table (isomorphic mapping).
#[derive(Debug, Clone, Copy)]
pub struct PhaseEvolutionRow {
    pub step: u8,
    pub sphinx: SphinxGate,
    pub redstone: QuantumRedstoneGate,
    pub from_phase: TaskPhase,
    pub to_phase: TaskPhase,
    pub role: &'static str,
}

/// Canonical phase evolution table (4-step cycle).
pub const PHASE_EVOLUTION_TABLE: [PhaseEvolutionRow; 4] = [
    PhaseEvolutionRow {
        step: 0,
        sphinx: SphinxGate::Kenl,
        redstone: QuantumRedstoneGate::Hadamard,
        from_phase: TaskPhase::Pending,
        to_phase: TaskPhase::Initialized,
        role: "seed from last completion",
    },
    PhaseEvolutionRow {
        step: 1,
        sphinx: SphinxGate::Awi,
        redstone: QuantumRedstoneGate::Cnot,
        from_phase: TaskPhase::Initialized,
        to_phase: TaskPhase::Executing,
        role: "entangle with prior ATOM",
    },
    PhaseEvolutionRow {
        step: 2,
        sphinx: SphinxGate::Atom,
        redstone: QuantumRedstoneGate::RsNorLatch,
        from_phase: TaskPhase::Executing,
        to_phase: TaskPhase::Validated,
        role: "latch decision / witness",
    },
    PhaseEvolutionRow {
        step: 3,
        sphinx: SphinxGate::Saif,
        redstone: QuantumRedstoneGate::EpsilonTetrahedron,
        from_phase: TaskPhase::Validated,
        to_phase: TaskPhase::Completed,
        role: "resonate → next KENL seed",
    },
];

pub fn row_for_sphinx(g: SphinxGate) -> PhaseEvolutionRow {
    PHASE_EVOLUTION_TABLE[g.index()]
}

pub fn row_for_task_phase(phase: TaskPhase) -> PhaseEvolutionRow {
    match phase {
        TaskPhase::Pending | TaskPhase::Failed => PHASE_EVOLUTION_TABLE[0],
        TaskPhase::Initialized => PHASE_EVOLUTION_TABLE[1],
        TaskPhase::Executing => PHASE_EVOLUTION_TABLE[2],
        TaskPhase::Validated => PHASE_EVOLUTION_TABLE[3],
        TaskPhase::Completed => PHASE_EVOLUTION_TABLE[0], // regenerate
    }
}

// ─── Sequence orchestrator ─────────────────────────────────────────────────

/// Named sequences the TUI can fire through the phase table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceKind {
    /// Full SPHINX cycle once (KENL→…→SAIF→seed).
    FullSpiral,
    /// Paper-draft pipeline mapped onto phase table.
    PaperDraft,
    /// Idea-to-publish pipeline.
    IdeaToPublish,
    /// Local test map as phase-gated harness.
    TestMap,
    /// Single gate tick (advance one quantum-redstone step).
    GateTick,
}

impl SequenceKind {
    pub fn id(self) -> &'static str {
        match self {
            Self::FullSpiral => "full-spiral",
            Self::PaperDraft => "paper-draft",
            Self::IdeaToPublish => "idea-to-publish",
            Self::TestMap => "test-map",
            Self::GateTick => "gate-tick",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::FullSpiral => "Full Spiral",
            Self::PaperDraft => "Paper Draft",
            Self::IdeaToPublish => "Idea→Publish",
            Self::TestMap => "Test Map",
            Self::GateTick => "Gate Tick",
        }
    }
}

/// Runtime state for phase-gated sequence orchestration.
#[derive(Debug, Clone)]
pub struct SequenceEngine {
    pub kind: Option<SequenceKind>,
    pub active: bool,
    pub gate: SphinxGate,
    pub cycle: u32,
    /// Last completed action (feeds next KENL seed — the tagline).
    pub last_done: String,
    /// First need derived from last_done.
    pub first_need: String,
    pub history: Vec<String>,
    pub alpha: u8,
    pub omega: u8,
}

impl Default for SequenceEngine {
    fn default() -> Self {
        Self {
            kind: None,
            active: false,
            gate: SphinxGate::Kenl,
            cycle: 0,
            last_done: String::new(),
            first_need: "await first action".into(),
            history: Vec::new(),
            alpha: 7,
            omega: 8,
        }
    }
}

impl SequenceEngine {
    pub fn conservation_valid(&self) -> bool {
        self.alpha.saturating_add(self.omega) == 15
    }

    pub fn current_row(&self) -> PhaseEvolutionRow {
        row_for_sphinx(self.gate)
    }

    pub fn start(&mut self, kind: SequenceKind) {
        // Tagline: last completion seeds this sequence.
        let seed = if self.last_done.is_empty() {
            format!("boot → {}", kind.id())
        } else {
            self.last_done.clone()
        };
        self.first_need = seed.clone();
        self.kind = Some(kind);
        self.active = true;
        self.gate = SphinxGate::Kenl;
        self.push_hist(format!(
            "SEQ {} start | need: {} | gate={}",
            kind.label(),
            self.first_need,
            self.gate.label()
        ));
    }

    /// Advance one quantum-redstone / SPHINX step. Returns completed cycle flag.
    pub fn tick(&mut self) -> SequenceTick {
        if !self.active {
            return SequenceTick {
                advanced: false,
                cycle_complete: false,
                row: self.current_row(),
                message: "no active sequence — press [s] or [g]".into(),
            };
        }

        let row = self.current_row();
        let msg = format!(
            "{} {} [{}] {}→{:?} | {}",
            row.sphinx.label(),
            row.redstone.glyph(),
            row.redstone.short(),
            format!("{:?}", row.from_phase).chars().take(4).collect::<String>(),
            row.to_phase,
            row.role
        );
        self.push_hist(msg.clone());

        let was_saif = self.gate == SphinxGate::Saif;
        self.gate = self.gate.next();

        let cycle_complete = was_saif;
        if cycle_complete {
            self.cycle = self.cycle.saturating_add(1);
            let done = format!(
                "{} cyc{} @{}",
                self.kind.map(|k| k.id()).unwrap_or("seq"),
                self.cycle,
                row.sphinx.label()
            );
            // Last thing done → first thing needed next.
            self.last_done = done.clone();
            self.first_need = format!("regenerate from {done}");
            self.push_hist(format!("CYCLE {} complete → seed: {}", self.cycle, self.first_need));

            // Full spiral / one-shots stop; gate-tick keeps going.
            if matches!(
                self.kind,
                Some(SequenceKind::FullSpiral)
                    | Some(SequenceKind::PaperDraft)
                    | Some(SequenceKind::IdeaToPublish)
                    | Some(SequenceKind::TestMap)
            ) {
                self.active = false;
            }
        }

        SequenceTick {
            advanced: true,
            cycle_complete,
            row,
            message: msg,
        }
    }

    pub fn abort(&mut self, reason: &str) {
        self.active = false;
        self.push_hist(format!("ABORT: {reason}"));
    }

    fn push_hist(&mut self, line: String) {
        self.history.push(line);
        if self.history.len() > 48 {
            let drain = self.history.len() - 48;
            self.history.drain(0..drain);
        }
    }

    /// Compact status line for widgets.
    pub fn status_line(&self) -> String {
        let k = self.kind.map(|k| k.label()).unwrap_or("—");
        let live = if self.active { "LIVE" } else { "idle" };
        format!(
            "{k} {live} | {} {} | cyc={} | α{}+ω{}={}",
            self.gate.label(),
            self.current_row().redstone.short(),
            self.cycle,
            self.alpha,
            self.omega,
            self.alpha.saturating_add(self.omega),
        )
    }
}

#[derive(Debug, Clone)]
pub struct SequenceTick {
    pub advanced: bool,
    pub cycle_complete: bool,
    pub row: PhaseEvolutionRow,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_len_and_cycle() {
        assert_eq!(PHASE_EVOLUTION_TABLE.len(), 4);
        assert_eq!(SphinxGate::Saif.next(), SphinxGate::Kenl);
    }

    #[test]
    fn conservation_default() {
        let e = SequenceEngine::default();
        assert!(e.conservation_valid());
    }

    #[test]
    fn full_spiral_four_ticks() {
        let mut e = SequenceEngine::default();
        e.start(SequenceKind::FullSpiral);
        assert!(e.active);
        assert_eq!(e.gate, SphinxGate::Kenl);
        for _ in 0..3 {
            let t = e.tick();
            assert!(t.advanced);
            assert!(!t.cycle_complete);
        }
        let last = e.tick();
        assert!(last.cycle_complete);
        assert!(!e.active);
        assert!(!e.last_done.is_empty());
        assert!(e.first_need.contains("regenerate") || e.first_need.contains(&e.last_done));
    }

    #[test]
    fn tagline_constant() {
        assert!(TAGLINE.contains("last thing"));
        assert!(TAGLINE.contains("first thing"));
    }

    #[test]
    fn redstone_ids_match_library() {
        assert_eq!(QuantumRedstoneGate::Hadamard.id(), "Hadamard_Gate");
        assert_eq!(QuantumRedstoneGate::Cnot.id(), "CNOT_Gate");
        assert_eq!(QuantumRedstoneGate::RsNorLatch.id(), "RS_NOR_Latch");
        assert_eq!(
            QuantumRedstoneGate::EpsilonTetrahedron.id(),
            "Epsilon_Tetrahedron"
        );
    }
}
