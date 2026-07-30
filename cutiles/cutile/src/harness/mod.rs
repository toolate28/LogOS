//! sm_100 harness + KernelWitness scaffold (HeisenForge v0.2 ingest).

pub mod kernel_witness;
pub mod mehler_levin;
pub mod polarity;

#[cfg(kani)]
pub mod polarity_harness;

pub use kernel_witness::{CutileHarness, KernelWitness, LiftOk};
pub use mehler_levin::{
    MehlerLevinHarness, MehlerLevinResult, CERTIFIED_ERROR_TOL, FAST_PATH_ERROR_TYPICAL,
    N_LEVIN_NODES,
};
pub use polarity::{check_polarity_and_proceed, Polarity, PolarityResult};