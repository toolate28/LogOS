//! TdaLaunchProvenance and related types for cryptographic hardware logging.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TdaTilingPhase {
    Filtration,
    BoundaryReduction,
    HistoricalCompare,
}

#[derive(Debug, Clone)]
pub struct TdaLaunchProvenance {
    pub phase: TdaTilingPhase,
    pub launch_config_hash: String,
    pub algorithm: String,
    pub reduction_mode: String,
    pub apparent_pairs_count: u64,
    pub reduction_iterations: u64,
    pub execution_time_ms: Option<f64>,
    pub shared_memory_bytes: usize,
    pub block_size: u32,
    pub grid_size: u32,
}