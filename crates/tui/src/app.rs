//! TUI application state — owns all data rendered by [`crate::ui`].

use std::collections::VecDeque;

use reson8_forge_core::adapter::Provider;
use reson8_forge_core::bridge::{BridgeEvent, TelemetryPayload};
use crate::qr_meta::{residual_r, residual_zero_claim, RESIDUAL_EPS};
use reson8_forge_core::protocol::{LogEntry, LogLevel, LogSink, MemoryLogSink};
use reson8_forge_core::superskill::SuperskillEvent;
use reson8_forge_core::task::{Task, TaskPhase};

use crate::codes::CodesLab;
use crate::git_lab::{GitAction, GitSnapshot};
use crate::human_actions::{HitlDecision, HumanActionQueue};
use crate::layout_presets::LayoutKind;
use crate::lsp::{DiagnosticRow, DiagnosticSeverityUi, LspEvent, rows_from_diagnostics};
use crate::net_proxy::{NetAction, NetProxyState};
use crate::phase_evolution::{
    SequenceEngine, SequenceTick, SphinxGate, TAGLINE, PRODUCT_NAME,
};
use crate::qr_meta::{CircuitIntent, Effect};
use crate::lattice::LatticeSnapshot;
use crate::strands::{self, StrandBackend, StrandProbe};
use crate::surface::HostSurface;

// Re-export for main keybinds / external UI (must be `pub use` of public enum).
pub use crate::phase_evolution::SequenceKind;

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
    /// Selective proxy stack: Tor / i2pd / Privoxy / DNSCrypt / gaming clearnet.
    Net,
    /// SAIF outstanding human actions — request / escalate / approve.
    Actions,
    /// Observe-only git (guitar graph · fetch in ε).
    Git,
    /// Classical codes lab: Hexacode · Golay G24 · Reed–Muller.
    Codes,
}

impl FocusPanel {
    /// Full-ring next (ops layout). Prefer [`LayoutKind::next_focus`] at runtime.
    pub fn next(self) -> Self {
        match self {
            Self::Providers => Self::Tasks,
            Self::Tasks => Self::Braid,
            Self::Braid => Self::Net,
            Self::Net => Self::Actions,
            Self::Actions => Self::Git,
            Self::Git => Self::Phases,
            Self::Phases => Self::Logs,
            Self::Logs => Self::Formal,
            Self::Formal => Self::Codes,
            Self::Codes => Self::Tests,
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
    pub stage: crate::smoke::Stage,
    pub detail: String,
    pub category: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestStatus {
    Pending,
    Observed,
    DryRun,
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
    /// Residual \(R=\max(0,15-\alpha-\omega)\) — Category **C** telemetry only.
    pub residual_r: f64,
    /// \(R \le \varepsilon\) claim (lab / display). **Not** cert.pass / deploy-green.
    pub residual_zero_claim: bool,
    /// True once a Coherence payload has set α/ω (measured-ish under bridge).
    pub residual_has_sample: bool,
    /// Events drained this frame (H budget telemetry).
    pub last_drain_count: usize,
    /// Engine fan-out queue depth awaiting ε ack.
    pub engine_queue_depth: usize,
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
            residual_r: 15.0,
            residual_zero_claim: false,
            residual_has_sample: false,
            last_drain_count: 0,
            engine_queue_depth: 0,
        }
    }
}

impl BridgeState {
    /// Refresh residual from latched α/ω (Category C; never a hard gate).
    pub fn refresh_residual(&mut self) {
        if !self.residual_has_sample {
            // Prefer static braid Viviani peak as display seed until coherence arrives.
            return;
        }
        self.residual_r = residual_r(self.alpha as f64, self.omega as f64);
        self.residual_zero_claim = residual_zero_claim(self.alpha as f64, self.omega as f64);
    }

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
    /// cli | sdk | cli+sdk | miss
    pub backend: StrandBackend,
    pub detail: String,
    pub launch: String,
}

pub struct TaskEntry {
    pub id: String,
    pub kind: String,
    pub phase: TaskPhase,
}

fn provider_from_probe(p: &StrandProbe) -> ProviderStatus {
    ProviderStatus {
        provider: p.provider,
        healthy: p.backend.healthy(),
        label: p.label,
        backend: p.backend,
        detail: p.detail.clone(),
        launch: p.launch.clone(),
    }
}

// ---------------------------------------------------------------------------
// Top-level App state
// ---------------------------------------------------------------------------

pub struct App {
    pub running: bool,
    pub focus: FocusPanel,
    /// Active workspace layout (ops / formal / agent / monitor / quantum / minimal).
    pub layout: LayoutKind,
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
    /// In-frame operator guide (agent terminal + keys). Toggle with `?` / `h`.
    pub help_open: bool,
    /// Detected host frame (Claude Desktop / editor / WT / plain).
    pub surface: HostSurface,
    /// True when the operator pinned the layout via env rather than detection.
    pub layout_pinned: bool,
    /// Local proxy stack probe + controller (ops/net).
    pub net: NetProxyState,
    /// When true, popup confirm maps to a net stack action.
    pub net_menu_open: bool,
    /// SAIF / HITL queue (file-backed; in-session latch only).
    pub actions: HumanActionQueue,
    /// When true, popup confirm maps to a HITL decision.
    pub hitl_menu_open: bool,
    /// Observe-only git snapshot.
    pub git: GitSnapshot,
    /// Activator daily-ops kit (have / don't-have).
    pub kit: Vec<reson8_activator::CapProbe>,
    /// Hexacode / Golay / Reed–Muller interactive lab.
    pub codes: CodesLab,
    /// apps / cutiles / crates / kernels / ops presence (Category B).
    pub lattice: LatticeSnapshot,
    /// Bridge events queued for superskill engine — ε-phase only (QDI ack).
    pub engine_pending: VecDeque<BridgeEvent>,
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
        log_sink.emit(&LogEntry::new(
            LogLevel::Info,
            "hint",
            "Press ? for agent-terminal + panel guide (Tab / splits / claude|grok)",
        ));

