use anyhow::{Context, Result};
use gltf::Gltf;
use crate::mesh::types::{MeshData, Vertex, Face, BoundingBox};

pub struct GltfLoader {
    _private: (),
}

impl GltfLoader {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn load_from_bytes(&self, data: &[u8]) -> Result<MeshData> {
        let gltf = Gltf::from_slice(data).context("Failed to parse GLTF data")?;
        let blob = gltf.blob.as_deref();
        
        let mut all_vertices = Vec::new();
        let mut all_faces = Vec::new();
        let mut vertex_offset = 0u32;
        
        // Load buffer data
        let buffer_data = gltf.buffers().map(|buffer| {
            match buffer.source() {
                gltf::buffer::Source::Bin => {
                    blob.map(|b| gltf::buffer::Data(b.to_vec()))
                        .ok_or_else(|| anyhow::anyhow!("Binary buffer missing"))
                }
                gltf::buffer::Source::Uri(_) => {
                    Err(anyhow::anyhow!("External URIs not supported in WASM"))
                }
            }
        }).collect::<Result<Vec<_>>>()?;
        
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                self.process_node(&node, &buffer_data, blob, &mut all_vertices, &mut all_faces, &mut vertex_offset)?;
            }
        }
        
        if all_vertices.is_empty() {
            anyhow::bail!("No mesh data found in GLTF file");
        }
        
        let bounds = self.compute_bounds(&all_vertices);
        
        Ok(MeshData {
            vertices: all_vertices,
            faces: all_faces,
            bounds,
        })
    }

    fn process_node(
        &self,
        node: &gltf::Node,
        buffers: &[gltf::buffer::Data],
        blob: Option<&[u8]>,
        vertices: &mut Vec<Vertex>,
        faces: &mut Vec<Face>,
        vertex_offset: &mut u32,
    ) -> Result<()> {
        let transform = node.transform().matrix();
        
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                self.process_primitive(&primitive, buffers, blob, &transform, vertices, faces, vertex_offset)?;
            }
        }
        
        for child in node.children() {
            self.process_node(&child, buffers, blob, vertices, faces, vertex_offset)?;
        }
        
        Ok(())
    }

    fn process_primitive(
        &self,
        primitive: &gltf::Primitive,
        buffers: &[gltf::buffer::Data],
        blob: Option<&[u8]>,
        transform: &[[f32; 4]; 4],
        vertices: &mut Vec<Vertex>,
        faces: &mut Vec<Face>,
        vertex_offset: &mut u32,
    ) -> Result<()> {
        let reader = primitive.reader(|buffer| {
            match buffer.source() {
                gltf::buffer::Source::Bin => blob,
                gltf::buffer::Source::Uri(_) => buffers.get(buffer.index()).map(|b| b.0.as_slice()),
            }
        });
        
        let positions = reader.read_positions()
            .context("Missing position attribute")?;
        
        let normals = reader.read_normals();
        let tex_coords = reader.read_tex_coords(0).map(|tc| tc.into_f32());
        
        let start_idx = vertices.len();
        
        for (i, position) in positions.enumerate() {
            let pos = self.apply_transform(position, transform);
            
            let normal = if let Some(ref normals) = normals {
                normals.clone().nth(i).map(|n| self.apply_transform_normal(n, transform))
                    .unwrap_or([0.0, 1.0, 0.0])
            } else {
                [0.0, 1.0, 0.0]
            };
            
            let uv = if let Some(ref tex_coords) = tex_coords {
                tex_coords.clone().nth(i).unwrap_or([0.0, 0.0])
            } else {
                [0.0, 0.0]
            };
            
            vertices.push(Vertex {
                position: pos,
                normal,
                uv,
                curvature: None,
            });
        }
        
        if let Some(indices) = reader.read_indices() {
            let indices_vec: Vec<u32> = indices.into_u32().collect();
            for chunk in indices_vec.chunks_exact(3) {
                faces.push(Face {
                    indices: [
                        chunk[0] + *vertex_offset,
                        chunk[1] + *vertex_offset,
                        chunk[2] + *vertex_offset,
                    ],
                });
            }
        } else {
            let vertex_count = vertices.len() - start_idx;
            for i in (0..vertex_count).step_by(3) {
                if i + 2 < vertex_count {
                    faces.push(Face {
                        indices: [
                            (start_idx + i) as u32,
                            (start_idx + i + 1) as u32,
                            (start_idx + i + 2) as u32,
                        ],
                    });
                }
            }
        }
        
        *vertex_offset = vertices.len() as u32;
        
        Ok(())
    }

    fn apply_transform(&self, position: [f32; 3], transform: &[[f32; 4]; 4]) -> [f32; 3] {
        let x = position[0] * transform[0][0] + position[1] * transform[1][0] + position[2] * transform[2][0] + transform[3][0];
        let y = position[0] * transform[0][1] + position[1] * transform[1][1] + position[2] * transform[2][1] + transform[3][1];
        let z = position[0] * transform[0][2] + position[1] * transform[1][2] + position[2] * transform[2][2] + transform[3][2];
        [x, y, z]
    }

    fn apply_transform_normal(&self, normal: [f32; 3], transform: &[[f32; 4]; 4]) -> [f32; 3] {
        let x = normal[0] * transform[0][0] + normal[1] * transform[1][0] + normal[2] * transform[2][0];
        let y = normal[0] * transform[0][1] + normal[1] * transform[1][1] + normal[2] * transform[2][1];
        let z = normal[0] * transform[0][2] + normal[1] * transform[1][2] + normal[2] * transform[2][2];
        
        let len = (x * x + y * y + z * z).sqrt();
        if len > 1e-6 {
            [x / len, y / len, z / len]
        } else {
            [0.0, 1.0, 0.0]
        }
    }

    fn compute_bounds(&self, vertices: &[Vertex]) -> BoundingBox {
        if vertices.is_empty() {
            return BoundingBox {
                min: [0.0, 0.0, 0.0],
                max: [0.0, 0.0, 0.0],
            };
        }
        
        let mut min = vertices[0].position;
        let mut max = vertices[0].position;
        
        for vertex in vertices.iter().skip(1) {
            min[0] = min[0].min(vertex.position[0]);
            min[1] = min[1].min(vertex.position[1]);
            min[2] = min[2].min(vertex.position[2]);
            
            max[0] = max[0].max(vertex.position[0]);
            max[1] = max[1].max(vertex.position[1]);
            max[2] = max[2].max(vertex.position[2]);
        }
        
        BoundingBox { min, max }
    }
}

impl Default for GltfLoader {
    fn default() -> Self {
        Self::new()
    }
}
