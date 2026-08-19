//! Classical codes lab for reson8-tui —
//! Hexacode · Golay \(G_{24}\) · Reed–Muller · SC-LDPC.
//!
//! Surfaces the MOG / HexacodeGolay formal spine (Lean **A**-green construction)
//! and Category **B** runtime decoders / iterative demos for the operator.
//!
//! Keys:
//! - `c` focus Codes · `d` demo · `D` multi-family battery · `y` family
//! - `e` inject weight · `[`/`]` RM r or SC w · `{`/`}` RM m or SC L

pub mod golay;
pub mod hexacode;
pub mod reed_muller;
pub mod sc_ldpc;

use golay::GolayDecode;
use hexacode::HexDecode;
use reed_muller::{Rm1Decode, RmParams};

/// Which classical family the panel is driving.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeFamily {
    Hexacode,
    Golay24,
    ReedMuller,
    ScLdpc,
}

impl CodeFamily {
    pub const ALL: [Self; 4] = [
        Self::Hexacode,
        Self::Golay24,
        Self::ReedMuller,
        Self::ScLdpc,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Hexacode => "Hexacode [6,3,4]_4",
            Self::Golay24 => "Golay G24 [24,12,8]",
            Self::ReedMuller => "Reed–Muller RM(r,m)",
            Self::ScLdpc => "SC-LDPC (coupled)",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Hexacode => Self::Golay24,
            Self::Golay24 => Self::ReedMuller,
            Self::ReedMuller => Self::ScLdpc,
            Self::ScLdpc => Self::Hexacode,
        }
    }

    pub fn epistemic(self) -> &'static str {
        match self {
            Self::Hexacode => "params A · decoder B (enum/syndrome)",
            Self::Golay24 => "MOG construction A-green · decoder B (NN t≤3)",
            Self::ReedMuller => "params A · FHT RM(1,m) B · not Golay",
            Self::ScLdpc => "threshold sat. classical · lab B · ≠ Golay/MOG",
        }
    }
}

/// Interactive lab state for the Codes panel.
#[derive(Debug, Clone)]
pub struct CodesLab {
    pub family: CodeFamily,
    pub rm_r: u32,
    pub rm_m: u32,
    /// SC-LDPC chain length \(L\).
    pub sc_l: u32,
    /// Coupling width \(w\).
    pub sc_w: u32,
    /// Decode window \(W\).
    pub sc_window: u32,
    /// Lift size \(Z\).
    pub sc_z: u32,
    pub error_t: u32,
    pub seed: u32,
    pub lines: Vec<String>,
    pub last_ok: bool,
    pub runs: u32,
}

impl Default for CodesLab {
    fn default() -> Self {
        Self {
            family: CodeFamily::Golay24,
            rm_r: 1,
            rm_m: 4,
            sc_l: 12,
            sc_w: 3,
            sc_window: 6,
            sc_z: 4,
            error_t: 3,
            seed: 42,
            lines: vec![
                "Codes lab — Hex · G24 · RM · SC-LDPC".into(),
                "d demo · D battery · y family · e t± · [ ] param · { } size".into(),
                "A: Lean construction · B: runtime decoders / iterative toys".into(),
            ],
            last_ok: true,
            runs: 0,
        }
    }
}

impl CodesLab {
    pub fn cycle_family(&mut self) {
        self.family = self.family.next();
        self.push(format!("family → {}", self.family.label()));
    }

    /// `[` / `]` — RM order r, or SC coupling width w.
    pub fn bump_rm_r(&mut self, delta: i32) {
        match self.family {
            CodeFamily::ScLdpc => {
                let w = (self.sc_w as i32 + delta).clamp(1, self.sc_l.max(1) as i32) as u32;
                self.sc_w = w;
                if self.sc_window < self.sc_w {
                    self.sc_window = self.sc_w;
                }
                self.push(format!("SC couple w → {} (window W={})", self.sc_w, self.sc_window));
            }
            _ => {
                let m = self.rm_m;
                let r = self.rm_r as i32 + delta;
                self.rm_r = r.clamp(0, m as i32) as u32;
                self.push(format!("RM r → {}", self.rm_r));
            }
        }
    }

    /// `{` / `}` — RM m, or SC chain length L.
    pub fn bump_rm_m(&mut self, delta: i32) {
        match self.family {
            CodeFamily::ScLdpc => {
                let l = (self.sc_l as i32 + delta).clamp(2, 48) as u32;
                self.sc_l = l;
                if self.sc_w > l {
                    self.sc_w = l;
                }
                if self.sc_window > l {
                    self.sc_window = l;
                }
                self.push(format!(
                    "SC chain L → {} (w={} W={} Z={})",
                    self.sc_l, self.sc_w, self.sc_window, self.sc_z
                ));
            }
            _ => {
                let m = (self.rm_m as i32 + delta).clamp(2, 8) as u32;
                self.rm_m = m;
                if self.rm_r > m {
                    self.rm_r = m;
                }
                self.push(format!("RM m → {} (n={})", self.rm_m, 1usize << self.rm_m));
            }
        }
    }

