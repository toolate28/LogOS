//! K22_SHEAF traversals and micro-local sheaf ops (skeleton).

pub struct K22SheafTraversal {
    pub dimension: usize,
}

impl K22SheafTraversal {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    pub fn traverse_nodes(&self, node_count: usize) -> usize {
        node_count.min(self.dimension)
    }
}