use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CubicalCell {
    pub dimension: u8,
    pub id: u64,
}

#[derive(Debug)]
pub struct TriWeavonHIT {
    points: Vec<CubicalCell>,
    weaves: HashMap<(u64, u64), Vec<CubicalCell>>,
}

impl TriWeavonHIT {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            weaves: HashMap::new(),
        }
    }

    pub fn add_point(&mut self) -> u64 {
        let id = self.points.len() as u64;
        self.points.push(CubicalCell { dimension: 0, id });
        id
    }

    pub fn weave(&mut self, a: u64, b: u64) -> u64 {
        let weave_id = (self.points.len() as u64) + 1000;
        let weave_cell = CubicalCell {
            dimension: 1,
            id: weave_id,
        };
        self.points.push(weave_cell.clone());

        self.weaves.entry((a, b)).or_default().push(weave_cell);
        weave_id
    }

    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// All cells (0- and 1-dimensional) for visualization.
    pub fn cells(&self) -> &[CubicalCell] {
        &self.points
    }

    /// Directed weave edges `(source, target)` between 0-cells.
    pub fn weave_edges(&self) -> Vec<(u64, u64)> {
        self.weaves.keys().copied().collect()
    }

    /// Cubical filler between two 0-cells (Agda `hcomp` executable face).
    pub fn hcomp_edge(&mut self, a: u64, b: u64, t: f32) -> Option<u64> {
        let has_a = self.points.iter().any(|c| c.id == a && c.dimension == 0);
        let has_b = self.points.iter().any(|c| c.id == b && c.dimension == 0);
        if !has_a || !has_b {
            return None;
        }
        if t <= 0.0 {
            Some(a)
        } else if t >= 1.0 {
            Some(b)
        } else {
            Some(self.weave(a, b))
        }
    }
}

impl Default for TriWeavonHIT {
    fn default() -> Self {
        Self::new()
    }
}