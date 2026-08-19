//! Binary Reed–Muller codes \(\mathrm{RM}(r,m)\).
//!
//! \[
//! n=2^m,\quad k=\sum_{i=0}^{r}\binom{m}{i},\quad d=2^{m-r}
//! \]
//!
//! Dual: \(\mathrm{RM}(r,m)^\perp = \mathrm{RM}(m-r-1,m)\).
//! Plotkin: \(\mathrm{RM}(r,m)=\{(|u|\,u+v|): u\in\mathrm{RM}(r,m-1),\,v\in\mathrm{RM}(r-1,m-1)\}\).
//!
//! **Not** the Golay code (\(n=24\neq 2^m\)). Category **B** runtime lab;
//! classical parameters (A) with executable encode / order-1 FHT decode.

/// Parameters of \(\mathrm{RM}(r,m)\).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RmParams {
    pub r: u32,
    pub m: u32,
    pub n: usize,
    pub k: usize,
    pub d: usize,
    pub t: usize, // floor((d-1)/2)
}

impl RmParams {
    pub fn new(r: u32, m: u32) -> Option<Self> {
        if r > m || m > 12 {
            // m>12 → n>4096; keep lab interactive
            return None;
        }
        let n = 1usize << m;
        let k = (0..=r).map(|i| binom(m, i)).sum();
        let d = 1usize << (m - r);
        let t = (d.saturating_sub(1)) / 2;
        Some(Self { r, m, n, k, d, t })
    }

    pub fn dual(self) -> Option<Self> {
        if self.m >= self.r + 1 {
            Self::new(self.m - self.r - 1, self.m)
        } else {
            None
        }
    }

    pub fn label(self) -> String {
        format!(
            "RM({},{})  [{},{},{}]_2  t≤{}",
            self.r, self.m, self.n, self.k, self.d, self.t
        )
    }

    pub fn classical_name(self) -> Option<&'static str> {
        match (self.r, self.m) {
            (1, 3) => Some("extended Hamming [8,4,4]"),
            (1, 4) => Some("1st-order RM [16,5,8]"),
            (2, 5) => Some("[32,16,8]"),
            (0, m) if m == self.m => Some("repetition"),
            (r, m) if r == m => Some("full space"),
            (r, m) if r + 1 == m => Some("even-weight subcode"),
            (r, m) if r + 2 == m => Some("extended Hamming family"),
            _ => None,
        }
    }
}

/// Binomial coefficient \(\binom{n}{k}\).
pub fn binom(n: u32, k: u32) -> usize {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut acc = 1usize;
    for i in 0..k {
        acc = acc * (n - i) as usize / (i + 1) as usize;
    }
    acc
}

/// Coordinates of \(\mathbb{F}_2^m\) in standard binary order \(0..2^m-1\).
#[inline]
fn coord_bit(x: usize, j: u32) -> u8 {
    ((x >> j) & 1) as u8
}

/// All monomials of degree ≤ r as evaluation vectors (generator rows).
/// Order: degree-lex — constant, then x_0..x_{m-1}, then products...
pub fn generator_rows(r: u32, m: u32) -> Vec<Vec<u8>> {
    let n = 1usize << m;
    let mut rows = Vec::new();
    // Enumerate subsets of {0..m-1} of size ≤ r
    let max_mask = 1u32 << m;
    for mask in 0..max_mask {
        let deg = mask.count_ones();
        if deg > r {
            continue;
        }
        let mut row = vec![0u8; n];
        for x in 0..n {
            let mut v = 1u8;
            for j in 0..m {
                if (mask >> j) & 1 == 1 {
                    v &= coord_bit(x, j);
                }
            }
            row[x] = v;
        }
        rows.push(row);
    }
    // Stable order by popcount then mask (matches sum binom)
    // Ordered by degree then mask value
    let mut ordered = Vec::new();
    for deg in 0..=r {
        for mask in 0..max_mask {
            if mask.count_ones() != deg {
                continue;
            }
            let mut row = vec![0u8; n];
            for x in 0..n {
                let mut v = 1u8;
                for j in 0..m {
                    if (mask >> j) & 1 == 1 {
                        v &= coord_bit(x, j);
                    }
                }
                row[x] = v;
            }
            ordered.push(row);
        }
    }
    let _ = rows;
    ordered
}

/// Encode message bits (length k) as codeword of length n.
pub fn encode(r: u32, m: u32, msg: &[u8]) -> Option<Vec<u8>> {
    let p = RmParams::new(r, m)?;
    if msg.len() < p.k {
        return None;
    }
    let rows = generator_rows(r, m);
    assert_eq!(rows.len(), p.k);
    let mut c = vec![0u8; p.n];
    for (i, row) in rows.iter().enumerate() {
        if msg[i] & 1 == 1 {
            for x in 0..p.n {
                c[x] ^= row[x];
            }
        }
    }
    Some(c)
}

/// Fast Hadamard Transform over \(\{\pm 1\}\) (in-place butterfly).
/// Input length must be a power of 2.
pub fn fht(a: &mut [i32]) {
    let n = a.len();
    debug_assert!(n.is_power_of_two());
    let mut h = 1;
    while h < n {
        let mut i = 0;
        while i < n {
            for j in i..i + h {
                let x = a[j];
                let y = a[j + h];
                a[j] = x + y;
                a[j + h] = x - y;
            }
            i += 2 * h;
        }
        h *= 2;
    }
}

/// ML decode for \(\mathrm{RM}(1,m)\) via FHT (Hadamard / simplex family).
///
/// Maps bits \(0/1 \to +1/-1\), runs FHT, picks max |spectrum| peak for the
/// affine linear function. Corrects \(t = 2^{m-2}-1\) errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rm1Decode {
    pub corrected: Vec<u8>,
    pub message: Vec<u8>, // k = m+1 bits: [const, x0, ..., x_{m-1}]
    pub peak: i32,
    pub accepted: bool,
}