        let surface = HostSurface::detect();
        let pinned = LayoutKind::from_env_explicit();
        let layout = pinned.unwrap_or_else(|| surface.default_layout());
        let mut app = Self {
            surface,
            layout_pinned: pinned.is_some(),
            running: true,
            focus: layout.primary_focus(),
            layout,
            providers: Vec::new(),
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
            help_open: false,
            net: NetProxyState::probe_fresh(),
            net_menu_open: false,
            actions: HumanActionQueue::load(),
            hitl_menu_open: false,
            git: GitSnapshot::probe(),
            kit: reson8_activator::probe_ops_caps(),
            codes: CodesLab::default(),
            lattice: LatticeSnapshot::probe(),
            engine_pending: VecDeque::with_capacity(256),
        };
        // Category C display seed (Viviani peak) until bridge coherence samples.
        app.bridge.alpha = 7;
        app.bridge.omega = 8;
        app.bridge.residual_has_sample = true;
        app.bridge.refresh_residual();
        app.refresh_strands();
        if let Some(need) = app.actions.first_need() {
            app.push_notification(
                NotifLevel::Warn,
                "first_need",
                format!("{} · [o] HITL · [u] gate · {}", need.summary_line(), app.actions.short()),
            );
        } else {
            app.push_notification(
                NotifLevel::Info,
                "hitl",
                format!("{} · no open ⚑", app.actions.short()),
            );
        }
        app.push_notification(
            NotifLevel::Info,
            "lattice",
            format!(
                "{} · {} · logos-activate / [A]",
                app.lattice.short(),
                app.lattice.interweave_short()
            ),
        );
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
            NotifLevel::Info,
            "agent",
            "Press ? · r refresh strands · host split → claude|grok|gcloud",
        );
        app.push_notification(
            NotifLevel::Warn,
            "formal",
            "LSP pane: B placeholders until lake serve / als attach",
        );
        let origin = if app.layout_pinned {
            "pinned by RESON8_LAYOUT".to_string()
        } else {
            format!("auto from {}", app.surface.label())
        };
        app.push_notification(
            NotifLevel::Info,
            "layout",
            format!(
                "{} — {origin} · [l] cycle · [1-6] jump",
                app.layout.label()
            ),
        );
        app
    }

    /// Switch workflow layout; snap focus into a visible panel.
    pub fn set_layout(&mut self, kind: LayoutKind) {
        self.layout = kind;
        if !kind.contains(self.focus) {
            self.focus = kind.primary_focus();
        }
        self.log_sink.emit(&LogEntry::new(
            LogLevel::Info,
            "layout",
            format!("{} — {}", kind.label(), kind.description()),
        ));
        self.push_notification(
            NotifLevel::Success,
            "layout",
            format!(
                "{} ({}/{}) — {}",
                kind.label(),
                kind.index() + 1,
                crate::layout_presets::LayoutKind::ALL.len(),
                kind.id()
            ),
        );
    }

    pub fn cycle_layout(&mut self) {
        self.set_layout(self.layout.next());
    }

    pub fn cycle_layout_prev(&mut self) {
        self.set_layout(self.layout.prev());
    }

    /// Re-probe apps/cutiles/crates/kernels/ops + sibling interweave (no cargo).
    pub fn refresh_lattice(&mut self) {
        self.lattice = LatticeSnapshot::probe();
        let miss = self.lattice.missing_ids();
        let body = if miss.is_empty() {
            format!(
                "{} · {}",
                self.lattice.short(),
                self.lattice.interweave_short()
            )
        } else {
            format!("{} missing {}", self.lattice.short(), miss.join(","))
        };
        self.log_sink.emit(&LogEntry::new(LogLevel::Info, "lattice", body.clone()));
        self.push_notification(
            if self.lattice.all_ready() {
                NotifLevel::Success
            } else {
                NotifLevel::Warn
            },
            "lattice",
            body,
        );
    }

    /// Re-probe host CLIs / API-key flags (no paid calls).
    pub fn refresh_strands(&mut self) {
        let probes = strands::probe_all();
        self.providers = probes.iter().map(provider_from_probe).collect();
        let summary = strands::probe_summary(&probes);
        self.log_sink.emit(&LogEntry::new(
            LogLevel::Info,
            "strands",
            format!("probe {summary}"),
        ));
        for p in &probes {
            if p.backend.healthy() {
                self.log_sink.emit(&LogEntry::new(
                    LogLevel::Info,
                    "strand",
                    format!("{} [{}] {} · launch: {}", p.label, p.backend.short(), p.detail, p.launch),
                ));
            }
        }
        self.push_notification(NotifLevel::Info, "strands", summary);
    }

    /// Log launch recipes for healthy strands (operator copies into WT split).
    pub fn log_strand_launches(&mut self) {
        self.log_sink.emit(&LogEntry::new(
            LogLevel::Info,
            "strands",
            "=== launch recipes (paste into host split; no auto-API spend) ===",
        ));
        for p in &self.providers {
            self.log_sink.emit(&LogEntry::new(
                LogLevel::Info,
                p.label,
                format!("[{}] {} → {}", p.backend.short(), p.detail, p.launch),
            ));
        }
        self.push_notification(
            NotifLevel::Success,
            "strands",
            "Launch recipes written to Logs — open WT split & paste",
        );
    }

    pub fn toggle_help(&mut self) {
        self.help_open = !self.help_open;
        if self.help_open {
            self.push_notification(NotifLevel::Info, "help", "Agent terminal guide open — Esc or ? to close");
        }
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
        // Sequences need the Phases panel — land on Quantum layout.
        self.focus_panel(FocusPanel::Phases);
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
        crate::smoke::CATALOG
            .iter()
            .map(|s| TestCase {
                id: s.id.into(),
                name: s.name.into(),
                crate_name: s.crate_name.into(),
                status: TestStatus::Pending,
                stage: crate::smoke::Stage::Observe,
                detail: s.dry_run.into(),
                category: 'D',
            })
            .collect()
    }

    pub fn cycle_focus(&mut self) {
        // Don't steal focus while a blocking popup is open.
        if self.popup.as_ref().is_some_and(|p| p.blocking) {
            return;
        }
        self.focus = self.layout.next_focus(self.focus);
    }

    /// Focus a panel if visible in the current layout; otherwise switch layout that shows it.
    pub fn focus_panel(&mut self, panel: FocusPanel) {
        if !self.layout.contains(panel) {
            // Prefer a layout that includes the target without leaving the workflow family.
            let pick = match panel {
                FocusPanel::Formal => LayoutKind::Formal,
                FocusPanel::Phases => LayoutKind::Quantum,
                FocusPanel::Codes => LayoutKind::Codes,
                FocusPanel::Actions => LayoutKind::Hitl,
                FocusPanel::Git => LayoutKind::Monitor,
                FocusPanel::Tests | FocusPanel::Net => LayoutKind::Ops,
                FocusPanel::Providers | FocusPanel::Tasks => LayoutKind::Ops,
                _ => LayoutKind::Ops,
            };
            if pick.contains(panel) {
                self.layout = pick;
            } else if let Some(k) = LayoutKind::ALL.iter().copied().find(|k| k.contains(panel)) {
                self.layout = k;
            }
        }
        self.focus = panel;
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
        self.bridge.engine_queue_depth = self.engine_pending.len();
        self.bridge.refresh_residual();
    }

    /// Queue bridge event for ε-phase superskill fan-out (never await in H).
    pub fn enqueue_engine_bridge(&mut self, ev: BridgeEvent) {
        if self.engine_pending.len() >= 256 {
            self.engine_pending.pop_front();
        }
        self.engine_pending.push_back(ev);
        self.bridge.engine_queue_depth = self.engine_pending.len();
    }

    /// Residual ε label for widgets (Category C — never deploy-green alone).
    pub fn residual_eps() -> f64 {
        RESIDUAL_EPS
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

    /// Confirm popup; if it was the net menu, return the mapped [`NetAction`].
    pub fn confirm_popup_net(&mut self) -> Option<NetAction> {
        let was_net = self.net_menu_open;
        self.confirm_popup();
        self.net_menu_open = false;
        if !was_net {
            return None;
        }
        self.last_popup_answer
            .as_deref()
            .and_then(NetAction::from_popup_answer)
    }

    pub fn open_net_menu(&mut self) {
        self.net_menu_open = true;
        self.focus_panel(FocusPanel::Net);
        self.open_urgent(
            "Net Proxy Stack",
            "Selective routing — gaming = clearnet; privacy = Tor/i2pd/Privoxy/DNSCrypt:",
            NetAction::popup_options(),
        );
    }

    pub fn refresh_net(&mut self) {
        self.net.refresh_probe();
        self.push_notification(NotifLevel::Info, "net", self.net.last_message.clone());
        self.log_sink.emit(&LogEntry::new(
            LogLevel::Info,
            "net",
            self.net.summary_line(),
        ));
    }

    /// Re-load the SAIF queue (JSON preferred, markdown fallback).
    pub fn refresh_actions(&mut self) {
        let seq = self.actions.seq;
        let selected_id = self.actions.selected().map(|a| a.id.clone());
        self.actions = HumanActionQueue::load();
        self.actions.seq = seq;
        if let Some(id) = selected_id {
            if let Some(i) = self.actions.items.iter().position(|a| a.id == id) {
                self.actions.selected = i;
            }
        }
        let body = match self.actions.first_need() {
            Some(need) => format!(
                "{} from {} · first_need {}",
                self.actions.short(),
                self.actions.loaded_from,
                need.summary_line()
            ),
            None => format!("{} · no open ⚑", self.actions.short()),
        };
        self.log_sink.emit(&LogEntry::new(LogLevel::Info, "hitl", body.clone()));
        self.push_notification(NotifLevel::Info, "hitl", body);
        self.refresh_kit();
    }

    pub fn refresh_git(&mut self) {
        self.git.refresh();
        let body = self.git.reconcile_note();
        self.log_sink.emit(&LogEntry::new(LogLevel::Info, "git", body.clone()));
        self.push_notification(NotifLevel::Info, "git", body);
    }

    pub fn refresh_kit(&mut self) {
        self.kit = reson8_activator::probe_ops_caps();
    }

    /// Open the approval subroutine on the selected / first_need item.
    pub fn open_hitl_gate(&mut self) {
        self.actions.select_first_need();
        let (id, title, question) = match self.actions.selected() {
            Some(item) => (
                item.id.clone(),
                format!("HITL {} {}", item.id, item.who.glyph()),
                item.gate_question(),
            ),
            None => {
                self.open_urgent(
                    "HITL gate",
                    "Queue empty — check ops/human-actions.json / SAIF list.",
                    vec!["ok".into()],
                );
                return;
            }
        };
        let _ = self.actions.apply_decision(HitlDecision::Request, Some(&id));
        self.hitl_menu_open = true;
        self.net_menu_open = false;
        self.focus_panel(FocusPanel::Actions);
        self.open_urgent(title, question, HitlDecision::popup_options());
    }

    /// Confirm popup; if it was the HITL gate, apply the decision and emit ε receipt.
    pub fn confirm_popup_hitl(&mut self) -> Option<(HitlDecision, String)> {
        let was_hitl = self.hitl_menu_open;
        self.confirm_popup();
        self.hitl_menu_open = false;
        if !was_hitl {
            return None;
        }
        let decision = self
            .last_popup_answer
            .as_deref()
            .and_then(HitlDecision::from_popup_answer)?;
        let id = self
            .actions
            .apply_decision(decision, None)
            .unwrap_or_else(|| "—".into());
        if decision == HitlDecision::Next {
            return None;
        }
        Some((decision, id))
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

    /// Staged observe → dry-run → in-process execute. External crates stay DryRun.
    pub fn run_test_map(&mut self) {
        self.push_notification(
            NotifLevel::Info,
            "tests",
            "smoke: observe → dry-run → execute (in-process only)",
        );
        for t in self.tests.iter_mut() {
            t.status = TestStatus::Running;
        }

        let mut p = FocusPanel::Providers;
        let start = p;
        let mut hops = 0usize;
        loop {
            p = p.next();
            hops += 1;
            if p == start || hops > 16 {
                break;
            }
        }
        let extras = crate::smoke::Extras {
            focus_ring_ok: hops >= 8 && p == start,
            focus_ring_detail: format!("FocusPanel ring closed after {hops} hops"),
        };

        let reports = crate::smoke::run_staged(&self.lattice, &extras);
        self.tests = reports
            .iter()
            .map(|r| TestCase {
                id: r.spec.id.into(),
                name: r.spec.name.into(),
                crate_name: r.spec.crate_name.into(),
                status: match r.status {
                    crate::smoke::Status::Pending => TestStatus::Pending,
                    crate::smoke::Status::Observed => TestStatus::Observed,
                    crate::smoke::Status::DryRun => TestStatus::DryRun,
                    crate::smoke::Status::Running => TestStatus::Running,
                    crate::smoke::Status::Pass => TestStatus::Pass,
                    crate::smoke::Status::Fail => TestStatus::Fail,
                    crate::smoke::Status::Skip => TestStatus::Skip,
                },
                stage: r.stage,
                detail: r.detail.clone(),
                category: r.category,
            })
            .collect();

        let (pass, fail, dry, skip) = crate::smoke::tally(&reports);
        let level = if fail > 0 {
            NotifLevel::Warn
        } else {
            NotifLevel::Success
        };
        self.push_notification(
            level,
            "tests",
            format!("{pass} exec-pass / {fail} fail / {dry} dry-run / {skip} skip · {} total", self.tests.len()),
        );
        self.log_sink.emit(&LogEntry::new(
            LogLevel::Info,
            "smoke",
            format!("staged harness {pass}P {fail}F {dry}D {skip}S"),
        ));
        self.focus_panel(FocusPanel::Tests);
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

    /// RS-NOR set-line: blocking popup holds the key bus.
    pub fn popup_blocking(&self) -> bool {
        self.popup.as_ref().is_some_and(|p| p.blocking)
    }

    /// CNOT reduce: entangle [`CircuitIntent`] with `App` state.
    ///
    /// Pure of network I/O — pipeline engine work returns [`Effect::Side`] for
    /// the ε-phase in `main` (await after latch). Never opens residual-zero.
    pub fn apply_intent(&mut self, intent: CircuitIntent) -> Effect {
        match intent {
            CircuitIntent::Quit => {
                self.quit();
                Effect::None
            }
            CircuitIntent::CycleFocus => {
                self.cycle_focus();
                Effect::None
            }
            CircuitIntent::ToggleHelp => {
                self.toggle_help();
                Effect::None
            }
            CircuitIntent::GateStep => {
                if !self.sequence.active {
                    self.start_sequence(SequenceKind::GateTick);
                }
                self.gate_tick();
                Effect::None
            }
            CircuitIntent::FullSpiral => {
                self.start_sequence(SequenceKind::FullSpiral);
                for _ in 0..4 {
                    self.gate_tick();
                }
                Effect::None
            }
            CircuitIntent::FocusFormal => {
                self.focus_panel(FocusPanel::Formal);
                Effect::None
            }
            CircuitIntent::RefreshLattice => {
                self.refresh_lattice();
                Effect::None
            }
            CircuitIntent::FocusActions => {
                self.focus_panel(FocusPanel::Actions);
                self.push_notification(
                    NotifLevel::Info,
                    "hitl",
                    match self.actions.first_need() {
                        Some(n) => format!("{} · j/k select · u gate", n.summary_line()),
                        None => format!("{} · no open ⚑", self.actions.short()),
                    },
                );
                Effect::None
            }
            CircuitIntent::RefreshActions => {
                self.refresh_actions();
                Effect::None
            }
            CircuitIntent::FocusGit => {
                self.focus_panel(FocusPanel::Git);
                self.refresh_git();
                Effect::None
            }
            CircuitIntent::GitFetch => {
                self.focus_panel(FocusPanel::Git);
                Effect::Git {
                    action: GitAction::Fetch,
                }
            }
            CircuitIntent::FocusNet => {
                self.focus_panel(FocusPanel::Net);
                self.refresh_net();
                Effect::None
            }
            CircuitIntent::NetRefresh => {
                self.refresh_net();
                Effect::None
            }
            CircuitIntent::NetMenu => {
                self.open_net_menu();
                Effect::None
            }
            CircuitIntent::FocusCodes => {
                self.focus_panel(FocusPanel::Codes);
                self.push_notification(
                    NotifLevel::Info,
                    "codes",
                    format!("{} — d demo · D battery · y family", self.codes.family.label()),
                );
                Effect::None
            }
            CircuitIntent::CodesDemo => {
                self.codes.run_demo();
                self.focus_panel(FocusPanel::Codes);
                let lvl = if self.codes.last_ok {
                    NotifLevel::Success
                } else {
                    NotifLevel::Warn
                };
                self.push_notification(
                    lvl,
                    "codes",
                    format!(
                        "{} demo {}",
                        self.codes.family.label(),
                        if self.codes.last_ok { "OK" } else { "FAIL/DETECT" }
                    ),
                );
                self.log_sink.emit(&LogEntry::new(
                    LogLevel::Info,
                    "codes",
                    self.codes.lines.first().cloned().unwrap_or_default(),
                ));
                Effect::None
            }
            CircuitIntent::CodesDemoAll => {
                self.codes.run_all_demos();
                self.focus_panel(FocusPanel::Codes);
                self.push_notification(
                    NotifLevel::Success,
                    "codes",
                    "Hex · G24 · RM battery complete",
                );
                Effect::None
            }
            CircuitIntent::CodesCycleFamily => {
                self.codes.cycle_family();
                self.focus_panel(FocusPanel::Codes);
                self.push_notification(
                    NotifLevel::Info,
                    "codes",
                    self.codes.family.label(),
                );
                Effect::None
            }
            CircuitIntent::CodesBumpT => {
                self.codes.bump_error_t();
                self.focus_panel(FocusPanel::Codes);
                Effect::None
            }
            CircuitIntent::CodesRmRInc => {
                self.codes.bump_rm_r(1);
                self.focus_panel(FocusPanel::Codes);
                Effect::None
            }
            CircuitIntent::CodesRmRDec => {
                self.codes.bump_rm_r(-1);
                self.focus_panel(FocusPanel::Codes);
                Effect::None
            }
            CircuitIntent::CodesRmMInc => {
                self.codes.bump_rm_m(1);
                self.focus_panel(FocusPanel::Codes);
                Effect::None
            }
            CircuitIntent::CodesRmMDec => {
                self.codes.bump_rm_m(-1);
                self.focus_panel(FocusPanel::Codes);
                Effect::None
            }
            CircuitIntent::CycleLayout => {
                self.cycle_layout();
                Effect::None
            }
            CircuitIntent::CycleLayoutPrev => {
                self.cycle_layout_prev();
                Effect::None
            }
            CircuitIntent::LayoutOps => {
                self.set_layout(LayoutKind::Ops);
                Effect::None
            }
            CircuitIntent::LayoutFormal => {
                self.set_layout(LayoutKind::Formal);
                Effect::None
            }
            CircuitIntent::LayoutAgent => {
                self.set_layout(LayoutKind::Agent);
                Effect::None
            }
            CircuitIntent::LayoutMonitor => {
                self.set_layout(LayoutKind::Monitor);
                Effect::None
            }
            CircuitIntent::LayoutQuantum => {
                self.set_layout(LayoutKind::Quantum);
                Effect::None
            }
            CircuitIntent::LayoutMinimal => {
                self.set_layout(LayoutKind::Minimal);
                Effect::None
            }
            CircuitIntent::LayoutCodes => {
                self.set_layout(LayoutKind::Codes);
                Effect::None
            }
            CircuitIntent::LayoutHitl => {
                self.set_layout(LayoutKind::Hitl);
                Effect::None
            }
            CircuitIntent::TestMap => {
                self.start_sequence(SequenceKind::TestMap);
                self.run_test_map();
                for _ in 0..4 {
                    self.gate_tick();
                }
                Effect::None
            }
            CircuitIntent::UrgentPopup => {
                if self.actions.is_empty() {
                    self.open_urgent(
                        "Director Gate",
                        "Urgent decision required — choose path:",
                        vec![
                            "proceed".into(),
                            "defer".into(),
                            "abort".into(),
                            "open Gitea migrate".into(),
                        ],
                    );
                } else {
                    self.open_hitl_gate();
                }
                Effect::None
            }
            CircuitIntent::PaperDraft => {
                self.start_sequence(SequenceKind::PaperDraft);
                self.push_notification(
                    NotifLevel::Info,
                    "pipeline",
                    "paper-draft + phase spiral",
                );
                Effect::Side {
                    name: "paper-draft",
                }
            }
            CircuitIntent::IdeaToPublish => {
                self.start_sequence(SequenceKind::IdeaToPublish);
                self.push_notification(
                    NotifLevel::Info,
                    "pipeline",
                    "idea-to-publish + phase spiral",
                );
                Effect::Side {
                    name: "idea-to-publish",
                }
            }
            CircuitIntent::PopupConfirm => {
                if self.net_menu_open {
                    self.confirm_popup_net()
                        .map(|action| Effect::Net { action })
                        .unwrap_or(Effect::None)
                } else if self.hitl_menu_open {
                    self.confirm_popup_hitl()
                        .map(|(decision, action_id)| Effect::Hitl {
                            decision,
                            action_id,
                        })
                        .unwrap_or(Effect::None)
                } else {
                    self.confirm_popup();
                    Effect::None
                }
            }
            CircuitIntent::PopupDismiss => {
                self.net_menu_open = false;
                self.hitl_menu_open = false;
                self.dismiss_popup();
                Effect::None
            }
            CircuitIntent::PopupUp => {
                if self.popup.is_some() {
                    self.popup_select_prev();
                } else {
                    self.actions.select_prev();
                }
                Effect::None
            }
            CircuitIntent::PopupDown => {
                if self.popup.is_some() {
                    self.popup_select_next();
                } else {
                    self.actions.select_next();
                }
                Effect::None
            }
        }
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
                self.bridge.residual_has_sample = true;
                self.bridge.refresh_residual();
                self.log_sink.emit(&LogEntry::new(
                    LogLevel::Info,
                    "wave",
                    format!(
                        "Φ {:.3}  α{}+ω{}={} {}  R={:.5}{}",
                        p.wave_score,
                        p.conservation.alpha,
                        p.conservation.omega,
                        p.conservation.alpha + p.conservation.omega,
                        if self.bridge.conservation_valid { "✓" } else { "✗" },
                        self.bridge.residual_r,
                        if self.bridge.residual_zero_claim {
                            " ≤ε (lab claim C)"
                        } else {
                            " >ε"
                        },
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
                self.bridge.residual_has_sample = true;
                self.bridge.refresh_residual();
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
        assert_eq!(FocusPanel::Braid.next(), FocusPanel::Net);
        assert_eq!(FocusPanel::Net.next(), FocusPanel::Actions);
        assert_eq!(FocusPanel::Actions.next(), FocusPanel::Git);
        assert_eq!(FocusPanel::Git.next(), FocusPanel::Phases);
        assert_eq!(FocusPanel::Phases.next(), FocusPanel::Logs);
        assert_eq!(FocusPanel::Logs.next(), FocusPanel::Formal);
        assert_eq!(FocusPanel::Formal.next(), FocusPanel::Codes);
        assert_eq!(FocusPanel::Codes.next(), FocusPanel::Tests);
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
        assert!(
            app.tests
                .iter()
                .filter(|t| t.status == TestStatus::Pass)
                .all(|t| t.stage == crate::smoke::Stage::Execute),
            "Pass without Execute is a painted result"
        );
        assert!(app.tests.iter().any(|t| t.status == TestStatus::DryRun));
    }

    #[test]
    fn app_starts_running() {
        let app = App::new();
        assert!(app.running);
        assert_eq!(app.focus, app.layout.primary_focus());
        assert_eq!(app.providers.len(), 5);
        assert!(!app.log_sink.entries().is_empty());
    }

    #[test]
    fn layout_cycle_snaps_focus() {
        let mut app = App::new();
        app.set_layout(LayoutKind::Minimal);
        assert_eq!(app.layout, LayoutKind::Minimal);
        assert!(app.layout.contains(app.focus));
        app.cycle_layout();
        assert!(app.layout.contains(app.focus));
    }

    #[test]
    fn focus_cycles_within_layout() {
        let mut app = App::new();
        app.set_layout(LayoutKind::Minimal);
        app.focus = FocusPanel::Braid;
        app.cycle_focus();
        assert_eq!(app.focus, FocusPanel::Logs);
        app.cycle_focus();
        assert_eq!(app.focus, FocusPanel::Braid);
    }

    #[test]
    fn formal_key_path_focuses_formal_layout() {
        let mut app = App::new();
        app.set_layout(LayoutKind::Minimal);
        app.focus_panel(FocusPanel::Formal);
        assert_eq!(app.focus, FocusPanel::Formal);
        assert!(app.layout.contains(FocusPanel::Formal));
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
        use reson8_forge_core::bridge::{CoherenceComponents, CoherencePayload};
        use reson8_forge_core::protocol::ConservationState;
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

    #[test]
    fn apply_intent_full_spiral_and_pipeline_effect() {
        use crate::qr_meta::{CircuitIntent, Effect};
        let mut app = App::new();
        let e = app.apply_intent(CircuitIntent::FullSpiral);
        assert!(matches!(e, Effect::None));
        assert!(!app.sequence.last_done.is_empty() || app.sequence.cycle >= 1 || !app.sequence.active);

        let e = app.apply_intent(CircuitIntent::PaperDraft);
        assert!(matches!(e, Effect::Side { name: "paper-draft" }));
        // Gate ticks for paper-draft run in main after engine side-effect.
        assert!(app.sequence.active || app.sequence.kind.is_some());
    }

    #[test]
    fn apply_intent_refresh_lattice() {
        use crate::qr_meta::{CircuitIntent, Effect};
        let mut app = App::new();
        let e = app.apply_intent(CircuitIntent::RefreshLattice);
        assert!(matches!(e, Effect::None));
        assert!(app.lattice.short().starts_with("lat "));
        assert_eq!(app.lattice.layers.len(), 5);
    }

    #[test]
    fn apply_intent_hitl_gate_requests_a1() {
        use crate::human_actions::{HitlDecision, SessionStatus};
        use crate::qr_meta::{CircuitIntent, Effect};
        let mut app = App::new();
        assert!(!app.actions.is_empty());
        let e = app.apply_intent(CircuitIntent::UrgentPopup);
        assert!(matches!(e, Effect::None));
        assert!(app.hitl_menu_open);
        assert!(app.popup_blocking());
        assert_eq!(app.actions.items[0].status, SessionStatus::Requested);
        assert_eq!(app.actions.items[0].id, "A1");

        // Confirm first option = approve → ε receipt effect, no deploy.
        if let Some(p) = app.popup.as_mut() {
            p.selected = 0;
        }
        let e = app.apply_intent(CircuitIntent::PopupConfirm);
        match e {
            Effect::Hitl { decision, action_id } => {
                assert_eq!(decision, HitlDecision::Approve);
                assert_eq!(action_id, "A1");
            }
            other => panic!("expected Hitl effect, got {other:?}"),
        }
        assert_eq!(app.actions.items[0].status, SessionStatus::Approved);
        assert_eq!(app.actions.first_need().map(|a| a.id.as_str()), Some("A2"));
    }

    #[test]
    fn apply_intent_gate_step_advances() {
        use crate::qr_meta::{CircuitIntent, Effect};
        let mut app = App::new();
        let e = app.apply_intent(CircuitIntent::GateStep);
        assert!(matches!(e, Effect::None));
        assert!(app.sequence.active || app.sequence.cycle > 0 || !app.sequence.history.is_empty());
    }
}
