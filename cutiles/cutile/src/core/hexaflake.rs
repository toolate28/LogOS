/// Hexaflake tiling nodes for cubical manifold discretization (radius in hex steps).
pub fn hexaflake_nodes(radius: u32) -> Vec<(i32, i32)> {
    let r = radius as i32;
    let mut nodes = Vec::new();
    for q in -r..=r {
        let r1 = (-r).max(-q - r);
        let r2 = r.min(-q + r);
        for s in r1..=r2 {
            nodes.push((q, s));
        }
    }
    nodes
}