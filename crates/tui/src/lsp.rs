//! Lean / Agda LSP client — decoupled reader + writer (Category A mechanism).
//!
//! JSON-RPC over stdio with `Content-Length` framing. Background task pulls
//! stdout continuously; responses complete oneshot waiters; notifications
//! (`publishDiagnostics`, `window/logMessage`) fan out on `LspEvent` mpsc so
//! the Ratatui loop never drops amber `sorry` / diagnostics.
//!
//! Status: scaffold **B** until `lake serve` / `als` attach successfully.
//! Placeholders must never render as live-green in the TUI.

use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;

use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, Command};
use tokio::sync::{mpsc, oneshot, Mutex};

/// Messages bound for the UI thread (eye of the needle).
#[derive(Debug, Clone)]
pub enum LspEvent {
    Diagnostics {
        server: LspServerKind,
        params: Value,
    },
    LogMessage {
        server: LspServerKind,
        params: Value,
    },
    /// Gate / plan / scaffold honesty — never paint as success green.
    Placeholder {
        id: String,
        message: String,
    },
    /// Server process ended or failed to spawn.
    ServerStatus {
        server: LspServerKind,
        ok: bool,
        detail: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspServerKind {
    Lean,
    Agda,
}

impl LspServerKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Lean => "lean",
            Self::Agda => "agda/als",
        }
    }
}

/// One row in the Formal diagnostics pane.
#[derive(Debug, Clone)]
pub struct DiagnosticRow {
    pub server: String,
    pub severity: DiagnosticSeverityUi,
    pub path: String,
    pub line: u32,
    pub message: String,
    /// When true, this is an honest B stub — render amber, never green.
    pub placeholder: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverityUi {
    Error,
    Warning,
    Info,
    Hint,
    /// JFA / sorry / open obligation — amber SlowStep
    SlowStep,
    PlaceholderB,
}

impl DiagnosticSeverityUi {
    pub fn from_lsp(n: u64, message: &str) -> Self {
        // LSP: 1 Error, 2 Warning, 3 Info, 4 Hint
        let lower = message.to_ascii_lowercase();
        if lower.contains("sorry") || lower.contains("unsolved") || lower.contains("obligation") {
            return Self::SlowStep;
        }
        match n {
            1 => Self::Error,
            2 => Self::Warning,
            3 => Self::Info,
            4 => Self::Hint,
            _ => Self::Info,
        }
    }
}

/// Held for process lifetime so drop kills the server; request APIs ready for init/didOpen.
#[allow(dead_code)]
pub struct LspConnection {
    kind: LspServerKind,
    stdin: Arc<Mutex<ChildStdin>>,
    next_id: Arc<Mutex<u64>>,
    pending_requests: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
}

impl LspConnection {
    /// Spawn `command args` as an LSP server; start background reader → `event_tx`.
    pub async fn spawn(
        kind: LspServerKind,
        program: &str,
        args: &[&str],
        cwd: Option<&str>,
        event_tx: mpsc::Sender<LspEvent>,
    ) -> Result<Self, String> {
        let mut cmd = Command::new(program);
        cmd.args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let mut child = cmd.spawn().map_err(|e| {
            format!(
                "[{}] spawn `{} {:?}` failed: {}",
                kind.label(),
                program,
                args,
                e
            )
        })?;

        let stdin = child.stdin.take().ok_or_else(|| "stdin missing".to_string())?;
        let stdout = child.stdout.take().ok_or_else(|| "stdout missing".to_string())?;

        let pending_requests: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_clone = pending_requests.clone();
        let kind_bg = kind;
        let status_tx = event_tx.clone();
        let reader_tx = event_tx.clone();

        tokio::spawn(async move {
            // Keep child alive for stderr drain optional later; drop ends process via kill_on_drop.
            let _child = child;
            let mut reader = BufReader::new(stdout);
            loop {
                let mut headers = String::new();
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line).await {
                        Ok(0) => {
                            let _ = status_tx
                                .send(LspEvent::ServerStatus {
                                    server: kind_bg,
                                    ok: false,
                                    detail: "stdout EOF".into(),
                                })
                                .await;
                            return;
                        }
                        Ok(_) => {
                            if line.trim().is_empty() {
                                break;
                            }
                            headers.push_str(&line);
                        }
                        Err(e) => {
                            let _ = status_tx
                                .send(LspEvent::ServerStatus {
                                    server: kind_bg,
                                    ok: false,
                                    detail: format!("read error: {e}"),
                                })
                                .await;
                            return;
                        }
                    }
                }

                let content_length: usize = headers
                    .lines()
                    .find(|l| l.to_lowercase().starts_with("content-length:"))
                    .and_then(|l| l.split(':').nth(1))
                    .and_then(|s| s.trim().parse().ok())
                    .unwrap_or(0);

                if content_length == 0 {
                    continue;
                }

                let mut body = vec![0u8; content_length];
                if reader.read_exact(&mut body).await.is_err() {
                    break;
                }

                if let Ok(msg) = serde_json::from_slice::<Value>(&body) {
                    route_message(kind_bg, msg, &pending_clone, &reader_tx).await;
                }
            }
        });

