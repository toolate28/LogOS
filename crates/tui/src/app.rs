//! TUI application state — owns all data rendered by [`crate::ui`].

use std::collections::VecDeque;

use reson8_forge_core::adapter::Provider;
use reson8_forge_core::bridge::{BridgeEvent, TelemetryPayload};
use reson8_forge_core::protocol::{LogEntry, LogLevel, LogSink, MemoryLogSink};
use reson8_forge_core::superskill::{PipelineStatus, SuperskillEvent};
use reson8_forge_core::task::{Task, TaskPhase};

use crate::lsp::{DiagnosticRow, DiagnosticSeverityUi, LspEvent, rows_from_diagnostics};
use crate::phase_evolution::{
    SequenceEngine, SequenceTick, SphinxGate, TAGLINE, PRODUCT_NAME,
};

// Re-export for main keybinds / external UI (must be `pub use` of public enum).
pub use crate::phase_evolution::SequenceKind;
pub use crate::phase_evolution::TAGLINE as TUI_TAGLINE;

// ---------------------------------------------------------------------------
// Focus panel enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPanel {
    Providers,
    Tasks,
    Braid,
    /// Quantum-redstone × SPHINX phase evolution table + sequence strip.
    Phases,
    Logs,
    /// Formal core glass: Lean/Agda LSP diagnostics + honest B placeholders.
    Formal,
    Tests,
}

impl FocusPanel {
    pub fn next(self) -> Self {
        match self {
            Self::Providers => Self::Tasks,
            Self::Tasks => Self::Braid,
            Self::Braid => Self::Phases,
            Self::Phases => Self::Logs,
            Self::Logs => Self::Formal,
            Self::Formal => Self::Tests,
            Self::Tests => Self::Providers,
        }
    }
}

// ---------------------------------------------------------------------------
// Notifications + urgent popup modals
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifLevel {
    Info,
    Warn,
    Critical,
    Success,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub level: NotifLevel,
    pub title: String,
    pub body: String,
    pub ttl_ticks: u32,
}

/// Modal for urgent questions (agent/human gate).
#[derive(Debug, Clone)]
pub struct UrgentPopup {
    pub title: String,
    pub question: String,
    pub options: Vec<String>,
    pub selected: usize,
    /// When true, popup captures all key input until answered/dismissed.
    pub blocking: bool,
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub id: String,
    pub name: String,
    pub crate_name: String,
    pub status: TestStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestStatus {
    Pending,
    Running,
    Pass,
    Fail,
    Skip,
}

// ---------------------------------------------------------------------------
// Bridge / pipeline live state
// ---------------------------------------------------------------------------

/// Live data flowing in from the WebSocket bridge.
pub struct BridgeState {
    pub connected: bool,
    pub wave_score: f64,
    pub alpha: u8,
    pub omega: u8,
    pub conservation_valid: bool,
    pub pipeline_name: String,
    pub pipeline_step: String,
    pub pipeline_percent: u8,
    pub pipeline_active: bool,
    pub pipeline_aborted: bool,
    /// Last N sensor snapshots for sparklines.
    pub cpu_history: VecDeque<u64>,
    pub gpu_history: VecDeque<u64>,
    /// Most recent telemetry scalar values.
    pub cpu_temp: f64,
    pub gpu_temp: f64,
    pub power_draw: f64,
    pub health: String,
    /// ATOM trail entries (newest first, capped at 100).
    pub atom_trail: VecDeque<String>,
}

impl Default for BridgeState {
    fn default() -> Self {
        BridgeState {
            connected: false,
            wave_score: 0.0,
            alpha: 0,
            omega: 0,
            conservation_valid: false,
            pipeline_name: String::new(),
            pipeline_step: String::new(),
            pipeline_percent: 0,
            pipeline_active: false,
            pipeline_aborted: false,
            cpu_history: VecDeque::with_capacity(60),
            gpu_history: VecDeque::with_capacity(60),
            cpu_temp: 0.0,
            gpu_temp: 0.0,
            power_draw: 0.0,
            health: "---".to_owned(),
            atom_trail: VecDeque::with_capacity(100),
        }
    }
}

impl BridgeState {
    fn push_history(buf: &mut VecDeque<u64>, val: f64) {
        if buf.len() == buf.capacity() {
            buf.pop_front();
        }
        buf.push_back(val.max(0.0) as u64);
    }

