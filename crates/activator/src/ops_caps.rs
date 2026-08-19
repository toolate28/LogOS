//! Daily-ops capability probe — editor · files · git · Actions · IDE.
//!
//! ATOM: `ATOM-GROK-TUI-HITL-GATE-20260818` (kit slice)
//! Live activator surface besides [`crate::ctqw_pass`].
//! `router` / `pipeline` / `awesome_skill` stay on-disk orphans.
//!
//! This module does **not** embed an editor or IDE. It answers
//! "does the host have the tool?" so the TUI can show have / don't-have.
//! Launch stays on the host split (capability ≠ authority).

use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpsCap {
    Git,
    GhActions,
    Editor,
    Ide,
    FileMgr,
    Tui,
    Bridge,
    ActivatorCtqw,
    ActivatorRouter,
}

impl OpsCap {
    pub const DAILY: [Self; 9] = [
        Self::Git,
        Self::GhActions,
        Self::Editor,
        Self::Ide,
        Self::FileMgr,
        Self::Tui,
        Self::Bridge,
        Self::ActivatorCtqw,
        Self::ActivatorRouter,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Git => "git",
            Self::GhActions => "gh-actions",
            Self::Editor => "editor",
            Self::Ide => "ide",
            Self::FileMgr => "files",
            Self::Tui => "tui",
            Self::Bridge => "bridge",
            Self::ActivatorCtqw => "activator-ctqw",
            Self::ActivatorRouter => "activator-router",
        }
    }

    pub fn need(self) -> &'static str {
        match self {
            Self::Git => "observe branch / dirty / ahead-behind",
            Self::GhActions => "entangle ingest + workflow_dispatch",
            Self::Editor => "edit markdown / Rust / Lean (host)",
            Self::Ide => "LSP + debug (host Claude/VS Code/Cursor)",
            Self::FileMgr => "browse LOGOS_ROOT (host / 9P later)",
            Self::Tui => "cockpit frame (operator only)",
            Self::Bridge => "shared telemetry wire :8088",
            Self::ActivatorCtqw => "CTQW / Griess pass (crate live)",
            Self::ActivatorRouter => "intent → editor/git/IDE (orphan)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CapProbe {
    pub cap: OpsCap,
    pub present: bool,
    pub launch: String,
    pub detail: String,
}

impl CapProbe {
    pub fn glyph(self: &Self) -> char {
        if self.present { '●' } else { '○' }
    }

    pub fn line(&self) -> String {
        format!(
            "{} {:<16} {}",
            self.glyph(),
            self.cap.id(),
            if self.detail.is_empty() {
                self.cap.need()
            } else {
                self.detail.as_str()
            }
        )
    }
}

/// Probe host PATH + in-tree markers. Category B. No paid calls.
pub fn probe_ops_caps() -> Vec<CapProbe> {
    let root = logos_root();
    OpsCap::DAILY
        .iter()
        .copied()
        .map(|cap| probe_one(cap, &root))
        .collect()
}

