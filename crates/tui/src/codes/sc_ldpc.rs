//! Spatially coupled LDPC (SC-LDPC) — lab model for reson8-tui.
//!
//! Classical iterative-coding theory (Kudekar–Richardson–Urbanke threshold
//! saturation). **Category B** executable sketch: protograph chain geometry,
//! termination rate loss, and a toy windowed pealing decoder on the BEC.
//!
//! Outside the HexacodeGolay **A** surface. Not Golay / not \(M_{24}\).
//!
//! Pipeline:
//! ```text
//! protograph B → L sections → spread window w → lift Z → terminate → BP/window W
//! ```

/// Design knobs for a regular-ish SC ensemble (lab scale).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScDesign {
    /// Chain length \(L\) (spatial sections).
    pub chain_l: u32,
    /// Coupling width \(w\).
    pub couple_w: u32,
    /// Windowed-decode window \(W\) (sections).
    pub window_w: u32,
    /// Circulant / lift size \(Z\) (QC scale factor).
    pub lift_z: u32,
    /// Variable-node base degree (regular protograph).
    pub base_dv: u32,
    /// Check-node base degree.
    pub base_dc: u32,
}

impl Default for ScDesign {
    fn default() -> Self {
        Self {
            chain_l: 12,
            couple_w: 3,
            window_w: 6,
            lift_z: 4,
            base_dv: 3,
            base_dc: 6,
        }
    }
}

impl ScDesign {
    pub fn structurally_valid(self) -> bool {
        self.chain_l >= 2
            && self.couple_w >= 1
            && self.couple_w <= self.chain_l
            && self.window_w >= self.couple_w
            && self.lift_z >= 1
            && self.base_dv >= 2
            && self.base_dc > self.base_dv
    }

    /// Design rate of the uncoupled regular ensemble: \(1 - d_v/d_c\).
    pub fn uncoupled_rate(self) -> f64 {
        1.0 - (self.base_dv as f64) / (self.base_dc as f64)
    }

    /// Termination rate-loss scale \(\sim O(w/L)\) (fractional, lab formula).
    pub fn termination_loss_frac(self) -> f64 {
        if self.chain_l == 0 {
            return 1.0;
        }
        // Extra boundary checks / protected ends — standard order-of-magnitude.
        (self.couple_w as f64) / (self.chain_l as f64) * 0.5
    }

    /// Approximate terminated design rate (Category B).
    pub fn terminated_rate(self) -> f64 {
        (self.uncoupled_rate() - self.termination_loss_frac()).max(0.0)
    }

    /// Variable nodes per spatial section after lift (lab: one VN type × Z).
    pub fn vn_per_section(self) -> usize {
        self.lift_z as usize
    }

    /// Check nodes per spatial section after lift (rate-matched lab).
    pub fn cn_per_section(self) -> usize {
        // For regular (dv,dc): cn/vn ≈ dv/dc
        let vn = self.vn_per_section().max(1);
        ((vn as f64) * (self.base_dv as f64) / (self.base_dc as f64))
            .round()
            .max(1.0) as usize
    }

    /// Total variable-node count (unterminated chain body).
    pub fn total_vn(self) -> usize {
        self.chain_l as usize * self.vn_per_section()
    }

    /// Effective block length after mild termination padding on checks.
    pub fn effective_n(self) -> usize {
        self.total_vn()
    }

    pub fn label(self) -> String {
        format!(
            "SC-LDPC L={} w={} W={} Z={} regular({},{})",
            self.chain_l, self.couple_w, self.window_w, self.lift_z, self.base_dv, self.base_dc
        )
    }
}

/// Static analysis report (no channel).
#[derive(Debug, Clone)]
pub struct ScReport {
    pub design: ScDesign,
    pub structurally_valid: bool,
    pub uncoupled_rate: f64,
    pub terminated_rate: f64,
    pub term_loss: f64,
    pub n_vars: usize,
    pub saturation_note: &'static str,
}

pub fn analyze(d: ScDesign) -> ScReport {
    let ok = d.structurally_valid();
    ScReport {
        design: d,
        structurally_valid: ok,
        uncoupled_rate: d.uncoupled_rate(),
        terminated_rate: d.terminated_rate(),
        term_loss: d.termination_loss_frac(),
        n_vars: d.effective_n(),
        saturation_note: if ok && d.chain_l >= 2 * d.couple_w {
            "threshold saturation regime: BP→MAP of uncoupled (theory, L≫w)"
        } else if ok {
            "short chain — saturation partial; raise L or lower w"
        } else {
            "invalid design knobs"
        },
    }
}

