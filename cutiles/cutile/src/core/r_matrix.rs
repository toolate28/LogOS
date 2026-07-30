//! Fundamental R-matrix (U_q(sl_2)-style 4×4) — cascade L2 canonical executable.
//!
//! Mirrors:
//! - `kernels/fundamental_r_matrix.cu` / `.wgsl`
//! - notebook `Agent_M24_RMatrix.ipynb` (nalgebra cell)
//!
//! Layout: row-major complex entries as `(re, im)`.

/// Complex number as (re, im).
pub type C = (f64, f64);

/// 4×4 complex matrix, row-major.
pub type Mat4C = [[C; 4]; 4];

/// Fundamental R-matrix for deformation parameter `q` (`q ≠ 0`).
///
/// ```text
/// [ q     0      0     0  ]
/// [ 0    1/q   1-q²    0  ]
/// [ 0     0      q     0  ]
/// [ 0     0      0    1/q ]
/// ```
pub fn fundamental_r_matrix(q: f64) -> Mat4C {
    assert!(q != 0.0 && q.is_finite(), "q must be finite and non-zero");
    let z = (0.0, 0.0);
    let q_val = (q, 0.0);
    let q_inv = (1.0 / q, 0.0);
    let off = (1.0 - q * q, 0.0);
    [
        [q_val, z, z, z],
        [z, q_inv, off, z],
        [z, z, q_val, z],
        [z, z, z, q_inv],
    ]
}

/// Flatten to 16 `(re, im)` pairs for JSON / FFI / receipts.
pub fn flatten_r_matrix(m: &Mat4C) -> Vec<(f64, f64)> {
    m.iter().flat_map(|row| row.iter().copied()).collect()
}

/// Conservation dual: ω = 15 − α (Universal Invariant).
pub const CONSERVATION_SUM: u8 = 15;

pub fn is_conserved(alpha: u8, omega: u8) -> bool {
    alpha.saturating_add(omega) == CONSERVATION_SUM
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_matrix_sqrt2_off_diag() {
        let q = 2.0_f64.sqrt();
        let m = fundamental_r_matrix(q);
        assert!((m[0][0].0 - q).abs() < 1e-12);
        assert!((m[1][1].0 - 1.0 / q).abs() < 1e-12);
        assert!((m[1][2].0 - (1.0 - q * q)).abs() < 1e-12);
        assert!((m[2][2].0 - q).abs() < 1e-12);
        assert!((m[3][3].0 - 1.0 / q).abs() < 1e-12);
    }

    #[test]
    fn conservation_peak() {
        assert!(is_conserved(7, 8));
        assert!(!is_conserved(8, 8));
    }

    #[test]
    fn flatten_len_16() {
        let flat = flatten_r_matrix(&fundamental_r_matrix(1.5));
        assert_eq!(flat.len(), 16);
    }
}
