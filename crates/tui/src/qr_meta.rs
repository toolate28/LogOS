//! Quantum-redstone metaprogramming for reson8-tui.
//!
//! Maps Ratatui/Tokio reactiveness onto the H → CNOT → RS-NOR → ε circuit
//! (see `phase_evolution` and `docs/architecture/TUI-QR-METAPROGRAMMING.md`).
//!
//! ATOM: ATOM-GROK-TUI-QR-META-20260806
//! kind: BUILD-DIRECTIVE · α+ω: 15
//!
//! Fail-closed: macros accelerate wiring; they never open residual-zero.

use crate::phase_evolution::{QuantumRedstoneGate, SphinxGate};

// ─── Typed dust (wires) ────────────────────────────────────────────────────

/// Circuit-level event after H-superposition (before CNOT reduce).
///
/// Prefer emitting these from async tasks; the main loop latches into `App`.
#[derive(Debug, Clone)]
pub enum CircuitEvent {
    /// Bridge / superskill / external bus dust.
    Bus { kind: &'static str, detail: String },
    /// Key intent already decoded (control qubit for CNOT).
    Intent(CircuitIntent),
    /// Timer / ε pulse (frame budget tick).
    Pulse,
    /// Sequence gate advanced.
    GateTick,
}

/// Pure intents — CNOT control lines. No I/O here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitIntent {
    Quit,
    CycleFocus,
    ToggleHelp,
    GateStep,
    FullSpiral,
    FocusFormal,
    /// Re-probe apps/cutiles/crates/kernels/ops + sibling interweave.
    RefreshLattice,
    /// Focus Net proxy stack panel (Tor / i2pd / gaming clearnet).
    FocusNet,
    /// Re-probe localhost proxy ports.
    NetRefresh,
    /// Open net control menu (privacy / gaming / stop / install).
    NetMenu,
    /// Focus HITL / SAIF human-action board.
    FocusActions,
    /// Re-load ops/human-actions.json (or MD fallback).
    RefreshActions,
    /// Focus observe-only git pane (guitar graph).
    FocusGit,
    /// ε git fetch --prune.
    GitFetch,
    /// Focus classical codes lab (Hex · G24 · RM).
    FocusCodes,
    /// Run multi-family codes decode battery.
    CodesDemoAll,
    /// Run demo for active code family.
    CodesDemo,
    /// Cycle Hex → G24 → RM.
    CodesCycleFamily,
    /// Bump inject-error weight for active family.
    CodesBumpT,
    /// RM order r ± 1
    CodesRmRInc,
    CodesRmRDec,
    /// RM m ± 1
    CodesRmMInc,
    CodesRmMDec,
    CycleLayout,
    CycleLayoutPrev,
    LayoutOps,
    LayoutFormal,
    LayoutAgent,
    LayoutMonitor,
    LayoutQuantum,
    LayoutMinimal,
    LayoutCodes,
    LayoutHitl,
    TestMap,
    UrgentPopup,
    PaperDraft,
    IdeaToPublish,
    PopupConfirm,
    PopupDismiss,
    PopupUp,
    PopupDown,
}

// ─── Effects (ε-phase only) ────────────────────────────────────────────────

/// Work scheduled **after** RS-NOR latch / after draw, never inside widgets.
///
/// Isochronic-fork rule (ATOM-CLAUDE-REASON-QDI-DRAIN-AUDIT-20260807):
/// every `.await` reachable from H must be acknowledged or bounded.
/// `EngineBridge` is the **named acknowledgement** for superskill fan-out —
/// it runs only in ε, budgeted by [`DRAIN_BUDGET`].
#[derive(Debug, Clone)]
pub enum Effect {
    None,
    Notify {
        level: &'static str,
        title: String,
        body: String,
    },
    /// Placeholder for engine / bridge side-effects (call site fills in).
    Side { name: &'static str },
    /// Run ops/net/LogOS.NetProxy.ps1 action in ε-phase (after latch).
    Net { action: crate::net_proxy::NetAction },
    /// Superskill engine bridge fan-out — ε only (ack: engine wire + budget).
    EngineBridge {
        event: reson8_forge_core::bridge::BridgeEvent,
    },
    /// HITL approval subroutine — ε writes the spine receipt (no deploy).
    Hitl {
        decision: crate::human_actions::HitlDecision,
        action_id: String,
    },
    /// Observe-only git (refresh is in-reducer; fetch is ε).
    Git { action: crate::git_lab::GitAction },
}

// ─── Gate law helpers ──────────────────────────────────────────────────────

/// Which QR gate a main-loop stage corresponds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopStage {
    /// Drain all ready sources (superposition).
    HadamardDrain,
    /// Apply intents to state (entangle).
    CnotReduce,
    /// Hold overlay / focus bits.
    RsNorLatch,
    /// Draw + regenerate seed.
    EpsilonMeasure,
}

