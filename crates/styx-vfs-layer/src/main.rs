//! styx-bookshelf — TCP 9P2000.L Bookshelf daemon (GB-03)

use anyhow::Result;
use styx_vfs_layer::{serve, BookshelfConfig};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let mut cfg = BookshelfConfig::from_env();
    // Prefer LOGOS_ROOT; else current dir if it looks like LogOS; else ~/LogOS
    if std::env::var_os("LOGOS_ROOT").is_none() {
        let cwd = std::env::current_dir()?;
        if cwd.join("docs/schemas/v0.1").is_dir() {
            cfg.repo_root = cwd;
        } else {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/home/toolated".into());
            cfg.repo_root = std::path::PathBuf::from(home).join("LogOS");
        }
    }
    cfg.repo_root = cfg.repo_root.canonicalize().unwrap_or(cfg.repo_root);

    serve(cfg).await
}
