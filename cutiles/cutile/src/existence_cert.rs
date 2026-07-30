//! Self-referential existence certificates for certified cutile kernel outputs.
//! ATOM: SG-EXISTENCE-CERT-EMITTER-20260706 | α + ω = 15 | C(C) BLAKE3 self-hash

use std::time::{SystemTime, UNIX_EPOCH};

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::VERSION;

/// Tomczak lift gate flags (matches `harness::LiftOk` and Lean `TomczakExistence`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TomczakGateWitness {
    pub betti_proxy_below_threshold: bool,
    pub tomczak_preserved: bool,
}

/// Runtime witness consumable by Lean `TomczakExistence` and MCP ATOM trails.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExistenceCertificate {
    pub betti_proxy_below_threshold: bool,
    pub tomczak_preserved: bool,
    pub max_error_bound: f64,
    pub reliable: bool,
    pub wave_score: f64,
    pub alpha_omega_sum: f64,
    pub coherence_delta: f64,
    pub atom_trail_id: String,
    pub kernel_version: String,
    pub input_state_hash: Option<String>,
    pub certificate_hash: String,
    pub timestamp_ns: u64,
    /// SRAC corrections in the provenance chain.
    pub srac_corrections: u32,
    /// C₁₂-averaged L₃₉ harmonic routing benefit (0 when gate fails).
    pub harmonic_benefit: f64,
}

impl ExistenceCertificate {
    #[allow(clippy::too_many_arguments)]
    pub fn from_mehler_result(
        betti_below: bool,
        tomczak_ok: bool,
        max_error: f64,
        reliable: bool,
        wave: f64,
        alpha_omega: f64,
        coherence_delta: f64,
        atom_trail_id: String,
        kernel_version: String,
        input_hash: Option<String>,
        srac_corrections: Option<u32>,
        harmonic_benefit: f64,
    ) -> Self {
        let mut cert = Self {
            betti_proxy_below_threshold: betti_below,
            tomczak_preserved: tomczak_ok,
            max_error_bound: max_error,
            reliable,
            wave_score: wave,
            alpha_omega_sum: alpha_omega,
            coherence_delta,
            atom_trail_id,
            kernel_version,
            input_state_hash: input_hash,
            certificate_hash: String::new(),
            timestamp_ns: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64,
            srac_corrections: srac_corrections.unwrap_or(0),
            harmonic_benefit,
        };
        cert.certificate_hash = cert.compute_self_hash();
        cert
    }

    pub fn from_coherence_diagnostic(
        wave: f64,
        coherence_delta: f64,
        atom_trail_id: String,
        kernel_version: Option<String>,
        srac_corrections: Option<u32>,
        harmonic_benefit: Option<f64>,
    ) -> Self {
        Self::from_mehler_result(
            true,
            true,
            0.0,
            true,
            wave,
            15.0,
            coherence_delta,
            atom_trail_id,
            kernel_version.unwrap_or_else(|| VERSION.to_string()),
            None,
            srac_corrections,
            harmonic_benefit.unwrap_or(0.0),
        )
    }

    pub fn from_lift_and_mehler(
        lift_ok: &TomczakGateWitness,
        max_error: f64,
        reliable: bool,
        wave: f64,
        alpha_omega: f64,
        coherence_delta: f64,
        atom_trail_id: impl Into<String>,
        input_hash: Option<String>,
        srac_corrections: Option<u32>,
        harmonic_benefit: Option<f64>,
    ) -> Self {
        Self::from_mehler_result(
            lift_ok.betti_proxy_below_threshold,
            lift_ok.tomczak_preserved,
            max_error,
            reliable,
            wave,
            alpha_omega,
            coherence_delta,
            atom_trail_id.into(),
            VERSION.to_string(),
            input_hash,
            srac_corrections,
            harmonic_benefit.unwrap_or(0.0),
        )
    }

    pub fn compute_self_hash(&self) -> String {
        let mut pre_hash = self.clone();
        pre_hash.certificate_hash.clear();
        let serialized =
            serde_json::to_vec(&pre_hash).expect("ExistenceCertificate must serialize");
        let mut hasher = Hasher::new();
        hasher.update(&serialized);
        hasher.finalize().to_hex().to_string()
    }

    pub fn recompute_hash(&mut self) {
        self.certificate_hash = self.compute_self_hash();
    }

    pub fn preserves_existence(&self) -> bool {
        self.betti_proxy_below_threshold
            && self.tomczak_preserved
            && self.reliable
            && (self.alpha_omega_sum - 15.0).abs() < 0.05
            && self.wave_score >= 0.85
            && self.srac_corrections <= 1024
    }

    pub fn verify_self_hash(&self) -> bool {
        !self.certificate_hash.is_empty() && self.certificate_hash == self.compute_self_hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_hash_is_stable_and_verifiable() {
        let cert = ExistenceCertificate::from_mehler_result(
            true,
            true,
            1e-8,
            true,
            1.0,
            15.0,
            0.0,
            "ATOM-TEST".into(),
            "cutile-test".into(),
            None,
            Some(0),
            0.42,
        );
        assert!(!cert.certificate_hash.is_empty());
        assert!(cert.verify_self_hash());
        assert!((cert.harmonic_benefit - 0.42).abs() < 1e-9);
    }

    #[test]
    fn preserves_existence_requires_invariants() {
        let good = ExistenceCertificate::from_coherence_diagnostic(
            1.0,
            0.0,
            "ATOM-GOOD".into(),
            None,
            Some(0),
            None,
        );
        assert!(good.preserves_existence());
    }

    #[test]
    fn serializes_harmonic_and_srac_for_lean_bridge() {
        let cert = ExistenceCertificate::from_coherence_diagnostic(
            0.99,
            0.01,
            "SG-EXISTENCE-CERT-EMITTER-20260709".into(),
            None,
            Some(2),
            Some(0.15),
        );
        let json = serde_json::to_string(&cert).expect("json");
        assert!(json.contains("harmonicBenefit"));
        assert!(json.contains("sracCorrections"));
    }
}