//! Reusable kernel launch helpers (TriWeavon occupancy hints).

pub const DEFAULT_BLOCK: u32 = 256;

pub fn grid_dim(total: u32, block: u32) -> u32 {
    (total.saturating_add(block.saturating_sub(1))) / block.max(1)
}