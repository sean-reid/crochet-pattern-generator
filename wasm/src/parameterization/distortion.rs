use anyhow::Result;
use crate::mesh::types::MeshData;

pub struct DistortionAnalyzer {
    _private: (),
}

impl DistortionAnalyzer {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn compute_distortion(&self, mesh: &MeshData, uv_coords: &[[f32; 2]]) -> Result<Vec<f32>> {
        let mut distortions = vec![0.0; mesh.faces.len()];
        
        for (i, face) in mesh.faces.iter().enumerate() {
            let v0_3d = mesh.vertices[face.indices[0] as usize].position;
            let v1_3d = mesh.vertices[face.indices[1] as usize].position;
            let v2_3d = mesh.vertices[face.indices[2] as usize].position;
            
            let v0_2d = uv_coords[face.indices[0] as usize];
            let v1_2d = uv_coords[face.indices[1] as usize];
            let v2_2d = uv_coords[face.indices[2] as usize];
            
            // Compute edge lengths in 3D
            let e1_3d = [
                v1_3d[0] - v0_3d[0],
                v1_3d[1] - v0_3d[1],
                v1_3d[2] - v0_3d[2],
            ];
            let e2_3d = [
                v2_3d[0] - v0_3d[0],
                v2_3d[1] - v0_3d[1],
                v2_3d[2] - v0_3d[2],
            ];
            
            let len1_3d = (e1_3d[0].powi(2) + e1_3d[1].powi(2) + e1_3d[2].powi(2)).sqrt();
            let len2_3d = (e2_3d[0].powi(2) + e2_3d[1].powi(2) + e2_3d[2].powi(2)).sqrt();
            
            // Compute edge lengths in 2D
            let e1_2d = [v1_2d[0] - v0_2d[0], v1_2d[1] - v0_2d[1]];
            let e2_2d = [v2_2d[0] - v0_2d[0], v2_2d[1] - v0_2d[1]];
            
            let len1_2d = (e1_2d[0].powi(2) + e1_2d[1].powi(2)).sqrt();
            let len2_2d = (e2_2d[0].powi(2) + e2_2d[1].powi(2)).sqrt();
            
            // Compute stretch factors
            let stretch1 = if len1_3d > 1e-6 { len1_2d / len1_3d } else { 1.0 };
            let stretch2 = if len2_3d > 1e-6 { len2_2d / len2_3d } else { 1.0 };
            
            // Distortion is deviation from uniform scaling
            distortions[i] = ((stretch1 - stretch2).abs() / (stretch1 + stretch2 + 1e-6)).min(1.0);
        }
        
        Ok(distortions)
    }
}

impl Default for DistortionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
