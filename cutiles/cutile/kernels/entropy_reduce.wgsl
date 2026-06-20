// Portable entropy reduction — mirrors blackwell_entropy_v2 scalar outputs.
// One thread per DOF; atomics accumulate block totals (acceptable for MVP).

struct Params {
    total_dof: u32,
    np: u32,
    nu: f32,
    tau: f32,
    surge_threshold: f32,
    prev_w_avg: f32,
}

@group(0) @binding(0) var<storage, read> omega_tilde: array<f32>;
@group(0) @binding(1) var<storage, read> d_perp_rho_sq: array<f32>;
@group(0) @binding(2) var<storage, read> rho: array<f32>;
@group(0) @binding(3) var<storage, read> strain_norms: array<f32>;
@group(0) @binding(4) var<uniform> params: Params;
@group(0) @binding(5) var<storage, read_write> out_w: atomic<u32>;
@group(0) @binding(6) var<storage, read_write> out_visc: atomic<u32>;
@group(0) @binding(7) var<storage, read_write> out_stretch: atomic<u32>;
@group(0) @binding(8) var<storage, read_write> out_betti: atomic<u32>;

fn f32_to_bits(x: f32) -> u32 {
    return bitcast<u32>(x);
}

fn bits_to_f32(x: u32) -> f32 {
    return bitcast<f32>(x);
}

fn is_finite_f32(x: f32) -> bool {
    return x == x && abs(x) < 3.402823e+38;
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if idx >= params.total_dof {
        return;
    }

    let grad_len = max(params.np, 1u);
    let strain_len = max(params.np, 1u);
    var point = idx % grad_len;
    if point >= grad_len {
        point = idx % grad_len;
    }

    let r = rho[idx];
    if !is_finite_f32(r) {
        return;
    }

    let grad_sq = d_perp_rho_sq[point % grad_len];
    let omega = omega_tilde[idx];
    let strain = strain_norms[point % strain_len];

    let local_w = params.tau * grad_sq + r;
    let local_visc = grad_sq;
    let local_stretch = strain * omega;

    loop {
        let old_bits = atomicLoad(&out_w);
        let new_bits = f32_to_bits(bits_to_f32(old_bits) + local_w);
        let res = atomicCompareExchangeWeak(&out_w, old_bits, new_bits);
        if res.exchanged {
            break;
        }
    }

    loop {
        let old_bits = atomicLoad(&out_visc);
        let new_bits = f32_to_bits(bits_to_f32(old_bits) - params.nu * local_visc);
        let res = atomicCompareExchangeWeak(&out_visc, old_bits, new_bits);
        if res.exchanged {
            break;
        }
    }

    loop {
        let old_bits = atomicLoad(&out_stretch);
        let new_bits = f32_to_bits(bits_to_f32(old_bits) - params.tau * local_stretch);
        let res = atomicCompareExchangeWeak(&out_stretch, old_bits, new_bits);
        if res.exchanged {
            break;
        }
    }

    if grad_sq > params.surge_threshold {
        loop {
            let old_bits = atomicLoad(&out_betti);
            let new_bits = f32_to_bits(bits_to_f32(old_bits) + 1.0);
            let res = atomicCompareExchangeWeak(&out_betti, old_bits, new_bits);
            if res.exchanged {
                break;
            }
        }
    }
}