pub fn decode_rm1_fht(y: &[u8], m: u32) -> Option<Rm1Decode> {
    let p = RmParams::new(1, m)?;
    if y.len() != p.n {
        return None;
    }
    let mut a: Vec<i32> = y
        .iter()
        .map(|&b| if b & 1 == 0 { 1 } else { -1 })
        .collect();
    fht(&mut a);
    // Spectrum index encodes the linear functional; max |a[i]| wins.
    let mut best_i = 0usize;
    let mut best_abs = 0i32;
    for (i, &v) in a.iter().enumerate() {
        let av = v.abs();
        if av > best_abs {
            best_abs = av;
            best_i = i;
        }
    }
    // Message: constant = sign, linear bits from best_i
    // Convention: FHT peak at i means correlation with character χ_i.
    // For RM(1,m), codewords are eval of a0 + a·x; Hadamard index often
    // matches the linear part when constant is absorbed in sign.
    let sign_neg = a[best_i] < 0;
    let mut msg = vec![0u8; p.k];
    msg[0] = if sign_neg { 1 } else { 0 };
    for j in 0..m as usize {
        msg[1 + j] = ((best_i >> j) & 1) as u8;
    }
    // If peak is negative, some conventions flip constant — re-encode and
    // pick better of two constant flips for honesty under noise.
    let c0 = encode(1, m, &msg)?;
    let mut msg1 = msg.clone();
    msg1[0] ^= 1;
    let c1 = encode(1, m, &msg1)?;
    let d0: u32 = y
        .iter()
        .zip(c0.iter())
        .map(|(a, b)| (a ^ b) as u32)
        .sum();
    let d1: u32 = y
        .iter()
        .zip(c1.iter())
        .map(|(a, b)| (a ^ b) as u32)
        .sum();
    let (corrected, message, dist) = if d0 <= d1 {
        (c0, msg, d0)
    } else {
        (c1, msg1, d1)
    };
    Some(Rm1Decode {
        corrected,
        message,
        peak: best_abs,
        accepted: dist as usize <= p.t,
    })
}

/// Plotkin combine: \(|u|\,u+v|\) from two half-length vectors.
pub fn plotkin_combine(u: &[u8], v: &[u8]) -> Option<Vec<u8>> {
    if u.len() != v.len() {
        return None;
    }
    let mut out = Vec::with_capacity(u.len() * 2);
    out.extend_from_slice(u);
    for i in 0..u.len() {
        out.push(u[i] ^ v[i]);
    }
    Some(out)
}

/// Hamming distance.
pub fn distance(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
}

/// Inject `t` errors deterministically.
pub fn inject_errors(c: &[u8], t: usize, seed: u32) -> Vec<u8> {
    let mut y = c.to_vec();
    let n = y.len();
    if n == 0 || t == 0 {
        return y;
    }
    let mut s = seed;
    let mut flipped = 0usize;
    let mut used = vec![false; n];
    while flipped < t.min(n) {
        s = s.wrapping_mul(1_103_515_245).wrapping_add(12345);
        let i = ((s >> 16) as usize) % n;
        if !used[i] {
            y[i] ^= 1;
            used[i] = true;
            flipped += 1;
        }
    }
    y
}

/// G24 is not an RM code — parameter witness for UI.
pub fn golay_is_not_rm() -> bool {
    // 24 is not a power of 2
    !(24usize.is_power_of_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_rm13_extended_hamming() {
        let p = RmParams::new(1, 3).unwrap();
        assert_eq!((p.n, p.k, p.d), (8, 4, 4));
        assert_eq!(p.classical_name(), Some("extended Hamming [8,4,4]"));
    }

    #[test]
    fn params_rm14() {
        let p = RmParams::new(1, 4).unwrap();
        assert_eq!((p.n, p.k, p.d), (16, 5, 8));
    }

    #[test]
    fn duality_rm1m() {
        let p = RmParams::new(1, 5).unwrap();
        let d = p.dual().unwrap();
        // RM(1,5)^perp = RM(3,5)
        assert_eq!((d.r, d.m), (3, 5));
        assert_eq!(d.k, p.n - p.k); // for this pair dim adds? dim dual = n-k for non-deg
        // RM(1,5): k=6, n=32; RM(3,5): k=1+5+10+10=26; 6+26=32
        assert_eq!(p.k + d.k, p.n);
    }

    #[test]
    fn encode_dim_matches() {
        let p = RmParams::new(1, 4).unwrap();
        let rows = generator_rows(1, 4);
        assert_eq!(rows.len(), p.k);
        assert_eq!(rows[0].len(), p.n);
    }

    #[test]
    fn fht_rm1_corrects() {
        let m = 4u32;
        let p = RmParams::new(1, m).unwrap();
        let msg = vec![1u8, 0, 1, 1, 0];
        let c = encode(1, m, &msg).unwrap();
        assert_eq!(c.len(), p.n);
        let y = inject_errors(&c, p.t.min(3), 7);
        let d = decode_rm1_fht(&y, m).unwrap();
        assert!(d.accepted, "dist peak failed");
        assert_eq!(d.corrected, c);
    }

    #[test]
    fn plotkin_length() {
        let u = vec![0, 1, 1, 0];
        let v = vec![1, 1, 0, 0];
        let w = plotkin_combine(&u, &v).unwrap();
        assert_eq!(w.len(), 8);
        assert_eq!(&w[..4], &u[..]);
        assert_eq!(w[4], 1);
        assert_eq!(w[5], 0);
    }

    #[test]
    fn golay_not_rm() {
        assert!(golay_is_not_rm());
    }
}
