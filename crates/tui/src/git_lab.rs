//! Guitar-inspired git pane — graph first, uncommitted row 0, observe-safe.
//!
//! Mental model borrowed from `asinglebit/guitar` (ratatui + topology graph):
//! the commit graph is the primary view, row 0 is synthetic uncommitted work,
//! status marks match guitar (`!` `~` `+` `-` `→`), and sharp ops stay gated.
//!
//! This pane is **observe + fetch**. It does not stage, commit, reset, rebase,
//! force-push, or discard. Fetch is the only network write and runs in ε.
//!
//! Host probe via `git` CLI (no libgit2). Category **B** — porcelain parsing
//! is tested; live `git` presence is best-effort.
//!
//! Closes the 0.2.1 follow-on: reconcile `main` remote (ahead/behind) before
//! a publish push.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

/// Safe actions the ε-phase may run (after RS-NOR latch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitAction {
    Refresh,
    Fetch,
}

impl GitAction {
    pub fn id(self) -> &'static str {
        match self {
            Self::Refresh => "refresh",
            Self::Fetch => "fetch",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Refresh => "refresh status + graph",
            Self::Fetch => "git fetch --prune (default remote)",
        }
    }
}

/// Working-tree / index mark (guitar status symbols).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMark {
    Conflict,
    Modified,
    Added,
    Deleted,
    Renamed,
    Untracked,
}

impl FileMark {
    pub fn glyph(self) -> char {
        match self {
            Self::Conflict => '!',
            Self::Modified => '~',
            Self::Added => '+',
            Self::Deleted => '-',
            Self::Renamed => '→',
            Self::Untracked => '?',
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Conflict => "conflict",
            Self::Modified => "modified",
            Self::Added => "added",
            Self::Deleted => "deleted",
            Self::Renamed => "renamed",
            Self::Untracked => "untracked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusFile {
    pub path: String,
    pub mark: FileMark,
    pub staged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphRow {
    /// Synthetic uncommitted-work row above HEAD (guitar row 0).
    pub is_uncommitted: bool,
    pub sha: String,
    pub summary: String,
    pub refs: Vec<String>,
    pub parent_count: usize,
}

impl GraphRow {
    /// TermGL-lite lane glyph — ASCII, no 3D engine.
    pub fn lane(self: &Self, is_head: bool) -> char {
        if self.is_uncommitted {
            '○'
        } else if is_head {
            '●'
        } else if self.parent_count >= 2 {
            '┳'
        } else {
            '*'
        }
    }

    pub fn display_line(&self, is_head: bool, width: usize) -> String {
        let refs = if self.refs.is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.refs.join(" "))
        };
        let sha = if self.is_uncommitted {
            "WORK".to_string()
        } else {
            self.sha.clone()
        };
        let raw = format!("{} {}{} {}", self.lane(is_head), sha, refs, self.summary);
        if width == 0 || raw.len() <= width {
            raw
        } else {
            raw.chars().take(width.saturating_sub(1)).collect::<String>() + "…"
        }
    }
}

#[derive(Debug, Clone)]
pub struct GitSnapshot {
    pub root: Option<PathBuf>,
    pub git_ok: bool,
    pub branch: String,
    pub detached: bool,
    pub upstream: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub staged: Vec<StatusFile>,
    pub unstaged: Vec<StatusFile>,
    pub conflicts: Vec<StatusFile>,
    pub graph: Vec<GraphRow>,
    pub remotes: Vec<String>,
    pub last_error: Option<String>,
    pub last_fetch: Option<String>,
    pub probed_at: Instant,
}

impl Default for GitSnapshot {
    fn default() -> Self {
        Self::empty("git not probed")
    }
}

impl GitSnapshot {
    fn empty(reason: &str) -> Self {
        Self {
            root: None,
            git_ok: false,
            branch: "—".into(),
            detached: false,
            upstream: None,
            ahead: 0,
            behind: 0,
            staged: Vec::new(),
            unstaged: Vec::new(),
            conflicts: Vec::new(),
            graph: Vec::new(),
            remotes: Vec::new(),
            last_error: Some(reason.into()),
            last_fetch: None,
            probed_at: Instant::now(),
        }
    }

    /// Probe from `LOGOS_ROOT` / crate-relative workspace / cwd.
    pub fn probe() -> Self {
        let root = discover_repo();
        match root {
            Some(root) => probe_at(&root),
            None => Self::empty("not a git work tree"),
        }
    }

