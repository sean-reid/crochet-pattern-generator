use anyhow::Result;
use crate::mesh::types::{MeshData, HalfEdgeMesh};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Clone)]
struct PathNode {
    vertex: u32,
    cost: f32,
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for PathNode {}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct SeamPlacer {
    _private: (),
}

impl SeamPlacer {
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Place seam to cut surface for parameterization
    /// Returns edges that form the seam as (v0, v1) pairs
    pub fn place_seam(&self, mesh: &MeshData) -> Result<Vec<(u32, u32)>> {
        // Find boundary loops
        let half_edge = HalfEdgeMesh::from_mesh(mesh);
        let boundaries = self.find_boundary_loops(&half_edge);
        
        // If mesh already has boundaries, use them
        if !boundaries.is_empty() {
            return Ok(self.boundary_to_edges(&boundaries[0]));
        }
        
        // Mesh is closed - need to cut it
        // Strategy: Find shortest path between two distant vertices
        let (start, end) = self.find_distant_vertices(mesh);
        let path = self.find_shortest_path(mesh, start, end)?;
        
        // Convert path to edges
        let mut seam_edges = Vec::new();
        for i in 0..path.len() - 1 {
            seam_edges.push((path[i], path[i + 1]));
        }
        
        Ok(seam_edges)
    }

    /// Find boundary loops in the mesh
    fn find_boundary_loops(&self, half_edge: &HalfEdgeMesh) -> Vec<Vec<u32>> {
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

    /// Convert boundary loop to edges
    fn boundary_to_edges(&self, boundary: &[u32]) -> Vec<(u32, u32)> {
        let mut edges = Vec::new();
        for i in 0..boundary.len() {
            let v0 = boundary[i];
            let v1 = boundary[(i + 1) % boundary.len()];
            edges.push((v0, v1));
        }
        edges
    }

    /// Find two vertices that are far apart
    fn find_distant_vertices(&self, mesh: &MeshData) -> (u32, u32) {
        let mut max_dist = 0.0;
        let mut best_pair = (0, 1);
        
        // Sample a subset of vertices for efficiency
        let sample_rate = (mesh.vertices.len() / 20).max(10).min(100);
        
        for i in (0..mesh.vertices.len()).step_by(sample_rate) {
            for j in (i + sample_rate..mesh.vertices.len()).step_by(sample_rate) {
                let p1 = mesh.vertices[i].position;
                let p2 = mesh.vertices[j].position;
                
                let dx = p2[0] - p1[0];
                let dy = p2[1] - p1[1];
                let dz = p2[2] - p1[2];
                let dist = dx * dx + dy * dy + dz * dz;
                
                if dist > max_dist {
                    max_dist = dist;
                    best_pair = (i as u32, j as u32);
                }
            }
        }
        
        best_pair
    }

    /// Find shortest path between two vertices using Dijkstra
    fn find_shortest_path(&self, mesh: &MeshData, start: u32, end: u32) -> Result<Vec<u32>> {
        let n = mesh.vertices.len();
        let mut distances = vec![f32::INFINITY; n];
        let mut previous: Vec<Option<u32>> = vec![None; n];
        let mut heap = BinaryHeap::new();
        
        distances[start as usize] = 0.0;
        heap.push(PathNode { vertex: start, cost: 0.0 });
        
        // Build adjacency list
        let mut adjacency: HashMap<u32, Vec<u32>> = HashMap::new();
        for face in &mesh.faces {
            for i in 0..3 {
                let v0 = face.indices[i];
                let v1 = face.indices[(i + 1) % 3];
                adjacency.entry(v0).or_insert_with(Vec::new).push(v1);
                adjacency.entry(v1).or_insert_with(Vec::new).push(v0);
            }
        }
        
        while let Some(PathNode { vertex, cost }) = heap.pop() {
            if vertex == end {
                break;
            }
            
            if cost > distances[vertex as usize] {
                continue;
            }
            
            if let Some(neighbors) = adjacency.get(&vertex) {
                for &neighbor in neighbors {
                    let v1 = mesh.vertices[vertex as usize].position;
                    let v2 = mesh.vertices[neighbor as usize].position;
                    
                    let dx = v2[0] - v1[0];
                    let dy = v2[1] - v1[1];
                    let dz = v2[2] - v1[2];
                    let edge_cost = (dx * dx + dy * dy + dz * dz).sqrt();
                    
                    let new_cost = cost + edge_cost;
                    
                    if new_cost < distances[neighbor as usize] {
                        distances[neighbor as usize] = new_cost;
                        previous[neighbor as usize] = Some(vertex);
                        heap.push(PathNode { vertex: neighbor, cost: new_cost });
                    }
                }
            }
        }
        
        // Reconstruct path
        let mut path = Vec::new();
        let mut current = end;
        
        while let Some(prev) = previous[current as usize] {
            path.push(current);
            current = prev;
            if current == start {
                break;
            }
        }
        
        path.push(start);
        path.reverse();
        
        if path.len() < 2 {
            anyhow::bail!("Could not find path between vertices");
        }
        
        Ok(path)
    }
}

impl Default for SeamPlacer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::types::{Vertex, Face, BoundingBox};

    #[test]
    fn test_find_distant_vertices() {
        let placer = SeamPlacer::new();
        
        let mesh = MeshData {
            vertices: vec![
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 1.0, 0.0],
                    uv: [0.0, 0.0],
                    curvature: None,
                },
                Vertex {
                    position: [10.0, 10.0, 10.0],
                    normal: [0.0, 1.0, 0.0],
                    uv: [1.0, 1.0],
                    curvature: None,
                },
            ],
            faces: vec![],
            bounds: BoundingBox {
                min: [0.0, 0.0, 0.0],
                max: [10.0, 10.0, 10.0],
            },
        };
        
        let (v0, v1) = placer.find_distant_vertices(&mesh);
        assert!(v0 != v1);
    }
}
