//! Extended binary Golay \(G_{24}=[24,12,8]_2\) via MOG + hexacode + \(R_{\mathrm{parity}}\).
//!
//! Runtime twin of Lean `golayMaskOkN` / cutile `mog_bridge` (Category **B**).
//! Construction enumerators / Steiner claims are green on `HexacodeGolay` (A);
//! decoders here are bounded-distance runtime unless pinned to a verified table.
//!
//! Corrects \(t=3\) binary errors (unique nearest neighbour for wt ≤ 3).

use super::hexacode::{is_hexacodeword, Gf4};

/// Golay parameters.
pub const N: usize = 24;
pub const K: usize = 12;
pub const D: usize = 8;
pub const M: usize = 4096;
pub const CORRECT_T: usize = 3;
/// Cosets with weight-≤3 leaders (sphere packing for \(t=3\)).
pub const RADIUS3_COSETS: usize = 2325;
/// Total cosets \(2^{12}\).
pub const TOTAL_COSETS: usize = 4096;

/// Same 12-basis masks as Lean `golayBasisN` / cutile tests.
pub const GOLAY_BASIS: [u32; 12] = [
    12_782_640, 6_391_320, 3_195_660, 1_597_830, 798_915, 266_366, 198_419, 117_552, 3_510,
    26_405, 975, 38_117,
];

#[inline]
pub const fn bit_val(n: u32, i: u32) -> u32 {
    (n >> i) & 1
}

#[inline]
pub const fn bit_on(n: u32, i: u32) -> bool {
    bit_val(n, i) == 1
}

#[inline]
pub const fn hamming_wt(n: u32) -> u32 {
    n.count_ones()
}

/// Score of MOG column `c` (rows 1–3 labeled 1,2,3; row 0 unlabeled).
#[inline]
pub const fn mask_col_score_n(n: u32, c: u32) -> u32 {
    // bit layout: row r, col c → bit (6*r + c)
    let b1 = bit_val(n, 6 + c);
    let b2 = bit_val(n, 12 + c);
    let b3 = bit_val(n, 18 + c);
    // gf_add(b1, gf_mul(2,b2), gf_mul(3,b3)) as u32 codes
    let t = match (b1, b2, b3) {
        // expand via same tables as hexacode
        _ => {
            let mut s = b1;
            // mul 2 * b2
            let m2 = if b2 == 0 { 0 } else { 2 };
            s = gf_add_u(s, m2);
            let m3 = if b3 == 0 { 0 } else { 3 };
            s = gf_add_u(s, m3);
            s
        }
    };
    t
}

