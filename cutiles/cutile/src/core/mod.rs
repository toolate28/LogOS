pub mod c12_hexaflake;
pub mod entropy;
pub mod hexaflake;
pub mod r_matrix;
pub mod srac;
pub mod srac_strategies;

pub use c12_hexaflake::{
    c12_average_harmony, c12_group_average, c12_harmony_contribution, c12_hexaflake_nodes,
    c12_orbit, rotate_hex_point_30deg, C12HexaflakeTiler,
};
pub use entropy::{betti_proxy, compute_entropy_diagnostic};
pub use hexaflake::hexaflake_nodes;
pub use r_matrix::{
    flatten_r_matrix, fundamental_r_matrix, is_conserved, Mat4C, CONSERVATION_SUM,
};
pub use srac::{srac_cascade_step, SRACorrection};
pub use srac_strategies::{apply_correction, DivergenceReason, SracState};