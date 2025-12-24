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
        
        // For GLB files, binary data is embedded in blob
        let blob = gltf.blob.as_deref();
        
        // Parse buffer data - handle both blob and data URIs
        let buffer_data = self.load_buffers(&gltf, blob)?;
        
        let mut all_vertices = Vec::new();
        let mut all_faces = Vec::new();
        let mut vertex_offset = 0u32;
        
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                self.process_node(&node, &buffer_data, &mut all_vertices, &mut all_faces, &mut vertex_offset)?;
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

    fn load_buffers(&self, gltf: &Gltf, blob: Option<&[u8]>) -> Result<Vec<Option<Vec<u8>>>> {
        let mut buffer_data = Vec::new();
        
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Bin => {
                    // GLB embedded binary data
                    if let Some(blob_data) = blob {
                        buffer_data.push(Some(blob_data.to_vec()));
                    } else {
                        buffer_data.push(None);
                    }
                }
                gltf::buffer::Source::Uri(uri) => {
                    // Check if it's a data URI
                    if uri.starts_with("data:") {
                        match self.decode_data_uri(uri) {
                            Ok(decoded) => buffer_data.push(Some(decoded)),
                            Err(e) => {
                                crate::utils::log_error(&format!("Failed to decode data URI: {}", e));
                                buffer_data.push(None);
                            }
                        }
                    } else {
                        crate::utils::log_error(&format!("External URI not supported: {}", uri));
                        buffer_data.push(None);
                    }
                }
            }
        }
        
        Ok(buffer_data)
    }

    fn decode_data_uri(&self, uri: &str) -> Result<Vec<u8>> {
        // Parse data URI format: data:[<mediatype>][;base64],<data>
        if !uri.starts_with("data:") {
            anyhow::bail!("Not a data URI");
        }
        
        let uri = &uri[5..]; // Skip "data:"
        
        // Find the comma that separates metadata from data
        let comma_pos = uri.find(',')
            .ok_or_else(|| anyhow::anyhow!("Invalid data URI: no comma found"))?;
        
        let metadata = &uri[..comma_pos];
        let data = &uri[comma_pos + 1..];
        
        // Check if it's base64 encoded
        if metadata.contains("base64") {
            // Decode base64
            self.decode_base64(data)
        } else {
            // URL encoded - not commonly used for binary data
            anyhow::bail!("Only base64 data URIs are supported")
        }
    }

    fn decode_base64(&self, data: &str) -> Result<Vec<u8>> {
        // Simple base64 decoder
        let chars: Vec<char> = data.chars().filter(|c| !c.is_whitespace()).collect();
        let mut result = Vec::new();
        let mut i = 0;
        
        while i < chars.len() {
            let mut sextet = [0u8; 4];
            let mut sextet_count = 0;
            
            for j in 0..4 {
                if i + j >= chars.len() {
                    break;
                }
                
                let c = chars[i + j];
                sextet[j] = match c {
                    'A'..='Z' => (c as u8) - b'A',
                    'a'..='z' => (c as u8) - b'a' + 26,
                    '0'..='9' => (c as u8) - b'0' + 52,
                    '+' => 62,
                    '/' => 63,
                    '=' => break, // Padding
                    _ => anyhow::bail!("Invalid base64 character: {}", c),
                };
                sextet_count += 1;
            }
            
            if sextet_count >= 2 {
                result.push((sextet[0] << 2) | (sextet[1] >> 4));
            }
            if sextet_count >= 3 {
                result.push((sextet[1] << 4) | (sextet[2] >> 2));
            }
            if sextet_count >= 4 {
                result.push((sextet[2] << 6) | sextet[3]);
            }
            
            i += 4;
        }
        
        Ok(result)
    }

    fn process_node(
        &self,
        node: &gltf::Node,
        buffer_data: &[Option<Vec<u8>>],
        vertices: &mut Vec<Vertex>,
        faces: &mut Vec<Face>,
        vertex_offset: &mut u32,
    ) -> Result<()> {
        let transform = node.transform().matrix();
        
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                self.process_primitive(&primitive, buffer_data, &transform, vertices, faces, vertex_offset)?;
            }
        }
        
        for child in node.children() {
            self.process_node(&child, buffer_data, vertices, faces, vertex_offset)?;
        }
        
        Ok(())
    }

    fn process_primitive(
        &self,
        primitive: &gltf::Primitive,
        buffer_data: &[Option<Vec<u8>>],
        transform: &[[f32; 4]; 4],
        vertices: &mut Vec<Vertex>,
        faces: &mut Vec<Face>,
        vertex_offset: &mut u32,
    ) -> Result<()> {
        let reader = primitive.reader(|buffer| {
            buffer_data.get(buffer.index())
                .and_then(|opt| opt.as_ref().map(|v| v.as_slice()))
        });
        
        let positions = reader.read_positions()
            .context("Missing position attribute or buffer data not available")?;
        
        let normals = reader.read_normals();
        let tex_coords = reader.read_tex_coords(0).map(|tc| tc.into_f32());
        
        let start_idx = vertices.len();
        
        for (i, position) in positions.enumerate() {
            let pos = self.apply_transform(position, transform);
            
            let normal = if let Some(ref normals) = normals {
                normals.clone().nth(i)
                    .map(|n| self.apply_transform_normal(n, transform))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_data_uri() {
        let loader = GltfLoader::new();
        
        // Test base64 decoding
        let uri = "data:application/octet-stream;base64,AAAA";
        let result = loader.decode_data_uri(uri);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transform_identity() {
        let loader = GltfLoader::new();
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let pos = [1.0, 2.0, 3.0];
        let result = loader.apply_transform(pos, &identity);
        assert_eq!(result, pos);
    }
}
