//! Hypervisor management — WSL2 distributions, Hyper-V, and container runtimes.

use serde::{Deserialize, Serialize};

/// A managed WSL2 distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WslDistro {
    pub name: String,
    pub state: DistroState,
    pub version: u8,
    pub default: bool,
    /// Mapped 9P2000.L mount point (if configured)
    pub mount_point: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistroState {
    Running,
    Stopped,
    Installing,
    Unregistering,
    Converting,
}

/// Commands for WSL2 management.
pub struct WslCommands;

impl WslCommands {
    pub fn list() -> &'static str { "wsl --list --verbose" }
    pub fn shutdown_all() -> &'static str { "wsl --shutdown" }
    pub fn terminate(name: &str) -> String { format!("wsl --terminate {name}") }
    pub fn set_default(name: &str) -> String { format!("wsl --set-default {name}") }
    pub fn install(distro: &str) -> String { format!("wsl --install -d {distro}") }
    pub fn unregister(name: &str) -> String { format!("wsl --unregister {name}") }
    pub fn export(name: &str, path: &str) -> String {
        format!("wsl --export {name} \"{path}\"")
    }
    pub fn import(name: &str, install_dir: &str, tar: &str) -> String {
        format!("wsl --import {name} \"{install_dir}\" \"{tar}\"")
    }
    pub fn set_version(name: &str, version: u8) -> String {
        format!("wsl --set-version {name} {version}")
    }
    pub fn status() -> &'static str { "wsl --status" }
    pub fn update() -> &'static str { "wsl --update" }

    /// Mount a 9P2000.L share into a WSL2 distro.
    pub fn mount_9p(distro: &str, tag: &str, mount_point: &str) -> String {
        format!(
            r#"wsl -d {distro} -- sudo mount -t 9p -o trans=virtio,version=9p2000.L,msize=8388608,cache=loose {tag} {mount_point}"#
        )
    }
}

/// Hyper-V feature management.
pub struct HyperVCommands;

impl HyperVCommands {
    pub fn status() -> &'static str {
        "powershell -Command \"Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V | Format-List\""
    }
    pub fn enable() -> &'static str {
        "powershell -Command \"Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All -NoRestart\""
    }
    pub fn disable() -> &'static str {
        "powershell -Command \"Disable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -NoRestart\""
    }
    pub fn list_vms() -> &'static str {
        "powershell -Command \"Get-VM | Format-Table Name, State, CPUUsage, MemoryAssigned\""
    }
    pub fn list_switches() -> &'static str {
        "powershell -Command \"Get-VMSwitch | Format-Table Name, SwitchType\""
    }
}
