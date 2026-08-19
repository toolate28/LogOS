//! Hexacode \(H \subset \mathbb{F}_4^6\) — \([6,3,4]_4\) MDS.
//!
//! Twin of Lean `K22.HexacodeGolay` GF(4) Nat engine (runtime Category **B**
//! unless pinned against a green table). Parameters and weight distribution
//! are classical; membership matches `isHexacodewordN` / cutile `mog_bridge`.

/// GF(4) symbols coded as `0,1,2,3` (= \(0,1,\omega,\bar\omega\)).
pub type Gf4 = u8;

/// Code parameters.
pub const N: usize = 6;
pub const K: usize = 3;
pub const D: usize = 4;
pub const M: usize = 64; // 4^3
pub const CORRECT_T: usize = 1;
pub const DETECT_UP_TO: usize = 3;
pub const COVERING_RADIUS: usize = 2;

/// GF(4) addition.
#[inline]
pub const fn gf_add(x: Gf4, y: Gf4) -> Gf4 {
    match (x, y) {
        (0, y) => y,
        (x, 0) => x,
        (1, 1) => 0,
        (1, 2) => 3,
        (1, 3) => 2,
        (2, 1) => 3,
        (2, 2) => 0,
        (2, 3) => 1,
        (3, 1) => 2,
        (3, 2) => 1,
        (3, 3) => 0,
        _ => 0,
    }
}

/// GF(4) multiplication.
#[inline]
pub const fn gf_mul(x: Gf4, y: Gf4) -> Gf4 {
    match (x, y) {
        (0, _) | (_, 0) => 0,
        (1, y) => y,
        (x, 1) => x,
        (2, 2) => 3,
        (2, 3) => 1,
        (3, 2) => 1,
        (3, 3) => 2,
        _ => 0,
    }
}

/// Dot product of two length-6 vectors.
#[inline]
pub const fn gf_dot6(a: [Gf4; 6], w: [Gf4; 6]) -> Gf4 {
    let mut s = 0u8;
    let mut i = 0;
    while i < 6 {
        s = gf_add(s, gf_mul(a[i], w[i]));
        i += 1;
    }
    s
}

/// Parity-check rows (same as Lean / cutile `is_hexacodeword_n`).
const H_ROW0: [Gf4; 6] = [1, 1, 1, 1, 0, 0];
const H_ROW1: [Gf4; 6] = [1, 2, 3, 0, 1, 0];
const H_ROW2: [Gf4; 6] = [1, 3, 2, 0, 0, 1];

/// Membership oracle — hexacode word?
pub const fn is_hexacodeword(w: [Gf4; 6]) -> bool {
    gf_dot6(H_ROW0, w) == 0 && gf_dot6(H_ROW1, w) == 0 && gf_dot6(H_ROW2, w) == 0
}

/// Hamming weight over \(\mathbb{F}_4\) (nonzero coordinates).
pub fn weight(w: &[Gf4; 6]) -> usize {
    w.iter().filter(|&&x| x != 0).count()
}

/// Hamming distance.
pub fn distance(a: &[Gf4; 6], b: &[Gf4; 6]) -> usize {
    a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
}

/// Systematic generator: message \((m_0,m_1,m_2)\) → codeword via dual of H.
/// Rows of a generator orthogonal to H (parametric form: free last 3 coords
/// when using dual basis). We enumerate all 64 words by free message bits
/// on a known generator matrix \(G = [I_3 | A]\).
///
/// Generator (standard hexacode form used with MOG column scores):
/// free symbols \(u_0,u_1,u_2\) place at positions 3,4,5 and determine 0,1,2
/// so that \(Hw=0\). Solved offline to match Lean `hexacodeGenerator`.
///
/// Practical: linear span of three independent codewords.
const GEN_ROWS: [[Gf4; 6]; 3] = [
    [1, 0, 0, 1, 1, 1],
    [0, 1, 0, 1, 2, 3],
    [0, 0, 1, 1, 3, 2],
];

/// Encode message \(m \in \mathbb{F}_4^3\).
pub fn encode(m: [Gf4; 3]) -> [Gf4; 6] {
    let mut w = [0u8; 6];
    for (j, row) in GEN_ROWS.iter().enumerate() {
        if m[j] == 0 {
            continue;
        }
        for i in 0..6 {
            w[i] = gf_add(w[i], gf_mul(m[j], row[i]));
        }
    }
    w
}

/// Enumerate all 64 codewords.
pub fn all_codewords() -> Vec<[Gf4; 6]> {
    let mut out = Vec::with_capacity(64);
    for a in 0u8..4 {
        for b in 0u8..4 {
            for c in 0u8..4 {
                out.push(encode([a, b, c]));
            }
        }
    }
    out
}

