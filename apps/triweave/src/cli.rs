use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start the dual-protocol WebSocket bridge (TriWeavon extension + POP JSON-RPC)
    Serve {
        #[arg(long, default_value = "127.0.0.1:8088")]
        addr: String,
    },
    Init,
    Up {
        #[arg(value_enum)]
        strand: StrandArg,
    },
    Down {
        #[arg(value_enum)]
        strand: StrandArg,
    },
    Status,
    Doctor,
    Deploy {
        #[arg(value_enum)]
        target: DeployTarget,
    },
    Search {
        query: String,
    },
    Theme {
        name: Option<String>,
    },
    Vault {
        #[command(subcommand)]
        action: VaultAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum VaultAction {
    List,
    Rotate { key: String },
    Audit,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum StrandArg {
    Claude,
    Grok,
    Gemini,
    All,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum DeployTarget {
    Amazon,
    Local,
    AmazonRoom,
    City,
    NpcSuite,
}