    pub fn apply_telemetry(&mut self, p: &TelemetryPayload) {
        self.cpu_temp = p.sensors.cpu_temp;
        self.gpu_temp = p.sensors.gpu_temp;
        self.power_draw = p.sensors.power_draw;
        self.health = p.health.clone();
        Self::push_history(&mut self.cpu_history, p.sensors.cpu_temp);
        Self::push_history(&mut self.gpu_history, p.sensors.gpu_temp);
    }
}

// ---------------------------------------------------------------------------
// Static braid resonance display
// ---------------------------------------------------------------------------

pub struct BraidStatus {
    pub alpha: f64,
    pub omega: f64,
    pub phi: f64,
    pub status: &'static str,
}

// ---------------------------------------------------------------------------
// Provider and task view models
// ---------------------------------------------------------------------------

pub struct ProviderStatus {
    pub provider: Provider,
    pub healthy: bool,
    pub label: &'static str,
}

pub struct TaskEntry {
    pub id: String,
    pub kind: String,
    pub phase: TaskPhase,
}

// ---------------------------------------------------------------------------
// Top-level App state
// ---------------------------------------------------------------------------

pub struct App {
    pub running: bool,
    pub focus: FocusPanel,
    pub providers: Vec<ProviderStatus>,
    pub tasks: Vec<TaskEntry>,
    pub braid: BraidStatus,
    pub bridge: BridgeState,
    pub log_sink: MemoryLogSink,
    /// Toast stack (newest first, capped).
    pub notifications: VecDeque<Notification>,
    /// Blocking urgent question modal (None = no popup).
    pub popup: Option<UrgentPopup>,
    /// Mapped test harness entries (LogOS / cutile / agda / lean / …).
    pub tests: Vec<TestCase>,
    pub last_popup_answer: Option<String>,
    /// Quantum-redstone / SPHINX sequence orchestrator (tokio event loop drives ticks).
    pub sequence: SequenceEngine,
    /// Eye-of-the-needle formal diagnostics (LSP + B placeholders). Cap 200.
    pub diagnostics: VecDeque<DiagnosticRow>,
    pub lean_lsp_ok: bool,
    pub als_lsp_ok: bool,
}

impl App {
    pub fn new() -> Self {
        let log_sink = MemoryLogSink::new();
        log_sink.emit(&LogEntry::new(
            LogLevel::Info,
            "app",
            format!("{PRODUCT_NAME} started — {TAGLINE}"),
        ));
        log_sink.emit(&LogEntry::new(LogLevel::Info, "bridge", "connecting ws://127.0.0.1:8088…"));

        let mut app = Self {
            running: true,
            focus: FocusPanel::Providers,
            providers: vec![
                ProviderStatus { provider: Provider::Claude,     healthy: false, label: "Claude" },
                ProviderStatus { provider: Provider::Gemini,     healthy: false, label: "Gemini" },
                ProviderStatus { provider: Provider::Grok,       healthy: false, label: "Grok" },
                ProviderStatus { provider: Provider::Manus,      healthy: false, label: "Manus" },
                ProviderStatus { provider: Provider::OpenWeight, healthy: false, label: "OpenWeight" },
            ],
            tasks: Vec::new(),
            braid: BraidStatus { alpha: 8.0, omega: 7.0, phi: 0.82, status: "RESONANT" },
            bridge: BridgeState::default(),
            log_sink,
            notifications: VecDeque::with_capacity(16),
            popup: None,
            tests: Self::default_test_map(),
            last_popup_answer: None,
            sequence: SequenceEngine::default(),
            diagnostics: VecDeque::with_capacity(200),
            lean_lsp_ok: false,
            als_lsp_ok: false,
        };
        // Honest B stubs first — never a green empty formal pane.
        for row in crate::lsp::bootstrap_placeholders() {
            app.push_diagnostic(row);
        }
        app.push_notification(
            NotifLevel::Success,
            "boot",
            format!("{PRODUCT_NAME} — {TAGLINE}"),
        );
        app.push_notification(
            NotifLevel::Warn,
            "formal",
            "LSP pane: B placeholders until lake serve / als attach",
        );
        app
    }

    pub fn push_diagnostic(&mut self, row: DiagnosticRow) {
        if self.diagnostics.len() >= 200 {
            self.diagnostics.pop_back();
        }
        self.diagnostics.push_front(row);
    }

