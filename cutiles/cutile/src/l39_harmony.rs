//! L₃₉L₃₉ harmony detection, C₁₂-averaged bias, and certificate enrichment.
//! ATOM: L39-HARMONY-BIAS-20260709 | α + ω = 15

use crate::core::c12_hexaflake::{c12_average_harmony, c12_harmony_contribution};
use crate::core::srac_strategies::DivergenceReason;
use crate::existence_cert::{ExistenceCertificate, TomczakGateWitness};

/// Detected L₃₉ harmonic pair (39-strata viewport cross-section).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct L39HarmonyObservation {
    pub token_i: usize,
    pub token_j: usize,
    pub raw_score: f32,
    pub lattice_point: (i32, i32),
}

/// Map token indices into C₁₂ hexaflake lattice (deterministic, ATOM-trail stable).
pub fn map_tokens_to_lattice(i: usize, j: usize) -> (i32, i32) {
    let x = (i as i32).rem_euclid(13) - 6;
    let y = (j as i32).rem_euclid(13) - 6;
    (x, y)
}

/// Score a candidate pair; returns `Some` when harmony exceeds threshold.
pub fn detect_l39_harmony_pair(i: usize, j: usize, affinity: f32) -> Option<L39HarmonyObservation> {
    if affinity < 0.55 {
        return None;
    }
    Some(L39HarmonyObservation {
        token_i: i,
        token_j: j,
        raw_score: affinity,
        lattice_point: map_tokens_to_lattice(i, j),
    })
}

/// Modulate `betti_proxy` downward when high harmony stabilizes the filtration.
pub fn betti_proxy_with_l39_harmony(base_betti: f32, harmony_score: f32) -> f32 {
    let reduction = (harmony_score * 0.12).min(0.25);
    (base_betti * (1.0 - reduction)).max(0.0)
}

/// Tomczak + WAVE-modulated bias scale for harmonic routing.
pub fn compute_bias_scale(gate: &TomczakGateWitness, wave_score: f64) -> f32 {
    let mut scale = 1.0f32;
    if gate.tomczak_preserved {
        scale *= 1.15;
    }
    if gate.betti_proxy_below_threshold {
        scale *= 1.05;
    }
    scale *= (wave_score as f32).clamp(0.85, 1.0);
    scale
}

/// C₁₂-averaged harmonic benefit before certificate emission.
pub fn compute_harmonic_benefit(
    obs: &L39HarmonyObservation,
    gate: &TomczakGateWitness,
    wave_score: f64,
    harmony_field: &[((i32, i32), f32)],
) -> f32 {
    let averaged = if harmony_field.is_empty() {
        c12_harmony_contribution(
            obs.lattice_point,
            obs.raw_score,
            gate.tomczak_preserved,
            wave_score as f32,
        )
    } else {
        c12_average_harmony(obs.lattice_point, harmony_field)
    };
    averaged * compute_bias_scale(gate, wave_score)
}

/// Optional training auxiliary: reward high-harmony routing while enforcing invariants.
pub fn harmonic_auxiliary_loss(
    harmonic_benefit: f32,
    alpha_omega_sum: f64,
    gate: &TomczakGateWitness,
) -> f64 {
    let music_penalty = if (alpha_omega_sum - 15.0).abs() > 0.05 {
        1.0
    } else {
        0.0
    };
    let tomczak_penalty = if gate.tomczak_preserved { 0.0 } else { 2.0 };
    -(f64::from(harmonic_benefit)) + music_penalty + tomczak_penalty
}

/// Emit updated certificate after a biased forward pass.
pub fn emit_biased_existence_certificate(
    prior: &ExistenceCertificate,
    gate: &TomczakGateWitness,
    harmonic_benefit: f32,
    srac_delta: u32,
    atom_trail_id: impl Into<String>,
) -> ExistenceCertificate {
    let mut cert = ExistenceCertificate::from_mehler_result(
        gate.betti_proxy_below_threshold,
        gate.tomczak_preserved,
        prior.max_error_bound,
        prior.reliable,
        prior.wave_score,
        prior.alpha_omega_sum,
        prior.coherence_delta,
        atom_trail_id.into(),
        prior.kernel_version.clone(),
        prior.input_state_hash.clone(),
        Some(prior.srac_corrections.saturating_add(srac_delta)),
        f64::from(harmonic_benefit),
    );
    cert.recompute_hash();
    cert
}

/// Classify strong L₃₉ harmony as a divergence-reason subtype (positive anomaly).
pub fn l39_harmony_divergence(obs: &L39HarmonyObservation) -> DivergenceReason {
    DivergenceReason::L39HarmonyDetected {
        token_i: obs.token_i,
        token_j: obs.token_j,
        harmony_score: obs.raw_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bias_scale_respects_wave_floor() {
        let gate = TomczakGateWitness {
            betti_proxy_below_threshold: true,
            tomczak_preserved: true,
        };
        let scale = compute_bias_scale(&gate, 0.9);
        assert!(scale > 1.0);
    }

    #[test]
    fn harmonic_benefit_zero_when_tomczak_fails() {
        let obs = detect_l39_harmony_pair(3, 7, 0.8).unwrap();
        let gate = TomczakGateWitness {
            betti_proxy_below_threshold: true,
            tomczak_preserved: false,
        };
        assert_eq!(
            compute_harmonic_benefit(&obs, &gate, 0.99, &[]),
            0.0
        );
    }

    #[test]
    fn auxiliary_loss_penalizes_broken_invariants() {
        let gate = TomczakGateWitness {
            betti_proxy_below_threshold: true,
            tomczak_preserved: false,
        };
        assert!(harmonic_auxiliary_loss(0.5, 15.0, &gate) > 0.0);
    }
}