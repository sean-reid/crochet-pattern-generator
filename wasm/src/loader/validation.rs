use anyhow::Result;
use crate::mesh::types::MeshData;

pub struct ModelValidator {
    _private: (),
}

impl ModelValidator {
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Validate mesh and return warnings
    pub fn validate(&self, mesh: &MeshData) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Check vertex count
        if mesh.vertices.is_empty() {
            anyhow::bail!("Mesh has no vertices");
        }

        // Check face count
        if mesh.faces.is_empty() {
            anyhow::bail!("Mesh has no faces");
        }

        // Warn if mesh is very large
        if mesh.vertices.len() > 100_000 {
            warnings.push(format!(
                "Large mesh ({} vertices). Consider simplification for better performance.",
                mesh.vertices.len()
            ));
        }

        // Check for degenerate faces
        let degenerate_count = self.count_degenerate_faces(mesh);
        if degenerate_count > 0 {
            warnings.push(format!(
                "Found {} degenerate faces (zero area). These will be removed.",
                degenerate_count
            ));
        }

        // Check for invalid indices
        let max_index = mesh.vertices.len() as u32;
        for (i, face) in mesh.faces.iter().enumerate() {
            for &idx in &face.indices {
                if idx >= max_index {
                    anyhow::bail!(
                        "Face {} has invalid vertex index {} (max: {})",
                        i, idx, max_index - 1
                    );
                }
            }
        }

        // Check for duplicate vertices
        let duplicate_count = self.estimate_duplicate_vertices(mesh);
        if duplicate_count > mesh.vertices.len() / 10 {
            warnings.push(format!(
                "Approximately {} duplicate vertices detected. Consider welding.",
                duplicate_count
            ));
        }

        // Check mesh bounds
        let size = mesh.bounds.size();
        let min_size = size.iter().copied().fold(f32::INFINITY, f32::min);
        let max_size = size.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        
        if max_size / min_size > 100.0 {
            warnings.push(format!(
                "Mesh is very elongated (aspect ratio: {:.1}). May be difficult to crochet.",
                max_size / min_size
            ));
        }

        // Check for non-manifold edges (simplified check)
        if self.has_non_manifold_edges(mesh) {
            warnings.push(
                "Mesh may have non-manifold geometry. Results may be unexpected.".to_string()
            );
        }

        Ok(warnings)
    }

    fn count_degenerate_faces(&self, mesh: &MeshData) -> usize {
        mesh.faces.iter().filter(|face| {
            let v0 = mesh.vertices[face.indices[0] as usize].position;
            let v1 = mesh.vertices[face.indices[1] as usize].position;
            let v2 = mesh.vertices[face.indices[2] as usize].position;

            // Check if all three vertices are the same
            if face.indices[0] == face.indices[1] || 
               face.indices[1] == face.indices[2] || 
               face.indices[0] == face.indices[2] {
                return true;
            }

            // Compute area using cross product
            let e1 = [
                v1[0] - v0[0],
                v1[1] - v0[1],
                v1[2] - v0[2],
            ];
            let e2 = [
                v2[0] - v0[0],
                v2[1] - v0[1],
                v2[2] - v0[2],
            ];

            let cross = [
                e1[1] * e2[2] - e1[2] * e2[1],
                e1[2] * e2[0] - e1[0] * e2[2],
                e1[0] * e2[1] - e1[1] * e2[0],
            ];

            let area = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
            area < 1e-10
        }).count()
    }

    fn estimate_duplicate_vertices(&self, mesh: &MeshData) -> usize {
        use std::collections::HashMap;

        let mut position_map: HashMap<[i32; 3], usize> = HashMap::new();
        let mut duplicate_count = 0;

        for vertex in &mesh.vertices {
            // Discretize position for comparison
            let key = [
                (vertex.position[0] * 10000.0) as i32,
                (vertex.position[1] * 10000.0) as i32,
                (vertex.position[2] * 10000.0) as i32,
            ];

            if let Some(count) = position_map.get_mut(&key) {
                *count += 1;
                duplicate_count += 1;
            } else {
                position_map.insert(key, 1);
            }
        }

        duplicate_count
    }

    fn has_non_manifold_edges(&self, mesh: &MeshData) -> bool {
        use std::collections::HashMap;

        // Count how many times each edge appears
        let mut edge_count: HashMap<(u32, u32), usize> = HashMap::new();

        for face in &mesh.faces {
            let edges = [
                (face.indices[0].min(face.indices[1]), face.indices[0].max(face.indices[1])),
                (face.indices[1].min(face.indices[2]), face.indices[1].max(face.indices[2])),
                (face.indices[2].min(face.indices[0]), face.indices[2].max(face.indices[0])),
            ];

            for edge in edges {
                *edge_count.entry(edge).or_insert(0) += 1;
            }
        }

        // Non-manifold edges appear more than twice
        edge_count.values().any(|&count| count > 2)
    }
}

impl Default for ModelValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::types::{Vertex, Face, BoundingBox};

    fn create_simple_mesh() -> MeshData {
        MeshData {
            vertices: vec![
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 1.0, 0.0],
                    uv: [0.0, 0.0],
                    curvature: None,
                },
                Vertex {
                    position: [1.0, 0.0, 0.0],
                    normal: [0.0, 1.0, 0.0],
                    uv: [1.0, 0.0],
                    curvature: None,
                },
                Vertex {
                    position: [0.0, 1.0, 0.0],
                    normal: [0.0, 1.0, 0.0],
                    uv: [0.0, 1.0],
                    curvature: None,
                },
            ],
            faces: vec![
                Face {
                    indices: [0, 1, 2],
                },
            ],
            bounds: BoundingBox {
                min: [0.0, 0.0, 0.0],
                max: [1.0, 1.0, 0.0],
            },
        }
    }

    #[test]
    fn test_validate_simple_mesh() {
        let validator = ModelValidator::new();
        let mesh = create_simple_mesh();
        let result = validator.validate(&mesh);
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_degenerate() {
        let validator = ModelValidator::new();
        let mut mesh = create_simple_mesh();
        
        // Add degenerate face
        mesh.faces.push(Face {
            indices: [0, 0, 0],
        });
        
        let count = validator.count_degenerate_faces(&mesh);
        assert_eq!(count, 1);
    }
}
