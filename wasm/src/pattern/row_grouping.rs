use super::types::{CrochetPattern, RowInstruction};

pub struct RowGrouper {
    _private: (),
}

impl RowGrouper {
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Group similar consecutive rows together for pattern simplification
    pub fn group_rows(&self, pattern: &mut CrochetPattern) {
        if pattern.instructions.rows.len() < 2 {
            return;
        }

        let mut grouped_rows = Vec::new();
        let mut current_group_start = 0;
        let mut repeat_count = 1;

        for i in 1..pattern.instructions.rows.len() {
            if self.rows_are_similar(&pattern.instructions.rows[current_group_start], &pattern.instructions.rows[i]) {
                repeat_count += 1;
            } else {
                // Add the group
                if repeat_count > 1 {
                    // Create a grouped row instruction
                    let mut grouped = pattern.instructions.rows[current_group_start].clone();
                    grouped.number = pattern.instructions.rows[current_group_start].number;
                    
                    // Update instruction text to show repeat
                    for stitch_group in &mut grouped.stitches {
                        if repeat_count > 1 {
                            stitch_group.instruction = format!(
                                "{} (repeat for rows {}-{})",
                                stitch_group.instruction,
                                pattern.instructions.rows[current_group_start].number,
                                pattern.instructions.rows[i - 1].number
                            );
                        }
                    }
                    
                    grouped_rows.push(grouped);
                } else {
                    grouped_rows.push(pattern.instructions.rows[current_group_start].clone());
                }
                
                current_group_start = i;
                repeat_count = 1;
            }
        }

        // Add the last group
        if repeat_count > 1 {
            let mut grouped = pattern.instructions.rows[current_group_start].clone();
            for stitch_group in &mut grouped.stitches {
                stitch_group.instruction = format!(
                    "{} (repeat for rows {}-{})",
                    stitch_group.instruction,
                    pattern.instructions.rows[current_group_start].number,
                    pattern.instructions.rows[pattern.instructions.rows.len() - 1].number
                );
            }
            grouped_rows.push(grouped);
        } else {
            grouped_rows.push(pattern.instructions.rows[current_group_start].clone());
        }

        pattern.instructions.rows = grouped_rows;
    }

    /// Check if two rows have the same stitch pattern
    fn rows_are_similar(&self, row1: &RowInstruction, row2: &RowInstruction) -> bool {
        if row1.total_stitches != row2.total_stitches {
            return false;
        }

        if row1.stitches.len() != row2.stitches.len() {
            return false;
        }

        for (sg1, sg2) in row1.stitches.iter().zip(row2.stitches.iter()) {
            if sg1.count != sg2.count || sg1.stitch_type != sg2.stitch_type {
                return false;
            }
        }

        true
    }
}

impl Default for RowGrouper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stitch::StitchType;

    #[test]
    fn test_rows_are_similar() {
        let grouper = RowGrouper::new();
        
        let row1 = RowInstruction {
            number: 1,
            stitches: vec![StitchGroup {
                count: 10,
                stitch_type: StitchType::SingleCrochet,
                instruction: "10 sc".to_string(),
            }],
            total_stitches: 10,
        };

        let row2 = RowInstruction {
            number: 2,
            stitches: vec![StitchGroup {
                count: 10,
                stitch_type: StitchType::SingleCrochet,
                instruction: "10 sc".to_string(),
            }],
            total_stitches: 10,
        };

        assert!(grouper.rows_are_similar(&row1, &row2));
    }
}