impl LoopStage {
    pub fn redstone(self) -> QuantumRedstoneGate {
        match self {
            Self::HadamardDrain => QuantumRedstoneGate::Hadamard,
            Self::CnotReduce => QuantumRedstoneGate::Cnot,
            Self::RsNorLatch => QuantumRedstoneGate::RsNorLatch,
            Self::EpsilonMeasure => QuantumRedstoneGate::EpsilonTetrahedron,
        }
    }

    pub fn sphinx(self) -> SphinxGate {
        match self {
            Self::HadamardDrain => SphinxGate::Kenl,
            Self::CnotReduce => SphinxGate::Awi,
            Self::RsNorLatch => SphinxGate::Atom,
            Self::EpsilonMeasure => SphinxGate::Saif,
        }
    }

    pub fn law(self) -> &'static str {
        match self {
            Self::HadamardDrain => "P-QDI: superpose sources; never block the draw path",
            Self::CnotReduce => "P-CNOT: pure reduce; effects after latch only",
            Self::RsNorLatch => "P-LATCH: overlays mask lower dust while set",
            Self::EpsilonMeasure => "P-MEASURE: widgets read-only; last_done → first_need",
        }
    }
}

/// Priority ring for RS-NOR key masking (highest first).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatchLayer {
    BlockingPopup,
    HelpOverlay,
    Normal,
}

impl LatchLayer {
    pub const RING: [Self; 3] = [Self::BlockingPopup, Self::HelpOverlay, Self::Normal];
}

// ─── Macros ────────────────────────────────────────────────────────────────

/// Compact gate-row tuple for declarative tables (same fields as phase evolution).
#[inline]
pub const fn gate_row(
    step: u8,
    sphinx: SphinxGate,
    redstone: QuantumRedstoneGate,
    role: &'static str,
) -> (u8, SphinxGate, QuantumRedstoneGate, &'static str) {
    (step, sphinx, redstone, role)
}

/// Drain multiple `try_recv` sources into a closure (Hadamard / KENL stage).
///
/// **Bounded** by [`DRAIN_BUDGET`] per source per invocation — channel capacity
/// is not an iteration budget. Excess dust stays in the channel for the next
/// frame (honest QDI acknowledgement window).
///
/// Returns total events drained across all arms (for telemetry).
///
/// ```ignore
/// let n = drain_dust! {
///     ss_ev_rx => |ev| app.handle_superskill_event(ev),
///     lsp_rx => |ev| app.handle_lsp_event(ev),
/// };
/// ```
macro_rules! drain_dust {
    ( $( $rx:expr => |$ev:ident| $body:expr ),+ $(,)? ) => {
        {
            let mut __drained: usize = 0;
            $(
                for _ in 0..$crate::qr_meta::DRAIN_BUDGET {
                    match $rx.try_recv() {
                        ::core::result::Result::Ok($ev) => {
                            $body;
                            __drained += 1;
                        }
                        ::core::result::Result::Err(_) => break,
                    }
                }
            )+
            __drained
        }
    };
}

/// RS-NOR priority dispatch: first matching layer wins.
///
/// ```ignore
/// latch_priority! {
///     app.popup_blocking() => { /* popup keys */ }
///     app.help_open => { /* help keys */ }
///     else => { /* normal keys */ }
/// }
/// ```
macro_rules! latch_priority {
    (
        $($cond:expr => $block:block)+
        else => $else_block:block
    ) => {
        {
            if false {
                unreachable!()
            }
            $(else if $cond $block)+
            else $else_block
        }
    };
}

// Re-export macro names for call sites in this binary via `use` of functions;
// macros stay crate-private to the binary modules (invoke as `qr_meta::…` only
// if re-exported — prefer copying the drain pattern or calling helpers).
#[allow(unused_imports)]
pub(crate) use drain_dust;
#[allow(unused_imports)]
pub(crate) use latch_priority;

/// QDI poll budget in milliseconds (ε motif: operator-feel ~100 Hz default).
pub const DEFAULT_POLL_MS: u64 = 10;

/// Per-source max `try_recv` iterations per frame (Hadamard drain budget).
///
/// Prevents H-stage spin under sustained producers so `terminal.draw` always
/// runs. Channel depth (e.g. 256) bounds backlog; this bounds *work per frame*.
/// ATOM-CLAUDE-REASON-QDI-DRAIN-AUDIT-20260807 · Track A hygiene.
pub const DRAIN_BUDGET: usize = 32;

