use crate::mesh::types::MeshData;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone)]
struct Node {
    vertex: u32,
    distance: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct GeodesicSolver {
    _private: (),
}

impl GeodesicSolver {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn compute_distances(&self, mesh: &MeshData, source: u32) -> Vec<f32> {
        let n = mesh.vertices.len();
        let mut distances = vec![f32::INFINITY; n];
        distances[source as usize] = 0.0;

        let mut heap = BinaryHeap::new();
        heap.push(Node { vertex: source, distance: 0.0 });

        while let Some(Node { vertex, distance }) = heap.pop() {
            if distance > distances[vertex as usize] {
                continue;
            }

            // Find neighbors (simplified)
            for face in &mesh.faces {
                let mut found = false;
                let mut neighbor = 0;

                for i in 0..3 {
                    if face.indices[i] == vertex {
                        found = true;
                        neighbor = face.indices[(i + 1) % 3];
                        break;
                    }
                }

                if found {
                    let v0 = mesh.vertices[vertex as usize].position;
                    let v1 = mesh.vertices[neighbor as usize].position;
                    let edge_length = ((v1[0] - v0[0]).powi(2) + 
                                      (v1[1] - v0[1]).powi(2) + 
                                      (v1[2] - v0[2]).powi(2)).sqrt();

                    let new_distance = distance + edge_length;
                    if new_distance < distances[neighbor as usize] {
                        distances[neighbor as usize] = new_distance;
                        heap.push(Node { vertex: neighbor, distance: new_distance });
                    }
                }
            }
        }

        distances
    }
}

impl Default for GeodesicSolver {
    fn default() -> Self {
        Self::new()
    }
}