const fn gf_add_u(x: u32, y: u32) -> u32 {
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

#[inline]
pub const fn mask_col_count_n(n: u32, c: u32) -> u32 {
    bit_val(n, c) + bit_val(n, 6 + c) + bit_val(n, 12 + c) + bit_val(n, 18 + c)
}

#[inline]
pub const fn mask_top_count_n(n: u32) -> u32 {
    bit_val(n, 0)
        + bit_val(n, 1)
        + bit_val(n, 2)
        + bit_val(n, 3)
        + bit_val(n, 4)
        + bit_val(n, 5)
}

/// MOG membership: hexacode scores + column parity residual \(R_{\mathrm{parity}}\).
pub const fn golay_mask_ok(n: u32) -> bool {
    let scores: [u32; 6] = [
        mask_col_score_n(n, 0),
        mask_col_score_n(n, 1),
        mask_col_score_n(n, 2),
        mask_col_score_n(n, 3),
        mask_col_score_n(n, 4),
        mask_col_score_n(n, 5),
    ];
    let w: [Gf4; 6] = [
        scores[0] as u8,
        scores[1] as u8,
        scores[2] as u8,
        scores[3] as u8,
        scores[4] as u8,
        scores[5] as u8,
    ];
    if !is_hexacodeword(w) {
        return false;
    }
    let p = mask_top_count_n(n) % 2;
    mask_col_count_n(n, 0) % 2 == p
        && mask_col_count_n(n, 1) % 2 == p
        && mask_col_count_n(n, 2) % 2 == p
        && mask_col_count_n(n, 3) % 2 == p
        && mask_col_count_n(n, 4) % 2 == p
        && mask_col_count_n(n, 5) % 2 == p
}

/// Linear encode: 12 message bits → 24-bit codeword.
pub fn encode(msg: u32) -> u32 {
    let mut acc = 0u32;
    for i in 0..12 {
        if bit_on(msg, i as u32) {
            acc ^= GOLAY_BASIS[i];
        }
    }
    acc
}

/// Enumerate all 4096 codewords (cached helper for NN).
pub fn all_codewords() -> Vec<u32> {
    (0u32..4096).map(encode).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GolayDecode {
    pub corrected: u32,
    pub error_mask: u32,
    pub distance: u32,
    /// True when unique NN at distance ≤ 3 (or y already a codeword).
    pub unique: bool,
    /// Bounded-distance accepted (wt(e) ≤ 3).
    pub accepted: bool,
}

/// Nearest-neighbour decode among 4096 codewords (guaranteed for wt(e) ≤ 3).
pub fn decode_nn(y: u32) -> GolayDecode {
    let y = y & 0x00FF_FFFF; // 24 bits
    let mut best_c = 0u32;
    let mut best_d = 25u32;
    let mut ties = 0u32;
    for msg in 0u32..4096 {
        let c = encode(msg);
        let d = hamming_wt(y ^ c);
        if d < best_d {
            best_d = d;
            best_c = c;
            ties = 1;
        } else if d == best_d {
            ties += 1;
        }
    }
    let e = y ^ best_c;
    GolayDecode {
        corrected: best_c,
        error_mask: e,
        distance: best_d,
        unique: ties == 1,
        accepted: best_d <= CORRECT_T as u32 && ties == 1,
    }
}

/// Exhaustive search over error patterns of weight ≤ `t` (coset-leader style).
/// Finds a codeword at distance ≤ t if one exists uniquely among those patterns.
pub fn decode_bounded(y: u32, t: u32) -> GolayDecode {
    let y = y & 0x00FF_FFFF;
    if golay_mask_ok(y) {
        return GolayDecode {
            corrected: y,
            error_mask: 0,
            distance: 0,
            unique: true,
            accepted: true,
        };
    }
    // Prefer NN for correctness; same guarantee at t=3.
    let nn = decode_nn(y);
    if nn.distance <= t && nn.unique {
        nn
    } else {
        GolayDecode {
            corrected: y,
            error_mask: 0,
            distance: nn.distance,
            unique: false,
            accepted: false,
        }
    }
}

/// Flip up to `t` random-ish bits (deterministic from seed) for demos.
pub fn inject_errors(codeword: u32, t: u32, seed: u32) -> u32 {
    let mut y = codeword & 0x00FF_FFFF;
    let mut used = 0u32;
    let mut s = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
    let mut flipped = 0u32;
    while flipped < t {
        s = s.wrapping_mul(1_103_515_245).wrapping_add(12345);
        let bit = (s >> 16) % 24;
        let mask = 1u32 << bit;
        if used & mask == 0 {
            y ^= mask;
            used |= mask;
            flipped += 1;
        }
    }
    y
}

/// Count octads (wt-8 codewords) by full enumeration — should be 759.
pub fn count_octads() -> u32 {
    octads_from_basis().len() as u32
}

/// All 759 weight-8 supports of G24, indices ascending in `{0..23}`.
pub fn octads_from_basis() -> Vec<[u8; 8]> {
    let mut out = Vec::with_capacity(759);
    for msg in 0u32..4096 {
        let w = encode(msg);
        if hamming_wt(w) != 8 {
            continue;
        }
        let mut support = [0u8; 8];
        let mut k = 0;
        for i in 0..24 {
            if bit_on(w, i) {
                support[k] = i as u8;
                k += 1;
            }
        }
        out.push(support);
    }
    out
}

/// Empirical NN uniqueness check for random codeword + wt-t error.
pub fn empirical_nn_unique(trials: u32, t: u32, seed0: u32) -> (u32, u32) {
    let mut ok = 0u32;
    let mut s = seed0;
    for _ in 0..trials {
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        let msg = s % 4096;
        let c = encode(msg);
        let y = inject_errors(c, t, s);
        let d = decode_nn(y);
        if d.accepted && d.corrected == c && d.distance == t {
            ok += 1;
        }
    }
    (ok, trials)
}

/// Column scores as hexacode word (for UI).
pub fn scores_of(mask: u32) -> [Gf4; 6] {
    [
        mask_col_score_n(mask, 0) as u8,
        mask_col_score_n(mask, 1) as u8,
        mask_col_score_n(mask, 2) as u8,
        mask_col_score_n(mask, 3) as u8,
        mask_col_score_n(mask, 4) as u8,
        mask_col_score_n(mask, 5) as u8,
    ]
}

/// Hexacode gate half of MOG (scores only).
pub fn hex_gate_ok(mask: u32) -> bool {
    is_hexacodeword(scores_of(mask))
}

/// Binary parity residual gate.
pub fn r_parity_ok(mask: u32) -> bool {
    let p = mask_top_count_n(mask) % 2;
    (0..6).all(|c| mask_col_count_n(mask, c) % 2 == p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_codeword() {
        assert!(golay_mask_ok(0));
        assert_eq!(encode(0), 0);
    }

    #[test]
    fn basis_vectors_ok() {
        for i in 0..12 {
            let w = encode(1 << i);
            assert!(golay_mask_ok(w), "basis {i} mask={w}");
            assert_eq!(w, GOLAY_BASIS[i]);
        }
    }

    #[test]
    fn all_linear_combos_ok_sample() {
        for msg in 0u32..64 {
            assert!(golay_mask_ok(encode(msg)), "msg={msg}");
        }
    }

    #[test]
    fn min_distance_at_least_8_on_sample() {
        // d=8: nonzero codewords have wt ≥ 8
        for msg in 1u32..256 {
            let w = hamming_wt(encode(msg));
            assert!(w == 0 || w >= 8, "msg={msg} wt={w}");
        }
    }

    #[test]
    fn nn_corrects_t_le_3() {
        for t in 1..=3 {
            let (ok, n) = empirical_nn_unique(20, t, 42 + t);
            assert_eq!(ok, n, "t={t} ok={ok}/{n}");
        }
    }

    #[test]
    fn octad_count_is_759() {
        assert_eq!(count_octads(), 759);
    }

    #[test]
    fn steiner_octad_intersection_law() {
        let octads = octads_from_basis();
        assert_eq!(octads.len(), 759);
        let mut pair = [0u32; 9];
        for (i, a) in octads.iter().enumerate() {
            for b in octads.iter().skip(i + 1) {
                let k = a.iter().filter(|x| b.contains(x)).count();
                pair[k] += 1;
            }
        }
        assert_eq!(pair[1], 0);
        assert_eq!(pair[3], 0);
        assert_eq!(pair[5], 0);
        assert_eq!(pair[0], 11_385);
        assert_eq!(pair[2], 170_016);
        assert_eq!(pair[4], 106_260);
    }
}