/// Max superskill `engine.handle` awaits per ε phase (named ack window).
pub const ENGINE_HANDLE_BUDGET: usize = 16;

/// Symbolic ε resonance (Category C label — not a hardware clock).
/// Display-only attractor motif; never a hardware frame clock or residual gate.
pub const EPSILON_HZ_LABEL: f64 = 42.00055;

/// Residual tolerance under Category-C gauge (lab convention, not production cert).
/// \(R \le \varepsilon\) with \(R = \max(0, 15 - \alpha - \omega)\).
pub const RESIDUAL_EPS: f64 = 0.00055;

/// Category-C conservation sum (Nat skeleton label).
pub const CONSERVATION_SUM_F: f64 = 15.0;

/// Residual \(R = \max(0, 15 - \alpha - \omega)\) — Category C telemetry.
#[inline]
pub fn residual_r(alpha: f64, omega: f64) -> f64 {
    (CONSERVATION_SUM_F - alpha - omega).max(0.0)
}

/// Operational residual-zero *claim* under ε — **not** deploy-green / not cert.pass.
#[inline]
pub fn residual_zero_claim(alpha: f64, omega: f64) -> bool {
    residual_r(alpha, omega) <= RESIDUAL_EPS
}

/// Documentation-only: recommended main-loop stage order.
pub const LOOP_ORDER: [LoopStage; 4] = [
    LoopStage::HadamardDrain,
    LoopStage::CnotReduce,
    LoopStage::RsNorLatch,
    LoopStage::EpsilonMeasure,
];

/// Decode common navigation keys into intents (shared by future pure reducers).
pub fn intent_from_key(code: crossterm::event::KeyCode) -> Option<CircuitIntent> {
    use crossterm::event::KeyCode;
    match code {
        KeyCode::Char('q') => Some(CircuitIntent::Quit),
        KeyCode::Tab => Some(CircuitIntent::CycleFocus),
        KeyCode::Char('?') | KeyCode::Char('h') => Some(CircuitIntent::ToggleHelp),
        KeyCode::Char('g') => Some(CircuitIntent::GateStep),
        KeyCode::Char('s') => Some(CircuitIntent::FullSpiral),
        KeyCode::Char('f') => Some(CircuitIntent::FocusFormal),
        KeyCode::Char('A') => Some(CircuitIntent::RefreshLattice),
        KeyCode::Char('o') => Some(CircuitIntent::FocusActions),
        KeyCode::Char('O') => Some(CircuitIntent::RefreshActions),
        KeyCode::Char('G') => Some(CircuitIntent::FocusGit),
        KeyCode::Char('V') => Some(CircuitIntent::GitFetch),
        KeyCode::Char('N') => Some(CircuitIntent::FocusNet),
        KeyCode::Char('R') => Some(CircuitIntent::NetRefresh),
        KeyCode::Char('M') => Some(CircuitIntent::NetMenu),
        KeyCode::Char('c') => Some(CircuitIntent::FocusCodes),
        KeyCode::Char('D') => Some(CircuitIntent::CodesDemoAll),
        KeyCode::Char('d') => Some(CircuitIntent::CodesDemo),
        KeyCode::Char('y') => Some(CircuitIntent::CodesCycleFamily),
        KeyCode::Char('e') => Some(CircuitIntent::CodesBumpT),
        KeyCode::Char('[') => Some(CircuitIntent::CodesRmRDec),
        KeyCode::Char(']') => Some(CircuitIntent::CodesRmRInc),
        KeyCode::Char('{') => Some(CircuitIntent::CodesRmMDec),
        KeyCode::Char('}') => Some(CircuitIntent::CodesRmMInc),
        KeyCode::Char('l') => Some(CircuitIntent::CycleLayout),
        KeyCode::Char('L') => Some(CircuitIntent::CycleLayoutPrev),
        KeyCode::Char('1') => Some(CircuitIntent::LayoutOps),
        KeyCode::Char('2') => Some(CircuitIntent::LayoutFormal),
        KeyCode::Char('3') => Some(CircuitIntent::LayoutAgent),
        KeyCode::Char('4') => Some(CircuitIntent::LayoutMonitor),
        KeyCode::Char('5') => Some(CircuitIntent::LayoutQuantum),
        KeyCode::Char('6') => Some(CircuitIntent::LayoutMinimal),
        KeyCode::Char('7') => Some(CircuitIntent::LayoutCodes),
        KeyCode::Char('8') => Some(CircuitIntent::LayoutHitl),
        KeyCode::Char('t') => Some(CircuitIntent::TestMap),
        KeyCode::Char('u') => Some(CircuitIntent::UrgentPopup),
        KeyCode::Char('p') => Some(CircuitIntent::PaperDraft),
        KeyCode::Char('n') => Some(CircuitIntent::IdeaToPublish),
        KeyCode::Enter => Some(CircuitIntent::PopupConfirm),
        KeyCode::Esc => Some(CircuitIntent::PopupDismiss),
        KeyCode::Up | KeyCode::Char('k') => Some(CircuitIntent::PopupUp),
        KeyCode::Down | KeyCode::Char('j') => Some(CircuitIntent::PopupDown),
        _ => None,
    }
}

