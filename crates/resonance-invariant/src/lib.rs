// `no_std` for every real build; tests need std for the harness only.
#![cfg_attr(not(test), no_std)]

pub mod phase;

use core::sync::atomic::{AtomicU32, Ordering};
const ALPHA_OMEGA_LIMIT: u32 = 15;
const PCH_FREQUENCY_HZ: u32 = 1232;

#[derive(Debug, PartialEq, Eq)]
pub enum TopologicalState {
    Coherent,
    Decoherent,
    VoidLocked,
}

pub struct LevinWenLattice {
    alpha: AtomicU32,
    omega: AtomicU32,
    frequency: AtomicU32,
}

impl LevinWenLattice {
    pub const fn new(initial_alpha: u32, initial_omega: u32) -> Self {
        Self {
            alpha: AtomicU32::new(initial_alpha),
            omega: AtomicU32::new(initial_omega),
            frequency: AtomicU32::new(PCH_FREQUENCY_HZ),
        }
    }

    /// Current α. Exposed so [`phase`] can classify without owning the fields.
    #[inline]
    pub fn alpha_value(&self, order: Ordering) -> u32 {
        self.alpha.load(order)
    }

    /// Current ω. Counterpart to [`Self::alpha_value`].
    #[inline]
    pub fn omega_value(&self, order: Ordering) -> u32 {
        self.omega.load(order)
    }

    /// Configured resonance frequency (Hz).
    #[inline]
    pub fn frequency_hz(&self) -> u32 {
        self.frequency.load(Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn verify_conservation(&self) -> TopologicalState {
        let a = self.alpha.load(Ordering::Acquire);
        let w = self.omega.load(Ordering::Acquire);
        
        if a + w == ALPHA_OMEGA_LIMIT {
            TopologicalState::Coherent
        } else {
            TopologicalState::Decoherent
        }
    }

    pub fn apply_transformation(&self, delta_alpha: i32) -> Result<(), &'static str> {
        let current_a = self.alpha.load(Ordering::SeqCst);
        let new_a = (current_a as i32 + delta_alpha).clamp(0, ALPHA_OMEGA_LIMIT as i32) as u32;
        let new_w = ALPHA_OMEGA_LIMIT - new_a;

        self.alpha.store(new_a, Ordering::SeqCst);
        self.omega.store(new_w, Ordering::SeqCst);

        if self.verify_conservation() != TopologicalState::Coherent {
            return Err("Isomorphism collapsed: invariant breached.");
        }
        Ok(())
    }
}
