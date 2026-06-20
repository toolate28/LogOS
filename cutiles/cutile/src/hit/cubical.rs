//! Executable face of cubical primitives (hcomp/transp stubs for Agda bridge).

use crate::hit::TriWeavonHIT;

/// Agda `Cubical.HITs` ↔ Rust executable mapping surface.
pub trait CubicalHIT {
    fn add_point(&mut self) -> u64;
    fn weave(&mut self, a: u64, b: u64) -> u64;
    fn hcomp_edge(&mut self, a: u64, b: u64, t: f32) -> Option<u64>;
    fn point_count(&self) -> usize;
}

impl CubicalHIT for TriWeavonHIT {
    fn add_point(&mut self) -> u64 {
        TriWeavonHIT::add_point(self)
    }

    fn weave(&mut self, a: u64, b: u64) -> u64 {
        TriWeavonHIT::weave(self, a, b)
    }

    fn hcomp_edge(&mut self, a: u64, b: u64, t: f32) -> Option<u64> {
        TriWeavonHIT::hcomp_edge(self, a, b, t)
    }

    fn point_count(&self) -> usize {
        TriWeavonHIT::point_count(self)
    }
}

/// Placeholder for Agda `hcomp` — composes a 1-dimensional homotopy filler.
#[derive(Debug, Clone, Copy)]
pub struct HComp {
    pub face0: f32,
    pub face1: f32,
}

impl HComp {
    pub fn fill(&self, t: f32) -> f32 {
        self.face0 + t * (self.face1 - self.face0)
    }
}