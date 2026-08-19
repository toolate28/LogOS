//! Host strand probes — Grok Build / xAI, Claude Code, Google ADK / gcloud.
//!
//! Clean path into reson8-tui Providers:
//! - **CLI present** (grok, claude, gcloud) → local agent shell capable
//! - **API key present** (env name only; never log values) → SDK HTTP capable
//! - **Bridge live** is separate (Providers title / BridgeState)
//!
//! No auto-spend: probes never call paid APIs. Launch recipes go to Logs + help.

use std::env;
use std::path::PathBuf;
use std::process::Command;

use reson8_forge_core::adapter::Provider;

/// How a strand can be engaged from this host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrandBackend {
    /// No local CLI and no configured API key.
    Missing,
    /// Interactive CLI on PATH (preferred for TUI side-by-side splits).
    Cli,
    /// Env key present for SDK/HTTP (vortex / OpenAI-compatible clients).
    SdkKey,
    /// Both CLI and key available.
    CliAndSdk,
}

impl StrandBackend {
    pub fn healthy(self) -> bool {
        !matches!(self, Self::Missing)
    }

    pub fn short(self) -> &'static str {
        match self {
            Self::Missing => "miss",
            Self::Cli => "cli",
            Self::SdkKey => "sdk",
            Self::CliAndSdk => "cli+sdk",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StrandProbe {
    pub provider: Provider,
    pub label: &'static str,
    pub backend: StrandBackend,
    /// One-line operator hint (no secrets).
    pub detail: String,
    /// Suggested host split command (learning / orchestration later).
    pub launch: String,
}

/// Probe local host once (sync, fast). Safe on every TUI boot / refresh.
pub fn probe_all() -> Vec<StrandProbe> {
    let grok_cli = which_bin(&["grok", "grok.exe"]);
    let claude_cli = which_bin(&["claude", "claude.exe", "claude.cmd"]);
    let gcloud_cli = which_bin(&["gcloud", "gcloud.cmd", "gcloud.exe"]);

    let xai_key = env_flag(&["XAI_API_KEY", "GROK_API_KEY"]);
    let anthropic_key = env_flag(&["ANTHROPIC_API_KEY", "CLAUDE_API_KEY"]);
    let google_key = env_flag(&[
        "GOOGLE_API_KEY",
        "GEMINI_API_KEY",
        "GOOGLE_APPLICATION_CREDENTIALS",
    ]);

    vec![
        strand(
            Provider::Grok,
            "Grok",
            grok_cli.as_ref(),
            xai_key,
            "xAI API / Grok Build TUI",
            if grok_cli.is_some() {
                "grok \"probe reson8-tui Formal pane\"".into()
            } else if xai_key {
                "set XAI_API_KEY · vortex-bridge or OpenAI-compat → api.x.ai/v1".into()
            } else {
                "install Grok Build CLI · or set XAI_API_KEY".into()
            },
        ),
        strand(
            Provider::Claude,
            "Claude",
            claude_cli.as_ref(),
            anthropic_key,
            "Claude Code / Anthropic API",
            if claude_cli.is_some() {
                "claude   # Reason strand in WT split".into()
            } else if anthropic_key {
                "ANTHROPIC_API_KEY set · SDK path (no auto-call)".into()
            } else {
                "npm i -g @anthropic-ai/claude-code · or set ANTHROPIC_API_KEY".into()
            },
        ),
        strand(
            Provider::Gemini,
            "Gemini",
            gcloud_cli.as_ref(),
            google_key,
            "Google ADK / gcloud / Gemini",
            if gcloud_cli.is_some() {
                "gcloud auth application-default login · ADK agent later".into()
            } else if google_key {
                "GOOGLE_* creds set · install gcloud for console parity".into()
            } else {
                "install Google Cloud SDK · gcloud components · ADK".into()
            },
        ),
        // Local / open-weight observers — PATH hints only for now.
        StrandProbe {
            provider: Provider::Manus,
            label: "Manus",
            backend: StrandBackend::Missing,
            detail: "local observer — open-weight slot".into(),
            launch: "optional: ollama / local OpenWeight".into(),
        },
        StrandProbe {
            provider: Provider::OpenWeight,
            label: "OpenWeight",
            backend: if which_bin(&["ollama", "ollama.exe"]).is_some() {
                StrandBackend::Cli
            } else {
                StrandBackend::Missing
            },
            detail: if which_bin(&["ollama", "ollama.exe"]).is_some() {
                "ollama on PATH".into()
            } else {
                "no local runtime probed".into()
            },
            launch: "ollama serve · ollama run <model>".into(),
        },
    ]
}

fn strand(
    provider: Provider,
    label: &'static str,
    cli: Option<&PathBuf>,
    key: bool,
    _role: &str,
    launch: String,
) -> StrandProbe {
    let backend = match (cli.is_some(), key) {
        (true, true) => StrandBackend::CliAndSdk,
        (true, false) => StrandBackend::Cli,
        (false, true) => StrandBackend::SdkKey,
        (false, false) => StrandBackend::Missing,
    };
    let detail = match (cli, key) {
        (Some(p), true) => format!("{} + api-key", short_path(p)),
        (Some(p), false) => short_path(p),
        (None, true) => "api-key env present".into(),
        (None, false) => "not on PATH / no key".into(),
    };
    StrandProbe {
        provider,
        label,
        backend,
        detail,
        launch,
    }
}

fn env_flag(names: &[&str]) -> bool {
    names.iter().any(|n| {
        env::var(n)
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false)
    })
}

fn which_bin(names: &[&str]) -> Option<PathBuf> {
    for name in names {
        if let Ok(p) = which_command(name) {
            return Some(p);
        }
    }
    // Common install locations (Windows Grok Build / npm)
    let home = env::var_os("USERPROFILE").or_else(|| env::var_os("HOME"));
    if let Some(home) = home {
        let home = PathBuf::from(home);
        let candidates = [
            home.join(".grok").join("bin").join("grok.exe"),
            home.join("AppData")
                .join("Roaming")
                .join("npm")
                .join("claude.cmd"),
            home.join("AppData")
                .join("Local")
                .join("Google")
                .join("Cloud SDK")
                .join("google-cloud-sdk")
                .join("bin")
                .join("gcloud.cmd"),
        ];
        for c in candidates {
            if c.exists() {
                let fname = c
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                for name in names {
                    let n = name.to_ascii_lowercase();
                    if fname.starts_with(n.trim_end_matches(".exe").trim_end_matches(".cmd")) {
                        return Some(c);
                    }
                }
            }
        }
    }
    None
}

fn which_command(name: &str) -> Result<PathBuf, ()> {
    #[cfg(windows)]
    let output = Command::new("where.exe").arg(name).output();
    #[cfg(not(windows))]
    let output = Command::new("which").arg(name).output();

    let output = output.map_err(|_| ())?;
    if !output.status.success() {
        return Err(());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next().unwrap_or("").trim();
    if line.is_empty() {
        return Err(());
    }
    let p = PathBuf::from(line);
    if p.exists() {
        Ok(p)
    } else {
        Err(())
    }
}

fn short_path(p: &PathBuf) -> String {
    p.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("bin")
        .to_string()
}

/// One-line summary for Logs / toast after probe.
pub fn probe_summary(probes: &[StrandProbe]) -> String {
    probes
        .iter()
        .map(|p| format!("{}:{}", p.label, p.backend.short()))
        .collect::<Vec<_>>()
        .join(" ")
}
