//! Phase classification for the α/ω conservation pair.
//!
//! `lib.rs` answers a binary question — does `α + ω == 15` hold? That is the
//! conservation check. This module answers the finer question the binary check
//! throws away: *where on the conserved line does the pair sit?*
//!
//! Both ends of the line satisfy conservation and are still degenerate:
//! all-α is rigid and cannot move, all-ω has no structure to anchor to. The
//! documented balance point is the Viviani peak (α=7, ω=8).
//!
//! Category C: the partition labels are a tracking convention, not a physical
//! law and not a gate. Nothing here rejects work — [`Phase`] is descriptive.
//!
//! `no_std`, integer-domain, zero dependencies — matches the crate contract.

use crate::TopologicalState;

/// Conserved total for the pair (mirrors `ALPHA_OMEGA_LIMIT` in `lib.rs`).
pub const CONSERVATION_SUM: u32 = 15;

/// Viviani peak — the documented balance point.
pub const VIVIANI_ALPHA: u32 = 7;
pub const VIVIANI_OMEGA: u32 = 8;

/// Where a conserving (α, ω) pair sits along the conserved line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// α ≥ 13 — structure dominates; rigid, slow to adapt.
    Rigid,
    /// 9 ≤ α ≤ 12 — structure-leaning, still mobile.
    Structured,
    /// α ∈ {7, 8} — the Viviani balance band.
    Balanced,
    /// 3 ≤ α ≤ 6 — intent-leaning, structure still present.
    Fluid,
    /// α ≤ 2 — intent dominates; little structure to anchor to.
    Unanchored,
    /// α + ω ≠ 15 — the pair does not conserve, so no phase is defined.
    Broken,
}

impl Phase {
    /// Classify a raw pair. Non-conserving pairs are [`Phase::Broken`].
    pub const fn classify(alpha: u32, omega: u32) -> Self {
        // Guard the add so an overflowing pair reports Broken rather than panicking.
        if omega > CONSERVATION_SUM || alpha > CONSERVATION_SUM {
            return Self::Broken;
        }
        if alpha + omega != CONSERVATION_SUM {
            return Self::Broken;
        }
        match alpha {
            0..=2 => Self::Unanchored,
            3..=6 => Self::Fluid,
            7..=8 => Self::Balanced,
            9..=12 => Self::Structured,
            _ => Self::Rigid,
        }
    }

    /// Stable label for logs, ATOM records, and TUI panes.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Rigid => "rigid",
            Self::Structured => "structured",
            Self::Balanced => "balanced",
            Self::Fluid => "fluid",
            Self::Unanchored => "unanchored",
            Self::Broken => "broken",
        }
    }

    /// True when the pair conserves — i.e. any phase other than [`Phase::Broken`].
    pub const fn conserves(self) -> bool {
        !matches!(self, Self::Broken)
    }

    /// Bridge to the crate's coarse state enum.
    pub const fn topological_state(self) -> TopologicalState {
        if self.conserves() {
            TopologicalState::Coherent
        } else {
            TopologicalState::Decoherent
        }
    }

    /// Signed distance from the Viviani peak, in α units.
    ///
    /// Negative leans toward ω (intent), positive toward α (structure).
    /// [`Phase::Broken`] has no position on the line, so this returns `None`.
    pub const fn drift_from_peak(alpha: u32, omega: u32) -> Option<i32> {
        match Self::classify(alpha, omega) {
            Self::Broken => None,
            _ => Some(alpha as i32 - VIVIANI_ALPHA as i32),
        }
    }
}

impl crate::LevinWenLattice {
    /// Classify this lattice's current pair.
    pub fn phase(&self) -> Phase {
        use core::sync::atomic::Ordering;
        Phase::classify(
            self.alpha_value(Ordering::Acquire),
            self.omega_value(Ordering::Acquire),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viviani_peak_is_balanced() {
        assert_eq!(
            Phase::classify(VIVIANI_ALPHA, VIVIANI_OMEGA),
            Phase::Balanced
        );
        assert_eq!(Phase::drift_from_peak(VIVIANI_ALPHA, VIVIANI_OMEGA), Some(0));
    }

    #[test]
    fn every_conserving_pair_gets_a_real_phase() {
        for alpha in 0..=CONSERVATION_SUM {
            let omega = CONSERVATION_SUM - alpha;
            let p = Phase::classify(alpha, omega);
            assert!(p.conserves(), "α={alpha} ω={omega} classified {p:?}");
            assert_eq!(p.topological_state(), TopologicalState::Coherent);
        }
    }

    #[test]
    fn non_conserving_pairs_are_broken() {
        assert_eq!(Phase::classify(8, 8), Phase::Broken); // sums to 16
        assert_eq!(Phase::classify(0, 0), Phase::Broken);
        assert_eq!(Phase::drift_from_peak(8, 8), None);
    }

    #[test]
    fn overflow_pairs_report_broken_not_panic() {
        assert_eq!(Phase::classify(u32::MAX, 1), Phase::Broken);
        assert_eq!(Phase::classify(1, u32::MAX), Phase::Broken);
    }

    #[test]
    fn drift_sign_points_the_right_way() {
        // More α than the peak → positive (structure-leaning).
        assert_eq!(Phase::drift_from_peak(12, 3), Some(5));
        // Less α than the peak → negative (intent-leaning).
        assert_eq!(Phase::drift_from_peak(2, 13), Some(-5));
    }

    #[test]
    fn extremes_are_degenerate() {
        assert_eq!(Phase::classify(15, 0), Phase::Rigid);
        assert_eq!(Phase::classify(0, 15), Phase::Unanchored);
    }

    #[test]
    fn lattice_reports_its_phase() {
        let l = crate::LevinWenLattice::new(VIVIANI_ALPHA, VIVIANI_OMEGA);
        assert_eq!(l.phase(), Phase::Balanced);

        // α 7 → 12 stays conserving and leans structural.
        l.apply_transformation(5).expect("stays conserving");
        assert_eq!(l.phase(), Phase::Structured);
        assert_eq!(Phase::drift_from_peak(12, 3), Some(5));

        // apply_transformation clamps at the conserved total.
        l.apply_transformation(99).expect("clamped, still conserving");
        assert_eq!(l.phase(), Phase::Rigid);
    }
}
