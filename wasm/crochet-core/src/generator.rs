use crochet_types::*;
use std::f64::consts::PI;

use crate::stitch_count::calculate_stitch_counts;
use crate::optimization::optimize_stitch_placement;

/// Find the radius at a specific height by searching through the curve
fn find_radius_at_height(curve: &ProfileCurve, target_height: f64) -> f64 {
    // Find which segment contains this height
    for segment in &curve.segments {
        let start_height = segment.start.y;
        let end_height = segment.end.y;
        
        // Check if target height is in this segment's range
        let (min_h, max_h) = if start_height < end_height {
            (start_height, end_height)
        } else {
            (end_height, start_height)
        };
        
        if target_height >= min_h && target_height <= max_h {
            // Binary search for the t value that gives us this height
            let t = find_t_for_height(segment, target_height);
            let point = segment.evaluate(t);
            return point.x.max(0.0);
        }
    }
    
    // If height is outside curve range, use nearest endpoint
    if target_height < curve.segments[0].start.y {
        return curve.segments[0].start.x.max(0.0);
    } else {
        let last = curve.segments.last().unwrap();
        return last.end.x.max(0.0);
    }
}

/// Find parameter t that gives a specific y-coordinate using binary search
fn find_t_for_height(segment: &SplineSegment, target_y: f64) -> f64 {
    let start_y = segment.start.y;
    let end_y = segment.end.y;
    
    // Handle edge cases
    if (target_y - start_y).abs() < 1e-6 {
        return 0.0;
    }
    if (target_y - end_y).abs() < 1e-6 {
        return 1.0;
    }
    
    // Check if target is outside segment range
    let (min_y, max_y) = if start_y < end_y {
        (start_y, end_y)
    } else {
        (end_y, start_y)
    };
    
    if target_y < min_y {
        return if start_y < end_y { 0.0 } else { 1.0 };
    }
    if target_y > max_y {
        return if start_y < end_y { 1.0 } else { 0.0 };
    }
    
    let mut t_min = 0.0;
    let mut t_max = 1.0;
    
    // Binary search for t value
    for _ in 0..30 {
        let t = (t_min + t_max) / 2.0;
        let point = segment.evaluate(t);
        
        if (point.y - target_y).abs() < 1e-6 {
            return t;
        }
        
        if start_y < end_y {
            // Increasing y
            if point.y < target_y {
                t_min = t;
            } else {
                t_max = t;
            }
        } else {
            // Decreasing y
            if point.y > target_y {
                t_min = t;
            } else {
                t_max = t;
            }
        }
    }
    
    (t_min + t_max) / 2.0
}

/// Main entry point for pattern generation
pub fn generate_pattern(
    curve: &ProfileCurve,
    config: &AmigurumiConfig,
) -> Result<CrochetPattern> {
    // Validate inputs
    validate_curve(curve)?;
    validate_config(config)?;

    // Determine the curve's y-range
    let curve_min_y = curve.segments[0].start.y;
    let curve_max_y = curve.segments.last().unwrap().end.y;
    let curve_height = curve_max_y - curve_min_y;
    
    if curve_height <= 0.0 {
        return Err(PatternError::InvalidProfileCurve(
            "Curve must have positive height".to_string(),
        ));
    }

    // Step 1: Calculate row heights based on gauge
    let row_height = 1.0 / config.yarn.gauge_rows_per_cm;
    let num_rows = (config.total_height_cm / row_height).round() as usize;
    let num_rows = num_rows.max(1);

    // Step 2: Get radius at each row height by evaluating the curve directly
    let mut row_radii = Vec::with_capacity(num_rows);
    for row_idx in 0..num_rows {
        // Map from config height to curve height
        let config_height = if row_idx == num_rows - 1 {
            config.total_height_cm
        } else {
            row_idx as f64 * row_height
        };
        
        // Scale to curve's coordinate system
        let curve_y = curve_min_y + (config_height / config.total_height_cm) * curve_height;
        let radius = find_radius_at_height(curve, curve_y);
        
        // Validate radius is reasonable
        if radius.is_nan() || radius.is_infinite() {
            return Err(PatternError::InternalError(
                format!("Invalid radius calculated at height {}: {}", config_height, radius),
            ));
        }
        
        row_radii.push(radius);
    }

    if row_radii.is_empty() {
        return Err(PatternError::InvalidProfileCurve(
            "No rows generated".to_string(),
        ));
    }

    // Step 3: Calculate stitch counts per row
    let stitch_counts = calculate_stitch_counts(&row_radii, config);

    // Step 4: Generate initial row patterns
    let mut rows = Vec::with_capacity(stitch_counts.len());

    for (row_idx, &total_stitches) in stitch_counts.iter().enumerate() {
        let pattern = if row_idx == 0 {
            // Special case: Row 1 is always the magic circle (all SC)
            (0..total_stitches)
                .map(|i| {
                    let angle = 2.0 * PI * i as f64 / total_stitches as f64;
                    StitchInstruction {
                        stitch_type: StitchType::SC,
                        angular_position: angle,
                        stitch_index: i,
                    }
                })
                .collect()
        } else {
            let prev_stitches = stitch_counts[row_idx - 1];
            generate_row_pattern(row_idx + 1, prev_stitches, total_stitches)
        };

        rows.push(Row {
            row_number: row_idx + 1,
            total_stitches,
            pattern,
        });
    }

    // Step 5: Optimize stitch placement
    let optimized_rows = optimize_stitch_placement(&rows);

    // Step 5.5: Validate patterns
    for (idx, row) in optimized_rows.iter().enumerate() {
        if idx > 0 {
            let prev_stitches = optimized_rows[idx - 1].total_stitches;
            validate_pattern(row, prev_stitches)?;
        }
    }

    // Step 6: Calculate metadata
    let metadata = calculate_metadata(&optimized_rows, config);

    Ok(CrochetPattern {
        rows: optimized_rows,
        metadata,
    })
}

