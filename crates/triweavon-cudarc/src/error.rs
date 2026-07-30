use thiserror::Error;

#[derive(Debug, Error)]
pub enum TriweavonCudarcError {
    #[error("CUDA error: {0}")]
    Cuda(String),
    #[error("Cutile error: {0}")]
    Cutile(#[from] cutile::CutileError),
    #[error("Reduction error: {0}")]
    Reduction(#[from] crate::m24::ReductionError),
    #[error("Lift check failed after SRAC correction")]
    LiftCheckFailedAfterSrac,
}