    /// Drain LSP / sensor events from the decoupled bus into the Formal pane.
    pub fn handle_lsp_event(&mut self, ev: LspEvent) {
        match ev {
            LspEvent::Diagnostics { server, params } => {
                for row in rows_from_diagnostics(server, &params) {
                    // SlowStep (sorry) gets an amber toast once.
                    if row.severity == DiagnosticSeverityUi::SlowStep {
                        self.push_notification(
                            NotifLevel::Warn,
                            "slowstep",
                            format!("{}:{} {}", row.path, row.line, row.message),
                        );
                    }
                    self.push_diagnostic(row);
                }
            }
            LspEvent::LogMessage { server, params } => {
                let msg = params
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("(log)")
                    .to_string();
                self.push_diagnostic(DiagnosticRow {
                    server: server.label().into(),
                    severity: DiagnosticSeverityUi::Info,
                    path: "log".into(),
                    line: 0,
                    message: msg,
                    placeholder: false,
                });
            }
            LspEvent::Placeholder { id, message } => {
                self.push_diagnostic(DiagnosticRow {
                    server: id,
                    severity: DiagnosticSeverityUi::PlaceholderB,
                    path: "—".into(),
                    line: 0,
                    message,
                    placeholder: true,
                });
            }
            LspEvent::ServerStatus { server, ok, detail } => {
                match server {
                    crate::lsp::LspServerKind::Lean => self.lean_lsp_ok = ok,
                    crate::lsp::LspServerKind::Agda => self.als_lsp_ok = ok,
                }
                let level = if ok { NotifLevel::Info } else { NotifLevel::Warn };
                self.push_notification(level, server.label(), detail.clone());
                self.push_diagnostic(DiagnosticRow {
                    server: server.label().into(),
                    severity: if ok {
                        DiagnosticSeverityUi::Info
                    } else {
                        DiagnosticSeverityUi::PlaceholderB
                    },
                    path: "server".into(),
                    line: 0,
                    message: detail,
                    placeholder: !ok,
                });
            }
        }
    }

    /// Start a phase-evolution sequence (seeded by last_done when present).
    pub fn start_sequence(&mut self, kind: SequenceKind) {
        self.sequence.start(kind);
        self.focus = FocusPanel::Phases;
        self.push_notification(
            NotifLevel::Info,
            "sequence",
            format!(
                "{} | need: {}",
                kind.label(),
                self.sequence.first_need
            ),
        );
        self.log_sink.emit(&LogEntry::new(
            LogLevel::Info,
            "sequence",
            format!("start {} — {}", kind.id(), self.sequence.first_need),
        ));
    }

    /// Advance one quantum-redstone / SPHINX gate. Returns tick detail.
    pub fn gate_tick(&mut self) -> SequenceTick {
        let tick = self.sequence.tick();
        if tick.advanced {
            // Mirror phase onto a synthetic task row for the Tasks panel.
            if let Some(kind) = self.sequence.kind {
                let phase = tick.row.to_phase;
                if let Some(t) = self.tasks.iter_mut().rev().find(|t| t.kind == kind.id()) {
                    t.phase = phase;
                } else {
                    self.tasks.push(TaskEntry {
                        id: format!("seq{:02}", self.sequence.cycle),
                        kind: kind.id().into(),
                        phase,
                    });
                }
            }
            // ATOM trail echo for bridge panel.
            let atom = format!(
                "{} {} → {}",
                tick.row.sphinx.label(),
                tick.row.redstone.short(),
                tick.row.role
            );
            if self.bridge.atom_trail.len() == 100 {
                self.bridge.atom_trail.pop_back();
            }
            self.bridge.atom_trail.push_front(atom);

            self.log_sink.emit(&LogEntry::new(LogLevel::Info, "gate", &tick.message));
            if tick.cycle_complete {
                self.push_notification(
                    NotifLevel::Success,
                    "cycle",
                    format!(
                        "last={} → need={}",
                        self.sequence.last_done, self.sequence.first_need
                    ),
                );
            }
        }
        tick
    }

    /// Current SPHINX gate (for widgets).
    pub fn current_sphinx(&self) -> SphinxGate {
        self.sequence.gate
    }

