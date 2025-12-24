use crate::mesh::types::MeshData;
use super::{StitchGrid, StitchType};

pub struct StitchTypeClassifier {
    _private: (),
}

impl StitchTypeClassifier {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn classify(&self, grid: &mut StitchGrid, mesh: &MeshData) {
        for stitch in &mut grid.stitches {
            let curvature = self.estimate_curvature_at(mesh, stitch.position_3d);
            
            stitch.stitch_type = if curvature > 0.3 {
                StitchType::Increase  // Positive curvature -> increase
            } else if curvature < -0.3 {
                StitchType::Decrease  // Negative curvature -> decrease
            } else if curvature.abs() < 0.1 {
                StitchType::SingleCrochet  // Flat
            } else {
                StitchType::HalfDoubleCrochet  // Mild curvature
            };
        }
    }

    fn estimate_curvature_at(&self, mesh: &MeshData, position: [f32; 3]) -> f32 {
        // Find nearest vertex
        let mut min_dist = f32::INFINITY;
        let mut nearest = 0;

        for (i, vertex) in mesh.vertices.iter().enumerate() {
            let dx = vertex.position[0] - position[0];
            let dy = vertex.position[1] - position[1];
            let dz = vertex.position[2] - position[2];
            let dist = dx * dx + dy * dy + dz * dz;

            if dist < min_dist {
                min_dist = dist;
                nearest = i;
            }
        }

        mesh.vertices[nearest].curvature.unwrap_or(0.0)
    }
}

impl Default for StitchTypeClassifier {
    fn default() -> Self {
        Self::new()
    }
}
