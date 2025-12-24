use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub curvature: Option<f32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Face {
    pub indices: [u32; 3],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl BoundingBox {
    pub fn size(&self) -> [f32; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    pub fn center(&self) -> [f32; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    pub fn diagonal_length(&self) -> f32 {
        let size = self.size();
        (size[0] * size[0] + size[1] * size[1] + size[2] * size[2]).sqrt()
    }

    pub fn contains(&self, point: [f32; 3]) -> bool {
        point[0] >= self.min[0] && point[0] <= self.max[0] &&
        point[1] >= self.min[1] && point[1] <= self.max[1] &&
        point[2] >= self.min[2] && point[2] <= self.max[2]
    }

    pub fn expand(&mut self, point: [f32; 3]) {
        self.min[0] = self.min[0].min(point[0]);
        self.min[1] = self.min[1].min(point[1]);
        self.min[2] = self.min[2].min(point[2]);
        self.max[0] = self.max[0].max(point[0]);
        self.max[1] = self.max[1].max(point[1]);
        self.max[2] = self.max[2].max(point[2]);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub bounds: BoundingBox,
}

impl MeshData {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            bounds: BoundingBox {
                min: [0.0, 0.0, 0.0],
                max: [0.0, 0.0, 0.0],
            },
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty() || self.faces.is_empty()
    }
}

impl Default for MeshData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct HalfEdge {
    pub vertex: u32,
    pub face: u32,
    pub next: Option<u32>,
    pub prev: Option<u32>,
    pub twin: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct HalfEdgeMesh {
    pub edges: Vec<HalfEdge>,
    pub vertex_to_edge: Vec<Option<u32>>,
    pub face_to_edge: Vec<u32>,
}

impl HalfEdgeMesh {
    pub fn from_mesh(mesh: &MeshData) -> Self {
        let mut edges = Vec::new();
        let mut vertex_to_edge = vec![None; mesh.vertices.len()];
        let mut face_to_edge = Vec::new();

        for (face_idx, face) in mesh.faces.iter().enumerate() {
            let face_idx = face_idx as u32;
            let start_edge = edges.len() as u32;
            face_to_edge.push(start_edge);

            for i in 0..3 {
                let vertex = face.indices[i];
                let edge_idx = edges.len() as u32;

                edges.push(HalfEdge {
                    vertex,
                    face: face_idx,
                    next: Some(start_edge + ((i + 1) % 3) as u32),
                    prev: Some(start_edge + ((i + 2) % 3) as u32),
                    twin: None,
                });

                if vertex_to_edge[vertex as usize].is_none() {
                    vertex_to_edge[vertex as usize] = Some(edge_idx);
                }
            }
        }

        for i in 0..edges.len() {
            if edges[i].twin.is_some() {
                continue;
            }

            let v0 = edges[i].vertex;
            let v1 = edges[edges[i].next.unwrap() as usize].vertex;

            for j in (i + 1)..edges.len() {
                let w0 = edges[j].vertex;
                let w1 = edges[edges[j].next.unwrap() as usize].vertex;

                if v0 == w1 && v1 == w0 {
                    edges[i].twin = Some(j as u32);
                    edges[j].twin = Some(i as u32);
                    break;
                }
            }
        }

        Self {
            edges,
            vertex_to_edge,
            face_to_edge,
        }
    }

    pub fn vertex_valence(&self, vertex: u32) -> usize {
        let mut count = 0;
        if let Some(start_edge) = self.vertex_to_edge[vertex as usize] {
            let mut current = start_edge;
            loop {
                count += 1;
                let edge = &self.edges[current as usize];
                
                if let Some(twin) = edge.twin {
                    if let Some(next) = self.edges[twin as usize].next {
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
        count
    }
}
