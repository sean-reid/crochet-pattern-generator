use crochet_types::AmigurumiConfig;
use std::f64::consts::PI;

/// Calculate stitch count for each row based on radii
pub fn calculate_stitch_counts(radii: &[f64], config: &AmigurumiConfig) -> Vec<usize> {
    if radii.is_empty() {
        return vec![];
    }

    // Validate input radii
    for (i, &r) in radii.iter().enumerate() {
        if r.is_nan() || r.is_infinite() {
            eprintln!("Warning: Invalid radius at index {}: {}", i, r);
            return vec![];
        }
    }

    // Use radii directly from the drawn curve
    // No scaling - the drawn curve defines both shape and size
    let mut stitch_counts = Vec::with_capacity(radii.len());

    // First row (magic circle)
    let first_radius = radii[0].max(0.5); // Minimum 0.5cm radius
    let first_circumference = 2.0 * PI * first_radius;
    let first_stitches = (first_circumference * config.yarn.gauge_stitches_per_cm).round() as usize;
    let first_stitches = first_stitches.max(6); // Minimum 6 stitches for magic circle
    stitch_counts.push(first_stitches);

    // Subsequent rows
    for i in 1..radii.len() {
        let radius = radii[i].max(0.1); // Minimum 0.1cm radius
        let circumference = 2.0 * PI * radius;
        let ideal_stitches = circumference * config.yarn.gauge_stitches_per_cm;
        let mut stitches = ideal_stitches.round() as usize;

        // Enforce maximum change constraint (16.7% per row)
        let prev_stitches = stitch_counts[i - 1];
        let max_delta = (prev_stitches as f64 / 6.0).max(6.0) as usize;

        if stitches > prev_stitches + max_delta {
            stitches = prev_stitches + max_delta;
        } else if stitches + max_delta < prev_stitches {
            stitches = prev_stitches.saturating_sub(max_delta);
        }

        // Ensure at least 6 stitches
        stitches = stitches.max(6);

        stitch_counts.push(stitches);
    }

    stitch_counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crochet_types::YarnSpec;

    #[test]
    fn test_constant_radius() {
        let radii = vec![5.0; 10];
        let config = AmigurumiConfig {
            total_height_cm: 10.0,
            yarn: YarnSpec {
                gauge_stitches_per_cm: 3.0,
                gauge_rows_per_cm: 3.0,
                recommended_hook_size_mm: 3.5,
            },
        };

        let counts = calculate_stitch_counts(&radii, &config);
        assert_eq!(counts.len(), 10);

        // Should have approximately the same count for all rows
        let first = counts[0];
        for &count in &counts {
            assert!((count as i32 - first as i32).abs() <= 1);
        }
    }

    #[test]
    fn test_increasing_radius() {
        let radii: Vec<f64> = (0..10).map(|i| 2.0 + i as f64 * 0.5).collect();
        let config = AmigurumiConfig {
            total_height_cm: 10.0,
            yarn: YarnSpec {
                gauge_stitches_per_cm: 3.0,
                gauge_rows_per_cm: 3.0,
                recommended_hook_size_mm: 3.5,
            },
        };

        let counts = calculate_stitch_counts(&radii, &config);
        assert_eq!(counts.len(), 10);

        // Should be monotonically increasing
        for i in 1..counts.len() {
            assert!(counts[i] >= counts[i - 1]);
        }
    }

    #[test]
    fn test_minimum_stitches() {
        let radii = vec![0.1, 0.2, 0.3];
        let config = AmigurumiConfig {
            total_height_cm: 1.0,
            yarn: YarnSpec {
                gauge_stitches_per_cm: 3.0,
                gauge_rows_per_cm: 3.0,
                recommended_hook_size_mm: 3.5,
            },
        };

        let counts = calculate_stitch_counts(&radii, &config);

        // All counts should be at least 6
        for &count in &counts {
            assert!(count >= 6);
        }
    }

    #[test]
    fn test_max_delta_constraint() {
        // Create a scenario with large jump
        let radii = vec![2.0, 10.0, 10.0];
        let config = AmigurumiConfig {
            total_height_cm: 3.0,
            yarn: YarnSpec {
                gauge_stitches_per_cm: 3.0,
                gauge_rows_per_cm: 3.0,
                recommended_hook_size_mm: 3.5,
            },
        };

        let counts = calculate_stitch_counts(&radii, &config);

        // Check that change between rows doesn't exceed constraint
        for i in 1..counts.len() {
            let delta = (counts[i] as i32 - counts[i - 1] as i32).abs() as usize;
            let max_delta = (counts[i - 1] as f64 / 6.0).max(6.0) as usize;
            assert!(delta <= max_delta);
        }
    }
}