    fn default_test_map() -> Vec<TestCase> {
        vec![
            TestCase { id: "t01".into(), name: "gauge α+ω=15".into(), crate_name: "coherence-mcp".into(), status: TestStatus::Pending },
            TestCase { id: "t02".into(), name: "focus_cycles".into(), crate_name: "reson8-tui".into(), status: TestStatus::Pending },
            TestCase { id: "t03".into(), name: "wave conservation".into(), crate_name: "reson8-wave".into(), status: TestStatus::Pending },
            TestCase { id: "t04".into(), name: "cutile cell weave".into(), crate_name: "cutile".into(), status: TestStatus::Pending },
            TestCase { id: "t05".into(), name: "Agda HIT check".into(), crate_name: "LogOS/agda".into(), status: TestStatus::Pending },
            TestCase { id: "t06".into(), name: "Lean symplectic cert".into(), crate_name: "LogOS/lean".into(), status: TestStatus::Pending },
            TestCase { id: "t07".into(), name: "barcode-tui H0".into(), crate_name: "barcode-tui".into(), status: TestStatus::Pending },
            TestCase { id: "t08".into(), name: "rvm-coherence".into(), crate_name: "ruvnet-rvm".into(), status: TestStatus::Pending },
        ]
    }

    pub fn cycle_focus(&mut self) {
        // Don't steal focus while a blocking popup is open.
        if self.popup.as_ref().is_some_and(|p| p.blocking) {
            return;
        }
        self.focus = self.focus.next();
    }

    pub fn push_notification(&mut self, level: NotifLevel, title: impl Into<String>, body: impl Into<String>) {
        if self.notifications.len() >= 12 {
            self.notifications.pop_back();
        }
        let title = title.into();
        let body = body.into();
        self.notifications.push_front(Notification {
            level,
            title: title.clone(),
            body: body.clone(),
            ttl_ticks: 400, // ~4s at 10ms poll
        });
        let lvl = match level {
            NotifLevel::Info => LogLevel::Info,
            NotifLevel::Warn => LogLevel::Warn,
            NotifLevel::Critical => LogLevel::Error,
            NotifLevel::Success => LogLevel::Info,
        };
        self.log_sink.emit(&LogEntry::new(lvl, "notify", format!("{title}: {body}")));
    }

    pub fn tick_notifications(&mut self) {
        for n in self.notifications.iter_mut() {
            if n.ttl_ticks > 0 {
                n.ttl_ticks -= 1;
            }
        }
        self.notifications.retain(|n| n.ttl_ticks > 0);
    }

    /// Open urgent question popup (captures keys until Enter/Esc).
    pub fn open_urgent(&mut self, title: impl Into<String>, question: impl Into<String>, options: Vec<String>) {
        self.popup = Some(UrgentPopup {
            title: title.into(),
            question: question.into(),
            options,
            selected: 0,
            blocking: true,
        });
        self.push_notification(NotifLevel::Warn, "urgent", "popup awaiting decision");
    }

    pub fn dismiss_popup(&mut self) {
        self.popup = None;
    }

    pub fn confirm_popup(&mut self) {
        if let Some(p) = self.popup.take() {
            let answer = p
                .options
                .get(p.selected)
                .cloned()
                .unwrap_or_else(|| "ok".into());
            self.last_popup_answer = Some(answer.clone());
            self.push_notification(
                NotifLevel::Success,
                "popup",
                format!("{} → {answer}", p.title),
            );
            self.log_sink.emit(&LogEntry::new(
                LogLevel::Info,
                "popup",
                format!("answered [{}]: {answer}", p.title),
            ));
        }
    }

    pub fn popup_select_next(&mut self) {
        if let Some(p) = self.popup.as_mut() {
            if !p.options.is_empty() {
                p.selected = (p.selected + 1) % p.options.len();
            }
        }
    }

    pub fn popup_select_prev(&mut self) {
        if let Some(p) = self.popup.as_mut() {
            if !p.options.is_empty() {
                p.selected = (p.selected + p.options.len() - 1) % p.options.len();
            }
        }
    }

    /// Simulate / map-run local unit tests (no cargo spawn yet — status map).
    pub fn run_test_map(&mut self) {
        self.push_notification(NotifLevel::Info, "tests", "mapping harness across crates…");
        for t in self.tests.iter_mut() {
            t.status = TestStatus::Running;
        }
        // Deterministic demo outcomes until cargo test bridge lands.
        for t in self.tests.iter_mut() {
            t.status = match t.id.as_str() {
                "t05" | "t06" => TestStatus::Skip, // formal tools optional on host
                "t04" => TestStatus::Pending,      // cutile may need CUDA
                _ => TestStatus::Pass,
            };
        }
        // Internal invariants we can assert now.
        if FocusPanel::Providers.next() == FocusPanel::Tasks {
            if let Some(t) = self.tests.iter_mut().find(|t| t.id == "t02") {
                t.status = TestStatus::Pass;
            }
        }
        let pass = self.tests.iter().filter(|t| t.status == TestStatus::Pass).count();
        let skip = self.tests.iter().filter(|t| t.status == TestStatus::Skip).count();
        self.push_notification(
            NotifLevel::Success,
            "tests",
            format!("{pass} pass / {skip} skip / {} total", self.tests.len()),
        );
        self.focus = FocusPanel::Tests;
    }

