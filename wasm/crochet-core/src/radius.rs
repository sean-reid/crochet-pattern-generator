use crochet_types::Point2D;

/// Apply Gaussian smoothing to radius values
fn gaussian_smooth(values: &[f64], sigma: f64) -> Vec<f64> {
    if values.len() <= 2 {
        return values.to_vec();
    }

    let kernel_size = (6.0 * sigma).ceil() as usize;
    let kernel_size = kernel_size.max(1);
    let mut kernel = Vec::with_capacity(kernel_size * 2 + 1);

    // Generate Gaussian kernel
    let mut sum = 0.0;
    for i in -(kernel_size as i32)..=(kernel_size as i32) {
        let x = i as f64;
        let value = (-x * x / (2.0 * sigma * sigma)).exp();
        kernel.push(value);
        sum += value;
    }

    // Normalize kernel
    for value in kernel.iter_mut() {
        *value /= sum;
    }

    let mut smoothed = Vec::with_capacity(values.len());

    for i in 0..values.len() {
        let mut weighted_sum = 0.0;
        let mut weight_sum = 0.0;

        for (k, &kernel_value) in kernel.iter().enumerate() {
            let offset = k as i32 - kernel_size as i32;
            let idx = (i as i32 + offset).max(0).min(values.len() as i32 - 1) as usize;
            weighted_sum += values[idx] * kernel_value;
            weight_sum += kernel_value;
        }

        smoothed.push(weighted_sum / weight_sum);
    }

    smoothed
}

/// Calculate radius profile from sampled points
pub fn calculate_radius_profile(samples: &[Point2D]) -> Vec<f64> {
    if samples.is_empty() {
        return vec![];
    }

    // Extract radius (x-coordinate) from each point
    let radii: Vec<f64> = samples.iter().map(|p| p.x.max(0.0)).collect();

    // Apply Gaussian smoothing
    let spacing = if samples.len() > 1 {
        (samples.last().unwrap().y - samples[0].y) / (samples.len() - 1) as f64
    } else {
        1.0
    };
    let sigma = 0.5 * spacing;

    gaussian_smooth(&radii, sigma)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_constant_radius() {
        let samples: Vec<Point2D> = (0..10)
            .map(|i| Point2D::new(5.0, i as f64))
            .collect();

        let radii = calculate_radius_profile(&samples);
        assert_eq!(radii.len(), 10);

        for &r in &radii {
            assert_relative_eq!(r, 5.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_linear_radius() {
        let samples: Vec<Point2D> = (0..10)
            .map(|i| Point2D::new(i as f64, i as f64))
            .collect();

        let radii = calculate_radius_profile(&samples);
        assert_eq!(radii.len(), 10);

        // Should be close to linear after smoothing
        for (i, &r) in radii.iter().enumerate() {
            assert!((r - i as f64).abs() < 0.5);
        }
    }

    #[test]
    fn test_smoothing_reduces_noise() {
        let samples: Vec<Point2D> = vec![
            Point2D::new(5.0, 0.0),
            Point2D::new(5.0, 1.0),
            Point2D::new(8.0, 2.0), // Spike
            Point2D::new(5.0, 3.0),
            Point2D::new(5.0, 4.0),
        ];

        let radii = calculate_radius_profile(&samples);

        // Middle value should be smoothed down from 8.0
        assert!(radii[2] < 7.0);
        assert!(radii[2] > 5.5);
    }

    #[test]
    fn test_negative_radii_clamped() {
        let samples: Vec<Point2D> = vec![
            Point2D::new(-1.0, 0.0),
            Point2D::new(-2.0, 1.0),
            Point2D::new(-3.0, 2.0),
        ];

        let radii = calculate_radius_profile(&samples);

        for &r in &radii {
            assert!(r >= 0.0);
        }
    }
}
