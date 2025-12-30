use crochet_types::{AmigurumiConfig, Point2D};

/// Map sampled points to row indices based on yarn gauge
pub fn map_samples_to_rows(samples: &[Point2D], config: &AmigurumiConfig) -> Vec<usize> {
    if samples.is_empty() {
        return vec![];
    }

    // Calculate row height
    let row_height = 1.0 / config.yarn.gauge_rows_per_cm;

    // Calculate total number of rows
    let num_rows = (config.total_height_cm / row_height).ceil() as usize;
    let num_rows = num_rows.max(1);

    let mut row_to_sample = Vec::with_capacity(num_rows);

    // Map each row to nearest sample by height
    for row_idx in 0..num_rows {
        let target_height = row_idx as f64 * row_height;

        // Binary search for nearest sample
        let sample_idx = find_nearest_sample_by_height(samples, target_height);
        row_to_sample.push(sample_idx);
    }

    row_to_sample
}

/// Find the sample index with height closest to target height
fn find_nearest_sample_by_height(samples: &[Point2D], target_height: f64) -> usize {
    if samples.is_empty() {
        return 0;
    }

    if samples.len() == 1 {
        return 0;
    }

    // Binary search for insertion point
    let mut left = 0;
    let mut right = samples.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if samples[mid].y < target_height {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    // Check neighbors to find closest
    if left == 0 {
        return 0;
    }
    if left >= samples.len() {
        return samples.len() - 1;
    }

    let dist_left = (samples[left - 1].y - target_height).abs();
    let dist_right = (samples[left].y - target_height).abs();

    if dist_left < dist_right {
        left - 1
    } else {
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crochet_types::YarnSpec;

    #[test]
    fn test_uniform_mapping() {
        let samples: Vec<Point2D> = (0..11)
            .map(|i| Point2D::new(5.0, i as f64))
            .collect();

        let config = AmigurumiConfig {
            total_height_cm: 10.0,
            start_diameter_cm: 10.0,
            end_diameter_cm: 10.0,
            yarn: YarnSpec {
                gauge_stitches_per_cm: 3.0,
                gauge_rows_per_cm: 3.0,
                recommended_hook_size_mm: 3.5,
            },
        };

        let mapping = map_samples_to_rows(&samples, &config);

        // Should have 30 rows (10 cm * 3 rows/cm)
        assert_eq!(mapping.len(), 30);
    }

    #[test]
    fn test_find_nearest_sample() {
        let samples: Vec<Point2D> = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(0.0, 1.0),
            Point2D::new(0.0, 2.0),
            Point2D::new(0.0, 3.0),
            Point2D::new(0.0, 4.0),
        ];

        assert_eq!(find_nearest_sample_by_height(&samples, -0.5), 0);
        assert_eq!(find_nearest_sample_by_height(&samples, 0.0), 0);
        assert_eq!(find_nearest_sample_by_height(&samples, 0.4), 0);
        assert_eq!(find_nearest_sample_by_height(&samples, 0.6), 1);
        assert_eq!(find_nearest_sample_by_height(&samples, 2.5), 2);
        assert_eq!(find_nearest_sample_by_height(&samples, 4.5), 4);
    }

    #[test]
    fn test_single_sample() {
        let samples = vec![Point2D::new(5.0, 10.0)];

        let config = AmigurumiConfig {
            total_height_cm: 10.0,
            start_diameter_cm: 10.0,
            end_diameter_cm: 10.0,
            yarn: YarnSpec {
                gauge_stitches_per_cm: 3.0,
                gauge_rows_per_cm: 3.0,
                recommended_hook_size_mm: 3.5,
            },
        };

        let mapping = map_samples_to_rows(&samples, &config);
        assert!(mapping.len() > 0);
        
        // All should map to index 0
        for &idx in &mapping {
            assert_eq!(idx, 0);
        }
    }
}