        let _ = event_tx
            .send(LspEvent::ServerStatus {
                server: kind,
                ok: true,
                detail: format!("spawned {program}"),
            })
            .await;

        Ok(Self {
            kind,
            stdin: Arc::new(Mutex::new(stdin)),
            next_id: Arc::new(Mutex::new(1)),
            pending_requests,
        })
    }

    pub async fn send_request(&self, method: &str, params: Value) -> Result<Value, String> {
        let id = {
            let mut g = self.next_id.lock().await;
            let current = *g;
            *g += 1;
            current
        };

        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        let (tx, rx) = oneshot::channel();
        self.pending_requests.lock().await.insert(id, tx);
        self.write_message(&request).await?;

        let response = rx.await.map_err(|_| "LSP oneshot canceled".to_string())?;
        if let Some(error) = response.get("error") {
            return Err(format!("LSP error ({}): {}", self.kind.label(), error));
        }
        Ok(response.get("result").cloned().unwrap_or(Value::Null))
    }

    pub async fn send_notification(&self, method: &str, params: Value) -> Result<(), String> {
        let notification = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });
        self.write_message(&notification).await
    }

    async fn write_message(&self, msg: &Value) -> Result<(), String> {
        let content = serde_json::to_string(msg).map_err(|e| e.to_string())?;
        let header = format!("Content-Length: {}\r\n\r\n", content.len());
        let mut stdin = self.stdin.lock().await;
        stdin
            .write_all(header.as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        stdin
            .write_all(content.as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        stdin.flush().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

async fn route_message(
    kind: LspServerKind,
    msg: Value,
    pending: &Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    event_tx: &mpsc::Sender<LspEvent>,
) {
    if let Some(id) = msg.get("id").and_then(|v| v.as_u64()) {
        // Response (or server request with id — complete waiter if present).
        let mut map = pending.lock().await;
        if let Some(tx) = map.remove(&id) {
            let _ = tx.send(msg);
        }
        return;
    }

    if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
        match method {
            "textDocument/publishDiagnostics" => {
                let params = msg.get("params").cloned().unwrap_or(Value::Null);
                let _ = event_tx
                    .send(LspEvent::Diagnostics {
                        server: kind,
                        params,
                    })
                    .await;
            }
            "window/logMessage" | "window/showMessage" => {
                let params = msg.get("params").cloned().unwrap_or(Value::Null);
                let _ = event_tx
                    .send(LspEvent::LogMessage {
                        server: kind,
                        params,
                    })
                    .await;
            }
            _ => {}
        }
    }
}

/// Parse LSP publishDiagnostics params into UI rows.
pub fn rows_from_diagnostics(server: LspServerKind, params: &Value) -> Vec<DiagnosticRow> {
    let uri = params
        .get("uri")
        .and_then(|u| u.as_str())
        .unwrap_or("")
        .to_string();
    let path = uri_to_path(&uri);
    let mut rows = Vec::new();
    if let Some(diags) = params.get("diagnostics").and_then(|d| d.as_array()) {
        for d in diags {
            let severity_n = d.get("severity").and_then(|s| s.as_u64()).unwrap_or(3);
            let message = d
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("")
                .to_string();
            let line = d
                .get("range")
                .and_then(|r| r.get("start"))
                .and_then(|s| s.get("line"))
                .and_then(|l| l.as_u64())
                .unwrap_or(0) as u32;
            rows.push(DiagnosticRow {
                server: server.label().into(),
                severity: DiagnosticSeverityUi::from_lsp(severity_n, &message),
                path: path.clone(),
                line: line + 1,
                message,
                placeholder: false,
            });
        }
    }
    rows
}

fn uri_to_path(uri: &str) -> String {
    let s = uri.strip_prefix("file:///").unwrap_or(uri);
    let s = s.strip_prefix("file://").unwrap_or(s);
    // Windows file:///C:/...
    if s.len() >= 2 && s.as_bytes()[1] == b':' {
        return s.replace('/', "\\");
    }
    if let Some(rest) = s.strip_prefix('/') {
        if rest.len() >= 2 && rest.as_bytes()[1] == b':' {
            return rest.replace('/', "\\");
        }
    }
    s.to_string()
}

/// Honest B stubs shown until servers attach.
pub fn bootstrap_placeholders() -> Vec<DiagnosticRow> {
    vec![
        DiagnosticRow {
            server: "agda/als".into(),
            severity: DiagnosticSeverityUi::PlaceholderB,
            path: "—".into(),
            line: 0,
            message: "[CATEGORY B: PLANNED, NOT BUILT] agda-language-server (als) not on PATH; cubical pin GB-01 open".into(),
            placeholder: true,
        },
        DiagnosticRow {
            server: "lean".into(),
            severity: DiagnosticSeverityUi::PlaceholderB,
            path: "lean/".into(),
            line: 0,
            message: "[CATEGORY B: PLANNED, NOT BUILT] lake serve / lean --server attach pending — eye not yet live".into(),
            placeholder: true,
        },
        DiagnosticRow {
            server: "binary".into(),
            severity: DiagnosticSeverityUi::PlaceholderB,
            path: "—".into(),
            line: 0,
            message: "[CATEGORY B: PLANNED, NOT BUILT] product binary / continuous-Novikov (GB-06)".into(),
            placeholder: true,
        },
        DiagnosticRow {
            server: "cc-cert".into(),
            severity: DiagnosticSeverityUi::PlaceholderB,
            path: ".atom-trail/certs/claude-code/latest.json".into(),
            line: 0,
            message: "[CATEGORY B: PLANNED, NOT BUILT] Claude Code init cert pass:false until cold-start".into(),
            placeholder: true,
        },
    ]
}

/// Attempt Lean LSP spawn (prefer lake serve in lean/). On failure emit placeholders only.
pub async fn try_attach_lean(
    logos_root: &std::path::Path,
    event_tx: mpsc::Sender<LspEvent>,
) -> Option<LspConnection> {
    let lean_dir = logos_root.join("lean");
    let cwd = if lean_dir.is_dir() {
        Some(lean_dir.to_string_lossy().to_string())
    } else {
        None
    };

    // Prefer `lake serve` when cwd is lean package.
    if let Some(ref dir) = cwd {
        match LspConnection::spawn(
            LspServerKind::Lean,
            "lake",
            &["serve"],
            Some(dir.as_str()),
            event_tx.clone(),
        )
        .await
        {
            Ok(conn) => return Some(conn),
            Err(e) => {
                let _ = event_tx
                    .send(LspEvent::ServerStatus {
                        server: LspServerKind::Lean,
                        ok: false,
                        detail: format!("lake serve: {e}"),
                    })
                    .await;
            }
        }
    }

    match LspConnection::spawn(
        LspServerKind::Lean,
        "lean",
        &["--server"],
        cwd.as_deref(),
        event_tx.clone(),
    )
    .await
    {
        Ok(conn) => Some(conn),
        Err(e) => {
            let _ = event_tx
                .send(LspEvent::ServerStatus {
                    server: LspServerKind::Lean,
                    ok: false,
                    detail: format!("lean --server: {e}"),
                })
                .await;
            let _ = event_tx
                .send(LspEvent::Placeholder {
                    id: "lean-attach".into(),
                    message: "[CATEGORY B: PLANNED, NOT BUILT] Lean LSP not attached".into(),
                })
                .await;
            None
        }
    }
}

/// Attempt Agda ALS — almost always B until binary exists.
pub async fn try_attach_agda(event_tx: mpsc::Sender<LspEvent>) -> Option<LspConnection> {
    match LspConnection::spawn(LspServerKind::Agda, "als", &[], None, event_tx.clone()).await {
        Ok(conn) => Some(conn),
        Err(e) => {
            let _ = event_tx
                .send(LspEvent::ServerStatus {
                    server: LspServerKind::Agda,
                    ok: false,
                    detail: format!("als: {e}"),
                })
                .await;
            let _ = event_tx
                .send(LspEvent::Placeholder {
                    id: "als-attach".into(),
                    message: "[CATEGORY B: PLANNED, NOT BUILT] als not available".into(),
                })
                .await;
            None
        }
    }
}
