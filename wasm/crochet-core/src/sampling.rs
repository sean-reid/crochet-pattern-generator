use crochet_types::{Point2D, ProfileCurve, SplineSegment};

/// Calculate arc length of a spline segment using adaptive Simpson integration
fn segment_arc_length(segment: &SplineSegment, tolerance: f64) -> f64 {
    fn integrand(segment: &SplineSegment, t: f64) -> f64 {
        let deriv = segment.derivative(t);
        (deriv.x * deriv.x + deriv.y * deriv.y).sqrt()
    }

    fn simpson_adaptive(
        segment: &SplineSegment,
        a: f64,
        b: f64,
        epsilon: f64,
        whole: f64,
    ) -> f64 {
        let c = (a + b) / 2.0;
        let left = (integrand(segment, a) + 4.0 * integrand(segment, (a + c) / 2.0)
            + integrand(segment, c))
            * (c - a)
            / 6.0;
        let right = (integrand(segment, c) + 4.0 * integrand(segment, (c + b) / 2.0)
            + integrand(segment, b))
            * (b - c)
            / 6.0;
        let sum = left + right;

        if (sum - whole).abs() <= 15.0 * epsilon {
            sum
        } else {
            simpson_adaptive(segment, a, c, epsilon / 2.0, left)
                + simpson_adaptive(segment, c, b, epsilon / 2.0, right)
        }
    }

    let whole = (integrand(segment, 0.0) + 4.0 * integrand(segment, 0.5)
        + integrand(segment, 1.0))
        * 1.0
        / 6.0;
    simpson_adaptive(segment, 0.0, 1.0, tolerance, whole)
}

/// Find parameter t on segment for a given arc length from start using Newton-Raphson
fn find_t_for_arc_length(segment: &SplineSegment, target_length: f64, tolerance: f64) -> f64 {
    let mut t = target_length / segment_arc_length(segment, tolerance * 0.1); // Initial guess
    t = t.max(0.0).min(1.0);

    for _ in 0..20 {
        // Calculate current arc length from 0 to t
        let current_segment = SplineSegment {
            start: segment.start,
            control1: segment.control1,
            control2: segment.control2,
            end: segment.evaluate(t),
        };
        let current_length = segment_arc_length(&current_segment, tolerance * 0.1);

        let error = current_length - target_length;
        if error.abs() < tolerance {
            break;
        }

        // Derivative of arc length with respect to t (speed at t)
        let deriv = segment.derivative(t);
        let speed = (deriv.x * deriv.x + deriv.y * deriv.y).sqrt();

        if speed < 1e-10 {
            break;
        }

        t -= error / speed;
        t = t.max(0.0).min(1.0);
    }

    t
}

/// Sample profile curve uniformly along arc length
pub fn sample_profile_curve(curve: &ProfileCurve, num_samples: usize) -> Vec<Point2D> {
    if curve.segments.is_empty() {
        return vec![];
    }

    if num_samples == 0 {
        return vec![];
    }

    if num_samples == 1 {
        return vec![curve.segments[0].start];
    }

    let tolerance = 1e-6;

    // Calculate total arc length and segment lengths
    let segment_lengths: Vec<f64> = curve
        .segments
        .iter()
        .map(|seg| segment_arc_length(seg, tolerance))
        .collect();
    let total_length: f64 = segment_lengths.iter().sum();

    if total_length < 1e-10 {
        return vec![curve.segments[0].start];
    }

    let mut samples = Vec::with_capacity(num_samples);
    let spacing = total_length / (num_samples - 1) as f64;

    // Always include first point
    samples.push(curve.segments[0].start);

    for i in 1..num_samples - 1 {
        let target_arc_length = i as f64 * spacing;

        // Find which segment contains this arc length
        let mut accumulated_length = 0.0;
        let mut segment_idx = 0;

        for (idx, &length) in segment_lengths.iter().enumerate() {
            if accumulated_length + length >= target_arc_length {
                segment_idx = idx;
                break;
            }
            accumulated_length += length;
        }

        // Find t within the segment
        let remaining_length = target_arc_length - accumulated_length;
        let t = find_t_for_arc_length(&curve.segments[segment_idx], remaining_length, tolerance);
        let point = curve.segments[segment_idx].evaluate(t);
        samples.push(point);
    }

    // Always include last point
    samples.push(curve.segments.last().unwrap().end);

    samples
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_straight_line_segment() {
        let segment = SplineSegment {
            start: Point2D::new(0.0, 0.0),
            control1: Point2D::new(1.0, 1.0),
            control2: Point2D::new(2.0, 2.0),
            end: Point2D::new(3.0, 3.0),
        };

        let length = segment_arc_length(&segment, 1e-6);
        let expected = (3.0f64 * 3.0 + 3.0 * 3.0).sqrt();
        assert_relative_eq!(length, expected, epsilon = 1e-4);
    }

    #[test]
    fn test_sample_straight_line() {
        let curve = ProfileCurve {
            segments: vec![SplineSegment {
                start: Point2D::new(0.0, 0.0),
                control1: Point2D::new(0.0, 3.33),
                control2: Point2D::new(0.0, 6.67),
                end: Point2D::new(0.0, 10.0),
            }],
            start_radius: 0.0,
            end_radius: 0.0,
        };

        let samples = sample_profile_curve(&curve, 11);
        assert_eq!(samples.len(), 11);
        
        // Check uniform spacing
        for i in 0..samples.len() {
            assert_relative_eq!(samples[i].y, i as f64, epsilon = 1e-3);
        }
    }

    #[test]
    fn test_sample_includes_endpoints() {
        let curve = ProfileCurve {
            segments: vec![SplineSegment {
                start: Point2D::new(1.0, 0.0),
                control1: Point2D::new(2.0, 3.0),
                control2: Point2D::new(3.0, 7.0),
                end: Point2D::new(4.0, 10.0),
            }],
            start_radius: 1.0,
            end_radius: 4.0,
        };

        let samples = sample_profile_curve(&curve, 5);
        assert_eq!(samples.len(), 5);
        assert_relative_eq!(samples[0].x, 1.0, epsilon = 1e-6);
        assert_relative_eq!(samples[0].y, 0.0, epsilon = 1e-6);
        assert_relative_eq!(samples[4].x, 4.0, epsilon = 1e-6);
        assert_relative_eq!(samples[4].y, 10.0, epsilon = 1e-6);
    }
}
