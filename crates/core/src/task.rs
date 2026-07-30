//! Task lifecycle — the unit of work flowing through the orchestrator.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Task phase — the pipeline stage a task is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPhase {
    Pending,
    Initialized,
    Executing,
    Validated,
    Completed,
    Failed,
}

/// Metadata attached to every task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMeta {
    pub kind: String,
    pub provider: Option<crate::adapter::Provider>,
    pub description: String,
}

/// A unit of work in the orchestrator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub phase: TaskPhase,
    pub meta: TaskMeta,
    pub wave_score: Option<f64>,
    /// Anyon braid tag for provenance verification.
    pub braid_tag: Option<String>,
}

impl Task {
    pub fn new(kind: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            phase: TaskPhase::Pending,
            meta: TaskMeta {
                kind: kind.into(),
                provider: None,
                description: description.into(),
            },
            wave_score: None,
            braid_tag: None,
        }
    }

    pub fn advance(&mut self) {
        self.phase = match self.phase {
            TaskPhase::Pending     => TaskPhase::Initialized,
            TaskPhase::Initialized => TaskPhase::Executing,
            TaskPhase::Executing   => TaskPhase::Validated,
            TaskPhase::Validated   => TaskPhase::Completed,
            TaskPhase::Completed   => TaskPhase::Completed,
            TaskPhase::Failed      => TaskPhase::Failed,
        };
    }

    pub fn fail(&mut self) {
        self.phase = TaskPhase::Failed;
    }
}
