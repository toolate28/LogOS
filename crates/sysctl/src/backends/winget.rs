//! WinGet backend — reference implementation of [`PackageManager`].

use async_trait::async_trait;

use crate::package::{Package, PackageManager, PackageSource};
use crate::{ExecResult, Substrate};

pub struct WinGetBackend;

#[async_trait]
impl PackageManager for WinGetBackend {
    fn source(&self) -> PackageSource {
        PackageSource::WinGet
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<Package>> {
        let output = run_ps(&PackageSource::WinGet.search_cmd(query)).await?;
        Ok(parse_winget_table(&output.stdout, false))
    }

    async fn install(&self, pkg_id: &str) -> anyhow::Result<ExecResult> {
        run_ps(&PackageSource::WinGet.install_cmd(pkg_id)).await
    }

    async fn uninstall(&self, pkg_id: &str) -> anyhow::Result<ExecResult> {
        run_ps(&PackageSource::WinGet.uninstall_cmd(pkg_id)).await
    }

    async fn upgrade(&self, pkg_id: &str) -> anyhow::Result<ExecResult> {
        run_ps(&PackageSource::WinGet.upgrade_cmd(pkg_id)).await
    }

    async fn upgrade_all(&self) -> anyhow::Result<ExecResult> {
        run_ps(PackageSource::WinGet.upgrade_all_cmd()).await
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<Package>> {
        let output = run_ps(PackageSource::WinGet.list_installed_cmd()).await?;
        Ok(parse_winget_table(&output.stdout, true))
    }

    async fn list_upgradable(&self) -> anyhow::Result<Vec<Package>> {
        let output = run_ps("winget upgrade").await?;
        Ok(parse_winget_table(&output.stdout, true))
    }
}

async fn run_ps(cmd: &str) -> anyhow::Result<ExecResult> {
    let start = std::time::Instant::now();
    let output = tokio::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", cmd])
        .output()
        .await?;

    Ok(ExecResult {
        success: output.status.success(),
        exit_code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        substrate: Substrate::Windows,
        elapsed_ms: start.elapsed().as_millis() as u64,
    })
}

fn parse_winget_table(stdout: &str, installed: bool) -> Vec<Package> {
    // WinGet output is columnar with dashes separator line.
    // This is a best-effort parser — winget output varies by locale.
    let mut packages = Vec::new();
    let mut past_header = false;

    for line in stdout.lines() {
        if line.starts_with("---") || line.starts_with("===") {
            past_header = true;
            continue;
        }
        if !past_header || line.trim().is_empty() {
            continue;
        }
        // Split on 2+ whitespace chars
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            packages.push(Package {
                id: parts.last().unwrap_or(&"").to_string(),
                name: parts[0].to_string(),
                version: parts.get(1).unwrap_or(&"").to_string(),
                source: PackageSource::WinGet,
                substrate: Substrate::Windows,
                installed,
                available_update: None,
            });
        }
    }
    packages
}