    pub fn refresh(&mut self) {
        *self = Self::probe();
    }

    /// `git fetch --prune` against the default remote, then refresh.
    /// Network I/O — call from ε only.
    pub fn fetch(&mut self) -> String {
        let Some(root) = self.root.clone().or_else(discover_repo) else {
            self.last_error = Some("not a git work tree".into());
            return "git fetch skipped — no work tree".into();
        };
        let out = git_at(&root, &["fetch", "--prune", "--quiet"]);
        match out {
            Ok(_) => {
                let refreshed = probe_at(&root);
                let note = format!(
                    "fetched · {} ↑{} ↓{} · {}",
                    refreshed.branch, refreshed.ahead, refreshed.behind, refreshed.reconcile_note()
                );
                *self = refreshed;
                self.last_fetch = Some(note.clone());
                note
            }
            Err(e) => {
                self.last_error = Some(e.clone());
                format!("git fetch failed: {e}")
            }
        }
    }

    pub fn dirty_count(&self) -> usize {
        self.staged.len() + self.unstaged.len() + self.conflicts.len()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty_count() > 0
    }

    /// One-line operator reconcile (0.2.1 follow-on).
    pub fn reconcile_note(&self) -> String {
        if !self.git_ok {
            return self
                .last_error
                .clone()
                .unwrap_or_else(|| "git miss".into());
        }
        let sync = match (self.ahead, self.behind) {
            (0, 0) => "synced".into(),
            (a, 0) => format!("ahead {a} — local not pushed"),
            (0, b) => format!("behind {b} — fetch/rebase before push"),
            (a, b) => format!("diverged ↑{a} ↓{b} — reconcile before publish"),
        };
        let dirty = if self.is_dirty() {
            format!(" · dirty {}", self.dirty_count())
        } else {
            " · clean".into()
        };
        let conflicts = if self.conflicts.is_empty() {
            String::new()
        } else {
            format!(" · !{}", self.conflicts.len())
        };
        format!("{sync}{dirty}{conflicts}")
    }

    pub fn header_lines(&self) -> Vec<String> {
        let root = self
            .root
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("—");
        let head = if self.detached {
            format!("detached {}", self.branch)
        } else {
            self.branch.clone()
        };
        let up = self
            .upstream
            .as_deref()
            .map(|u| format!("…{u}"))
            .unwrap_or_else(|| "no-upstream".into());
        vec![
            format!("{root}  {head} {up}  ↑{} ↓{}", self.ahead, self.behind),
            self.reconcile_note(),
        ]
    }

