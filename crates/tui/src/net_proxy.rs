//! Net proxy stack control — Tor / i2pd / Privoxy / DNSCrypt / gaming clearnet.
//!
//! Callable from reson8-tui Net panel. Probes localhost ports (Category B with
//! timestamp). Spawns `ops/net/LogOS.NetProxy.ps1` for start/stop/install.
//!
//! ATOM: ATOM-NET-PROXY-STACK-20260807
//! Gaming never routes through Tor/I2P (high-speed clearnet lane).

use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Actions the ε-phase may run (after RS-NOR latch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetAction {
    Status,
    StartPrivacy,
    StartGaming,
    Stop,
    Install,
}

impl NetAction {
    pub fn id(self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::StartPrivacy => "start-privacy",
            Self::StartGaming => "start-gaming",
            Self::Stop => "stop",
            Self::Install => "install",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Status => "status / probe",
            Self::StartPrivacy => "start privacy (tor+privoxy+i2pd+dnscrypt)",
            Self::StartGaming => "start gaming clearnet (high-speed)",
            Self::Stop => "stop all proxy daemons",
            Self::Install => "install tools (winget/choco best-effort)",
        }
    }

    /// Map popup option text → action.
    pub fn from_popup_answer(answer: &str) -> Option<Self> {
        let a = answer.to_ascii_lowercase();
        if a.contains("privacy") {
            Some(Self::StartPrivacy)
        } else if a.contains("gaming") || a.contains("clearnet") {
            Some(Self::StartGaming)
        } else if a.contains("stop") {
            Some(Self::Stop)
        } else if a.contains("install") {
            Some(Self::Install)
        } else if a.contains("status") || a.contains("probe") {
            Some(Self::Status)
        } else {
            None
        }
    }

    pub fn popup_options() -> Vec<String> {
        vec![
            Self::Status.label().into(),
            Self::StartPrivacy.label().into(),
            Self::StartGaming.label().into(),
            Self::Stop.label().into(),
            Self::Install.label().into(),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct ServiceProbe {
    pub id: &'static str,
    pub port: u16,
    pub open: bool,
    pub lane: &'static str,
}

#[derive(Debug, Clone)]
pub struct NetProxyState {
    pub services: Vec<ServiceProbe>,
    pub active_lane: String,
    pub last_message: String,
    pub last_probe_unix: u64,
    pub controller_path: PathBuf,
    pub script_present: bool,
}

impl Default for NetProxyState {
    fn default() -> Self {
        Self::probe_fresh()
    }
}

impl NetProxyState {
    pub fn logos_root() -> PathBuf {
        std::env::var("LOGOS_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
            })
    }

    pub fn controller_script(root: &Path) -> PathBuf {
        root.join("ops/net/LogOS.NetProxy.ps1")
    }

    pub fn probe_fresh() -> Self {
        let root = Self::logos_root();
        let controller = Self::controller_script(&root);
        let script_present = controller.is_file();
        let services = probe_services();
        let lane = read_lane_file(&root).unwrap_or_else(|| "unknown".into());
        Self {
            services,
            active_lane: lane,
            last_message: if script_present {
                "probe complete · [M] menu · [R] refresh".into()
            } else {
                format!("controller missing: {}", controller.display())
            },
            last_probe_unix: now_unix(),
            controller_path: controller,
            script_present,
        }
    }

    pub fn refresh_probe(&mut self) {
        self.services = probe_services();
        self.active_lane =
            read_lane_file(&Self::logos_root()).unwrap_or_else(|| self.active_lane.clone());
        self.last_probe_unix = now_unix();
        let up = self.services.iter().filter(|s| s.open).count();
        self.last_message = format!(
            "probe: {up}/{} ports up · lane={} · [B]",
            self.services.len(),
            self.active_lane
        );
    }

    /// Run controller script; updates self from ports after.
    pub fn run_action(&mut self, action: NetAction) -> String {
        if !self.script_present {
            let msg = format!("controller missing: {}", self.controller_path.display());
            self.last_message = msg.clone();
            return msg;
        }

        let shell = if cfg!(windows) { "pwsh" } else { "pwsh" };
        let output = Command::new(shell)
            .arg("-NoProfile")
            .arg("-File")
            .arg(&self.controller_path)
            .arg("-Action")
            .arg(action.id())
            .arg("-Json")
            .output();

        let summary = match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                // Prefer compact message from JSON messages array if present
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&stdout) {
                    if let Some(lane) = v.get("active_lane").and_then(|x| x.as_str()) {
                        self.active_lane = lane.to_string();
                    }
                    if let Some(arr) = v.get("messages").and_then(|x| x.as_array()) {
                        let joined: Vec<&str> = arr
                            .iter()
                            .filter_map(|m| m.as_str())
                            .take(4)
                            .collect();
                        if !joined.is_empty() {
                            joined.join(" · ")
                        } else {
                            format!("{} ok (exit {})", action.id(), out.status)
                        }
                    } else if let Some(lane) = v.get("active_lane").and_then(|x| x.as_str()) {
                        format!("lane={lane} · {} done", action.id())
                    } else {
                        truncate(&stdout, 200)
                    }
                } else if !stdout.trim().is_empty() {
                    truncate(&stdout, 200)
                } else if !stderr.trim().is_empty() {
                    truncate(&stderr, 200)
                } else {
                    format!("{} exit={}", action.id(), out.status)
                }
            }
            Err(e) => format!("spawn failed: {e} (is pwsh on PATH?)"),
        };

        self.refresh_probe();
        self.last_message = summary.clone();
        summary
    }

    pub fn summary_line(&self) -> String {
        let up = self.services.iter().filter(|s| s.open).count();
        format!(
            "lane={}  ports {up}/{}  {}",
            self.active_lane,
            self.services.len(),
            if self.script_present { "ctl●" } else { "ctl○" }
        )
    }
}