fn probe_one(cap: OpsCap, root: &Path) -> CapProbe {
    match cap {
        OpsCap::Git => which_cap(cap, &["git"], "git status · TUI [G] observe"),
        OpsCap::GhActions => which_cap(
            cap,
            &["gh"],
            "gh for Actions; ship path is still entangle + human merge",
        ),
        OpsCap::Editor => {
            let (bin, present) = first_on_path(&["hx", "nvim", "vim", "notepad"]);
            CapProbe {
                cap,
                present,
                launch: bin.unwrap_or_else(|| "hx | nvim | host editor".into()),
                detail: if present {
                    "host editor on PATH — TUI will not embed one".into()
                } else {
                    "no CLI editor on PATH — use WT split / VS Code".into()
                },
            }
        }
        OpsCap::Ide => {
            let (bin, present) = first_on_path(&["code", "cursor", "claude"]);
            CapProbe {
                cap,
                present,
                launch: bin.unwrap_or_else(|| "code | cursor | claude".into()),
                detail: if present {
                    "host IDE — Formal pane is diagnostics only".into()
                } else {
                    "no IDE CLI — Formal pane stays amber placeholders".into()
                },
            }
        }
        OpsCap::FileMgr => CapProbe {
            cap,
            present: root.is_dir(),
            launch: "explorer / ranger / 9P bookshelf".into(),
            detail: if root.is_dir() {
                format!("LOGOS_ROOT {}", root.display())
            } else {
                "LOGOS_ROOT missing".into()
            },
        },
        OpsCap::Tui => CapProbe {
            cap,
            present: root.join("crates/tui/Cargo.toml").is_file(),
            launch: "logos-tui · cargo run -p reson8-tui".into(),
            detail: "operator frame — agent does not see pixels".into(),
        },
        OpsCap::Bridge => CapProbe {
            cap,
            present: port_open(8088),
            launch: "logos-bridge".into(),
            detail: if port_open(8088) {
                "ws://127.0.0.1:8088 live".into()
            } else {
                "down — agent and TUI do not share telemetry".into()
            },
        },
        OpsCap::ActivatorCtqw => CapProbe {
            cap,
            present: root.join("crates/activator/src/ctqw_pass.rs").is_file(),
            launch: "coherence-mcp / run_ctqw_pass".into(),
            detail: "live crate surface".into(),
        },
        OpsCap::ActivatorRouter => CapProbe {
            cap,
            present: false,
            launch: "crates/activator/src/router.rs (orphan)".into(),
            detail: "on disk, not in lib.rs — intent→IDE not compiled".into(),
        },
    }
}

fn which_cap(cap: OpsCap, bins: &[&str], detail: &str) -> CapProbe {
    let (bin, present) = first_on_path(bins);
    CapProbe {
        cap,
        present,
        launch: bin.unwrap_or_else(|| bins.join(" | ")),
        detail: detail.into(),
    }
}

fn first_on_path(bins: &[&str]) -> (Option<String>, bool) {
    for b in bins {
        if which(b) {
            return (Some((*b).into()), true);
        }
    }
    (None, false)
}

fn which(bin: &str) -> bool {
    let path = env::var_os("PATH").unwrap_or_default();
    let exts = if cfg!(windows) {
        env::var("PATHEXT").unwrap_or_else(|_| ".EXE;.CMD;.BAT".into())
    } else {
        String::new()
    };
    let extra: Vec<&str> = if cfg!(windows) {
        exts.split(';').filter(|s| !s.is_empty()).collect()
    } else {
        vec![""]
    };
    for dir in env::split_paths(&path) {
        let base = dir.join(bin);
        if base.is_file() {
            return true;
        }
        for ext in &extra {
            if ext.is_empty() {
                continue;
            }
            let mut p = base.clone();
            p.set_extension(ext.trim_start_matches('.'));
            if p.is_file() {
                return true;
            }
            // PATHEXT includes the dot; also try concat.
            let named = dir.join(format!("{bin}{}", ext.to_ascii_lowercase()));
            if named.is_file() {
                return true;
            }
        }
    }
    false
}

fn port_open(port: u16) -> bool {
    use std::net::{TcpStream, ToSocketAddrs};
    use std::time::Duration;
    let addr = format!("127.0.0.1:{port}");
    let Ok(mut addrs) = addr.to_socket_addrs() else {
        return false;
    };
    let Some(sa) = addrs.next() else {
        return false;
    };
    TcpStream::connect_timeout(&sa, Duration::from_millis(80)).is_ok()
}

fn logos_root() -> PathBuf {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_returns_all_daily_caps() {
        let caps = probe_ops_caps();
        assert_eq!(caps.len(), OpsCap::DAILY.len());
        assert!(caps.iter().any(|c| c.cap == OpsCap::ActivatorRouter && !c.present));
        assert!(caps.iter().any(|c| c.cap == OpsCap::Tui && c.present));
    }

    #[test]
    fn git_probe_is_boolean() {
        let git = probe_ops_caps()
            .into_iter()
            .find(|c| c.cap == OpsCap::Git)
            .unwrap();
        let _ = git.present;
        assert!(!git.line().is_empty());
    }
}
