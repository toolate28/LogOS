//! HITL queue — SAIF outstanding human actions in the cockpit.
//!
//! ATOM: `ATOM-GROK-TUI-HITL-GATE-20260818`
//! Depends on: `ATOM-SAIF-HUMAN-QUEUE-20260723`
//!             · `ATOM-CHECKPOINT-CHOKE-20260814`
//!             · `ATOM-LATTICE-ACTIVATE-20260815`
//!             · `ATOM-GROK-TUI-QR-META-20260806`
//!
//! Clock: **constraint order is ATOM stamps**, not wall clocks.
//! `constraint` / `seq` order the subroutine. A wall ISO is an observation
//! only (Category C) and never the sort key.
//!
//! Not a sidecar binary. Same H-probe pattern as lattice: read the queue
//! file, CNOT-reduce into `App`, RS-NOR latch selection, ε write receipts.
//! Approve does **not** run GCP / sudo / wrangler (capability ≠ authority).

use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Constraint stamp for this TUI gate (ordering key, not wall time).
pub const HITL_ATOM: &str = "ATOM-GROK-TUI-HITL-GATE-20260818";
pub const HITL_CONSTRAINT: &str = "20260818.hitl-gate";
pub const QUEUE_ATOM: &str = "ATOM-SAIF-HUMAN-QUEUE-20260723";

const JSON_REL: &str = "ops/human-actions.json";
const MD_REL: &str = "ops/SAIF-OUTSTANDING-HUMAN-ACTIONS.md";
const RECEIPT_REL: &str = "ops/marks/hitl-receipts.jsonl";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Priority {
    A,
    B,
    C,
    D,
}

impl Priority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s.trim().chars().next()? {
            'A' | 'a' => Some(Self::A),
            'B' | 'b' => Some(Self::B),
            'C' | 'c' => Some(Self::C),
            'D' | 'd' => Some(Self::D),
            _ => None,
        }
    }
}

/// Who must act. Glyphs match the SAIF list: ⚑ human · ⚒ agent · ⏸ blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Actor {
    Human,
    Agent,
    Blocked,
}

impl Actor {
    pub fn glyph(self) -> &'static str {
        match self {
            Self::Human => "⚑",
            Self::Agent => "⚒",
            Self::Blocked => "⏸",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Agent => "agent",
            Self::Blocked => "blocked",
        }
    }
}

/// In-session latch. Does not rewrite the markdown SoT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Open,
    Requested,
    Escalated,
    Approved,
    Deferred,
    Denied,
}

impl SessionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Requested => "requested",
            Self::Escalated => "escalated",
            Self::Approved => "approved",
            Self::Deferred => "deferred",
            Self::Denied => "denied",
        }
    }

    /// Still eligible as first_need.
    pub fn is_open(self) -> bool {
        matches!(self, Self::Open | Self::Requested)
    }
}

/// Operator decision in the approval subroutine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HitlDecision {
    /// Open the gate on this item (request).
    Request,
    /// Send ⚒ follow-on to the agent; human item stays in the MD.
    Escalate,
    /// Sign off. Receipt only — no deploy.
    Approve,
    /// Skip; stay in queue behind later first_need.
    Defer,
    /// Refuse this cycle.
    Deny,
    /// Advance selection to the next ⚑ without deciding.
    Next,
}

