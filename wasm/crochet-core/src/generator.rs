use crochet_types::*;
use std::f64::consts::PI;

use crate::sampling::sample_profile_curve;
use crate::radius::calculate_radius_profile;
use crate::row_mapping::map_samples_to_rows;
use crate::stitch_count::calculate_stitch_counts;
use crate::optimization::optimize_stitch_placement;

/// Main entry point for pattern generation
pub fn generate_pattern(
    curve: &ProfileCurve,
    config: &AmigurumiConfig,
) -> Result<CrochetPattern> {
    // Validate inputs
    validate_curve(curve)?;
    validate_config(config)?;

    // Step 1: Sample the profile curve uniformly along arc length
    let num_samples = 200; // High resolution for smooth curves
    let samples = sample_profile_curve(curve, num_samples);

    if samples.is_empty() {
        return Err(PatternError::InvalidProfileCurve(
            "No samples generated from curve".to_string(),
        ));
    }

    // Step 2: Calculate radius profile
    let radii = calculate_radius_profile(&samples);

    // Step 3: Map samples to rows based on gauge
    let row_to_sample = map_samples_to_rows(&samples, config);

    // Step 4: Get radii for each row
    let row_radii: Vec<f64> = row_to_sample.iter().map(|&idx| radii[idx]).collect();

    // Step 5: Calculate stitch counts per row
    let stitch_counts = calculate_stitch_counts(&row_radii, config);

    // Step 6: Generate initial row patterns
    let mut rows = Vec::with_capacity(stitch_counts.len());

    for (row_idx, &total_stitches) in stitch_counts.iter().enumerate() {
        let prev_stitches = if row_idx > 0 {
            stitch_counts[row_idx - 1]
        } else {
            0
        };

        let delta = total_stitches as i32 - prev_stitches as i32;

        let pattern = generate_row_pattern(row_idx + 1, total_stitches, delta);

        rows.push(Row {
            row_number: row_idx + 1,
            total_stitches,
            pattern,
        });
    }

    // Step 7: Optimize stitch placement
    let optimized_rows = optimize_stitch_placement(&rows);

    // Step 8: Calculate metadata
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

    if config.start_diameter_cm < 0.0 {
        return Err(PatternError::InvalidConfiguration(
            "Start diameter must be non-negative".to_string(),
        ));
    }

    if config.end_diameter_cm < 0.0 {
        return Err(PatternError::InvalidConfiguration(
            "End diameter must be non-negative".to_string(),
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
fn generate_row_pattern(
    _row_number: usize,
    total_stitches: usize,
    delta: i32,
) -> Vec<StitchInstruction> {
    let mut pattern = Vec::with_capacity(total_stitches);

    if delta == 0 {
        // All single crochet
        for i in 0..total_stitches {
            let angle = 2.0 * PI * i as f64 / total_stitches as f64;
            pattern.push(StitchInstruction {
                stitch_type: StitchType::SC,
                angular_position: angle,
                stitch_index: i,
            });
        }
    } else if delta > 0 {
        // Increases needed
        let num_increases = delta as usize;
        let spacing = total_stitches as f64 / num_increases as f64;

        let mut inc_count = 0;
        for i in 0..total_stitches {
            let angle = 2.0 * PI * i as f64 / total_stitches as f64;
            let should_inc = (i as f64 / spacing).floor() as usize > inc_count
                && inc_count < num_increases;

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
    } else {
        // Decreases needed
        let num_decreases = (-delta) as usize;
        let spacing = total_stitches as f64 / num_decreases as f64;

        // Use invisible decreases for better appearance
        let mut dec_count = 0;
        for i in 0..total_stitches {
            let angle = 2.0 * PI * i as f64 / total_stitches as f64;
            let should_dec = (i as f64 / spacing).floor() as usize > dec_count
                && dec_count < num_decreases;

            let stitch_type = if should_dec {
                dec_count += 1;
                StitchType::INVDEC
            } else {
                StitchType::SC
            };

            pattern.push(StitchInstruction {
                stitch_type,
                angular_position: angle,
                stitch_index: i,
            });
        }
    }

    pattern
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
    for (i, row) in rows.iter().enumerate() {
        let radius = if i < rows.len() {
            let row_height = config.total_height_cm / total_rows as f64;
            let height = i as f64 * row_height;
            // Rough estimate based on position
            config.start_diameter_cm / 2.0
                + (config.end_diameter_cm / 2.0 - config.start_diameter_cm / 2.0)
                    * height
                    / config.total_height_cm
        } else {
            config.start_diameter_cm / 2.0
        };

        let circumference = 2.0 * PI * radius;
        yarn_length_cm += row.total_stitches as f64 * 1.0 + circumference;
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
            start_diameter_cm: 4.0,
            end_diameter_cm: 4.0,
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
        let pattern = generate_row_pattern(1, 12, 0);
        assert_eq!(pattern.len(), 12);

        for stitch in &pattern {
            assert_eq!(stitch.stitch_type, StitchType::SC);
        }
    }

    #[test]
    fn test_generate_row_pattern_increases() {
        let pattern = generate_row_pattern(2, 18, 6);
        assert_eq!(pattern.len(), 18);

        let inc_count = pattern
            .iter()
            .filter(|s| s.stitch_type == StitchType::INC)
            .count();
        assert_eq!(inc_count, 6);
    }

    #[test]
    fn test_generate_row_pattern_decreases() {
        let pattern = generate_row_pattern(3, 12, -6);
        assert_eq!(pattern.len(), 12);

        let dec_count = pattern
            .iter()
            .filter(|s| s.stitch_type == StitchType::INVDEC)
            .count();
        assert_eq!(dec_count, 6);
    }
}
