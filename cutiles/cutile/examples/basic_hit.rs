use cutile::{hexaflake_nodes, TriWeavonHIT};

fn main() {
    let mut hit = TriWeavonHIT::new();
    let a = hit.add_point();
    let b = hit.add_point();
    let weave = hit.weave(a, b);
    let nodes = hexaflake_nodes(2);
    println!("cutile basic_hit — points={} weave={weave} hexaflake_nodes={}", hit.point_count(), nodes.len());
}