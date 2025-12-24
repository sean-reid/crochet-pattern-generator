use super::StitchGrid;

pub struct StitchConnectivity {
    _private: (),
}

impl StitchConnectivity {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn build_connections(&self, grid: &mut StitchGrid) {
        // Connect stitches within rows and between rows
        for row_idx in 0..grid.rows.len() {
            let row = &grid.rows[row_idx];

            // Connect within row
            for i in 0..row.len() {
                let stitch_id = row[i];
                let stitch = &mut grid.stitches[stitch_id as usize];

                // Connect to next stitch in same row
                if i + 1 < row.len() {
                    stitch.connections.push(row[i + 1]);
                }

                // Connect to stitches in next row
                if row_idx + 1 < grid.rows.len() {
                    let next_row = &grid.rows[row_idx + 1];
                    // Connect to stitches in similar position
                    let target_idx = (i * next_row.len()) / row.len();
                    if target_idx < next_row.len() {
                        stitch.connections.push(next_row[target_idx]);
                    }
                }
            }
        }
    }
}

impl Default for StitchConnectivity {
    fn default() -> Self {
        Self::new()
    }
}