    pub fn bump_error_t(&mut self) {
        self.error_t = match self.family {
            CodeFamily::Hexacode => (self.error_t % 2) + 1,
            CodeFamily::Golay24 => (self.error_t % 4) + 1,
            CodeFamily::ReedMuller => {
                let p = RmParams::new(self.rm_r, self.rm_m);
                let tmax = p.map(|p| p.t.max(1) as u32).unwrap_or(1);
                (self.error_t % tmax) + 1
            }
            CodeFamily::ScLdpc => (self.error_t % 12) + 1, // erasure budget
        };
        self.push(format!("inject/erase t → {}", self.error_t));
    }

    fn push(&mut self, s: impl Into<String>) {
        self.lines.insert(0, s.into());
        self.lines.truncate(28);
    }

    pub fn run_demo(&mut self) {
        self.runs += 1;
        self.seed = self.seed.wrapping_add(17);
        match self.family {
            CodeFamily::Hexacode => self.demo_hex(),
            CodeFamily::Golay24 => self.demo_golay(),
            CodeFamily::ReedMuller => self.demo_rm(),
            CodeFamily::ScLdpc => self.demo_sc_ldpc(),
        }
    }

    pub fn run_all_demos(&mut self) {
        let saved = self.family;
        for f in CodeFamily::ALL {
            self.family = f;
            self.run_demo();
        }
        self.family = saved;
        self.push("── battery complete (Hex · G24 · RM · SC-LDPC) ──");
    }

    fn demo_hex(&mut self) {
        let msg = [
            (self.seed % 4) as u8,
            ((self.seed >> 2) % 4) as u8,
            ((self.seed >> 4) % 4) as u8,
        ];
        let c = hexacode::encode(msg);
        let t = self.error_t.min(1);
        let mut y = c;
        if t >= 1 {
            let pos = (self.seed as usize) % 6;
            let e = 1 + ((self.seed >> 8) % 3) as u8;
            y[pos] = hexacode::gf_add(y[pos], e);
        }
        let d: HexDecode = hexacode::decode_syndrome(y);
        let (a0, a4, a6) = hexacode::weight_distribution();
        self.last_ok = d.unique && d.corrected == c;
        self.push(format!(
            "Hex [6,3,4]_4  A(x)=1+{a4}x^4+{a6}x^6 (A0={a0})  t_corr=1  cover=2"
        ));
        self.push(format!(
            "  msg={msg:?} → c={c:?}  y={y:?}  unique={} ok={}",
            d.unique, self.last_ok
        ));
        if let Some(p) = d.error_pos {
            self.push(format!(
                "  syndrome decode: pos={p} e={} → {:?}",
                d.error_val, d.corrected
            ));
        } else if d.unique {
            self.push("  syndrome: σ=0 (already codeword)");
        } else {
            self.push("  syndrome: uncorrectable coset (detect)");
        }
        self.push(format!("  epistemic: {}", self.family.epistemic()));
    }

    fn demo_golay(&mut self) {
        let msg = self.seed % 4096;
        let c = golay::encode(msg);
        let t = self.error_t.min(4);
        let y = golay::inject_errors(c, t, self.seed);
        let d: GolayDecode = golay::decode_bounded(y, 3);
        let scores = golay::scores_of(y);
        let hex_ok = golay::hex_gate_ok(y);
        let par_ok = golay::r_parity_ok(y);
        let mem_y = golay::golay_mask_ok(y);
        let mem_c = golay::golay_mask_ok(c);
        self.last_ok = d.accepted && d.corrected == c;
        self.push("G24 [24,12,8]  t=3  |B3|·M=2325·4096  cosets=4096  octads=759");
        self.push(format!(
            "  msg={msg:#05x} c={c:#08x} wt(c)={} mem(c)={}",
            golay::hamming_wt(c),
            mem_c
        ));
        self.push(format!(
            "  y={y:#08x} inj_t={t}  hex_gate={} R_par={} mem(y)={}",
            hex_ok, par_ok, mem_y
        ));
        self.push(format!("  scores(y)={scores:?}"));
        self.push(format!(
            "  NN/BDD: d={} e={:#08x} unique={} accepted={} ok={}",
            d.distance, d.error_mask, d.unique, d.accepted, self.last_ok
        ));
        if t <= 3 {
            let (ok, n) = golay::empirical_nn_unique(8, t, self.seed);
            self.push(format!("  empirical NN t={t}: {ok}/{n} unique recoveries"));
        } else {
            self.push("  t=4: beyond unique correction radius (detect path)");
        }
        self.push("  note: G24 ≠ RM(r,m) and ≠ SC-LDPC");
        self.push(format!("  epistemic: {}", self.family.epistemic()));
    }