/// Active latch layer from UI flags (RS-NOR set-line priority).
pub fn active_latch(popup_blocking: bool, help_open: bool) -> LatchLayer {
    if popup_blocking {
        LatchLayer::BlockingPopup
    } else if help_open {
        LatchLayer::HelpOverlay
    } else {
        LatchLayer::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phase_evolution::QuantumRedstoneGate;

    #[test]
    fn loop_order_maps_to_qr_chain() {
        assert_eq!(LOOP_ORDER[0].redstone(), QuantumRedstoneGate::Hadamard);
        assert_eq!(LOOP_ORDER[1].redstone(), QuantumRedstoneGate::Cnot);
        assert_eq!(LOOP_ORDER[2].redstone(), QuantumRedstoneGate::RsNorLatch);
        assert_eq!(LOOP_ORDER[3].redstone(), QuantumRedstoneGate::EpsilonTetrahedron);
    }

    #[test]
    fn latch_priority_ring() {
        assert_eq!(active_latch(true, true), LatchLayer::BlockingPopup);
        assert_eq!(active_latch(false, true), LatchLayer::HelpOverlay);
        assert_eq!(active_latch(false, false), LatchLayer::Normal);
    }

    #[test]
    fn intent_keys_cover_spiral() {
        use crossterm::event::KeyCode;
        assert_eq!(intent_from_key(KeyCode::Char('g')), Some(CircuitIntent::GateStep));
        assert_eq!(intent_from_key(KeyCode::Char('s')), Some(CircuitIntent::FullSpiral));
        assert_eq!(intent_from_key(KeyCode::Char('q')), Some(CircuitIntent::Quit));
        assert_eq!(
            intent_from_key(KeyCode::Char('A')),
            Some(CircuitIntent::RefreshLattice)
        );
        assert_eq!(
            intent_from_key(KeyCode::Char('o')),
            Some(CircuitIntent::FocusActions)
        );
        assert_eq!(
            intent_from_key(KeyCode::Char('8')),
            Some(CircuitIntent::LayoutHitl)
        );
    }

    #[test]
    fn gate_row_helper() {
        let row = gate_row(0, SphinxGate::Kenl, QuantumRedstoneGate::Hadamard, "seed");
        assert_eq!(row.0, 0);
        assert_eq!(row.3, "seed");
        assert_eq!(row.1, SphinxGate::Kenl);
    }

    #[test]
    fn drain_dust_and_latch_macros() {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel::<u32>();
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        let mut sum = 0u32;
        let n = drain_dust! {
            rx => |n| sum += n,
        };
        assert_eq!(sum, 3);
        assert_eq!(n, 2);

        let layer = latch_priority! {
            false => { LatchLayer::BlockingPopup }
            true => { LatchLayer::HelpOverlay }
            else => { LatchLayer::Normal }
        };
        assert_eq!(layer, LatchLayer::HelpOverlay);
    }

    #[test]
    fn drain_dust_respects_budget() {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel::<u32>();
        let over = DRAIN_BUDGET + 8;
        for i in 0..over {
            tx.send(i as u32).unwrap();
        }
        let mut count = 0usize;
        let n = drain_dust! {
            rx => |_x| count += 1,
        };
        assert_eq!(n, DRAIN_BUDGET);
        assert_eq!(count, DRAIN_BUDGET);
        // Remainder still in channel for next frame.
        let rest: usize = std::iter::from_fn(|| rx.try_recv().ok()).count();
        assert_eq!(rest, 8);
    }

    #[test]
    fn residual_category_c_helpers() {
        assert_eq!(residual_r(7.0, 8.0), 0.0);
        assert!(residual_zero_claim(7.0, 8.0));
        assert!((residual_r(7.0, 7.0) - 1.0).abs() < 1e-9);
        assert!(!residual_zero_claim(7.0, 7.0));
        assert!(RESIDUAL_EPS < 0.001);
    }

    #[test]
    fn laws_nonempty() {
        for stage in LOOP_ORDER {
            assert!(!stage.law().is_empty());
        }
    }
}
