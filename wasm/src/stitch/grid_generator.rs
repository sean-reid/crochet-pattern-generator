use anyhow::Result;
use crate::CrochetConfig;
use crate::mesh::types::MeshData;
use super::{Stitch, StitchGrid, StitchType};

pub struct StitchGridGenerator {
    config: CrochetConfig,
}

impl StitchGridGenerator {
    pub fn new(config: CrochetConfig) -> Self { Self { config } }

    pub fn generate(&self, mesh: &MeshData, uv_coords: &[[f32; 2]]) -> Result<StitchGrid> {
        let mut stitches = Vec::new();
        let mut stitch_id = 0;
    
        let height = (mesh.bounds.max[1] - mesh.bounds.min[1]).abs();
        let total_target_rows = (height * self.config.rows_per_inch).ceil() as u32;
    
        let mut min_v = f32::INFINITY;
        let mut max_v = f32::NEG_INFINITY;
        for &[_, v] in uv_coords {
            min_v = min_v.min(v);
            max_v = max_v.max(v);
        }
    
        let v_range = max_v - min_v;
        let v_step = v_range / (total_target_rows.max(1) as f32);
    
        let mut rows = Vec::new();
    
        for row_idx in 0..total_target_rows {
            let v = min_v + row_idx as f32 * v_step;
            
            // FIX: Find the U-range active at THIS vertical slice
            let (row_min_u, row_max_u) = self.get_active_u_range(uv_coords, v, v_step);
            
            // Calculate physical width at this specific latitude
            let p_start = self.interpolate_position(mesh, uv_coords, [row_min_u, v]);
            let p_end = self.interpolate_position(mesh, uv_coords, [row_max_u, v]);
            let dx = p_end[0] - p_start[0];
            let dy = p_end[1] - p_start[1];
            let dz = p_end[2] - p_start[2];
            let physical_width = (dx*dx + dy*dy + dz*dz).sqrt();

            let mut row_target_count = (physical_width * self.config.stitches_per_inch).ceil() as u32;
            if row_target_count < 6 && (row_idx == 0 || row_idx == total_target_rows - 1) {
                row_target_count = 6; // Minimum for poles
            } else if row_target_count == 0 {
                row_target_count = 1;
            }

            let u_step = (row_max_u - row_min_u) / (row_target_count.max(1) as f32);
            let mut row_stitches = Vec::new();

            for col_idx in 0..row_target_count {
                let u = row_min_u + col_idx as f32 * u_step;
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

    fn get_active_u_range(&self, uv_coords: &[[f32; 2]], target_v: f32, tolerance: f32) -> (f32, f32) {
        let mut min_u = f32::INFINITY;
        let mut max_u = f32::NEG_INFINITY;
        let mut found = false;
        for &[u, v] in uv_coords {
            if (v - target_v).abs() <= tolerance {
                min_u = min_u.min(u);
                max_u = max_u.max(u);
                found = true;
            }
        }
        if !found { (0.0, 1.0) } else { (min_u, max_u) }
    }

    fn interpolate_position(&self, mesh: &MeshData, uv_coords: &[[f32; 2]], target_uv: [f32; 2]) -> [f32; 3] {
        let mut min_dist = f32::INFINITY;
        let mut closest = 0;
        for (i, &uv) in uv_coords.iter().enumerate() {
            let d = (uv[0] - target_uv[0]).powi(2) + (uv[1] - target_uv[1]).powi(2);
            if d < min_dist { min_dist = d; closest = i; }
        }
        mesh.vertices[closest].position
    }
}
