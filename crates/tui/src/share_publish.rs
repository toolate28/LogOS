//! ε-phase share: publish a showcase asset via `ops/xai_files_public_url.py`.
//!
//! Track A only — spawns helper with `XAI_API_KEY`. Never residual-zero gate.
//! Public CDN URL is Category C share surface (revocable, independent of private file).

use std::path::{Path, PathBuf};
use std::process::Command;

/// Eligible public types (API limit).
pub const PUBLIC_EXTS: &[&str] = &["png", "jpg", "jpeg", "mp4", "pdf"];

#[derive(Debug, Clone)]
pub struct ShareResult {
    pub ok: bool,
    pub message: String,
    pub public_url: Option<String>,
    pub file_id: Option<String>,
    pub path: PathBuf,
}

/// Resolve default showcase path relative to LOGOS_ROOT / crate root.
pub fn default_showcase_path() -> PathBuf {
    let root = std::env::var("LOGOS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    let candidates = [
        root.join("docs/assets/reson8-tui-showcase-v0.2.1.jpg"),
        root.join("docs/assets/reson8-tui-showcase.png"),
        root.join("docs/assets/session-resume-2026-07-04.png"),
    ];
    for c in candidates {
        if c.is_file() {
            return c;
        }
    }
    candidates[0].clone()
}

pub fn script_path() -> PathBuf {
    let root = std::env::var("LOGOS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    root.join("ops/xai_files_public_url.py")
}

fn eligible(path: &Path) -> Result<(), String> {
    if !path.is_file() {
        return Err(format!("missing file: {}", path.display()));
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if !PUBLIC_EXTS.iter().any(|e| *e == ext || (*e == "jpeg" && ext == "jpg")) {
        return Err(format!(
            "extension .{ext} not public-eligible (need png/jpeg/mp4/pdf)"
        ));
    }
    let meta = std::fs::metadata(path).map_err(|e| e.to_string())?;
    if meta.len() > 50 * 1024 * 1024 {
        return Err("file exceeds 50 MiB public limit".into());
    }
    Ok(())
}

/// Publish path to xAI Files + public URL (blocking; call from ε only).
///
/// Default URL TTL: 7 days (604800). Requires `XAI_API_KEY` in environment.
pub fn publish_public(path: &Path, expires_secs: u32) -> ShareResult {
    if let Err(message) = eligible(path) {
        return ShareResult {
            ok: false,
            message,
            public_url: None,
            file_id: None,
            path: path.to_path_buf(),
        };
    }
    if std::env::var("XAI_API_KEY").is_err() && std::env::var("GROK_API_KEY").is_err() {
        return ShareResult {
            ok: false,
            message: "XAI_API_KEY not set — set key then press Shift+U again".into(),
            public_url: None,
            file_id: None,
            path: path.to_path_buf(),
        };
    }
    let script = script_path();
    if !script.is_file() {
        return ShareResult {
            ok: false,
            message: format!("helper missing: {}", script.display()),
            public_url: None,
            file_id: None,
            path: path.to_path_buf(),
        };
    }
    let py = if cfg!(windows) { "python" } else { "python3" };
    let output = Command::new(py)
        .arg(&script)
        .arg("publish")
        .arg(path)
        .arg("--expires")
        .arg(expires_secs.to_string())
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            if !out.status.success() {
                return ShareResult {
                    ok: false,
                    message: format!(
                        "publish failed: {}{}",
                        stderr.trim(),
                        if stdout.trim().is_empty() {
                            ""
                        } else {
                            stdout.trim()
                        }
                    ),
                    public_url: None,
                    file_id: None,
                    path: path.to_path_buf(),
                };
            }
            // Parse JSON stdout
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                let url = v
                    .get("public_url")
                    .and_then(|u| u.as_str())
                    .map(|s| s.to_string());
                let fid = v
                    .get("file_id")
                    .and_then(|u| u.as_str())
                    .map(|s| s.to_string());
                let msg = match &url {
                    Some(u) => format!("CDN ready (7d TTL): {u}"),
                    None => format!("uploaded; no public_url in response: {}", stdout.trim()),
                };
                return ShareResult {
                    ok: url.is_some(),
                    message: msg,
                    public_url: url,
                    file_id: fid,
                    path: path.to_path_buf(),
                };
            }
            ShareResult {
                ok: false,
                message: format!("non-JSON response: {}", stdout.trim()),
                public_url: None,
                file_id: None,
                path: path.to_path_buf(),
            }
        }
        Err(e) => ShareResult {
            ok: false,
            message: format!("spawn {py}: {e}"),
            public_url: None,
            file_id: None,
            path: path.to_path_buf(),
        },
    }
}

/// Companion tools for help / launch recipes (external — not embedded).
pub struct CompanionTool {
    pub id: &'static str,
    pub label: &'static str,
    pub launch: &'static str,
    pub why: &'static str,
    pub url: &'static str,
}

pub const COMPANIONS: &[CompanionTool] = &[
    CompanionTool {
        id: "scope-tui",
        label: "scope-tui",
        launch: "scope-tui audio",
        why: "oscilloscope / vectorscope / FFT eye",
        url: "https://github.com/alemidev/scope-tui",
    },
    CompanionTool {
        id: "psnet",
        label: "psnet",
        launch: "psnet",
        why: "Windows 9-tab network monitor",
        url: "https://github.com/psmux/psnet",
    },
    CompanionTool {
        id: "putzen",
        label: "putzen",
        launch: "putzen -d .",
        why: "clean cargo/node build artifacts (dry-run)",
        url: "https://github.com/sassman/putzen-rs",
    },
    CompanionTool {
        id: "ytsurf",
        label: "ytsurf",
        launch: "ytsurf --audio",
        why: "distraction-free terminal YouTube",
        url: "https://github.com/Stan-breaks/ytsurf",
    },
    CompanionTool {
        id: "durdraw",
        label: "durdraw",
        launch: "durdraw",
        why: "ANSI/Unicode art + animation frames",
        url: "https://github.com/durdraw/durdraw",
    },
    CompanionTool {
        id: "rura",
        label: "rura (catalog)",
        launch: "# browse https://awesometui.com/rura",
        why: "Awesome TUI discovery",
        url: "https://awesometui.com/rura",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn companions_nonempty() {
        assert!(COMPANIONS.len() >= 5);
        assert!(COMPANIONS.iter().any(|c| c.id == "psnet"));
    }

    #[test]
    fn default_path_is_under_docs_assets() {
        let p = default_showcase_path();
        assert!(p.to_string_lossy().contains("docs") || p.to_string_lossy().contains("assets") || !p.exists());
    }
}