/// Syndrome \(\sigma = H_{\mathrm{par}} y^\top \in \mathbb{F}_4^3\).
pub fn syndrome(y: [Gf4; 6]) -> [Gf4; 3] {
    [
        gf_dot6(H_ROW0, y),
        gf_dot6(H_ROW1, y),
        gf_dot6(H_ROW2, y),
    ]
}

/// Single-error syndrome decoder (\(t=1\)).
///
/// Returns corrected word + error position/value, or `None` if uncorrectable
/// (including double errors — Category B bounded-distance).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexDecode {
    pub corrected: [Gf4; 6],
    pub error_pos: Option<usize>,
    pub error_val: Gf4,
    pub unique: bool,
}

pub fn decode_syndrome(y: [Gf4; 6]) -> HexDecode {
    let s = syndrome(y);
    if s == [0, 0, 0] {
        return HexDecode {
            corrected: y,
            error_pos: None,
            error_val: 0,
            unique: true,
        };
    }
    // Columns of H: for each position j and e ∈ F4\{0}, σ ≟ e · col_j
    let cols: [[Gf4; 3]; 6] = [
        [H_ROW0[0], H_ROW1[0], H_ROW2[0]],
        [H_ROW0[1], H_ROW1[1], H_ROW2[1]],
        [H_ROW0[2], H_ROW1[2], H_ROW2[2]],
        [H_ROW0[3], H_ROW1[3], H_ROW2[3]],
        [H_ROW0[4], H_ROW1[4], H_ROW2[4]],
        [H_ROW0[5], H_ROW1[5], H_ROW2[5]],
    ];
    for j in 0..6 {
        for e in 1u8..4 {
            let col = [
                gf_mul(e, cols[j][0]),
                gf_mul(e, cols[j][1]),
                gf_mul(e, cols[j][2]),
            ];
            if col == s {
                let mut c = y;
                c[j] = gf_add(c[j], e); // subtract e (char 2)
                return HexDecode {
                    corrected: c,
                    error_pos: Some(j),
                    error_val: e,
                    unique: true,
                };
            }
        }
    }
    // Detectable but not uniquely correctable
    HexDecode {
        corrected: y,
        error_pos: None,
        error_val: 0,
        unique: false,
    }
}

/// Nearest-neighbour among 64 words (unique for \(t\le 1\)).
pub fn decode_nn(y: [Gf4; 6]) -> HexDecode {
    let mut best = y;
    let mut best_d = 7usize;
    let mut ties = 0u32;
    for c in all_codewords() {
        let d = distance(&y, &c);
        if d < best_d {
            best_d = d;
            best = c;
            ties = 1;
        } else if d == best_d {
            ties += 1;
        }
    }
    if best_d <= CORRECT_T && ties == 1 {
        let mut err_pos = None;
        let mut err_val = 0u8;
        for i in 0..6 {
            if y[i] != best[i] {
                err_pos = Some(i);
                err_val = gf_add(y[i], best[i]);
                break;
            }
        }
        HexDecode {
            corrected: best,
            error_pos: err_pos,
            error_val: err_val,
            unique: true,
        }
    } else {
        HexDecode {
            corrected: y,
            error_pos: None,
            error_val: 0,
            unique: false,
        }
    }
}

/// Weight distribution \(A(x)=1+45x^4+18x^6\) (verify by enumeration).
pub fn weight_distribution() -> (u32, u32, u32) {
    let mut a4 = 0u32;
    let mut a6 = 0u32;
    let mut a0 = 0u32;
    for c in all_codewords() {
        match weight(&c) {
            0 => a0 += 1,
            4 => a4 += 1,
            6 => a6 += 1,
            _ => {}
        }
    }
    (a0, a4, a6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_mds() {
        assert_eq!(D, N - K + 1);
        assert_eq!(all_codewords().len(), M);
    }

    #[test]
    fn all_encodes_are_codewords() {
        for c in all_codewords() {
            assert!(is_hexacodeword(c), "{c:?}");
        }
    }

    #[test]
    fn weight_dist_classical() {
        let (a0, a4, a6) = weight_distribution();
        assert_eq!(a0, 1);
        assert_eq!(a4, 45);
        assert_eq!(a6, 18);
    }

    #[test]
    fn corrects_single_error() {
        let c = encode([1, 2, 3]);
        for pos in 0..6 {
            for e in 1u8..4 {
                let mut y = c;
                y[pos] = gf_add(y[pos], e);
                let d = decode_syndrome(y);
                assert!(d.unique, "pos={pos} e={e}");
                assert_eq!(d.corrected, c);
                assert_eq!(d.error_pos, Some(pos));
            }
        }
    }

    #[test]
    fn nn_matches_syndrome_on_t1() {
        let c = encode([0, 1, 2]);
        let mut y = c;
        y[4] = gf_add(y[4], 2);
        assert_eq!(decode_nn(y).corrected, decode_syndrome(y).corrected);
    }
}