/// Validate profile curve
fn validate_curve(curve: &ProfileCurve) -> Result<()> {
    if curve.segments.is_empty() {
        return Err(PatternError::InvalidProfileCurve(
            "Curve has no segments".to_string(),
        ));
    }

    if curve.start_radius < 0.0 {
        return Err(PatternError::InvalidProfileCurve(
            "Start radius must be non-negative".to_string(),
        ));
    }

    if curve.end_radius < 0.0 {
        return Err(PatternError::InvalidProfileCurve(
            "End radius must be non-negative".to_string(),
        ));
    }

    // Check continuity
    for i in 1..curve.segments.len() {
        let prev_end = curve.segments[i - 1].end;
        let curr_start = curve.segments[i].start;

        let dist = prev_end.distance_to(&curr_start);
        if dist > 1e-6 {
            return Err(PatternError::InvalidProfileCurve(
                "Curve segments are not continuous".to_string(),
            ));
        }
    }

    Ok(())
}

/// Validate configuration
fn validate_config(config: &AmigurumiConfig) -> Result<()> {
    if config.total_height_cm <= 0.0 {
        return Err(PatternError::InvalidConfiguration(
            "Height must be positive".to_string(),
        ));
    }

    if config.yarn.gauge_stitches_per_cm <= 0.0 {
        return Err(PatternError::InvalidConfiguration(
            "Gauge stitches per cm must be positive".to_string(),
        ));
    }

    if config.yarn.gauge_rows_per_cm <= 0.0 {
        return Err(PatternError::InvalidConfiguration(
            "Gauge rows per cm must be positive".to_string(),
        ));
    }

    if config.yarn.recommended_hook_size_mm <= 0.0 {
        return Err(PatternError::InvalidConfiguration(
            "Hook size must be positive".to_string(),
        ));
    }

    Ok(())
}

