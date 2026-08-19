//! Workflow layout presets for reson8-tui.
//!
//! Operators switch between layouts for regular workflows without restarting:
//! ops dashboard, formal/LSP, agent watch, pipeline monitor, quantum phases,
//! classical codes lab, minimal.
//!
//! Keys: `l` next · `L` prev · `1`–`8` jump · env `RESON8_LAYOUT` / `FORGE_LAYOUT`

use crate::app::FocusPanel;

/// Named workspace layouts for day-to-day work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutKind {
    /// Full ops: providers · tasks · braid · logs · formal · tests
    Ops,
    /// Lean/Agda eye of the needle
    Formal,
    /// Strand + agent watch (providers · logs · braid · formal)
    Agent,
    /// Pipeline / WAVE monitor (braid · logs · tests)
    Monitor,
    /// Quantum-redstone × SPHINX phases
    Quantum,
    /// Smallest useful surface (braid · logs)
    Minimal,
    /// Classical codes lab: Hexacode · Golay · Reed–Muller
    Codes,
    /// HITL / SAIF outstanding human actions (approval subroutine).
    Hitl,
}

impl LayoutKind {
    pub const ALL: [Self; 8] = [
        Self::Ops,
        Self::Formal,
        Self::Agent,
        Self::Monitor,
        Self::Quantum,
        Self::Minimal,
        Self::Codes,
        Self::Hitl,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Ops => "ops",
            Self::Formal => "formal",
            Self::Agent => "agent",
            Self::Monitor => "monitor",
            Self::Quantum => "quantum",
            Self::Minimal => "minimal",
            Self::Codes => "codes",
            Self::Hitl => "hitl",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Ops => "Ops",
            Self::Formal => "Formal",
            Self::Agent => "Agent",
            Self::Monitor => "Monitor",
            Self::Quantum => "Quantum",
            Self::Minimal => "Minimal",
            Self::Codes => "Codes",
            Self::Hitl => "HITL",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Ops => "Full dashboard — strands, tasks, braid, net, logs, formal, tests",
            Self::Formal => "Lean/Agda LSP focus — formal top, logs+tests bottom",
            Self::Agent => "Agent watch — providers, logs, braid, formal",
            Self::Monitor => "Pipeline / WAVE — braid, net, git, logs, tests",
            Self::Quantum => "QR × SPHINX phases — phases top, braid·formal·logs",
            Self::Minimal => "Braid + logs only",
            Self::Codes => "Hexacode · Golay G24 · Reed–Muller lab + formal twin",
            Self::Hitl => "SAIF human ⚑ queue — request · escalate · approve (receipt only)",
        }
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|&k| k == self).unwrap_or(0)
    }

    pub fn from_index(i: usize) -> Self {
        Self::ALL[i % Self::ALL.len()]
    }

    pub fn next(self) -> Self {
        Self::from_index(self.index() + 1)
    }

    pub fn prev(self) -> Self {
        let n = Self::ALL.len();
        Self::from_index((self.index() + n - 1) % n)
    }

    /// Parse env / CLI token (`ops`, `formal`, `1`, …).
    pub fn parse(s: &str) -> Option<Self> {
        let t = s.trim().to_ascii_lowercase();
        match t.as_str() {
            "ops" | "default" | "full" | "1" => Some(Self::Ops),
            "formal" | "lsp" | "lean" | "2" => Some(Self::Formal),
            "agent" | "strands" | "3" => Some(Self::Agent),
            "monitor" | "pipe" | "pipeline" | "4" => Some(Self::Monitor),
            "quantum" | "phases" | "qr" | "5" => Some(Self::Quantum),
            "minimal" | "min" | "6" => Some(Self::Minimal),
            "codes" | "golay" | "hex" | "rm" | "7" => Some(Self::Codes),
            "hitl" | "saif" | "actions" | "human" | "8" => Some(Self::Hitl),
            _ => None,
        }
    }

    /// Operator's explicit choice via `RESON8_LAYOUT` / `FORGE_LAYOUT`.
    ///
    /// When absent the caller falls back to the detected host surface
    /// (see [`crate::surface::HostSurface::default_layout`]).
    pub fn from_env_explicit() -> Option<Self> {
        for key in ["RESON8_LAYOUT", "FORGE_LAYOUT"] {
            if let Ok(v) = std::env::var(key) {
                if let Some(k) = Self::parse(&v) {
                    return Some(k);
                }
            }
        }
        None
    }

    /// Primary focus when entering this layout.
    pub fn primary_focus(self) -> FocusPanel {
        match self {
            Self::Ops => FocusPanel::Providers,
            Self::Formal => FocusPanel::Formal,
            Self::Agent => FocusPanel::Providers,
            Self::Monitor => FocusPanel::Braid,
            Self::Quantum => FocusPanel::Phases,
            Self::Minimal => FocusPanel::Braid,
            Self::Codes => FocusPanel::Codes,
            Self::Hitl => FocusPanel::Actions,
        }
    }

    /// Row-major panel grid. Each inner slice is one horizontal row;
    /// weights are relative percentages (need not sum to 100 — normalized at draw).
    pub fn rows(self) -> &'static [&'static [(FocusPanel, u16)]] {
        match self {
            Self::Ops => &[&[
                (FocusPanel::Providers, 10),
                (FocusPanel::Tasks, 10),
                (FocusPanel::Braid, 14),
                (FocusPanel::Net, 12),
                (FocusPanel::Actions, 14),
                (FocusPanel::Logs, 12),
                (FocusPanel::Formal, 14),
                (FocusPanel::Tests, 14),
            ]],
            Self::Formal => &[
                &[(FocusPanel::Formal, 100)],
                &[
                    (FocusPanel::Logs, 55),
                    (FocusPanel::Tests, 25),
                    (FocusPanel::Braid, 20),
                ],
            ],
            Self::Agent => &[&[
                (FocusPanel::Providers, 14),
                (FocusPanel::Actions, 18),
                (FocusPanel::Logs, 24),
                (FocusPanel::Braid, 22),
                (FocusPanel::Formal, 22),
            ]],
            Self::Monitor => &[&[
                (FocusPanel::Braid, 24),
                (FocusPanel::Net, 18),
                (FocusPanel::Git, 20),
                (FocusPanel::Logs, 20),
                (FocusPanel::Tests, 18),
            ]],
            Self::Quantum => &[
                &[(FocusPanel::Phases, 100)],
                &[
                    (FocusPanel::Braid, 34),
                    (FocusPanel::Formal, 33),
                    (FocusPanel::Logs, 33),
                ],
            ],
            Self::Minimal => &[&[
                (FocusPanel::Braid, 50),
                (FocusPanel::Logs, 50),
            ]],
            Self::Codes => &[
                &[(FocusPanel::Codes, 100)],
                &[
                    (FocusPanel::Formal, 34),
                    (FocusPanel::Logs, 33),
                    (FocusPanel::Braid, 33),
                ],
            ],
            Self::Hitl => &[
                &[(FocusPanel::Actions, 100)],
                &[
                    (FocusPanel::Git, 34),
                    (FocusPanel::Logs, 33),
                    (FocusPanel::Formal, 33),
                ],
            ],
        }
    }

    /// Visible panels in Tab order (row-major, left→right).
    pub fn visible_panels(self) -> Vec<FocusPanel> {
        let mut out = Vec::with_capacity(8);
        for row in self.rows() {
            for &(p, _) in *row {
                if !out.contains(&p) {
                    out.push(p);
                }
            }
        }
        out
    }

    pub fn contains(self, panel: FocusPanel) -> bool {
        self.visible_panels().contains(&panel)
    }

    /// Next focus within this layout only.
    pub fn next_focus(self, current: FocusPanel) -> FocusPanel {
        let panels = self.visible_panels();
        if panels.is_empty() {
            return FocusPanel::Braid;
        }
        match panels.iter().position(|&p| p == current) {
            Some(i) => panels[(i + 1) % panels.len()],
            None => panels[0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_layouts_have_panels() {
        for k in LayoutKind::ALL {
            assert!(!k.visible_panels().is_empty(), "{}", k.id());
            assert!(k.contains(k.primary_focus()), "{} primary not visible", k.id());
        }
    }

    #[test]
    fn cycle_wraps() {
        let mut k = LayoutKind::Ops;
        for _ in 0..LayoutKind::ALL.len() {
            k = k.next();
        }
        assert_eq!(k, LayoutKind::Ops);
        assert_eq!(LayoutKind::Ops.prev(), LayoutKind::Hitl);
    }

    #[test]
    fn parse_aliases() {
        assert_eq!(LayoutKind::parse("formal"), Some(LayoutKind::Formal));
        assert_eq!(LayoutKind::parse("5"), Some(LayoutKind::Quantum));
        assert_eq!(LayoutKind::parse("codes"), Some(LayoutKind::Codes));
        assert_eq!(LayoutKind::parse("7"), Some(LayoutKind::Codes));
        assert_eq!(LayoutKind::parse("hitl"), Some(LayoutKind::Hitl));
        assert_eq!(LayoutKind::parse("8"), Some(LayoutKind::Hitl));
        assert_eq!(LayoutKind::parse("nope"), None);
    }

    #[test]
    fn ops_includes_hitl_board() {
        assert_eq!(LayoutKind::Ops.visible_panels().len(), 8);
        assert!(LayoutKind::Ops.contains(FocusPanel::Actions));
        assert!(LayoutKind::Agent.contains(FocusPanel::Actions));
        assert_eq!(LayoutKind::Hitl.primary_focus(), FocusPanel::Actions);
        assert!(LayoutKind::Monitor.contains(FocusPanel::Git));
        assert!(LayoutKind::Hitl.contains(FocusPanel::Git));
    }

    #[test]
    fn quantum_includes_phases() {
        assert!(LayoutKind::Quantum.contains(FocusPanel::Phases));
        assert!(!LayoutKind::Ops.contains(FocusPanel::Phases));
    }

    #[test]
    fn codes_layout_primary() {
        assert!(LayoutKind::Codes.contains(FocusPanel::Codes));
        assert_eq!(LayoutKind::Codes.primary_focus(), FocusPanel::Codes);
    }
}
