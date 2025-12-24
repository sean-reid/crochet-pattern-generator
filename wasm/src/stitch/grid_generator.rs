use anyhow::Result;
use crate::CrochetConfig;
use crate::mesh::types::MeshData;
use super::{Stitch, StitchGrid, StitchType};

pub struct StitchGridGenerator {
    config: CrochetConfig,
}

impl StitchGridGenerator {
    pub fn new(config: CrochetConfig) -> Self {
        Self { config }
    }

    pub fn generate(&self, mesh: &MeshData, uv_coords: &[[f32; 2]]) -> Result<StitchGrid> {
        let mut stitches = Vec::new();
        let mut stitch_id = 0;
    
        // 1. Calculate the physical dimensions of the mesh from the bounding box
        let width = (mesh.bounds.max[0] - mesh.bounds.min[0]).abs();
        let height = (mesh.bounds.max[1] - mesh.bounds.min[1]).abs();
        let depth = (mesh.bounds.max[2] - mesh.bounds.min[2]).abs();
        
        // Use the largest horizontal dimension for width and vertical for height
        let physical_width = width.max(depth);
        let physical_height = height;
    
        // 2. Compute how many stitches/rows fit into that physical space based on gauge
        let total_target_stitches = (physical_width * self.config.stitches_per_inch).ceil() as u32;
        let total_target_rows = (physical_height * self.config.rows_per_inch).ceil() as u32;
    
        // 3. Find UV bounds to map the grid correctly
        let mut min_u = f32::INFINITY;
        let mut max_u = f32::NEG_INFINITY;
        let mut min_v = f32::INFINITY;
        let mut max_v = f32::NEG_INFINITY;
    
        for &[u, v] in uv_coords {
            min_u = min_u.min(u);
            max_u = max_u.max(u);
            min_v = min_v.min(v);
            max_v = max_v.max(v);
        }
    
        let u_range = max_u - min_u;
        let v_range = max_v - min_v;
    
        // 4. Calculate step sizes in UV space
        let u_step = u_range / (total_target_stitches.max(1) as f32);
        let v_step = v_range / (total_target_rows.max(1) as f32);
    
        crate::utils::log(&format!(
            "Scaling: Mesh is {:.2}x{:.2}in. Generating {} rows, {} stitches per row.",
            physical_width, physical_height, total_target_rows, total_target_stitches
        ));
    
        let mut rows = Vec::new();
    
        for row_idx in 0..total_target_rows {
            let v = min_v + row_idx as f32 * v_step;
            let mut row_stitches = Vec::new();
    
            for col_idx in 0..total_target_stitches {
                let u = min_u + col_idx as f32 * u_step;
    
                let pos_3d = self.interpolate_position(mesh, uv_coords, [u, v]);
    
                stitches.push(Stitch {
                    id: stitch_id,
                    stitch_type: StitchType::SingleCrochet,
                    position_3d: pos_3d,
                    position_2d: [u, v],
                    row: row_idx,
                    connections: Vec::new(),
                });
    
                row_stitches.push(stitch_id);
                stitch_id += 1;
            }
            rows.push(row_stitches);
        }
    
        Ok(StitchGrid { stitches, rows })
    }

    fn interpolate_position(&self, mesh: &MeshData, uv_coords: &[[f32; 2]], target_uv: [f32; 2]) -> [f32; 3] {
        // Find closest vertex
        let mut min_dist = f32::INFINITY;
        let mut closest_vertex = 0;

        for (i, &uv) in uv_coords.iter().enumerate() {
            let dist = (uv[0] - target_uv[0]).powi(2) + (uv[1] - target_uv[1]).powi(2);
            if dist < min_dist {
                min_dist = dist;
                closest_vertex = i;
            }
        }

        mesh.vertices[closest_vertex].position
    }
}