/// Generate pattern for a single row
/// 
/// In crochet, you work INTO the stitches of the previous row.
/// - pattern length = prev_stitches (one instruction per stitch from previous row)
/// - each instruction consumes stitches from prev row and produces stitches in current row
/// - SC: consumes 1, produces 1
/// - INC: consumes 1, produces 2
/// - INVDEC: consumes 2, produces 1
fn generate_row_pattern(
    _row_number: usize,
    prev_stitches: usize,
    total_stitches: usize,
) -> Vec<StitchInstruction> {
    let delta = total_stitches as i32 - prev_stitches as i32;

    if delta == 0 {
        // All single crochet - one instruction per previous stitch
        let mut pattern = Vec::with_capacity(prev_stitches);
        for i in 0..prev_stitches {
            let angle = 2.0 * PI * i as f64 / prev_stitches as f64;
            pattern.push(StitchInstruction {
                stitch_type: StitchType::SC,
                angular_position: angle,
                stitch_index: i,
            });
        }
        pattern
    } else if delta > 0 {
        // Increases needed: some stitches will be INC (produces 2), rest SC (produces 1)
        let num_increases = delta as usize;
        
        let mut pattern = Vec::with_capacity(prev_stitches);
        let mut inc_count = 0;
        
        // Distribute increases evenly across all positions
        for i in 0..prev_stitches {
            let angle = 2.0 * PI * i as f64 / prev_stitches as f64;
            
            // How many increases should we have placed by position i+1?
            let target_inc_count = ((i + 1) * num_increases + prev_stitches - 1) / prev_stitches;
            
            // If we need more increases, place one here
            let should_inc = inc_count < target_inc_count;

            let stitch_type = if should_inc {
                inc_count += 1;
                StitchType::INC
            } else {
                StitchType::SC
            };

            pattern.push(StitchInstruction {
                stitch_type,
                angular_position: angle,
                stitch_index: i,
            });
        }
        pattern
    } else {
        // Decreases needed: INVDEC consumes 2 stitches, produces 1
        let num_decreases = (-delta) as usize;
        
        let mut pattern = Vec::new();
        let mut i = 0;
        let mut dec_count = 0;
        
        while i < prev_stitches {
            let angle = 2.0 * PI * i as f64 / prev_stitches as f64;
            
            // How many decreases should we have placed by consuming position i+1?
            let target_dec_count = ((i + 1) * num_decreases + prev_stitches - 1) / prev_stitches;
            
            let should_dec = dec_count < target_dec_count && i + 1 < prev_stitches;

            if should_dec {
                // INVDEC: work into this stitch and the next
                pattern.push(StitchInstruction {
                    stitch_type: StitchType::INVDEC,
                    angular_position: angle,
                    stitch_index: i,
                });
                dec_count += 1;
                i += 2; // Skip next stitch (it's consumed by INVDEC)
            } else {
                // SC: work into this stitch normally
                pattern.push(StitchInstruction {
                    stitch_type: StitchType::SC,
                    angular_position: angle,
                    stitch_index: i,
                });
                i += 1;
            }
        }
        
        pattern
    }
}

/// Validate pattern correctness
fn validate_pattern(row: &Row, prev_row_stitches: usize) -> Result<()> {
    // Calculate how many stitches from previous row are consumed
    let mut prev_consumed = 0;
    let mut current_produced = 0;
    
    for instruction in &row.pattern {
        match instruction.stitch_type {
            StitchType::SC => {
                prev_consumed += 1;
                current_produced += 1;
            }
            StitchType::INC => {
                prev_consumed += 1;
                current_produced += 2;
            }
            StitchType::DEC | StitchType::INVDEC => {
                prev_consumed += 2;
                current_produced += 1;
            }
        }
    }
    
    // Verify we consumed all stitches from previous row
    if prev_consumed != prev_row_stitches {
        return Err(PatternError::InternalError(
            format!(
                "Row {}: pattern consumes {} stitches but previous row has {}",
                row.row_number, prev_consumed, prev_row_stitches
            ),
        ));
    }
    
    // Verify we produced the expected number of stitches
    if current_produced != row.total_stitches {
        return Err(PatternError::InternalError(
            format!(
                "Row {}: pattern produces {} stitches but expects {}",
                row.row_number, current_produced, row.total_stitches
            ),
        ));
    }
    
    Ok(())
}

