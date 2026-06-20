use std::path::Path;

fn main() {
    let ptx = Path::new("kernels/blackwell_entropy_v2.ptx");
    let cu = Path::new("kernels/blackwell_entropy_v2.cu");

    let wgsl = Path::new("kernels/entropy_reduce.wgsl");

    println!("cargo:rerun-if-changed={}", cu.display());
    println!("cargo:rerun-if-changed={}", ptx.display());
    println!("cargo:rerun-if-changed={}", wgsl.display());

    if ptx.exists() {
        println!("cargo:rustc-cfg=ptx_embedded");
    }
}