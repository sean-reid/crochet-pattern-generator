use super::{StitchGrid, Stitch};

pub struct PlacementOptimizer {
    _private: (),
}

impl PlacementOptimizer {
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Optimize stitch placement for better coverage and transitions
    pub fn optimize(&self, grid: &mut StitchGrid) {
        // 1. Smooth stitch positions
        self.smooth_positions(grid);
        
        // 2. Balance stitch density
        self.balance_density(grid);
        
        // 3. Adjust edge stitches
        self.adjust_edges(grid);
    }

    /// Smooth stitch positions using Laplacian smoothing
    fn smooth_positions(&self, grid: &mut StitchGrid) {
        let iterations = 3;
        let lambda = 0.5; // Smoothing factor
        
        for _ in 0..iterations {
            let mut new_positions = Vec::new();
            
            for stitch in &grid.stitches {
                if stitch.connections.is_empty() {
                    new_positions.push(stitch.position_3d);
                    continue;
                }
                
                // Average neighbor positions
                let mut avg_pos = [0.0, 0.0, 0.0];
                let mut count = 0.0;
                
                for &conn_id in &stitch.connections {
                    if let Some(neighbor) = grid.stitches.get(conn_id as usize) {
                        avg_pos[0] += neighbor.position_3d[0];
                        avg_pos[1] += neighbor.position_3d[1];
                        avg_pos[2] += neighbor.position_3d[2];
                        count += 1.0;
                    }
                }
                
                if count > 0.0 {
                    avg_pos[0] /= count;
                    avg_pos[1] /= count;
                    avg_pos[2] /= count;
                    
                    // Blend with original position
                    let new_pos = [
                        stitch.position_3d[0] * (1.0 - lambda) + avg_pos[0] * lambda,
                        stitch.position_3d[1] * (1.0 - lambda) + avg_pos[1] * lambda,
                        stitch.position_3d[2] * (1.0 - lambda) + avg_pos[2] * lambda,
                    ];
                    new_positions.push(new_pos);
                } else {
                    new_positions.push(stitch.position_3d);
                }
            }
            
            // Apply new positions
            for (i, stitch) in grid.stitches.iter_mut().enumerate() {
                stitch.position_3d = new_positions[i];
            }
        }
    }

    /// Balance stitch density by adjusting spacing
    fn balance_density(&self, grid: &mut StitchGrid) {
        for row_idx in 0..grid.rows.len() {
            let row = &grid.rows[row_idx];
            if row.len() < 3 {
                continue;
            }
            
            // Calculate average spacing in this row
            let mut total_spacing = 0.0;
            let mut spacing_count = 0;
            
            for i in 0..row.len() - 1 {
                let s1 = &grid.stitches[row[i] as usize];
                let s2 = &grid.stitches[row[i + 1] as usize];
                
                let dx = s2.position_2d[0] - s1.position_2d[0];
                let dy = s2.position_2d[1] - s1.position_2d[1];
                let dist = (dx * dx + dy * dy).sqrt();
                
                total_spacing += dist;
                spacing_count += 1;
            }
            
            if spacing_count > 0 {
                let _avg_spacing = total_spacing / spacing_count as f32;
                
                // Collect ideal positions first to avoid borrowing issues
                let mut ideal_positions = Vec::new();
                for i in 1..row.len() - 1 {
                    let prev = &grid.stitches[row[i - 1] as usize];
                    let next = &grid.stitches[row[i + 1] as usize];
                    
                    // Interpolate between neighbors
                    let ideal_u = (prev.position_2d[0] + next.position_2d[0]) * 0.5;
                    let ideal_v = (prev.position_2d[1] + next.position_2d[1]) * 0.5;
                    
                    ideal_positions.push((row[i] as usize, ideal_u, ideal_v));
                }
                
                // Apply positions
                let blend = 0.3;
                for (stitch_idx, ideal_u, ideal_v) in ideal_positions {
                    let curr = &mut grid.stitches[stitch_idx];
                    curr.position_2d[0] = curr.position_2d[0] * (1.0 - blend) + ideal_u * blend;
                    curr.position_2d[1] = curr.position_2d[1] * (1.0 - blend) + ideal_v * blend;
                }
            }
        }
    }

    /// Adjust edge stitches for better boundary handling
    fn adjust_edges(&self, grid: &mut StitchGrid) {
        if grid.rows.is_empty() {
            return;
        }
        
        // Process first and last rows
        let first_row = &grid.rows[0];
        let last_row = &grid.rows[grid.rows.len() - 1];
        
        // Adjust first row stitches
        for &_stitch_id in first_row {
            // First row stitches stay at their positions (they're the foundation)
            // Could add chain stitch markers here if needed
        }
        
        // Adjust last row stitches
        for &_stitch_id in last_row {
            // Last row might need finishing stitches marked
            // This is a good place to add fasten-off markers
        }
    }

    /// Calculate local density at a stitch position
    fn _calculate_local_density(&self, grid: &StitchGrid, stitch: &Stitch, radius: f32) -> f32 {
        let mut count = 0;
        
        for other in &grid.stitches {
            if other.id == stitch.id {
                continue;
            }
            
            let dx = other.position_2d[0] - stitch.position_2d[0];
            let dy = other.position_2d[1] - stitch.position_2d[1];
            let dist = (dx * dx + dy * dy).sqrt();
            
            if dist < radius {
                count += 1;
            }
        }
        
        count as f32 / (std::f32::consts::PI * radius * radius)
    }
}

impl Default for PlacementOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stitch::StitchType;

    #[test]
    fn test_optimizer_doesnt_crash() {
        let optimizer = PlacementOptimizer::new();
        
        let mut grid = StitchGrid {
            stitches: vec![
                Stitch {
                    id: 0,
                    stitch_type: StitchType::SingleCrochet,
                    position_3d: [0.0, 0.0, 0.0],
                    position_2d: [0.0, 0.0],
                    row: 0,
                    connections: vec![1],
                },
                Stitch {
                    id: 1,
                    stitch_type: StitchType::SingleCrochet,
                    position_3d: [1.0, 0.0, 0.0],
                    position_2d: [1.0, 0.0],
                    row: 0,
                    connections: vec![0],
                },
            ],
            rows: vec![vec![0, 1]],
        };
        
        optimizer.optimize(&mut grid);
        
        // Should complete without panic
        assert_eq!(grid.stitches.len(), 2);
    }
}
