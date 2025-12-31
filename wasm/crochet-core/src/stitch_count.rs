use crochet_types::AmigurumiConfig;
use std::f64::consts::PI;

/// Calculate stitch count for each row based on radii
pub fn calculate_stitch_counts(radii: &[f64], config: &AmigurumiConfig) -> Vec<usize> {
    if radii.is_empty() {
        return vec![];
    }

    // Convert each radius to ideal stitch count
    let ideal_counts: Vec<usize> = radii.iter().enumerate().map(|(i, &radius)| {
        if i == 0 {
            // Magic ring: standard 6 SC (not calculated from circumference!)
            return 6;
        }
        
        let r = radius.max(0.1);
        let circumference = 2.0 * PI * r;
        let stitches = (circumference * config.yarn.gauge_stitches_per_cm).round() as usize;
        stitches.max(6)
    }).collect();
    
    // Apply physical constraints: can't increase/decrease too fast
    let mut actual_counts = Vec::with_capacity(ideal_counts.len());
    actual_counts.push(ideal_counts[0]); // Magic ring: 6 SC
    
    for i in 1..ideal_counts.len() {
        let prev = actual_counts[i - 1];
        let ideal = ideal_counts[i];
        
        // Physical limit: INC can double at most, INVDEC can halve at most
        let max_increase = prev; // Can double (all INC)
        let max_decrease = prev / 2; // Can halve (all INVDEC)
        
        let actual = if ideal > prev {
            // Increasing: cap at doubling
            ideal.min(prev + max_increase)
        } else if ideal < prev {
            // Decreasing: cap at halving
            ideal.max(prev.saturating_sub(max_decrease))
        } else {
            ideal
        };
        
        actual_counts.push(actual.max(6));
    }
    
    actual_counts
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
    fn test_follows_curve_exactly() {
        // Pattern should follow curve exactly
        let radii = vec![2.0, 10.0, 2.0]; // Expansion then contraction
        let config = AmigurumiConfig {
            total_height_cm: 3.0,
            yarn: YarnSpec {
                gauge_stitches_per_cm: 3.0,
                gauge_rows_per_cm: 3.0,
                recommended_hook_size_mm: 3.5,
            },
        };

        let counts = calculate_stitch_counts(&radii, &config);
        
        // Should follow the radii pattern
        assert!(counts[0] < counts[1]); // Increases
        assert!(counts[2] < counts[1]); // Decreases
    }
}
