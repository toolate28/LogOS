use super::palette::{entropy_to_hup, surge_color, tqec, Rgb};

/// Syndrome field sample for overlay rendering (prototype 2 data layer).
#[derive(Debug, Clone)]
pub struct SyndromeField {
    pub entropy: Vec<f32>,
    pub surge_flags: Vec<f32>,
    pub betti_proxy: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl SyndromeField {
    pub fn from_entropy_result(
        entropy: Vec<f32>,
        surge: f32,
        betti: f32,
        width: usize,
        height: usize,
    ) -> Self {
        let n = width * height;
        Self {
            entropy,
            surge_flags: vec![surge; n],
            betti_proxy: vec![betti; n],
            width,
            height,
        }
    }

    /// Pixel colour for syndrome map (purple HUP + amber survivors + cyan spikes).
    pub fn pixel_color(&self, x: usize, y: usize) -> Rgb {
        let idx = y * self.width + x;
        let e = self
            .entropy
            .get(idx % self.entropy.len())
            .copied()
            .unwrap_or(0.0);
        let surge = self
            .surge_flags
            .get(idx % self.surge_flags.len())
            .copied()
            .unwrap_or(0.0);
        let betti = self
            .betti_proxy
            .get(idx % self.betti_proxy.len())
            .copied()
            .unwrap_or(0.0);

        if betti > 1.0 {
            return tqec::SYNDROME_CYAN;
        }
        if surge > 0.5 {
            return surge_color(surge);
        }
        entropy_to_hup(e)
    }
}

/// Placeholder for wgpu render pass integration (prototype 2).
pub fn render_syndrome_field_cpu(field: &SyndromeField) -> Vec<Rgb> {
    (0..field.height)
        .flat_map(|y| (0..field.width).map(move |x| field.pixel_color(x, y)))
        .collect()
}