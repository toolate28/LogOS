//! AppState — the TUI state machine.
//!
//! Minimal, serialisable, deterministic. No tokio, no async — plain sync
//! event loop in `main.rs` drives this.

use crate::ph::{compute_h0, Barcode};
use crate::point_cloud::{self, Cloud};
use crate::vr;

use reson8_core::{coherence_functional, fibonacci, COHERENCE_K};

#[derive(Debug, Clone)]
pub struct AppState {
    pub cloud: Cloud,
    pub n_points: usize,
    pub points: Vec<(f64, f64)>,
    pub barcodes: Vec<Barcode>,

    /// Current ε cursor position in [0.0, eps_max].
    pub epsilon: f64,
    pub eps_max: f64,

    /// Invariant pair (α, ω). Steady state = (7, 8).
    pub alpha: f64,
    pub omega: f64,

    /// Latest coherence_functional() output.
    pub wave_score: f64,

    pub paused: bool,
    pub should_quit: bool,
}

impl AppState {
    /// Construct a fresh state, recomputing cloud and barcodes from
    /// deterministic inputs. Safe to call on every cloud change.
    pub fn new(cloud: Cloud, n_points: usize) -> Self {
        let points = point_cloud::generate(cloud, n_points);
        let edges = vr::edges(&points);
        let eps_max = edges.last().map(|e| e.2).unwrap_or(1.0);
        let barcodes = compute_h0(points.len(), &edges, eps_max);

        Self {
            cloud,
            n_points,
            points,
            barcodes,
            epsilon: 0.0,
            eps_max,
            alpha: 7.0,
            omega: 8.0,
            wave_score: 0.0,
            paused: false,
            should_quit: false,
        }
    }

    /// Rebuild state for a new cloud selection (keeps α, ω, paused flags).
    pub fn switch_cloud(&mut self, cloud: Cloud) {
        let preserved_alpha = self.alpha;
        let preserved_omega = self.omega;
        let preserved_paused = self.paused;
        *self = Self::new(cloud, self.n_points);
        self.alpha = preserved_alpha;
        self.omega = preserved_omega;
        self.paused = preserved_paused;
    }

    /// Count barcodes currently alive at `self.epsilon`.
    pub fn alive_bars(&self) -> usize {
        self.barcodes
            .iter()
            .filter(|b| b.birth <= self.epsilon && self.epsilon < b.death)
            .count()
    }

    /// Recompute `wave_score` from the current state, using the F(8)/F(5)/F(3)
    /// component weights from `reson8_core::fibonacci`.
    pub fn recompute_wave(&mut self) {
        let total = self.barcodes.len().max(1) as f64;
        let w_topo = self.alive_bars() as f64 / total; // topological term
        let w_sem = self.alpha / reson8_core::INVARIANT_TARGET; // semantic rail proxy
        let w_struct = self.omega / reson8_core::INVARIANT_TARGET; // structural rail proxy
        let w_temp = if self.eps_max > 0.0 {
            1.0 - (self.epsilon / self.eps_max)
        } else {
            1.0
        };

        let w = fibonacci::W_TOPO * w_topo
            + fibonacci::W_SEM * w_sem
            + fibonacci::W_STRUCT * w_struct
            + fibonacci::W_TEMP * w_temp;

        // Potential = deviation from the invariant sum (smaller is better).
        let p = 1.0 - ((self.alpha + self.omega - reson8_core::INVARIANT_TARGET).abs()
            / reson8_core::INVARIANT_TARGET);

        self.wave_score = coherence_functional(w, self.alpha, self.omega, p, COHERENCE_K);
    }

    /// Advance the ε cursor one tick (≈ 1/60 s of a 4-second full sweep).
    pub fn tick(&mut self) {
        if self.paused || self.eps_max <= 0.0 {
            self.recompute_wave();
            return;
        }
        self.epsilon += self.eps_max / 240.0;
        if self.epsilon > self.eps_max {
            self.epsilon = 0.0;
        }
        self.recompute_wave();
    }

    pub fn step_eps(&mut self, delta_fraction: f64) {
        self.epsilon = (self.epsilon + self.eps_max * delta_fraction)
            .clamp(0.0, self.eps_max);
        self.recompute_wave();
    }

    pub fn reset(&mut self) {
        self.epsilon = 0.0;
        self.recompute_wave();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alive_bars_at_zero_equals_point_count() {
        let s = AppState::new(Cloud::Circle, 12);
        assert_eq!(s.alive_bars(), 12, "at ε=0 all 12 components are alive");
    }

    #[test]
    fn wave_score_is_finite_and_positive() {
        let mut s = AppState::new(Cloud::Circle, 12);
        s.recompute_wave();
        assert!(s.wave_score.is_finite());
        assert!(s.wave_score > 0.0);
    }

    #[test]
    fn tick_advances_then_wraps() {
        let mut s = AppState::new(Cloud::Circle, 12);
        let start = s.epsilon;
        s.tick();
        assert!(s.epsilon > start, "tick did not advance ε");
        // Force wrap
        s.epsilon = s.eps_max;
        s.tick();
        assert_eq!(s.epsilon, 0.0, "tick did not wrap at eps_max");
    }
}
