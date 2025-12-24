use anyhow::Result;
use crate::mesh::types::MeshData;
use crate::mesh::simplification::MeshSimplifier;
use crate::CrochetConfig;

pub struct MeshProcessor {
    _private: (),
}

impl MeshProcessor {
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Process mesh: normalize, clean, optionally simplify
    pub fn process(&self, mesh: &mut MeshData, config: &CrochetConfig) -> Result<()> {
        // Remove degenerate faces
        self.remove_degenerate_faces(mesh);

        // Compute vertex normals if missing
        self.compute_vertex_normals(mesh);

        // Normalize mesh to unit scale
        self.normalize_scale(mesh, config);

        // Optionally simplify
        if config.simplify_mesh {
            if let Some(target_count) = config.target_stitch_count {
                // Target face count based on stitch count
                let target_faces = (target_count * 2) as usize; // Rough estimate
                if mesh.faces.len() > target_faces {
                    let simplifier = MeshSimplifier::new();
                    simplifier.simplify(mesh, target_faces)?;
                }
            }
        }

        Ok(())
    }

    fn remove_degenerate_faces(&self, mesh: &mut MeshData) {
        mesh.faces.retain(|face| {
            // Check for duplicate indices
            if face.indices[0] == face.indices[1] ||
               face.indices[1] == face.indices[2] ||
               face.indices[0] == face.indices[2] {
                return false;
            }

            // Check for zero area
            let v0 = mesh.vertices[face.indices[0] as usize].position;
            let v1 = mesh.vertices[face.indices[1] as usize].position;
            let v2 = mesh.vertices[face.indices[2] as usize].position;

            let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

            let cross = [
                e1[1] * e2[2] - e1[2] * e2[1],
                e1[2] * e2[0] - e1[0] * e2[2],
                e1[0] * e2[1] - e1[1] * e2[0],
            ];

            let area = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
            area > 1e-10
        });
    }

    fn compute_vertex_normals(&self, mesh: &mut MeshData) {
        // Reset normals
        for vertex in &mut mesh.vertices {
            vertex.normal = [0.0, 0.0, 0.0];
        }

        // Accumulate face normals
        for face in &mesh.faces {
            let v0 = mesh.vertices[face.indices[0] as usize].position;
            let v1 = mesh.vertices[face.indices[1] as usize].position;
            let v2 = mesh.vertices[face.indices[2] as usize].position;

            let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

            let normal = [
                e1[1] * e2[2] - e1[2] * e2[1],
                e1[2] * e2[0] - e1[0] * e2[2],
                e1[0] * e2[1] - e1[1] * e2[0],
            ];

            // Add to vertices (weighted by angle)
            for &idx in &face.indices {
                let vertex = &mut mesh.vertices[idx as usize];
                vertex.normal[0] += normal[0];
                vertex.normal[1] += normal[1];
                vertex.normal[2] += normal[2];
            }
        }

        // Normalize
        for vertex in &mut mesh.vertices {
            let len = (vertex.normal[0] * vertex.normal[0] + 
                      vertex.normal[1] * vertex.normal[1] + 
                      vertex.normal[2] * vertex.normal[2]).sqrt();
            
            if len > 1e-6 {
                vertex.normal[0] /= len;
                vertex.normal[1] /= len;
                vertex.normal[2] /= len;
            } else {
                vertex.normal = [0.0, 1.0, 0.0];
            }
        }
    }

    fn normalize_scale(&self, mesh: &mut MeshData, config: &CrochetConfig) {
        let size = mesh.bounds.size();
        let max_dim = size.iter().copied().fold(f32::NEG_INFINITY, f32::max);

        if max_dim < 1e-6 {
            return;
        }

        // Target scale based on configuration
        let target_size = if let Some(width) = config.target_width_inches {
            width
        } else if let Some(height) = config.target_height_inches {
            height
        } else {
            // Default: scale to reasonable crochet size (6 inches)
            6.0
        };

        let scale = target_size / max_dim;
        let center = mesh.bounds.center();

        // Scale and center
        for vertex in &mut mesh.vertices {
            vertex.position[0] = (vertex.position[0] - center[0]) * scale;
            vertex.position[1] = (vertex.position[1] - center[1]) * scale;
            vertex.position[2] = (vertex.position[2] - center[2]) * scale;
        }

        // Update bounds
        mesh.bounds.min[0] = (mesh.bounds.min[0] - center[0]) * scale;
        mesh.bounds.min[1] = (mesh.bounds.min[1] - center[1]) * scale;
        mesh.bounds.min[2] = (mesh.bounds.min[2] - center[2]) * scale;
        mesh.bounds.max[0] = (mesh.bounds.max[0] - center[0]) * scale;
        mesh.bounds.max[1] = (mesh.bounds.max[1] - center[1]) * scale;
        mesh.bounds.max[2] = (mesh.bounds.max[2] - center[2]) * scale;
    }
}

impl Default for MeshProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::types::{Vertex, Face, BoundingBox};

    fn create_test_mesh() -> MeshData {
        MeshData {
            vertices: vec![
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    uv: [0.0, 0.0],
                    curvature: None,
                },
                Vertex {
                    position: [1.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    uv: [1.0, 0.0],
                    curvature: None,
                },
                Vertex {
                    position: [0.0, 1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    uv: [0.0, 1.0],
                    curvature: None,
                },
            ],
            faces: vec![Face { indices: [0, 1, 2] }],
            bounds: BoundingBox {
                min: [0.0, 0.0, 0.0],
                max: [1.0, 1.0, 0.0],
            },
        }
    }

    #[test]
    fn test_compute_normals() {
        let processor = MeshProcessor::new();
        let mut mesh = create_test_mesh();
        
        processor.compute_vertex_normals(&mut mesh);
        
        // Check that normals are computed and normalized
        for vertex in &mesh.vertices {
            let len = (vertex.normal[0] * vertex.normal[0] + 
                      vertex.normal[1] * vertex.normal[1] + 
                      vertex.normal[2] * vertex.normal[2]).sqrt();
            assert!((len - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_remove_degenerate() {
        let processor = MeshProcessor::new();
        let mut mesh = create_test_mesh();
        
        // Add degenerate face
        mesh.faces.push(Face { indices: [0, 0, 0] });
        
        processor.remove_degenerate_faces(&mut mesh);
        
        assert_eq!(mesh.faces.len(), 1);
    }
}
