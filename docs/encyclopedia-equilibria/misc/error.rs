use thiserror::Error;

#[derive(Error, Debug)]
pub enum CudarcError {
    #[error("CUDA driver error: {0}")]
    Driver(#[from] cudarc::driver::DriverError),
    #[error("TriWeavon manifold invariant violation: {0}")]
    Invariant(String),
    #[error("Coherence computation failed")]
    CoherenceError,
}
