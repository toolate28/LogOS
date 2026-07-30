use std::path::Path;

fn main() {
    let entropy_ptx = Path::new("kernels/blackwell_entropy_v2.ptx");
    let entropy_cu = Path::new("kernels/blackwell_entropy_v2.cu");

    let mehler_ptx = Path::new("kernels/mehler_mma_levin_batched.ptx");
    let mehler_cu = Path::new("kernels/mehler_mma_levin_batched.cu");
    let mehler_cuh = Path::new("kernels/gpu_interval_arithmetic.cuh");
    let mehler_ffi_h = Path::new("kernels/mehler_levin_ffi.h");
    let mehler_ffi_cpp = Path::new("kernels/mehler_levin_ffi.cpp");

    let wgsl = Path::new("kernels/entropy_reduce.wgsl");

    println!("cargo:rerun-if-changed={}", entropy_cu.display());
    println!("cargo:rerun-if-changed={}", entropy_ptx.display());
    println!("cargo:rerun-if-changed={}", mehler_cu.display());
    println!("cargo:rerun-if-changed={}", mehler_cuh.display());
    println!("cargo:rerun-if-changed={}", mehler_ptx.display());
    println!("cargo:rerun-if-changed={}", mehler_ffi_h.display());
    println!("cargo:rerun-if-changed={}", mehler_ffi_cpp.display());
    println!("cargo:rerun-if-changed={}", wgsl.display());

    if entropy_ptx.exists() {
        println!("cargo:rustc-cfg=ptx_embedded");
    }

    if mehler_ptx.exists() {
        println!("cargo:rustc-cfg=mehler_ptx_embedded");
    }
}