use crate::hit::{CubicalCell, TriWeavonHIT};

use super::palette::{surge_color, tqec, Rgb};

/// Layout snapshot for 2D braid rendering.
#[derive(Debug, Clone)]
pub struct BraidLayout {
    pub node_positions: Vec<(u64, f32, f32)>,
    pub edges: Vec<(u64, u64)>,
    pub node_colors: Vec<Rgb>,
}

impl BraidLayout {
    pub fn from_hit(hit: &TriWeavonHIT, width: f32, height: f32, entropy_field: &[f32]) -> Self {
        let cells = hit.cells();
        let mut node_positions = Vec::new();
        let mut node_colors = Vec::new();

        let zero_cells: Vec<_> = cells.iter().filter(|c| c.dimension == 0).collect();
        let n = zero_cells.len().max(1);

        for (i, cell) in zero_cells.iter().enumerate() {
            let phase = (i as f32 / n as f32) * std::f32::consts::TAU;
            let x = width * 0.15 + (width * 0.7) * (i as f32 / n as f32);
            let y = height * 0.5 + phase.sin() * height * 0.18;
            node_positions.push((cell.id, x, y));

            let e = entropy_field.get(i % entropy_field.len()).copied().unwrap_or(0.0);
            let color = if e > 0.5 {
                tqec::CORE_CYAN
            } else {
                tqec::SURVIVOR_AMBER
            };
            node_colors.push(color);
        }

        Self {
            node_positions,
            edges: hit.weave_edges(),
            node_colors,
        }
    }

    pub fn color_for_cell(cell: &CubicalCell, surge: f32) -> Rgb {
        match cell.dimension {
            0 => tqec::CORE_CYAN,
            1 => surge_color(surge),
            _ => tqec::MIRROR_OBSERVER,
        }
    }
}