    pub fn status_files_budgeted(&self, budget: usize) -> Vec<(char, bool, &str)> {
        let mut out = Vec::with_capacity(budget);
        for f in &self.conflicts {
            if out.len() >= budget {
                break;
            }
            out.push((f.mark.glyph(), f.staged, f.path.as_str()));
        }
        for f in self.staged.iter().chain(self.unstaged.iter()) {
            if out.len() >= budget {
                break;
            }
            if f.mark == FileMark::Conflict {
                continue;
            }
            out.push((f.mark.glyph(), f.staged, f.path.as_str()));
        }
        out
    }
}

fn discover_repo() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("LOGOS_ROOT") {
        let p = PathBuf::from(root);
        if looks_like_repo(&p) {
            return Some(p);
        }
    }
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = crate_root.join("../..");
    if let Ok(canon) = workspace.canonicalize() {
        if looks_like_repo(&canon) {
            return Some(canon);
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(found) = find_git_up(&cwd) {
            return Some(found);
        }
    }
    None
}

fn looks_like_repo(p: &Path) -> bool {
    p.join(".git").exists()
}

fn find_git_up(start: &Path) -> Option<PathBuf> {
    let mut cur = start.to_path_buf();
    loop {
        if looks_like_repo(&cur) {
            return Some(cur);
        }
        if !cur.pop() {
            return None;
        }
    }
}

fn probe_at(root: &Path) -> GitSnapshot {
    let mut snap = GitSnapshot {
        root: Some(root.to_path_buf()),
        git_ok: true,
        branch: "HEAD".into(),
        detached: false,
        upstream: None,
        ahead: 0,
        behind: 0,
        staged: Vec::new(),
        unstaged: Vec::new(),
        conflicts: Vec::new(),
        graph: Vec::new(),
        remotes: Vec::new(),
        last_error: None,
        last_fetch: None,
        probed_at: Instant::now(),
    };

    match git_at(root, &["status", "--porcelain=v1", "-b"]) {
        Ok(text) => apply_porcelain(&mut snap, &text),
        Err(e) => {
            snap.git_ok = false;
            snap.last_error = Some(e);
            return snap;
        }
    }

    if let Ok(text) = git_at(root, &["remote"]) {
        snap.remotes = text
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
    }

    if let Ok(text) = git_at(
        root,
        &[
            "log",
            "--decorate=short",
            "--pretty=format:%h\t%P\t%d\t%s",
            "-n",
            "24",
        ],
    ) {
        snap.graph = parse_log(&text);
    }

    if snap.is_dirty() || !snap.conflicts.is_empty() {
        snap.graph.insert(
            0,
            GraphRow {
                is_uncommitted: true,
                sha: String::new(),
                summary: format!(
                    "uncommitted  +{} ~{} !{}",
                    snap.staged.len(),
                    snap.unstaged.len(),
                    snap.conflicts.len()
                ),
                refs: vec!["WORK".into()],
                parent_count: 0,
            },
        );
    }

    snap
}

fn git_at(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|e| format!("git spawn: {e}"))?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        let msg = err.lines().next().unwrap_or("git failed").trim();
        return Err(if msg.is_empty() {
            format!("git {} exit {}", args.first().unwrap_or(&"?"), output.status)
        } else {
            msg.to_string()
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Parse `git status --porcelain=v1 -b` into the snapshot.
pub fn apply_porcelain(snap: &mut GitSnapshot, text: &str) {
    for (i, line) in text.lines().enumerate() {
        if i == 0 && line.starts_with("## ") {
            parse_branch_line(snap, &line[3..]);
            continue;
        }
        if let Some(file) = parse_status_line(line) {
            if file.mark == FileMark::Conflict {
                snap.conflicts.push(file);
            } else if file.staged {
                snap.staged.push(file);
            } else {
                snap.unstaged.push(file);
            }
        }
    }
}

pub fn parse_branch_line(snap: &mut GitSnapshot, rest: &str) {
    let rest = rest.trim();
    if rest.starts_with("HEAD (no branch)") || rest == "HEAD" {
        snap.detached = true;
        snap.branch = "HEAD".into();
        return;
    }
    // `main...origin/main [ahead 24, behind 1]`
    let (names, trail) = match rest.split_once('[') {
        Some((n, t)) => (n.trim(), Some(t.trim_end_matches(']'))),
        None => (rest, None),
    };
    if let Some((local, remote)) = names.split_once("...") {
        snap.branch = local.trim().to_string();
        snap.upstream = Some(remote.trim().to_string());
    } else {
        snap.branch = names.trim().to_string();
        snap.upstream = None;
    }
    snap.git_ok = true;
    if let Some(t) = trail {
        snap.ahead = extract_count(t, "ahead");
        snap.behind = extract_count(t, "behind");
    }
}

fn extract_count(trail: &str, key: &str) -> u32 {
    let Some(idx) = trail.find(key) else {
        return 0;
    };
    let after = trail[idx + key.len()..].trim_start();
    after
        .split(|c: char| !c.is_ascii_digit())
        .find(|s| !s.is_empty())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

pub fn parse_status_line(line: &str) -> Option<StatusFile> {
    if line.len() < 4 {
        return None;
    }
    let bytes = line.as_bytes();
    let x = bytes[0] as char;
    let y = bytes[1] as char;
    if x == '#' {
        return None;
    }
    let path = line[3..].trim();
    if path.is_empty() {
        return None;
    }
    let path = if let Some((_, dest)) = path.split_once(" -> ") {
        dest.trim().to_string()
    } else {
        path.to_string()
    };

    let conflict = matches!(
        (x, y),
        ('U', _) | (_, 'U') | ('A', 'A') | ('D', 'D')
    );
    if conflict {
        return Some(StatusFile {
            path,
            mark: FileMark::Conflict,
            staged: true,
        });
    }

    let staged = !matches!(x, ' ' | '?');
    let code = if staged { x } else { y };
    let mark = match code {
        'A' => FileMark::Added,
        'D' => FileMark::Deleted,
        'R' | 'C' => FileMark::Renamed,
        '?' => FileMark::Untracked,
        'M' | 'T' => FileMark::Modified,
        _ => FileMark::Modified,
    };
    Some(StatusFile { path, mark, staged })
}

pub fn parse_log(text: &str) -> Vec<GraphRow> {
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut parts = line.splitn(4, '\t');
        let sha = parts.next().unwrap_or("").trim().to_string();
        let parents = parts.next().unwrap_or("").trim();
        let decorate = parts.next().unwrap_or("").trim();
        let summary = parts.next().unwrap_or("").trim().to_string();
        if sha.is_empty() {
            continue;
        }
        let parent_count = if parents.is_empty() {
            0
        } else {
            parents.split_whitespace().count()
        };
        rows.push(GraphRow {
            is_uncommitted: false,
            sha,
            summary,
            refs: parse_decorate(decorate),
            parent_count,
        });
    }
    rows
}

pub fn parse_decorate(raw: &str) -> Vec<String> {
    let s = raw.trim().trim_start_matches('(').trim_end_matches(')');
    if s.is_empty() {
        return Vec::new();
    }
    s.split(',')
        .map(str::trim)
        .filter(|t| !t.is_empty() && *t != "HEAD")
        .map(|t| t.trim_start_matches("HEAD -> ").to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_ahead_behind() {
        let mut snap = GitSnapshot::empty("t");
        parse_branch_line(&mut snap, "main...origin/main [ahead 24]");
        assert_eq!(snap.branch, "main");
        assert_eq!(snap.upstream.as_deref(), Some("origin/main"));
        assert_eq!(snap.ahead, 24);
        assert_eq!(snap.behind, 0);
        assert!(snap.reconcile_note().contains("ahead 24"));
    }

    #[test]
    fn branch_diverged() {
        let mut snap = GitSnapshot::empty("t");
        parse_branch_line(&mut snap, "topic...origin/topic [ahead 2, behind 3]");
        assert_eq!(snap.ahead, 2);
        assert_eq!(snap.behind, 3);
        assert!(snap.reconcile_note().contains("diverged"));
    }

    #[test]
    fn branch_detached() {
        let mut snap = GitSnapshot::empty("t");
        parse_branch_line(&mut snap, "HEAD (no branch)");
        assert!(snap.detached);
        assert_eq!(snap.branch, "HEAD");
    }

    #[test]
    fn porcelain_status_marks() {
        let mut snap = GitSnapshot::empty("t");
        apply_porcelain(
            &mut snap,
            "\
## main...origin/main [ahead 1]
M  staged.rs
 M unstaged.rs
?? new.txt
UU conflict.rs
R  old.rs -> renamed.rs
",
        );
        assert_eq!(snap.ahead, 1);
        assert_eq!(snap.staged.len(), 2); // staged.rs + renamed
        assert_eq!(snap.unstaged.len(), 2); // unstaged + untracked
        assert_eq!(snap.conflicts.len(), 1);
        assert_eq!(snap.conflicts[0].mark.glyph(), '!');
        assert_eq!(
            snap.staged.iter().find(|f| f.path == "renamed.rs").map(|f| f.mark),
            Some(FileMark::Renamed)
        );
    }

    #[test]
    fn log_rows_and_decorate() {
        let rows = parse_log(
            "\
abc123\tdef456\t (HEAD -> main, origin/main)\tprepare release
def456\taa bb\t (tag: v0.2.1)\tmerge codes
aa1111\t\t\troot
",
        );
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].refs, vec!["main".to_string(), "origin/main".to_string()]);
        assert_eq!(rows[1].parent_count, 2);
        assert_eq!(rows[1].lane(false), '┳');
        assert_eq!(rows[0].lane(true), '●');
        assert_eq!(rows[2].parent_count, 0);
    }

    #[test]
    fn uncommitted_row_display() {
        let row = GraphRow {
            is_uncommitted: true,
            sha: String::new(),
            summary: "uncommitted  +1 ~2 !0".into(),
            refs: vec!["WORK".into()],
            parent_count: 0,
        };
        let line = row.display_line(false, 80);
        assert!(line.contains("WORK"));
        assert!(line.contains('○'));
    }

    #[test]
    fn probe_this_workspace() {
        let snap = GitSnapshot::probe();
        // LogOS workspace is a git repo; if git is missing the test still
        // records an honest miss rather than panicking.
        if snap.git_ok {
            assert!(snap.root.is_some());
            assert!(!snap.branch.is_empty());
            assert!(!snap.header_lines().is_empty());
        } else {
            assert!(snap.last_error.is_some());
        }
    }

    #[test]
    fn action_ids() {
        assert_eq!(GitAction::Fetch.id(), "fetch");
        assert_eq!(GitAction::Refresh.id(), "refresh");
    }
}