fn probe_services() -> Vec<ServiceProbe> {
    const DEFS: &[(&str, u16, &str)] = &[
        ("tor_socks", 9050, "privacy"),
        ("tor_control", 9051, "privacy"),
        ("privoxy", 8118, "privacy"),
        ("i2pd_http", 4444, "privacy"),
        ("i2pd_sam", 7656, "privacy"),
        ("dnscrypt", 53553, "shared"),
    ];
    DEFS.iter()
        .map(|(id, port, lane)| ServiceProbe {
            id,
            port: *port,
            open: port_open(*port),
            lane,
        })
        .collect()
}

fn port_open(port: u16) -> bool {
    let addr = format!("127.0.0.1:{port}");
    let Ok(mut addrs) = addr.to_socket_addrs() else {
        return false;
    };
    let Some(sock) = addrs.next() else {
        return false;
    };
    TcpStream::connect_timeout(&SocketAddr::from(sock), Duration::from_millis(80)).is_ok()
        || (cfg!(windows) && TcpStream::connect_timeout(
            &SocketAddr::from(([127, 0, 0, 1], port)),
            Duration::from_millis(80),
        )
        .is_ok())
}

fn read_lane_file(root: &Path) -> Option<String> {
    let p = root.join("ops/net/data/active_lane.txt");
    std::fs::read_to_string(p)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn truncate(s: &str, max: usize) -> String {
    let t = s.trim().replace('\n', " · ");
    if t.len() <= max {
        t
    } else {
        format!("{}…", &t[..max.saturating_sub(1)])
    }
}

/// Elapsed helper for tests / future rate limit.
#[allow(dead_code)]
pub fn probe_duration_ms(f: impl FnOnce()) -> u128 {
    let t0 = Instant::now();
    f();
    t0.elapsed().as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_ids_stable() {
        assert_eq!(NetAction::StartGaming.id(), "start-gaming");
        assert_eq!(NetAction::from_popup_answer("start gaming clearnet (high-speed)"), Some(NetAction::StartGaming));
    }

    #[test]
    fn probe_does_not_panic() {
        let s = NetProxyState::probe_fresh();
        assert_eq!(s.services.len(), 6);
    }
}