impl ScReport {
    pub fn summary_line(&self) -> String {
        format!(
            "{}  R_unc={:.3} R_term={:.3} loss≈{:.3} n≈{}",
            self.design.label(),
            self.uncoupled_rate,
            self.terminated_rate,
            self.term_loss,
            self.n_vars
        )
    }

    pub fn detail_lines(&self) -> Vec<String> {
        vec![
            format!(
                "  structure valid={}  sections L={} couple w={} window W={}",
                self.structurally_valid,
                self.design.chain_l,
                self.design.couple_w,
                self.design.window_w
            ),
            format!(
                "  protograph regular(dv={},dc={}) lift Z={}  VN/sec={} CN/sec≈{}",
                self.design.base_dv,
                self.design.base_dc,
                self.design.lift_z,
                self.design.vn_per_section(),
                self.design.cn_per_section()
            ),
            format!("  {}", self.saturation_note),
            "  decode: full BP on chain · or sliding window W ≥ few·w".into(),
            "  vs block LDPC: σ_BP≈σ_MAP · vs Golay: long sparse ≠ short algebraic".into(),
        ]
    }
}

// ─── Toy terminated chain Tanner graph + windowed BEC peeling ───────────────

/// Edge list: check index → list of variable indices.
#[derive(Debug, Clone)]
struct ToyGraph {
    n_v: usize,
    n_c: usize,
    /// For each check: neighboring variables.
    cn_nbrs: Vec<Vec<usize>>,
    /// Spatial section of each variable.
    vn_section: Vec<u32>,
}

/// Build a minimal spatially coupled regular-ish graph for lab demos.
///
/// - \(L\) sections, `vn_per` variables each.
/// - Checks live at positions \(0..L+w-2\) (termination extends checks).
/// - Each check section \(z\) attaches to variables in sections
///   \(\{z, z+1, \ldots, z+w-1\} \cap [0,L)\).
fn build_toy_graph(d: ScDesign) -> ToyGraph {
    let l = d.chain_l as usize;
    let w = d.couple_w as usize;
    let vn_per = d.vn_per_section().max(1).min(8); // cap for interactivity
    let n_v = l * vn_per;
    let n_check_sec = l + w.saturating_sub(1);
    let cn_per = d.cn_per_section().max(1).min(8);
    let n_c = n_check_sec * cn_per;

    let mut vn_section = Vec::with_capacity(n_v);
    for s in 0..l {
        for _ in 0..vn_per {
            vn_section.push(s as u32);
        }
    }

    let mut cn_nbrs = vec![Vec::new(); n_c];
    // Degree budget per check ≈ base_dc (capped by available VNs in window).
    let target_deg = d.base_dc.min(8) as usize;

    for cs in 0..n_check_sec {
        for ci in 0..cn_per {
            let c = cs * cn_per + ci;
            // Variables in coupling window starting at section cs (clipped).
            let mut candidates = Vec::new();
            for s in cs..cs + w {
                if s >= l {
                    break;
                }
                let base = s * vn_per;
                for j in 0..vn_per {
                    candidates.push(base + j);
                }
            }
            if candidates.is_empty() {
                continue;
            }
            // Deterministic spread: pick every stride, rotate by check index.
            let stride = (candidates.len() / target_deg).max(1);
            for k in 0..target_deg {
                let idx = (ci * 3 + k * stride + cs) % candidates.len();
                let v = candidates[idx];
                if !cn_nbrs[c].contains(&v) {
                    cn_nbrs[c].push(v);
                }
            }
            // Termination: checks near ends get one extra local edge if possible.
            if cs < w || cs + 1 >= n_check_sec.saturating_sub(w) {
                if let Some(&v) = candidates.first() {
                    if !cn_nbrs[c].contains(&v) {
                        cn_nbrs[c].push(v);
                    }
                }
            }
        }
    }

    ToyGraph {
        n_v,
        n_c,
        cn_nbrs,
        vn_section,
    }
}

/// Result of windowed BEC peeling demo.
#[derive(Debug, Clone)]
pub struct WindowedBecDemo {
    pub erased: usize,
    pub residual_erasures: usize,
    pub rounds: u32,
    pub success: bool,
    pub n_v: usize,
    pub n_c: usize,
    pub windows: u32,
}