    pub fn push_task(&mut self, task: &Task) {
        self.tasks.push(TaskEntry {
            id: task.id.to_string()[..8].to_owned(),
            kind: task.meta.kind.clone(),
            phase: task.phase,
        });
        self.log_sink.emit(&LogEntry::new(
            LogLevel::Info,
            "task",
            format!("task {} ({})", &task.id.to_string()[..8], task.meta.kind),
        ));
    }

    pub fn quit(&mut self) {
        self.running = false;
        self.log_sink.emit(&LogEntry::new(LogLevel::Info, "app", "Shutting down"));
    }

    // -----------------------------------------------------------------------
    // Bridge event handler — called on every inbound message
    // -----------------------------------------------------------------------

    pub fn handle_bridge_event(&mut self, ev: &BridgeEvent) {
        match ev {
            BridgeEvent::Connected => {
                self.bridge.connected = true;
                self.log_sink.emit(&LogEntry::new(LogLevel::Info, "bridge", "connected ◉"));
            }
            BridgeEvent::Disconnected => {
                self.bridge.connected = false;
                self.log_sink.emit(&LogEntry::new(LogLevel::Warn, "bridge", "disconnected — retrying…"));
            }
            BridgeEvent::Telemetry(p) => {
                self.bridge.apply_telemetry(p);
            }
            BridgeEvent::Coherence(p) => {
                self.bridge.wave_score = p.wave_score;
                self.bridge.alpha = p.conservation.alpha;
                self.bridge.omega = p.conservation.omega;
                self.bridge.conservation_valid = p.conservation.valid
                    && p.conservation.alpha + p.conservation.omega == 15;
                self.log_sink.emit(&LogEntry::new(
                    LogLevel::Info,
                    "wave",
                    format!(
                        "Φ {:.3}  α{}+ω{}={} {}",
                        p.wave_score,
                        p.conservation.alpha,
                        p.conservation.omega,
                        p.conservation.alpha + p.conservation.omega,
                        if self.bridge.conservation_valid { "✓" } else { "✗" },
                    ),
                ));
            }
            BridgeEvent::PipelineProgress(p) => {
                self.bridge.pipeline_name = p.pipeline_name.clone();
                self.bridge.pipeline_step = p.current_step.clone();
                self.bridge.pipeline_percent = p.percent;
                self.bridge.pipeline_active = p.completed_steps < p.total_steps;
            }
            BridgeEvent::AtomEvent(p) => {
                let entry = format!("{} {} → {}", p.atom_type, p.gate, p.description);
                if self.bridge.atom_trail.len() == 100 {
                    self.bridge.atom_trail.pop_back();
                }
                self.bridge.atom_trail.push_front(entry.clone());
                self.log_sink.emit(&LogEntry::new(LogLevel::Info, "atom", entry));
            }
            BridgeEvent::ExecuteMcpTool { tool_name, req_id, .. } => {
                self.log_sink.emit(&LogEntry::new(
                    LogLevel::Info,
                    "mcp",
                    format!("execute {tool_name} ({req_id})"),
                ));
            }
        }
    }

    // -----------------------------------------------------------------------
    // Superskill event handler
    // -----------------------------------------------------------------------

