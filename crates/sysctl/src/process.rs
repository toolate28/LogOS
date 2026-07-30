//! Process and service management across substrates.

use serde::{Deserialize, Serialize};

use crate::Substrate;

/// A running process visible to the TUI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_mb: f64,
    pub substrate: Substrate,
    pub user: String,
}

/// A system service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub display_name: String,
    pub status: ServiceStatus,
    pub start_type: StartType,
    pub substrate: Substrate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Paused,
    StartPending,
    StopPending,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartType {
    Automatic,
    Manual,
    Disabled,
    Boot,
    System,
}

/// Commands for process management.
pub struct ProcessCommands;

impl ProcessCommands {
    // ── Windows ──────────────────────────────────────────────────────
    pub fn win_list_top(n: usize) -> String {
        format!(
            r#"powershell -Command "Get-Process | Sort-Object CPU -Descending | Select-Object -First {n} Id,ProcessName,CPU,@{{N='MemMB';E={{[math]::Round($_.WorkingSet64/1MB,1)}}}} | Format-Table -AutoSize""#
        )
    }

    pub fn win_kill(pid: u32) -> String {
        format!("taskkill /PID {pid} /F")
    }

    pub fn win_list_services() -> &'static str {
        r#"powershell -Command "Get-Service | Where-Object {$_.Status -eq 'Running'} | Format-Table Name, DisplayName, Status -AutoSize""#
    }

    pub fn win_stop_service(name: &str) -> String {
        format!(r#"powershell -Command "Stop-Service -Name '{name}' -Force""#)
    }

    pub fn win_start_service(name: &str) -> String {
        format!(r#"powershell -Command "Start-Service -Name '{name}'""#)
    }

    pub fn win_disable_service(name: &str) -> String {
        format!(r#"powershell -Command "Set-Service -Name '{name}' -StartupType Disabled""#)
    }

    // ── WSL2 / NixOS ─────────────────────────────────────────────────
    pub fn linux_list_top(distro: &str, n: usize) -> String {
        format!("wsl -d {distro} -- ps aux --sort=-%cpu | head -n {}", n + 1)
    }

    pub fn linux_kill(distro: &str, pid: u32) -> String {
        format!("wsl -d {distro} -- kill -9 {pid}")
    }

    pub fn linux_list_services(distro: &str) -> String {
        format!("wsl -d {distro} -- systemctl list-units --type=service --state=running --no-pager")
    }

    pub fn linux_stop_service(distro: &str, name: &str) -> String {
        format!("wsl -d {distro} -- sudo systemctl stop {name}")
    }

    pub fn linux_start_service(distro: &str, name: &str) -> String {
        format!("wsl -d {distro} -- sudo systemctl start {name}")
    }
}

/// Bloatware detection — identify high-resource or unnecessary processes.
pub fn known_bloat_processes() -> Vec<&'static str> {
    vec![
        "SearchApp",
        "YourPhone",
        "GameBarPresenceWriter",
        "MicrosoftEdgeUpdate",
        "OneDrive",
        "Teams",
        "WidgetService",
        "Cortana",
        "HxTsr",
        "HxOutlook",
        "TextInputHost",
        "StartMenuExperienceHost",
    ]
}