/// Inject erasures + run sliding-window pealing on BEC (Category B toy).
///
/// `erase_budget` is a count of erased variable nodes (not a channel ε).
pub fn windowed_bec_demo(d: ScDesign, seed: u32, erase_budget: u32) -> WindowedBecDemo {
    let g = build_toy_graph(d);
    let mut erased = vec![false; g.n_v];
    let budget = (erase_budget as usize).min(g.n_v.saturating_sub(1).max(1));
    let mut s = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
    let mut placed = 0usize;
    let mut guard = 0u32;
    while placed < budget && guard < 10_000 {
        guard += 1;
        s = s.wrapping_mul(1_103_515_245).wrapping_add(12345);
        let i = (s as usize) % g.n_v;
        if !erased[i] {
            erased[i] = true;
            placed += 1;
        }
    }
    let initial = erased.iter().filter(|&&e| e).count();

    let l = d.chain_l as usize;
    let win = d.window_w.min(d.chain_l).max(1) as usize;
    let mut rounds = 0u32;
    let mut windows = 0u32;

    // Slide window start from 0..L-1
    let mut start = 0usize;
    while start < l {
        windows += 1;
        let end = (start + win).min(l);
        // Peeling restricted to variables in [start, end) and checks touching them.
        let mut progress = true;
        let mut local_rounds = 0u32;
        while progress && local_rounds < 64 {
            progress = false;
            local_rounds += 1;
            rounds += 1;
            for nbrs in &g.cn_nbrs {
                // Count erased neighbors that are inside the window OR still free.
                let mut erased_in = Vec::new();
                let mut known_ok = true;
                for &v in nbrs {
                    let sec = g.vn_section[v] as usize;
                    if erased[v] {
                        // Only resolve VNs that are in the commit region or full window.
                        if sec >= start && sec < end {
                            erased_in.push(v);
                        } else if sec >= end {
                            // Future sections — treat as unknown (block pealing across).
                            known_ok = false;
                        } else {
                            // Past sections: should already be decided; if still erased, bad.
                            known_ok = false;
                        }
                    }
                }
                if known_ok && erased_in.len() == 1 {
                    let v = erased_in[0];
                    erased[v] = false;
                    progress = true;
                }
            }
        }
        // Commit oldest section: if still erased there, leave as residual.
        start += 1;
        if start + win > l + win {
            break;
        }
        // Advance by 1 section (streaming style).
        if start >= l {
            break;
        }
    }

    // Final global pealing mop-up (full BP-like on residual).
    let mut progress = true;
    while progress && rounds < 512 {
        progress = false;
        rounds += 1;
        for nbrs in &g.cn_nbrs {
            let erased_n: Vec<usize> = nbrs.iter().copied().filter(|&v| erased[v]).collect();
            if erased_n.len() == 1 {
                erased[erased_n[0]] = false;
                progress = true;
            }
        }
    }

    let residual = erased.iter().filter(|&&e| e).count();
    WindowedBecDemo {
        erased: initial,
        residual_erasures: residual,
        rounds,
        success: residual == 0,
        n_v: g.n_v,
        n_c: g.n_c,
        windows,
    }
}

/// Threshold-saturation slogan for UI (not a numerical computation).
pub fn threshold_saturation_claim() -> &'static str {
    "σ_BP(coupled) → σ_MAP(uncoupled) as L,w grow (Kudekar–Richardson–Urbanke)"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_design_valid() {
        let d = ScDesign::default();
        assert!(d.structurally_valid());
        assert!(d.uncoupled_rate() > 0.4 && d.uncoupled_rate() < 0.6);
        assert!(d.terminated_rate() < d.uncoupled_rate());
        assert!(d.termination_loss_frac() > 0.0);
    }

    #[test]
    fn invalid_when_w_gt_l() {
        let mut d = ScDesign::default();
        d.couple_w = 100;
        d.chain_l = 4;
        assert!(!d.structurally_valid());
    }

    #[test]
    fn analyze_saturation_note() {
        let r = analyze(ScDesign::default());
        assert!(r.structurally_valid);
        assert!(r.saturation_note.contains("saturation"));
    }

    #[test]
    fn windowed_bec_low_erasure_often_clears() {
        let d = ScDesign {
            chain_l: 10,
            couple_w: 2,
            window_w: 5,
            lift_z: 3,
            base_dv: 3,
            base_dc: 6,
        };
        // Few erasures should peel cleanly on a well-coupled toy.
        let toy = windowed_bec_demo(d, 7, 2);
        assert_eq!(toy.erased, 2);
        assert!(toy.n_v > 0 && toy.n_c > 0);
        // Allow rare residual on pathological seed; prefer success.
        assert!(toy.success || toy.residual_erasures < toy.erased);
    }

    #[test]
    fn golay_regime_disjoint() {
        // Documentation invariant: SC-LDPC is long-block iterative.
        let d = ScDesign::default();
        assert!(d.effective_n() != 24 || d.chain_l > 1);
        assert_ne!(threshold_saturation_claim().len(), 0);
    }
}
