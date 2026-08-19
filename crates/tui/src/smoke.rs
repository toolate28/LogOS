//! Staged smoke harness for the Tests pane (`[t]`).
//!
//! Three stages, always in order, fail-closed:
//!
//! 1. **Observe** — is the artifact on disk / in-process?
//! 2. **DryRun** — record the command or check that *would* run.
//! 3. **Execute** — run only in-process smokes. External tools (`cargo test`,
//!    `lake`, `agda`) stay at DryRun so `[t]` never freezes the event loop
//!    and never paints Pass for a command it did not run.
//!
//! Category **B** for executed in-process checks; **D** for unrun externals
//! (a DryRun is not a pass).

use crate::codes::{golay, hexacode, reed_muller, sc_ldpc};
use crate::lattice::{LatticeLayer, LatticeSnapshot};
use reson8_core::{enforce_invariant, InvariantStatus, INVARIANT_TARGET};
use reson8_wave::compute_wave;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Observe,
    DryRun,
    Execute,
}

impl Stage {
    pub fn id(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::DryRun => "dry-run",
            Self::Execute => "execute",
        }
    }

    pub fn glyph(self) -> char {
        match self {
            Self::Observe => '○',
            Self::DryRun => '▷',
            Self::Execute => '●',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Safe to run inside the TUI key handler.
    InProcess,
    /// Would spawn a process — DryRun only from `[t]`.
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pending,
    Observed,
    DryRun,
    Running,
    Pass,
    Fail,
    Skip,
}

impl Status {
    pub fn ok(self) -> bool {
        matches!(self, Self::Pass | Self::Skip | Self::DryRun | Self::Observed)
    }
}

#[derive(Debug, Clone)]
pub struct Spec {
    pub id: &'static str,
    pub name: &'static str,
    pub crate_name: &'static str,
    pub kind: Kind,
    pub dry_run: &'static str,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub spec: Spec,
    pub status: Status,
    pub stage: Stage,
    pub detail: String,
    /// A / B / C / D
    pub category: char,
}

pub const CATALOG: &[Spec] = &[
    Spec {
        id: "t01",
        name: "gauge α+ω=15",
        crate_name: "reson8-core",
        kind: Kind::InProcess,
        dry_run: "enforce_invariant(7, 8)",
    },
    Spec {
        id: "t02",
        name: "focus_cycles",
        crate_name: "reson8-tui",
        kind: Kind::InProcess,
        dry_run: "FocusPanel ring Providers→…→Tests→Providers",
    },
    Spec {
        id: "t03",
        name: "wave conservation",
        crate_name: "reson8-wave",
        kind: Kind::InProcess,
        dry_run: "compute_wave Fibonacci weights sum ≈ 1",
    },
    Spec {
        id: "t04",
        name: "cutile cell weave",
        crate_name: "cutile",
        kind: Kind::External,
        dry_run: "cargo test -p cutile --offline --lib",
    },
    Spec {
        id: "t05",
        name: "Agda HIT check",
        crate_name: "LogOS/agda",
        kind: Kind::External,
        dry_run: "agda --safe (HIT/formal tree)",
    },
    Spec {
        id: "t06",
        name: "Lean symplectic cert",
        crate_name: "LogOS/lean",
        kind: Kind::External,
        dry_run: "lake build TriWeavon",
    },
    Spec {
        id: "t07",
        name: "barcode-tui H0+H1",
        crate_name: "barcode-tui",
        kind: Kind::External,
        dry_run: "cargo test -p barcode-tui --offline --lib",
    },
    Spec {
        id: "t08",
        name: "rvm-coherence",
        crate_name: "ruvnet-rvm",
        kind: Kind::External,
        dry_run: "cargo test -p rvm-coherence --offline --lib",
    },
    Spec {
        id: "t09",
        name: "golay NN t≤3",
        crate_name: "reson8-tui/codes",
        kind: Kind::InProcess,
        dry_run: "golay::empirical_nn_unique + count_octads==759",
    },
    Spec {
        id: "t10",
        name: "hexacode MDS decode",
        crate_name: "reson8-tui/codes",
        kind: Kind::InProcess,
        dry_run: "hexacode weight_distribution A0=1 A4=45 A6=18",
    },
    Spec {
        id: "t11",
        name: "RM(1,m) FHT",
        crate_name: "reson8-tui/codes",
        kind: Kind::InProcess,
        dry_run: "reed_muller encode/inject/decode_rm1_fht",
    },
    Spec {
        id: "t12",
        name: "SC-LDPC design+BEC",
        crate_name: "reson8-tui/codes",
        kind: Kind::InProcess,
        dry_run: "sc_ldpc::analyze + windowed_bec_demo",
    },
    Spec {
        id: "t13",
        name: "lattice apps/cutiles/crates",
        crate_name: "reson8-tui/lattice",
        kind: Kind::InProcess,
        dry_run: "LatticeSnapshot apps+cutiles+crates present",
    },
    Spec {
        id: "t14",
        name: "lattice kernels/ops",
        crate_name: "reson8-tui/lattice",
        kind: Kind::InProcess,
        dry_run: "LatticeSnapshot kernels+ops present",
    },
    Spec {
        id: "t15",
        name: "QR Lean isomorphism",
        crate_name: "lean/TriWeavon",
        kind: Kind::External,
        dry_run: "lake env / QuantumRedstone.lean present",
    },
    Spec {
        id: "t16",
        name: "skill-chain self-test",
        crate_name: "ops/ci",
        kind: Kind::External,
        dry_run: "python ops/ci/skill_chain_scan.py --self-test",
    },
    Spec {
        id: "t17",
        name: "CODEX L7 layer present",
        crate_name: "ops/ci",
        kind: Kind::InProcess,
        dry_run: "ops/ci/codex_scan.py contains L7_skill_chain",
    },
];

fn repo_root(lattice: &LatticeSnapshot) -> &Path {
    &lattice.root
}

fn exists(root: &Path, rel: &str) -> bool {
    let p: PathBuf = root.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
    p.exists()
}

fn observe(spec: &Spec, lattice: &LatticeSnapshot) -> (bool, String) {
    let root = repo_root(lattice);
    match spec.id {
        "t01" | "t02" | "t03" | "t09" | "t10" | "t11" | "t12" => {
            (true, "in-process (no external tool)".into())
        }
        "t04" => {
            let ok = lattice
                .layers
                .iter()
                .any(|l| l.layer == LatticeLayer::Cutiles && l.present);
            (ok, format!("cutiles layer present={ok}"))
        }
        "t05" => {
            let ok = exists(root, "agda") || exists(root, "LogOS/agda");
            (ok, format!("agda tree present={ok}"))
        }
        "t06" | "t15" => {
            let ok = exists(root, "lean/TriWeavon") || exists(root, "lean");
            (ok, format!("lean tree present={ok}"))
        }
        "t07" => {
            let ok = exists(root, "crates/barcode-tui/src/ph.rs");
            (ok, format!("barcode-tui/src/ph.rs present={ok}"))
        }
        "t08" => {
            let ok = exists(root, "vendor/ruvnet-rvm") || exists(root, "crates/ruvnet-rvm");
            (ok, format!("rvm tree present={ok}"))
        }
        "t13" => {
            let n = lattice
                .layers
                .iter()
                .filter(|l| {
                    matches!(
                        l.layer,
                        LatticeLayer::Apps | LatticeLayer::Cutiles | LatticeLayer::Crates
                    ) && l.present
                })
                .count();
            (n > 0, format!("{n}/3 of apps/cutiles/crates present"))
        }
        "t14" => {
            let n = lattice
                .layers
                .iter()
                .filter(|l| {
                    matches!(l.layer, LatticeLayer::Kernels | LatticeLayer::Ops) && l.present
                })
                .count();
            (n > 0, format!("{n}/2 of kernels/ops present"))
        }
        "t16" => {
            let ok = exists(root, "ops/ci/skill_chain_scan.py");
            (ok, format!("skill_chain_scan.py present={ok}"))
        }
        "t17" => {
            let ok = exists(root, "ops/ci/codex_scan.py");
            (ok, format!("codex_scan.py present={ok}"))
        }
        _ => (false, "unknown spec".into()),
    }
}

/// Extra observations the App layer already owns (avoids smoke↔app cycle).
pub struct Extras {
    pub focus_ring_ok: bool,
    pub focus_ring_detail: String,
}

fn execute(spec: &Spec, lattice: &LatticeSnapshot, extras: &Extras) -> Result<String, String> {
    match spec.id {
        "t01" => {
            let r = enforce_invariant(7.0, 8.0);
            if r.status == InvariantStatus::Passed && (r.total - INVARIANT_TARGET).abs() < 1e-9 {
                Ok(format!("α+ω={} passed", r.total))
            } else {
                Err(format!("invariant rejected total={}", r.total))
            }
        }
        "t02" => {
            if extras.focus_ring_ok {
                Ok(extras.focus_ring_detail.clone())
            } else {
                Err(extras.focus_ring_detail.clone())
            }
        }
        "t03" => {
            let w = compute_wave(1.0, 1.0, 1.0, 1.0);
            if (w - 1.0).abs() < 0.02 {
                Ok(format!("compute_wave(1,1,1,1)={w:.4}"))
            } else {
                Err(format!("weights drifted w={w}"))
            }
        }
        "t09" => {
            let (ok, n) = golay::empirical_nn_unique(12, 3, 99);
            let octads = golay::count_octads();
            if ok == n && octads == 759 {
                Ok(format!("NN {ok}/{n} · octads={octads}"))
            } else {
                Err(format!("NN {ok}/{n} octads={octads}"))
            }
        }
        "t10" => {
            let (a0, a4, a6) = hexacode::weight_distribution();
            if a0 == 1 && a4 == 45 && a6 == 18 {
                Ok(format!("A(x)=1+{a4}x^4+{a6}x^6"))
            } else {
                Err(format!("A0={a0} A4={a4} A6={a6}"))
            }
        }
        "t11" => {
            let p = reed_muller::RmParams::new(1, 4).ok_or("RM(1,4) params")?;
            let msg = vec![1u8, 0, 1, 0, 1];
            let c = reed_muller::encode(1, 4, &msg).ok_or("encode")?;
            let y = reed_muller::inject_errors(&c, p.t.min(2), 3);
            let d = reed_muller::decode_rm1_fht(&y, 4).ok_or("decode")?;
            if d.corrected == c {
                Ok("FHT recovered injected errors".into())
            } else {
                Err("FHT did not recover".into())
            }
        }
        "t12" => {
            let d = sc_ldpc::ScDesign::default();
            let r = sc_ldpc::analyze(d);
            let toy = sc_ldpc::windowed_bec_demo(d, 11, 2);
            if r.structurally_valid
                && r.terminated_rate < r.uncoupled_rate
                && toy.n_v > 0
                && (toy.success || toy.residual_erasures < toy.erased)
            {
                Ok(format!(
                    "valid · R_term<{:.3} · residual {}/{}",
                    r.terminated_rate, toy.residual_erasures, toy.erased
                ))
            } else {
                Err("SC-LDPC smoke failed".into())
            }
        }
        "t13" => {
            let n = lattice
                .layers
                .iter()
                .filter(|l| {
                    matches!(
                        l.layer,
                        LatticeLayer::Apps | LatticeLayer::Cutiles | LatticeLayer::Crates
                    ) && l.present
                })
                .count();
            if n == 3 {
                Ok("apps + cutiles + crates present".into())
            } else {
                Err(format!("only {n}/3 layers present"))
            }
        }
        "t14" => {
            let n = lattice
                .layers
                .iter()
                .filter(|l| {
                    matches!(l.layer, LatticeLayer::Kernels | LatticeLayer::Ops) && l.present
                })
                .count();
            if n >= 1 {
                Ok(format!("{n}/2 kernels/ops present"))
            } else {
                Err("kernels and ops both missing".into())
            }
        }
        "t17" => {
            let path = repo_root(lattice).join("ops/ci/codex_scan.py");
            let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            if text.contains("L7_skill_chain") && text.contains("layer_skill_chain") {
                Ok("L7_skill_chain wired in codex_scan.py".into())
            } else {
                Err("L7_skill_chain not found in codex_scan.py".into())
            }
        }
        _ => Err("no in-process executor".into()),
    }
}

/// Observe → DryRun → Execute (in-process only). External cases stop at DryRun.
pub fn run_staged(lattice: &LatticeSnapshot, extras: &Extras) -> Vec<Report> {
    CATALOG
        .iter()
        .cloned()
        .map(|spec| {
            let (seen, obs) = observe(&spec, lattice);
            if !seen {
                return Report {
                    spec,
                    status: Status::Skip,
                    stage: Stage::Observe,
                    detail: format!("absent: {obs}"),
                    category: 'D',
                };
            }
            if spec.kind == Kind::External {
                return Report {
                    spec: spec.clone(),
                    status: Status::DryRun,
                    stage: Stage::DryRun,
                    detail: format!("{} · {}", spec.dry_run, obs),
                    category: 'D',
                };
            }
            match execute(&spec, lattice, extras) {
                Ok(detail) => Report {
                    spec,
                    status: Status::Pass,
                    stage: Stage::Execute,
                    detail,
                    category: 'B',
                },
                Err(detail) => Report {
                    spec,
                    status: Status::Fail,
                    stage: Stage::Execute,
                    detail,
                    category: 'B',
                },
            }
        })
        .collect()
}

pub fn tally(reports: &[Report]) -> (usize, usize, usize, usize) {
    let pass = reports.iter().filter(|r| r.status == Status::Pass).count();
    let fail = reports.iter().filter(|r| r.status == Status::Fail).count();
    let dry = reports.iter().filter(|r| r.status == Status::DryRun).count();
    let skip = reports.iter().filter(|r| r.status == Status::Skip).count();
    (pass, fail, dry, skip)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_ids_unique() {
        let mut ids: Vec<_> = CATALOG.iter().map(|s| s.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), CATALOG.len());
    }

    #[test]
    fn staged_run_never_paints_external_as_pass() {
        let snap = LatticeSnapshot::probe();
        let extras = Extras {
            focus_ring_ok: true,
            focus_ring_detail: "ring ok".into(),
        };
        let reports = run_staged(&snap, &extras);
        assert_eq!(reports.len(), CATALOG.len());
        for r in &reports {
            if r.spec.kind == Kind::External {
                assert_ne!(r.status, Status::Pass, "{} painted Pass without execute", r.spec.id);
                assert!(
                    matches!(r.status, Status::DryRun | Status::Skip),
                    "{} unexpected {:?}",
                    r.spec.id,
                    r.status
                );
                assert_eq!(r.category, 'D');
            }
        }
    }

    #[test]
    fn in_process_cases_execute() {
        let snap = LatticeSnapshot::probe();
        let extras = Extras {
            focus_ring_ok: true,
            focus_ring_detail: "ring ok".into(),
        };
        let reports = run_staged(&snap, &extras);
        let core = reports.iter().find(|r| r.spec.id == "t01").unwrap();
        assert_eq!(core.status, Status::Pass);
        assert_eq!(core.stage, Stage::Execute);
        let golay = reports.iter().find(|r| r.spec.id == "t09").unwrap();
        assert_eq!(golay.status, Status::Pass);
        let hex = reports.iter().find(|r| r.spec.id == "t10").unwrap();
        assert_eq!(hex.status, Status::Pass);
    }
}