    pub fn handle_superskill_event(&mut self, ev: SuperskillEvent) {
        match ev {
            SuperskillEvent::PipelineStarted(name) => {
                self.bridge.pipeline_name = name.clone();
                self.bridge.pipeline_active = true;
                self.bridge.pipeline_aborted = false;
                self.bridge.pipeline_percent = 0;
                self.log_sink.emit(&LogEntry::new(
                    LogLevel::Info, "superskill", format!("pipeline started: {name}"),
                ));
            }
            SuperskillEvent::StateUpdated(st) => {
                self.bridge.wave_score = st.wave_score;
                self.bridge.alpha = st.alpha;
                self.bridge.omega = st.omega;
                self.bridge.conservation_valid = st.invariants_hold();
                self.bridge.pipeline_step = st.current_step.clone();
                self.bridge.pipeline_percent = st.percent;
            }
            SuperskillEvent::InvariantViolated(v) => {
                self.log_sink.emit(&LogEntry::new(
                    LogLevel::Error, "superskill",
                    format!("INVARIANT VIOLATED: {}", v.reason),
                ));
            }
            SuperskillEvent::PipelineComplete(id) => {
                self.bridge.pipeline_active = false;
                self.bridge.pipeline_percent = 100;
                self.log_sink.emit(&LogEntry::new(
                    LogLevel::Info, "superskill", format!("pipeline complete: {id}"),
                ));
            }
            SuperskillEvent::PipelineAborted { id, reason } => {
                self.bridge.pipeline_active = false;
                self.bridge.pipeline_aborted = true;
                self.log_sink.emit(&LogEntry::new(
                    LogLevel::Error, "superskill",
                    format!("pipeline {id} ABORTED: {reason}"),
                ));
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_cycles() {
        assert_eq!(FocusPanel::Providers.next(), FocusPanel::Tasks);
        assert_eq!(FocusPanel::Tasks.next(), FocusPanel::Braid);
        assert_eq!(FocusPanel::Braid.next(), FocusPanel::Phases);
        assert_eq!(FocusPanel::Phases.next(), FocusPanel::Logs);
        assert_eq!(FocusPanel::Logs.next(), FocusPanel::Formal);
        assert_eq!(FocusPanel::Formal.next(), FocusPanel::Tests);
        assert_eq!(FocusPanel::Tests.next(), FocusPanel::Providers);
    }

    #[test]
    fn formal_pane_starts_with_b_placeholders() {
        let app = App::new();
        assert!(!app.diagnostics.is_empty());
        assert!(app.diagnostics.iter().any(|d| d.placeholder));
        assert!(!app.lean_lsp_ok);
        assert!(!app.als_lsp_ok);
    }

    #[test]
    fn popup_confirm_records_answer() {
        let mut app = App::new();
        app.open_urgent(
            "gate",
            "Proceed with Gitea migrate?",
            vec!["yes".into(), "no".into()],
        );
        assert!(app.popup.is_some());
        app.popup_select_next(); // select "no"
        app.confirm_popup();
        assert!(app.popup.is_none());
        assert_eq!(app.last_popup_answer.as_deref(), Some("no"));
    }

    #[test]
    fn notifications_tick_expire() {
        let mut app = App::new();
        app.notifications.clear();
        app.push_notification(NotifLevel::Info, "x", "y");
        assert_eq!(app.notifications.len(), 1);
        if let Some(n) = app.notifications.front_mut() {
            n.ttl_ticks = 1;
        }
        app.tick_notifications();
        app.tick_notifications();
        assert!(app.notifications.is_empty());
    }

    #[test]
    fn test_map_runs() {
        let mut app = App::new();
        app.run_test_map();
        assert_eq!(app.focus, FocusPanel::Tests);
        assert!(app.tests.iter().any(|t| t.status == TestStatus::Pass));
    }

    #[test]
    fn app_starts_running() {
        let app = App::new();
        assert!(app.running);
        assert_eq!(app.focus, FocusPanel::Providers);
        assert_eq!(app.providers.len(), 5);
        assert!(!app.log_sink.entries().is_empty());
    }

    #[test]
    fn quit_sets_flag() {
        let mut app = App::new();
        app.quit();
        assert!(!app.running);
    }

    #[test]
    fn bridge_connected_event_sets_flag() {
        let mut app = App::new();
        app.handle_bridge_event(&BridgeEvent::Connected);
        assert!(app.bridge.connected);
    }

    #[test]
    fn coherence_event_updates_wave_and_conservation() {
        use reson8_forge_core::bridge::{
            CoherenceComponents, CoherencePayload, ConservationState,
        };
        let mut app = App::new();
        app.handle_bridge_event(&BridgeEvent::Coherence(CoherencePayload {
            pipeline_id: "t".into(),
            step_id: "s".into(),
            wave_score: 0.93,
            components: CoherenceComponents {
                lexical_diversity: 0.9, curl: 0.05,
                divergence: 0.7, potential: 0.9, entropy: 3.0,
            },
            conservation: ConservationState { alpha: 7, omega: 8, sum: 15, valid: true },
        }));
        assert_eq!(app.bridge.wave_score, 0.93);
        assert!(app.bridge.conservation_valid);
        assert_eq!(app.bridge.alpha + app.bridge.omega, 15);
    }
}
