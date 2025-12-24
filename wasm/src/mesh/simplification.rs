use anyhow::Result;
use crate::mesh::types::MeshData;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone)]
struct EdgeCollapse {
    cost: f32,
    v0: u32,
    v1: u32,
    target_pos: [f32; 3],
}

impl PartialEq for EdgeCollapse {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for EdgeCollapse {}

impl PartialOrd for EdgeCollapse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for EdgeCollapse {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct MeshSimplifier {
    _private: (),
}

impl MeshSimplifier {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn simplify(&self, mesh: &mut MeshData, target_face_count: usize) -> Result<()> {
        if mesh.faces.len() <= target_face_count {
            return Ok(());
        }

        let mut edge_heap = BinaryHeap::new();
        
        // Build initial edge heap
        for face in &mesh.faces {
            for i in 0..3 {
                let v0 = face.indices[i];
                let v1 = face.indices[(i + 1) % 3];
                
                let collapse = self.compute_edge_collapse(mesh, v0, v1);
                edge_heap.push(collapse);
            }
        }

        // Perform edge collapses
        let collapses_needed = mesh.faces.len() - target_face_count;
        let mut collapses_done = 0;
        let mut removed_vertices = vec![false; mesh.vertices.len()];

        while collapses_done < collapses_needed && !edge_heap.is_empty() {
            if let Some(collapse) = edge_heap.pop() {
                if !removed_vertices[collapse.v0 as usize] && !removed_vertices[collapse.v1 as usize] {
                    // Perform collapse
                    mesh.vertices[collapse.v0 as usize].position = collapse.target_pos;
                    removed_vertices[collapse.v1 as usize] = true;

                    // Update faces
                    for face in &mut mesh.faces {
                        for idx in &mut face.indices {
                            if *idx == collapse.v1 {
                                *idx = collapse.v0;
                            }
                        }
                    }

                    collapses_done += 1;
                }
            }
        }

        // Remove degenerate faces
        mesh.faces.retain(|face| {
            face.indices[0] != face.indices[1] &&
            face.indices[1] != face.indices[2] &&
            face.indices[0] != face.indices[2]
        });

        Ok(())
    }

    fn compute_edge_collapse(&self, mesh: &MeshData, v0: u32, v1: u32) -> EdgeCollapse {
        let pos0 = mesh.vertices[v0 as usize].position;
        let pos1 = mesh.vertices[v1 as usize].position;

        // Simple midpoint collapse
        let target_pos = [
            (pos0[0] + pos1[0]) * 0.5,
            (pos0[1] + pos1[1]) * 0.5,
            (pos0[2] + pos1[2]) * 0.5,
        ];

        // Cost is edge length
        let dx = pos1[0] - pos0[0];
        let dy = pos1[1] - pos0[1];
        let dz = pos1[2] - pos0[2];
        let cost = (dx * dx + dy * dy + dz * dz).sqrt();

        EdgeCollapse {
            cost,
            v0,
            v1,
            target_pos,
        }
    }
}

impl Default for MeshSimplifier {
    fn default() -> Self {
        Self::new()
    }
}
