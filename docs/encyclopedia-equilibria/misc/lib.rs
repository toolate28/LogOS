pub mod tile;
pub mod launch;
pub mod memory;
pub mod strategy;
pub mod error;

pub use error::CutileError;
pub type Result<T> = std::result::Result<T, CutileError>;