/// Calculate pattern metadata
fn calculate_metadata(rows: &[Row], config: &AmigurumiConfig) -> PatternMetadata {
    let total_rows = rows.len();
    let total_stitches: usize = rows.iter().map(|r| r.total_stitches).sum();

    // Estimate time: ~2 seconds per stitch
    let estimated_time_minutes = (total_stitches as f64 * 2.0) / 60.0;

    // Estimate yarn length
    // Average stitch uses ~1cm of yarn, plus circumference for each row
    let mut yarn_length_cm = 0.0;
    for row in rows.iter() {
        // Estimate radius from stitch count (reverse of stitch calculation)
        let circumference = row.total_stitches as f64 / config.yarn.gauge_stitches_per_cm;
        let radius = circumference / (2.0 * PI);
        
        // Yarn used = circumference + ~1cm per stitch
        yarn_length_cm += circumference + row.total_stitches as f64 * 1.0;
    }

    PatternMetadata {
        total_rows,
        total_stitches,
        estimated_time_minutes,
        yarn_length_meters: yarn_length_cm / 100.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_curve() -> ProfileCurve {
        ProfileCurve {
            segments: vec![SplineSegment {
                start: Point2D::new(2.0, 0.0),
                control1: Point2D::new(2.0, 3.33),
                control2: Point2D::new(2.0, 6.67),
                end: Point2D::new(2.0, 10.0),
            }],
            start_radius: 2.0,
            end_radius: 2.0,
        }
    }

    fn create_test_config() -> AmigurumiConfig {
        AmigurumiConfig {
            total_height_cm: 10.0,
            yarn: YarnSpec {
                gauge_stitches_per_cm: 3.0,
                gauge_rows_per_cm: 3.0,
                recommended_hook_size_mm: 3.5,
            },
        }
    }

    #[test]
    fn test_generate_cylinder_pattern() {
        let curve = create_test_curve();
        let config = create_test_config();

        let result = generate_pattern(&curve, &config);
        assert!(result.is_ok());

        let pattern = result.unwrap();
        assert!(pattern.rows.len() > 0);
        assert_eq!(pattern.metadata.total_rows, pattern.rows.len());
    }

    #[test]
    fn test_validate_empty_curve() {
        let curve = ProfileCurve {
            segments: vec![],
            start_radius: 2.0,
            end_radius: 2.0,
        };

        assert!(validate_curve(&curve).is_err());
    }

    #[test]
    fn test_validate_negative_config() {
        let mut config = create_test_config();
        config.total_height_cm = -1.0;

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_generate_row_pattern_no_change() {
        let pattern = generate_row_pattern(1, 12, 12);
        assert_eq!(pattern.len(), 12);

        for stitch in &pattern {
            assert_eq!(stitch.stitch_type, StitchType::SC);
        }
    }

    #[test]
    fn test_generate_row_pattern_increases() {
        // Row has 12 stitches, next needs 18 (delta = +6)
        let pattern = generate_row_pattern(2, 12, 18);
        
        // Should have 12 instructions (one per previous stitch)
        assert_eq!(pattern.len(), 12);

        let inc_count = pattern
            .iter()
            .filter(|s| s.stitch_type == StitchType::INC)
            .count();
        
        // Should have 6 INC (produces 12 stitches) and 6 SC (produces 6 stitches) = 18 total
        assert_eq!(inc_count, 6);
        
        let sc_count = pattern
            .iter()
            .filter(|s| s.stitch_type == StitchType::SC)
            .count();
        assert_eq!(sc_count, 6);
        
        // Verify total: 6 INC * 2 + 6 SC * 1 = 18
        let total_produced: usize = pattern
            .iter()
            .map(|s| match s.stitch_type {
                StitchType::INC => 2,
                StitchType::SC => 1,
                _ => 0,
            })
            .sum();
        assert_eq!(total_produced, 18);
    }

    #[test]
    fn test_generate_row_pattern_decreases() {
        // Row has 18 stitches, next needs 12 (delta = -6)
        let pattern = generate_row_pattern(3, 18, 12);
        
        // Count stitches consumed from previous row
        let consumed: usize = pattern
            .iter()
            .map(|s| match s.stitch_type {
                StitchType::INVDEC => 2,  // consumes 2 from prev
                StitchType::SC => 1,       // consumes 1 from prev
                _ => 0,
            })
            .sum();
        assert_eq!(consumed, 18);
        
        // Count stitches produced in current row
        let produced: usize = pattern
            .iter()
            .map(|s| match s.stitch_type {
                StitchType::INVDEC => 1,  // produces 1 in current
                StitchType::SC => 1,       // produces 1 in current
                _ => 0,
            })
            .sum();
        assert_eq!(produced, 12);

        let dec_count = pattern
            .iter()
            .filter(|s| s.stitch_type == StitchType::INVDEC)
            .count();
        
        // Should have 6 INVDEC (consumes 12, produces 6) and 6 SC (consumes 6, produces 6)
        assert_eq!(dec_count, 6);
    }
}
