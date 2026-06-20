#[derive(Debug, thiserror::Error)]
pub enum TriweavonCudarcError {
    #[error("invariant violation: α + ω = {sum}, expected 15")]
    InvariantViolation { sum: u32 },

    #[error("CUDA unavailable: {0}")]
    CudaUnavailable(String),

    #[error("cutile backend: {0}")]
    Cutile(#[from] cutile::CutileError),
}