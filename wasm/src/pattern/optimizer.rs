use anyhow::Result;
use crate::CrochetConfig;
use crate::stitch::{StitchGrid, StitchType};
use super::types::{CrochetPattern, PatternMetadata, PatternInstructions, Dimensions, RowInstruction, StitchGroup};

pub struct PatternOptimizer {
    config: CrochetConfig,
}

impl PatternOptimizer {
    pub fn new(config: CrochetConfig) -> Self {
        Self { config }
    }

    pub fn optimize(&self, grid: StitchGrid) -> Result<CrochetPattern> {
        // Build row instructions
        let mut row_instructions = Vec::new();

        for (row_num, row_stitch_ids) in grid.rows.iter().enumerate() {
            let mut stitch_groups = Vec::new();
            let mut current_type: Option<StitchType> = None;
            let mut current_count = 0;

            for &stitch_id in row_stitch_ids {
                let stitch = &grid.stitches[stitch_id as usize];

                if Some(stitch.stitch_type) == current_type {
                    current_count += 1;
                } else {
                    if let Some(st_type) = current_type {
                        stitch_groups.push(StitchGroup {
                            count: current_count,
                            stitch_type: st_type,
                            instruction: format!("{} {}", current_count, st_type.abbreviation()),
                        });
                    }
                    current_type = Some(stitch.stitch_type);
                    current_count = 1;
                }
            }

            // Push final group
            if let Some(st_type) = current_type {
                stitch_groups.push(StitchGroup {
                    count: current_count,
                    stitch_type: st_type,
                    instruction: format!("{} {}", current_count, st_type.abbreviation()),
                });
            }

            row_instructions.push(RowInstruction {
                number: row_num as u32 + 1,
                stitches: stitch_groups,
                total_stitches: row_stitch_ids.len(),
            });
        }

        let metadata = PatternMetadata {
            stitch_count: grid.stitches.len(),
            row_count: grid.rows.len(),
            estimated_time: self.estimate_time(grid.stitches.len()),
            yarn_estimate: self.estimate_yarn(grid.stitches.len()),
            dimensions: Dimensions {
                width: 6.0,  // Placeholder
                height: 6.0,
                depth: 2.0,
            },
        };

        Ok(CrochetPattern {
            metadata,
            stitches: grid.stitches,
            instructions: PatternInstructions { rows: row_instructions },
            diagram: None,
        })
    }

    fn estimate_time(&self, stitch_count: usize) -> String {
        let minutes = (stitch_count as f32 * 0.5).round() as u32;
        let hours = minutes / 60;
        let mins = minutes % 60;
        format!("{}h {}m", hours, mins)
    }

    fn estimate_yarn(&self, stitch_count: usize) -> String {
        let yards = (stitch_count as f32 * 0.5).round() as u32;
        format!("{} yards", yards)
    }
}
