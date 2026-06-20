#[derive(Debug, thiserror::Error)]
pub enum CutileError {
    #[error("CUDA error: {0}")]
    Cuda(String),

    #[error("Invalid dimensions: expected {expected}, got {actual}")]
    InvalidDimensions { expected: usize, actual: usize },

    #[error("Numerical instability detected")]
    NumericalInstability,

    #[error("Backend not available: {0}")]
    BackendUnavailable(String),

    #[error("wgpu error: {0}")]
    Wgpu(String),

    #[error("{name} out of range: {value} not in [{min}, {max}]")]
    ParameterOutOfRange {
        name: String,
        value: f32,
        min: f32,
        max: f32,
    },
}