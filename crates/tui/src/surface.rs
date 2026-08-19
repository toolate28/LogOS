//! Host surface detection — which interface is reson8-tui running inside?
//!
//! The dashboard is embedded in very different frames: a Claude Desktop pane,
//! a plain agent CLI, an editor terminal, a Windows Terminal split. Each frame
//! implies a different useful default layout, so the operator lands on the
//! panels that matter for that surface without setting env vars by hand.
//!
//! Explicit `RESON8_LAYOUT` / `FORGE_LAYOUT` always wins — detection only
//! supplies the default. Category B: detection is heuristic (env sniffing),
//! never a load-bearing gate.

use crate::layout_presets::LayoutKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostSurface {
    /// Claude Code hosted in the Claude Desktop app.
    ClaudeDesktop,
    /// Claude Code in a terminal entrypoint.
    ClaudeCode,
    /// Editor-integrated terminal (VS Code / Cursor).
    Editor,
    /// Windows Terminal (host split workflow from the help overlay).
    WindowsTerminal,
    /// tmux / screen multiplexer pane.
    Multiplexer,
    /// Nothing distinctive detected.
    Plain,
}

impl HostSurface {
    pub fn id(self) -> &'static str {
        match self {
            Self::ClaudeDesktop => "claude-desktop",
            Self::ClaudeCode => "claude-code",
            Self::Editor => "editor",
            Self::WindowsTerminal => "wt",
            Self::Multiplexer => "tmux",
            Self::Plain => "plain",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::ClaudeDesktop => "Claude Desktop",
            Self::ClaudeCode => "Claude Code",
            Self::Editor => "Editor term",
            Self::WindowsTerminal => "Windows Terminal",
            Self::Multiplexer => "tmux",
            Self::Plain => "plain tty",
        }
    }

    /// Layout to open with when the operator has not chosen one.
    ///
    /// Agent-hosted surfaces watch strands and diagnostics while an agent works;
    /// editor terminals care about the formal/LSP eye; full terminals get ops.
    pub fn default_layout(self) -> LayoutKind {
        match self {
            Self::ClaudeDesktop | Self::ClaudeCode => LayoutKind::Agent,
            Self::Editor => LayoutKind::Formal,
            Self::WindowsTerminal | Self::Multiplexer | Self::Plain => LayoutKind::Ops,
        }
    }

    /// Detect from an arbitrary variable lookup (testable without touching env).
    pub fn detect_from(get: impl Fn(&str) -> Option<String>) -> Self {
        let non_empty = |k: &str| get(k).filter(|v| !v.trim().is_empty());

        if non_empty("CLAUDECODE").is_some() || non_empty("CLAUDE_CODE_ENTRYPOINT").is_some() {
            let entry = non_empty("CLAUDE_CODE_ENTRYPOINT").unwrap_or_default();
            return if entry.to_ascii_lowercase().contains("desktop") {
                Self::ClaudeDesktop
            } else {
                Self::ClaudeCode
            };
        }
        if non_empty("TERM_PROGRAM").is_some_and(|v| {
            let v = v.to_ascii_lowercase();
            v.contains("vscode") || v.contains("cursor")
        }) {
            return Self::Editor;
        }
        if non_empty("WT_SESSION").is_some() {
            return Self::WindowsTerminal;
        }
        if non_empty("TMUX").is_some() || non_empty("STY").is_some() {
            return Self::Multiplexer;
        }
        Self::Plain
    }

    pub fn detect() -> Self {
        Self::detect_from(|k| std::env::var(k).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lookup(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        move |k| {
            pairs
                .iter()
                .find(|(key, _)| *key == k)
                .map(|(_, v)| (*v).to_string())
        }
    }

    #[test]
    fn detects_claude_desktop() {
        let s = HostSurface::detect_from(lookup(&[
            ("CLAUDECODE", "1"),
            ("CLAUDE_CODE_ENTRYPOINT", "claude-desktop"),
        ]));
        assert_eq!(s, HostSurface::ClaudeDesktop);
        assert_eq!(s.default_layout(), LayoutKind::Agent);
    }

    #[test]
    fn detects_claude_cli_entrypoint() {
        let s = HostSurface::detect_from(lookup(&[
            ("CLAUDECODE", "1"),
            ("CLAUDE_CODE_ENTRYPOINT", "cli"),
        ]));
        assert_eq!(s, HostSurface::ClaudeCode);
    }

    #[test]
    fn claude_wins_over_terminal_hints() {
        // Claude Code running inside Windows Terminal is still an agent surface.
        let s = HostSurface::detect_from(lookup(&[
            ("CLAUDECODE", "1"),
            ("CLAUDE_CODE_ENTRYPOINT", "claude-desktop"),
            ("WT_SESSION", "abc"),
        ]));
        assert_eq!(s, HostSurface::ClaudeDesktop);
    }

    #[test]
    fn detects_editor_and_wt_and_plain() {
        assert_eq!(
            HostSurface::detect_from(lookup(&[("TERM_PROGRAM", "vscode")])),
            HostSurface::Editor
        );
        assert_eq!(
            HostSurface::detect_from(lookup(&[("WT_SESSION", "x")])),
            HostSurface::WindowsTerminal
        );
        assert_eq!(HostSurface::detect_from(lookup(&[])), HostSurface::Plain);
    }

    #[test]
    fn empty_values_are_not_signals() {
        assert_eq!(
            HostSurface::detect_from(lookup(&[("CLAUDECODE", ""), ("WT_SESSION", "  ")])),
            HostSurface::Plain
        );
    }

    #[test]
    fn every_surface_default_layout_is_renderable() {
        for s in [
            HostSurface::ClaudeDesktop,
            HostSurface::ClaudeCode,
            HostSurface::Editor,
            HostSurface::WindowsTerminal,
            HostSurface::Multiplexer,
            HostSurface::Plain,
        ] {
            let k = s.default_layout();
            assert!(k.contains(k.primary_focus()), "{}", s.id());
        }
    }
}
