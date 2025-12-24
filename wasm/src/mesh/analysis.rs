use anyhow::Result;
use crate::mesh::types::{MeshData, HalfEdgeMesh};

pub struct MeshAnalyzer {
    _private: (),
}

impl MeshAnalyzer {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn compute_curvature(&self, mesh: &mut MeshData) -> Result<()> {
        let half_edge = HalfEdgeMesh::from_mesh(mesh);
        
        let curvatures: Vec<f32> = (0..mesh.vertices.len())
            .map(|i| self.compute_vertex_curvature(i as u32, mesh, &half_edge))
            .collect();
        
        for (i, vertex) in mesh.vertices.iter_mut().enumerate() {
            vertex.curvature = Some(curvatures[i]);
        }

        Ok(())
    }

    fn compute_vertex_curvature(&self, vertex_idx: u32, mesh: &MeshData, half_edge: &HalfEdgeMesh) -> f32 {
        let vertex_pos = mesh.vertices[vertex_idx as usize].position;

        let mut angle_sum = 0.0;
        let mut neighbor_positions = Vec::new();

        if let Some(start_edge) = half_edge.vertex_to_edge[vertex_idx as usize] {
            let mut current = start_edge;
            loop {
                let edge = &half_edge.edges[current as usize];
                let next_vertex_idx = half_edge.edges[edge.next.unwrap() as usize].vertex;
                neighbor_positions.push(mesh.vertices[next_vertex_idx as usize].position);

                if let Some(twin) = edge.prev {
                    if let Some(twin_twin) = half_edge.edges[twin as usize].twin {
                        current = twin_twin;
                        if current == start_edge {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if neighbor_positions.len() < 3 {
            return 0.0;
        }

        for i in 0..neighbor_positions.len() {
            let v1 = neighbor_positions[i];
            let v2 = neighbor_positions[(i + 1) % neighbor_positions.len()];

            let e1 = [
                v1[0] - vertex_pos[0],
                v1[1] - vertex_pos[1],
                v1[2] - vertex_pos[2],
            ];
            let e2 = [
                v2[0] - vertex_pos[0],
                v2[1] - vertex_pos[1],
                v2[2] - vertex_pos[2],
            ];

            let len1 = (e1[0] * e1[0] + e1[1] * e1[1] + e1[2] * e1[2]).sqrt();
            let len2 = (e2[0] * e2[0] + e2[1] * e2[1] + e2[2] * e2[2]).sqrt();

            if len1 > 1e-6 && len2 > 1e-6 {
                let e1_norm = [e1[0] / len1, e1[1] / len1, e1[2] / len1];
                let e2_norm = [e2[0] / len2, e2[1] / len2, e2[2] / len2];

                let dot = e1_norm[0] * e2_norm[0] + e1_norm[1] * e2_norm[1] + e1_norm[2] * e2_norm[2];
                angle_sum += dot.clamp(-1.0, 1.0).acos();
            }
        }

        let angle_deficit = std::f32::consts::TAU - angle_sum;
        angle_deficit / (neighbor_positions.len() as f32)
    }

    pub fn compute_gaussian_curvature(&self, mesh: &MeshData) -> Vec<f32> {
        let half_edge = HalfEdgeMesh::from_mesh(mesh);
        let mut curvatures = vec![0.0; mesh.vertices.len()];

        for i in 0..mesh.vertices.len() {
            let mut angle_sum = 0.0;
            let mut area_sum = 0.0;

            if let Some(start_edge) = half_edge.vertex_to_edge[i] {
                let mut current = start_edge;
                loop {
                    let edge = &half_edge.edges[current as usize];
                    let _face_idx = edge.face;

                    let v0 = mesh.vertices[i].position;
                    let v1_idx = half_edge.edges[edge.next.unwrap() as usize].vertex as usize;
                    let v2_idx = half_edge.edges[edge.prev.unwrap() as usize].vertex as usize;
                    let v1 = mesh.vertices[v1_idx].position;
                    let v2 = mesh.vertices[v2_idx].position;

                    let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
                    let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

                    let len1 = (e1[0] * e1[0] + e1[1] * e1[1] + e1[2] * e1[2]).sqrt();
                    let len2 = (e2[0] * e2[0] + e2[1] * e2[1] + e2[2] * e2[2]).sqrt();

                    if len1 > 1e-6 && len2 > 1e-6 {
                        let dot = (e1[0] * e2[0] + e1[1] * e2[1] + e1[2] * e2[2]) / (len1 * len2);
                        angle_sum += dot.clamp(-1.0, 1.0).acos();

                        let cross = [
                            e1[1] * e2[2] - e1[2] * e2[1],
                            e1[2] * e2[0] - e1[0] * e2[2],
                            e1[0] * e2[1] - e1[1] * e2[0],
                        ];
                        let area = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt() * 0.5;
                        area_sum += area / 3.0;
                    }

                    if let Some(twin) = edge.twin {
                        if let Some(next) = half_edge.edges[twin as usize].next {
                            current = next;
                            if current == start_edge {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            if area_sum > 1e-6 {
                curvatures[i] = (std::f32::consts::TAU - angle_sum) / area_sum;
            }
        }

        curvatures
    }

    pub fn find_boundaries(&self, mesh: &MeshData) -> Vec<Vec<u32>> {
        let half_edge = HalfEdgeMesh::from_mesh(mesh);
        let mut visited = vec![false; half_edge.edges.len()];
        let mut boundaries = Vec::new();

        for (i, edge) in half_edge.edges.iter().enumerate() {
            if edge.twin.is_none() && !visited[i] {
                let mut boundary = Vec::new();
                let mut current = i as u32;

                loop {
                    visited[current as usize] = true;
                    boundary.push(half_edge.edges[current as usize].vertex);

                    if let Some(next) = half_edge.edges[current as usize].next {
                        current = next;
                        if current as usize == i {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if !boundary.is_empty() {
                    boundaries.push(boundary);
                }
            }
        }

        boundaries
    }
}

impl Default for MeshAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