    fn demo_rm(&mut self) {
        let Some(p) = RmParams::new(self.rm_r, self.rm_m) else {
            self.last_ok = false;
            self.push("RM params out of lab range");
            return;
        };
        self.push(p.label());
        if let Some(name) = p.classical_name() {
            self.push(format!("  classical: {name}"));
        }
        if let Some(dual) = p.dual() {
            self.push(format!(
                "  dual RM({},{}) [{},{},{}]",
                dual.r, dual.m, dual.n, dual.k, dual.d
            ));
        }
        if self.rm_r == 1 {
            let mut msg = vec![0u8; p.k];
            for i in 0..p.k {
                msg[i] = ((self.seed >> i) & 1) as u8;
            }
            let c = reed_muller::encode(1, self.rm_m, &msg).unwrap();
            let t = (self.error_t as usize).min(p.t);
            let y = reed_muller::inject_errors(&c, t, self.seed);
            let d: Rm1Decode = reed_muller::decode_rm1_fht(&y, self.rm_m).unwrap();
            self.last_ok = d.accepted && d.corrected == c;
            self.push(format!(
                "  FHT decode t_inj={t} peak={} accepted={} ok={}",
                d.peak, d.accepted, self.last_ok
            ));
            self.push(format!("  msg_in={msg:?} msg_hat={:?}", d.message));
        } else {
            let mut msg = vec![0u8; p.k];
            for i in 0..p.k.min(32) {
                msg[i] = ((self.seed.wrapping_mul(i as u32 + 3) >> 8) & 1) as u8;
            }
            match reed_muller::encode(self.rm_r, self.rm_m, &msg) {
                Some(c) => {
                    let wt: u32 = c.iter().map(|&b| b as u32).sum();
                    self.last_ok = true;
                    self.push(format!(
                        "  encode ok n={} k={} wt(c)≈{wt} (order-r decode = B peel / SC)",
                        p.n, p.k
                    ));
                    self.push("  tip: set r=1 for FHT ML demo");
                }
                None => {
                    self.last_ok = false;
                    self.push("  encode failed");
                }
            }
        }
        self.push("  Plotkin |u|u+v| · majority-logic · polar cousins");
        self.push(format!("  epistemic: {}", self.family.epistemic()));
    }

    fn demo_sc_ldpc(&mut self) {
        let design = sc_ldpc::ScDesign {
            chain_l: self.sc_l,
            couple_w: self.sc_w,
            window_w: self.sc_window,
            lift_z: self.sc_z,
            base_dv: 3,
            base_dc: 6,
        };
        let report = sc_ldpc::analyze(design);
        self.last_ok = report.structurally_valid;
        self.push(report.summary_line());
        for line in report.detail_lines() {
            self.push(line);
        }
        self.push(format!("  {}", sc_ldpc::threshold_saturation_claim()));
        let toy = sc_ldpc::windowed_bec_demo(design, self.seed, self.error_t.min(16));
        self.push(format!(
            "  windowed BEC: nV={} nC={} erased={} residual={} rounds={} win_slides={} ok={}",
            toy.n_v,
            toy.n_c,
            toy.erased,
            toy.residual_erasures,
            toy.rounds,
            toy.windows,
            toy.success
        ));
        if report.structurally_valid && toy.success {
            self.last_ok = true;
        } else if report.structurally_valid && toy.residual_erasures < toy.erased {
            self.last_ok = true; // partial peel still educational success
        }
        self.push("  note: SC-LDPC ≠ Golay/MOG — long sparse iterative vs short algebraic");
        self.push(format!("  epistemic: {}", self.family.epistemic()));
    }

    pub fn header_lines(&self) -> Vec<String> {
        let mut v = vec![
            format!("family: {}  runs:{}", self.family.label(), self.runs),
            format!(
                "inject/erase t={}  seed={}  last={}",
                self.error_t,
                self.seed,
                if self.last_ok {
                    "OK"
                } else {
                    "FAIL/DETECT"
                }
            ),
        ];
        match self.family {
            CodeFamily::Hexacode => {
                v.push(format!(
                    "H: n={} k={} d={} t={} MDS cover={}",
                    hexacode::N,
                    hexacode::K,
                    hexacode::D,
                    hexacode::CORRECT_T,
                    hexacode::COVERING_RADIUS
                ));
            }
            CodeFamily::Golay24 => {
                v.push(format!(
                    "G24: n={} k={} d={} t={} | codewords={}",
                    golay::N,
                    golay::K,
                    golay::D,
                    golay::CORRECT_T,
                    golay::M
                ));
            }
            CodeFamily::ReedMuller => {
                if let Some(p) = RmParams::new(self.rm_r, self.rm_m) {
                    v.push(p.label());
                }
            }
            CodeFamily::ScLdpc => {
                let d = sc_ldpc::ScDesign {
                    chain_l: self.sc_l,
                    couple_w: self.sc_w,
                    window_w: self.sc_window,
                    lift_z: self.sc_z,
                    base_dv: 3,
                    base_dc: 6,
                };
                v.push(d.label());
                v.push(format!(
                    "R_unc={:.3} R_term={:.3}  [ ]=w  {{ }}=L",
                    d.uncoupled_rate(),
                    d.terminated_rate()
                ));
            }
        }
        v.push(self.family.epistemic().to_string());
        v
    }
}
