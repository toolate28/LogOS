//! Unified package management across all substrates.
//!
//! Trait [`PackageManager`] is implemented by each backend:
//! - Windows: WinGet, Chocolatey, Scoop, MSStore, PSGallery, UnigetUI
//! - WSL2: apt, snap
//! - NixOS: nix flakes (declarative)

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{ExecResult, Substrate};

/// A software package, substrate-agnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub source: PackageSource,
    pub substrate: Substrate,
    pub installed: bool,
    pub available_update: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageSource {
    // Windows
    WinGet,
    Chocolatey,
    Scoop,
    MsStore,
    PsGallery,
    // WSL2
    Apt,
    Snap,
    // NixOS
    NixFlake,
    NixPkgs,
}

impl PackageSource {
    pub fn substrate(self) -> Substrate {
        match self {
            Self::WinGet | Self::Chocolatey | Self::Scoop
            | Self::MsStore | Self::PsGallery => Substrate::Windows,
            Self::Apt | Self::Snap => Substrate::Wsl2,
            Self::NixFlake | Self::NixPkgs => Substrate::NixOS,
        }
    }

    pub fn install_cmd(self, pkg_id: &str) -> String {
        match self {
            Self::WinGet => format!("winget install --id {pkg_id} --accept-package-agreements --accept-source-agreements"),
            Self::Chocolatey => format!("choco install {pkg_id} -y"),
            Self::Scoop => format!("scoop install {pkg_id}"),
            Self::MsStore => format!("winget install --source msstore --id {pkg_id} --accept-package-agreements"),
            Self::PsGallery => format!("Install-Module -Name {pkg_id} -Force -Scope CurrentUser"),
            Self::Apt => format!("sudo apt-get install -y {pkg_id}"),
            Self::Snap => format!("sudo snap install {pkg_id}"),
            Self::NixFlake => format!("nix profile install nixpkgs#{pkg_id}"),
            Self::NixPkgs => format!("nix-env -iA nixpkgs.{pkg_id}"),
        }
    }

    pub fn uninstall_cmd(self, pkg_id: &str) -> String {
        match self {
            Self::WinGet => format!("winget uninstall --id {pkg_id}"),
            Self::Chocolatey => format!("choco uninstall {pkg_id} -y"),
            Self::Scoop => format!("scoop uninstall {pkg_id}"),
            Self::MsStore => format!("winget uninstall --id {pkg_id}"),
            Self::PsGallery => format!("Uninstall-Module -Name {pkg_id} -Force"),
            Self::Apt => format!("sudo apt-get remove -y {pkg_id}"),
            Self::Snap => format!("sudo snap remove {pkg_id}"),
            Self::NixFlake => format!("nix profile remove nixpkgs#{pkg_id}"),
            Self::NixPkgs => format!("nix-env -e {pkg_id}"),
        }
    }

    pub fn search_cmd(self, query: &str) -> String {
        match self {
            Self::WinGet => format!("winget search {query}"),
            Self::Chocolatey => format!("choco search {query}"),
            Self::Scoop => format!("scoop search {query}"),
            Self::MsStore => format!("winget search --source msstore {query}"),
            Self::PsGallery => format!("Find-Module -Name *{query}*"),
            Self::Apt => format!("apt-cache search {query}"),
            Self::Snap => format!("snap find {query}"),
            Self::NixFlake | Self::NixPkgs => format!("nix search nixpkgs {query}"),
        }
    }

    pub fn list_installed_cmd(self) -> &'static str {
        match self {
            Self::WinGet => "winget list",
            Self::Chocolatey => "choco list",
            Self::Scoop => "scoop list",
            Self::MsStore => "winget list --source msstore",
            Self::PsGallery => "Get-InstalledModule | Format-Table Name, Version",
            Self::Apt => "dpkg --get-selections | grep -v deinstall",
            Self::Snap => "snap list",
            Self::NixFlake => "nix profile list",
            Self::NixPkgs => "nix-env -q",
        }
    }

    pub fn upgrade_cmd(self, pkg_id: &str) -> String {
        match self {
            Self::WinGet => format!("winget upgrade --id {pkg_id}"),
            Self::Chocolatey => format!("choco upgrade {pkg_id} -y"),
            Self::Scoop => format!("scoop update {pkg_id}"),
            Self::MsStore => format!("winget upgrade --source msstore --id {pkg_id}"),
            Self::PsGallery => format!("Update-Module -Name {pkg_id}"),
            Self::Apt => format!("sudo apt-get install --only-upgrade -y {pkg_id}"),
            Self::Snap => format!("sudo snap refresh {pkg_id}"),
            Self::NixFlake => format!("nix profile upgrade nixpkgs#{pkg_id}"),
            Self::NixPkgs => format!("nix-env -u {pkg_id}"),
        }
    }

    pub fn upgrade_all_cmd(self) -> &'static str {
        match self {
            Self::WinGet => "winget upgrade --all --accept-package-agreements",
            Self::Chocolatey => "choco upgrade all -y",
            Self::Scoop => "scoop update *",
            Self::MsStore => "winget upgrade --all --source msstore",
            Self::PsGallery => "Get-InstalledModule | Update-Module",
            Self::Apt => "sudo apt-get update && sudo apt-get upgrade -y",
            Self::Snap => "sudo snap refresh",
            Self::NixFlake => "nix profile upgrade '.*'",
            Self::NixPkgs => "nix-env -u",
        }
    }
}

/// Unified package manager interface.
#[async_trait]
pub trait PackageManager: Send + Sync {
    fn source(&self) -> PackageSource;
    fn substrate(&self) -> Substrate { self.source().substrate() }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<Package>>;
    async fn install(&self, pkg_id: &str) -> anyhow::Result<ExecResult>;
    async fn uninstall(&self, pkg_id: &str) -> anyhow::Result<ExecResult>;
    async fn upgrade(&self, pkg_id: &str) -> anyhow::Result<ExecResult>;
    async fn upgrade_all(&self) -> anyhow::Result<ExecResult>;
    async fn list_installed(&self) -> anyhow::Result<Vec<Package>>;
    async fn list_upgradable(&self) -> anyhow::Result<Vec<Package>>;
}
