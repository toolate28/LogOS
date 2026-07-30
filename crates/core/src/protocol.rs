//! Core protocol types — logging, WAVE scoring, ATOM trail, conservation state.

use std::fmt;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Bus message types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageKind {
    Intent,
    Status,
    Data,
    Command,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub source: Uuid,
    pub target: Option<Uuid>,
    pub kind: MessageKind,
    pub payload: serde_json::Value,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMeta {
    pub origin: String,
    pub kind: String,
    pub description: String,
}

// ---------------------------------------------------------------------------
// Log system
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trace => write!(f, "TRACE"),
            Self::Debug => write!(f, "DEBUG"),
            Self::Info  => write!(f, "INFO"),
            Self::Warn  => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub level: LogLevel,
    pub source: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl LogEntry {
    pub fn new(level: LogLevel, source: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level,
            source: source.into(),
            message: message.into(),
            timestamp: Utc::now(),
        }
    }
}

/// Trait for anything that can receive log entries.
pub trait LogSink: Send + Sync {
    fn emit(&self, entry: &LogEntry);
}

/// In-memory log sink backed by a shared `Vec<LogEntry>`.
#[derive(Clone)]
pub struct MemoryLogSink {
    entries: Arc<Mutex<Vec<LogEntry>>>,
}

impl MemoryLogSink {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn entries(&self) -> Vec<LogEntry> {
        self.entries.lock().unwrap().clone()
    }
}

impl Default for MemoryLogSink {
    fn default() -> Self {
        Self::new()
    }
}

impl LogSink for MemoryLogSink {
    fn emit(&self, entry: &LogEntry) {
        self.entries.lock().unwrap().push(entry.clone());
    }
}

// ---------------------------------------------------------------------------
// WAVE scoring
// ---------------------------------------------------------------------------

/// Full WAVE coherence measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveScore {
    pub structural: f64,
    pub semantic: f64,
    pub temporal: f64,
    /// Fibonacci-weighted composite: 50% structural + 31.25% semantic + 18.75% temporal
    pub composite: f64,
}

impl WaveScore {
    pub fn compute(structural: f64, semantic: f64, temporal: f64) -> Self {
        let w = crate::WAVE_WEIGHTS;
        let composite = structural * w.structural + semantic * w.semantic + temporal * w.temporal;
        Self { structural, semantic, temporal, composite }
    }

    pub fn passes_threshold(&self) -> bool {
        self.composite >= crate::WAVE_THRESHOLD
    }
}

// ---------------------------------------------------------------------------
// Conservation state
// ---------------------------------------------------------------------------

/// α + ω = 15 invariant.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConservationState {
    pub alpha: u8,
    pub omega: u8,
    pub sum: u8,
    pub valid: bool,
}

impl ConservationState {
    pub fn new(alpha: u8, omega: u8) -> Self {
        let sum = alpha.saturating_add(omega);
        Self { alpha, omega, sum, valid: sum == crate::CONSERVATION_SUM }
    }

    pub fn verify(&self) -> bool {
        self.alpha.saturating_add(self.omega) == crate::CONSERVATION_SUM
    }
}

// ---------------------------------------------------------------------------
// ATOM trail
// ---------------------------------------------------------------------------

/// Single entry in the ATOM (Adaptive Trail of Operations & Mutations) trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomEntry {
    pub id: String,
    pub atom_type: String,
    pub gate: String,
    pub description: String,
    pub coherence: f64,
    pub timestamp: DateTime<Utc>,
    /// SHA-256 of the previous entry — forms the provenance chain.
    pub prev_hash: String,
    /// SHA-256 of this entry (computed over all fields except `hash`).
    pub hash: String,
}