impl HitlDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Request => "request",
            Self::Escalate => "escalate",
            Self::Approve => "approve",
            Self::Defer => "defer",
            Self::Deny => "deny",
            Self::Next => "next",
        }
    }

    pub fn popup_options() -> Vec<String> {
        vec![
            "approve (receipt only — no deploy)".into(),
            "escalate to agent (⚒)".into(),
            "defer".into(),
            "deny".into(),
            "next ⚑".into(),
        ]
    }

    pub fn from_popup_answer(answer: &str) -> Option<Self> {
        let a = answer.to_ascii_lowercase();
        if a.starts_with("approve") {
            Some(Self::Approve)
        } else if a.starts_with("escalate") {
            Some(Self::Escalate)
        } else if a.starts_with("defer") {
            Some(Self::Defer)
        } else if a.starts_with("deny") {
            Some(Self::Deny)
        } else if a.starts_with("next") {
            Some(Self::Next)
        } else if a.starts_with("request") {
            Some(Self::Request)
        } else {
            None
        }
    }

    fn to_status(self) -> Option<SessionStatus> {
        match self {
            Self::Request => Some(SessionStatus::Requested),
            Self::Escalate => Some(SessionStatus::Escalated),
            Self::Approve => Some(SessionStatus::Approved),
            Self::Defer => Some(SessionStatus::Deferred),
            Self::Deny => Some(SessionStatus::Denied),
            Self::Next => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanAction {
    pub id: String,
    pub priority: Priority,
    #[serde(default = "default_actor")]
    pub who: Actor,
    pub title: String,
    #[serde(default)]
    pub why: String,
    #[serde(default)]
    pub do_steps: Vec<String>,
    #[serde(default)]
    pub reply: String,
    #[serde(default)]
    pub refs: Vec<String>,
    /// In-session latch — not persisted to the markdown SoT.
    #[serde(default = "default_status", skip_deserializing)]
    pub status: SessionStatus,
}

fn default_actor() -> Actor {
    Actor::Human
}

fn default_status() -> SessionStatus {
    SessionStatus::Open
}

impl HumanAction {
    pub fn summary_line(&self) -> String {
        format!(
            "{}{} {} {}",
            self.who.glyph(),
            self.priority.as_str(),
            self.id,
            self.title
        )
    }

    pub fn gate_question(&self) -> String {
        let mut q = format!("{} — {}\n", self.summary_line(), self.why);
        if !self.do_steps.is_empty() {
            q.push_str("Do:\n");
            for (i, step) in self.do_steps.iter().take(6).enumerate() {
                q.push_str(&format!("  {}. {}\n", i + 1, step));
            }
        }
        if !self.reply.is_empty() {
            q.push_str("Reply template logged on approve/escalate.\n");
        }
        q.push_str("Approve writes a receipt. It does not run the host action.");
        q
    }
}

#[derive(Debug, Clone, Deserialize)]
struct QueueFile {
    #[serde(default)]
    atom: String,
    #[serde(default)]
    constraint: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    choke: String,
    #[serde(default)]
    actions: Vec<FileAction>,
}

#[derive(Debug, Clone, Deserialize)]
struct FileAction {
    id: String,
    #[serde(default)]
    priority: Option<Priority>,
    #[serde(default)]
    who: Option<Actor>,
    title: String,
    #[serde(default)]
    why: String,
    #[serde(default, rename = "do")]
    do_steps: Vec<String>,
    #[serde(default)]
    reply: String,
    #[serde(default)]
    refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HitlReceipt {
    pub atom: &'static str,
    pub constraint: &'static str,
    pub seq: u32,
    pub action: String,
    pub decision: &'static str,
    pub who: &'static str,
    pub category: &'static str,
    pub verification: &'static str,
    pub depends_on: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HumanActionQueue {
    pub atom: String,
    pub constraint: String,
    pub source: String,
    pub choke: String,
    pub path: PathBuf,
    pub loaded_from: &'static str,
    pub items: Vec<HumanAction>,
    pub selected: usize,
    /// Monotonic in-session receipt sequence (constraint clock, not wall).
    pub seq: u32,
    pub last_error: Option<String>,
}

impl Default for HumanActionQueue {
    fn default() -> Self {
        Self {
            atom: QUEUE_ATOM.into(),
            constraint: "20260723.saif-human-queue".into(),
            source: MD_REL.into(),
            choke: "docs/ops/CHECKPOINT-CHOKE-20260814.md".into(),
            path: PathBuf::new(),
            loaded_from: "empty",
            items: Vec::new(),
            selected: 0,
            seq: 0,
            last_error: None,
        }
    }
}

impl HumanActionQueue {
    pub fn load() -> Self {
        Self::load_from(&logos_root())
    }

    pub fn load_from(root: &Path) -> Self {
        let json = root.join(JSON_REL.replace('/', std::path::MAIN_SEPARATOR_STR));
        if json.is_file() {
            match fs::read_to_string(&json) {
                Ok(text) => match parse_json(&text) {
                    Ok(mut q) => {
                        q.path = json;
                        q.loaded_from = "json";
                        q.sort_items();
                        return q;
                    }
                    Err(e) => {
                        let mut q = Self::load_markdown(root);
                        q.last_error = Some(format!("json: {e}"));
                        return q;
                    }
                },
                Err(e) => {
                    let mut q = Self::load_markdown(root);
                    q.last_error = Some(format!("json read: {e}"));
                    return q;
                }
            }
        }
        Self::load_markdown(root)
    }

    fn load_markdown(root: &Path) -> Self {
        let md = root.join(MD_REL.replace('/', std::path::MAIN_SEPARATOR_STR));
        let mut q = Self::default();
        q.path = md.clone();
        match fs::read_to_string(&md) {
            Ok(text) => {
                q.items = parse_markdown(&text);
                q.loaded_from = "markdown";
                q.sort_items();
            }
            Err(e) => {
                q.loaded_from = "missing";
                q.last_error = Some(format!("md: {e}"));
            }
        }
        q
    }

    fn sort_items(&mut self) {
        self.items
            .sort_by(|a, b| a.priority.cmp(&b.priority).then_with(|| a.id.cmp(&b.id)));
        if self.selected >= self.items.len() {
            self.selected = 0;
        }
        if let Some(i) = self.first_need_index() {
            self.selected = i;
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn selected(&self) -> Option<&HumanAction> {
        self.items.get(self.selected)
    }

    pub fn selected_mut(&mut self) -> Option<&mut HumanAction> {
        self.items.get_mut(self.selected)
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = if self.selected == 0 {
            self.items.len() - 1
        } else {
            self.selected - 1
        };
    }

    /// Next ⚑ / human item still open — last_done → first_need.
    pub fn first_need(&self) -> Option<&HumanAction> {
        self.first_need_index().and_then(|i| self.items.get(i))
    }

    pub fn first_need_index(&self) -> Option<usize> {
        self.items.iter().position(|a| {
            a.who == Actor::Human && a.status.is_open() && a.priority <= Priority::B
        })
        .or_else(|| {
            self.items
                .iter()
                .position(|a| a.who == Actor::Human && a.status.is_open())
        })
    }

    pub fn select_first_need(&mut self) {
        if let Some(i) = self.first_need_index() {
            self.selected = i;
        }
    }

    pub fn open_count(&self) -> usize {
        self.items
            .iter()
            .filter(|a| a.who == Actor::Human && a.status.is_open())
            .count()
    }

    pub fn short(&self) -> String {
        format!("hitl {}/{}", self.open_count(), self.len())
    }

    /// Apply a decision to the selected (or named) item. Returns the id.
    pub fn apply_decision(&mut self, decision: HitlDecision, id: Option<&str>) -> Option<String> {
        let idx = if let Some(id) = id {
            self.items.iter().position(|a| a.id == id)?
        } else {
            self.selected
        };
        let item = self.items.get_mut(idx)?;
        let action_id = item.id.clone();
        if let Some(st) = decision.to_status() {
            item.status = st;
        }
        if decision == HitlDecision::Next {
            self.select_next_open_from(idx);
        } else if !matches!(decision, HitlDecision::Request) {
            self.select_first_need();
        } else {
            self.selected = idx;
        }
        Some(action_id)
    }

    fn select_next_open_from(&mut self, from: usize) {
        if self.items.is_empty() {
            return;
        }
        let n = self.items.len();
        for step in 1..=n {
            let i = (from + step) % n;
            if self.items[i].status.is_open() && self.items[i].who == Actor::Human {
                self.selected = i;
                return;
            }
        }
        self.selected = (from + 1) % n;
    }

    /// ε-phase: append a spine receipt. Ordering key is constraint+seq.
    pub fn write_receipt(&mut self, root: &Path, decision: HitlDecision, action_id: &str) -> String {
        self.seq = self.seq.saturating_add(1);
        let receipt = HitlReceipt {
            atom: HITL_ATOM,
            constraint: HITL_CONSTRAINT,
            seq: self.seq,
            action: action_id.to_string(),
            decision: decision.as_str(),
            who: "operator",
            category: "B",
            verification: "build-asserted",
            depends_on: QUEUE_ATOM,
            wall: None,
        };
        let path = root.join(RECEIPT_REL.replace('/', std::path::MAIN_SEPARATOR_STR));
        match serde_json::to_string(&receipt) {
            Ok(line) => match append_line(&path, &line) {
                Ok(()) => format!(
                    "{} {} seq={} → {}",
                    decision.as_str(),
                    action_id,
                    self.seq,
                    path.display()
                ),
                Err(e) => format!("{} {} seq={} write-fail: {e}", decision.as_str(), action_id, self.seq),
            },
            Err(e) => format!("receipt encode fail: {e}"),
        }
    }
}

fn append_line(path: &Path, line: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(f, "{line}")
}

pub fn logos_root() -> PathBuf {
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

fn parse_json(text: &str) -> Result<HumanActionQueue, String> {
    let file: QueueFile = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let mut q = HumanActionQueue {
        atom: if file.atom.is_empty() {
            QUEUE_ATOM.into()
        } else {
            file.atom
        },
        constraint: if file.constraint.is_empty() {
            "20260723.saif-human-queue".into()
        } else {
            file.constraint
        },
        source: file.source,
        choke: file.choke,
        path: PathBuf::new(),
        loaded_from: "json",
        items: file
            .actions
            .into_iter()
            .map(|a| {
                let priority = a
                    .priority
                    .or_else(|| Priority::parse(&a.id))
                    .unwrap_or(Priority::D);
                HumanAction {
                    id: a.id,
                    priority,
                    who: a.who.unwrap_or(Actor::Human),
                    title: a.title,
                    why: a.why,
                    do_steps: a.do_steps,
                    reply: a.reply,
                    refs: a.refs,
                    status: SessionStatus::Open,
                }
            })
            .collect(),
        selected: 0,
        seq: 0,
        last_error: None,
    };
    if q.source.is_empty() {
        q.source = MD_REL.into();
    }
    Ok(q)
}

/// Best-effort MD parse (Category B). Headings `### A1. Title ⚑` plus D-table rows.
pub fn parse_markdown(text: &str) -> Vec<HumanAction> {
    let mut items = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if let Some(action) = parse_heading(line) {
            let mut action = action;
            i += 1;
            while i < lines.len() {
                let l = lines[i].trim();
                if l.starts_with("### ") || l.starts_with("## Priority") {
                    break;
                }
                if let Some(rest) = l.strip_prefix("**Why:**") {
                    action.why = rest.trim().to_string();
                } else if l.starts_with("**Do:**") {
                    i += 1;
                    while i < lines.len() {
                        let s = lines[i].trim();
                        if s.starts_with("**") || s.starts_with("### ") || s.starts_with("```") {
                            i -= 1;
                            break;
                        }
                        let step = s
                            .trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == '-' || c == '*')
                            .trim();
                        if !step.is_empty() {
                            action.do_steps.push(step.to_string());
                        }
                        i += 1;
                    }
                } else if l.starts_with("**Refs:**") {
                    for cap in l.split('`').skip(1).step_by(2) {
                        if !cap.trim().is_empty() {
                            action.refs.push(cap.trim().to_string());
                        }
                    }
                }
                i += 1;
            }
            items.push(action);
            continue;
        }
        if let Some(action) = parse_table_row(line) {
            if !items.iter().any(|a| a.id == action.id) {
                items.push(action);
            }
        }
        i += 1;
    }
    items
}

fn parse_heading(line: &str) -> Option<HumanAction> {
    let rest = line.strip_prefix("### ")?;
    let mut parts = rest.splitn(2, '.');
    let id = parts.next()?.trim();
    if id.len() < 2 || !id.starts_with(|c: char| matches!(c, 'A'..='D')) {
        return None;
    }
    if !id.chars().nth(1)?.is_ascii_digit() {
        return None;
    }
    let title_raw = parts.next()?.trim();
    let who = if title_raw.contains('⏸') {
        Actor::Blocked
    } else if title_raw.contains('⚒') && !title_raw.contains('⚑') {
        Actor::Agent
    } else {
        Actor::Human
    };
    let title = title_raw
        .replace(['⚑', '⚒', '⏸'], "")
        .trim()
        .trim_end_matches('/')
        .trim()
        .to_string();
    Some(HumanAction {
        id: id.to_string(),
        priority: Priority::parse(id).unwrap_or(Priority::D),
        who,
        title,
        why: String::new(),
        do_steps: Vec::new(),
        reply: String::new(),
        refs: Vec::new(),
        status: SessionStatus::Open,
    })
}

fn parse_table_row(line: &str) -> Option<HumanAction> {
    if !line.starts_with('|') {
        return None;
    }
    let cols: Vec<&str> = line.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
    if cols.len() < 2 {
        return None;
    }
    let id = cols[0];
    if id.len() < 2 || !matches!(id.chars().next(), Some('A'..='D')) {
        return None;
    }
    if !id.chars().nth(1)?.is_ascii_digit() {
        return None;
    }
    if id.contains('-') || id.eq_ignore_ascii_case("id") {
        return None;
    }
    let title = cols.get(1).copied().unwrap_or("").to_string();
    if title.is_empty() || title.eq_ignore_ascii_case("action") {
        return None;
    }
    let who = match cols.get(2).copied().unwrap_or("") {
        s if s.contains('⚒') || s.eq_ignore_ascii_case("agent") => Actor::Agent,
        s if s.contains('⏸') => Actor::Blocked,
        _ => Actor::Human,
    };
    Some(HumanAction {
        id: id.to_string(),
        priority: Priority::parse(id).unwrap_or(Priority::D),
        who,
        title,
        why: String::new(),
        do_steps: Vec::new(),
        reply: String::new(),
        refs: Vec::new(),
        status: SessionStatus::Open,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_MD: &str = r#"
# SAIF list

**ATOM:** `ATOM-SAIF-HUMAN-QUEUE-20260723`

## Priority A — unblocks ship path

### A1. GCP / Cloud Run (GB-06) ⚑
**Why:** Last deploy-waist packet.
**Do:**
1. Create project
2. Choose region
**Refs:** `ops/GB06-worklog.md`

### A2. Git push surfaces ⚑
**Why:** HTTP 408.

## Priority C — formal pins

### C1. Cubical pin decision ⏸/⚑
**Why:** Agda Everything may not typecheck.

## Priority D

| ID | Action | Who |
|----|--------|-----|
| D1 | Re-pin compose digests | ⚒ |
| D2 | Enable linger for styx | ⚑ |
"#;

    #[test]
    fn markdown_parses_priorities_and_glyphs() {
        let items = parse_markdown(FIXTURE_MD);
        assert!(items.iter().any(|a| a.id == "A1" && a.who == Actor::Human));
        assert!(items.iter().any(|a| a.id == "A2"));
        assert!(items.iter().any(|a| a.id == "C1" && a.who == Actor::Blocked));
        assert!(items.iter().any(|a| a.id == "D1" && a.who == Actor::Agent));
        assert!(items.iter().any(|a| a.id == "D2" && a.who == Actor::Human));
        let a1 = items.iter().find(|a| a.id == "A1").unwrap();
        assert!(a1.why.contains("deploy-waist"));
        assert!(a1.do_steps.iter().any(|s| s.contains("Create project")));
        assert!(a1.refs.iter().any(|r| r.contains("GB06")));
    }

    #[test]
    fn json_load_sets_first_need_to_a1() {
        let text = r#"{
          "atom": "ATOM-SAIF-HUMAN-QUEUE-20260723",
          "constraint": "20260723.saif-human-queue",
          "actions": [
            {"id":"B1","priority":"B","who":"human","title":"Claude cold start"},
            {"id":"A1","priority":"A","who":"human","title":"GCP","why":"waist"}
          ]
        }"#;
        let mut q = parse_json(text).unwrap();
        q.sort_items();
        let need = q.first_need().unwrap();
        assert_eq!(need.id, "A1");
        assert_eq!(q.selected().unwrap().id, "A1");
    }

    #[test]
    fn approve_advances_first_need() {
        let text = r#"{
          "actions": [
            {"id":"A1","priority":"A","who":"human","title":"GCP"},
            {"id":"A2","priority":"A","who":"human","title":"Git"}
          ]
        }"#;
        let mut q = parse_json(text).unwrap();
        q.sort_items();
        let id = q.apply_decision(HitlDecision::Approve, Some("A1")).unwrap();
        assert_eq!(id, "A1");
        assert_eq!(q.first_need().unwrap().id, "A2");
        assert_eq!(q.items[0].status, SessionStatus::Approved);
    }

    #[test]
    fn escalate_is_not_approve() {
        let text = r#"{"actions":[{"id":"A1","priority":"A","who":"human","title":"GCP"}]}"#;
        let mut q = parse_json(text).unwrap();
        q.apply_decision(HitlDecision::Escalate, Some("A1"));
        assert_eq!(q.items[0].status, SessionStatus::Escalated);
        assert!(q.first_need().is_none());
    }

    #[test]
    fn popup_answers_map() {
        assert_eq!(
            HitlDecision::from_popup_answer("approve (receipt only — no deploy)"),
            Some(HitlDecision::Approve)
        );
        assert_eq!(
            HitlDecision::from_popup_answer("escalate to agent (⚒)"),
            Some(HitlDecision::Escalate)
        );
        assert_eq!(HitlDecision::from_popup_answer("next ⚑"), Some(HitlDecision::Next));
    }

    #[test]
    fn receipt_orders_by_seq_not_wall() {
        let dir = std::env::temp_dir().join(format!("logos-hitl-{}", std::process::id()));
        let _ = fs::create_dir_all(dir.join("ops").join("marks"));
        let mut q = HumanActionQueue::default();
        let a = q.write_receipt(&dir, HitlDecision::Approve, "A1");
        let b = q.write_receipt(&dir, HitlDecision::Escalate, "A2");
        assert!(a.contains("seq=1"));
        assert!(b.contains("seq=2"));
        assert_eq!(q.seq, 2);
        let text = fs::read_to_string(dir.join("ops").join("marks").join("hitl-receipts.jsonl")).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("\"seq\":1"));
        assert!(lines[1].contains("\"seq\":2"));
        assert!(lines[0].contains(HITL_CONSTRAINT));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn repo_json_loads_a1_first() {
        let q = HumanActionQueue::load();
        assert!(
            q.len() >= 8,
            "expected seeded queue, got {} ({})",
            q.len(),
            q.loaded_from
        );
        let need = q.first_need().expect("first_need");
        assert_eq!(need.id, "A1");
        assert_eq!(need.who, Actor::Human);
    }
}
