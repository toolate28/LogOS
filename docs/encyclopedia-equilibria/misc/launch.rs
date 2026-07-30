use cudarc::driver::LaunchConfig;

pub fn safe_launch_config(grid: (u32, u32, u32), block: (u32, u32, u32)) -> LaunchConfig {
    LaunchConfig {
        grid_dim: grid,
        block_dim: block,
        shared_mem_bytes: 0,
    }
}

// TriWeavon-specific defaults for occupancy and shared memory for sheaf traversals
pub fn manifold_default_launch(problem_size: usize) -> LaunchConfig {
    // TODO: compute optimal based on device
    safe_launch_config(((problem_size as u32 + 255) / 256, 1, 1), (256, 1, 1))
}
