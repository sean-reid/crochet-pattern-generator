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

        // Compute stitch spacing based on gauge
        let stitch_width = 1.0 / self.config.stitches_per_inch;
        let row_height = 1.0 / self.config.rows_per_inch;

        // Find UV bounds
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

        // Generate grid in UV space
        let num_rows = ((max_v - min_v) / row_height).ceil() as u32;
        let mut rows = Vec::new();

        for row_idx in 0..num_rows {
            let v = min_v + row_idx as f32 * row_height;
            let num_stitches = ((max_u - min_u) / stitch_width).ceil() as u32;
            let mut row_stitches = Vec::new();

            for col_idx in 0..num_stitches {
                let u = min_u + col_idx as f32 * stitch_width;

                // Find closest vertex for 3D position
                let pos_3d = self.interpolate_position(mesh, uv_coords, [u, v]);

                stitches.push(Stitch {
                    id: stitch_id,
                    stitch_type: StitchType::SingleCrochet,  // Default, will be classified
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
