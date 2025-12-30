use crochet_types::{Row, StitchInstruction, StitchType};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use std::f64::consts::PI;

/// Optimize stitch placement using simulated annealing
pub fn optimize_stitch_placement(rows: &[Row]) -> Vec<Row> {
    let mut optimized = Vec::with_capacity(rows.len());
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    for (row_idx, row) in rows.iter().enumerate() {
        // Count special stitches
        let special_count = row
            .pattern
            .iter()
            .filter(|s| s.stitch_type != StitchType::SC)
            .count();

        if special_count == 0 {
            // No optimization needed
            optimized.push(row.clone());
            continue;
        }

        // Extract positions of special stitches as angles
        let mut angles: Vec<f64> = row
            .pattern
            .iter()
            .filter(|s| s.stitch_type != StitchType::SC)
            .map(|s| s.angular_position)
            .collect();

        // Get previous row's special stitch positions for staggering
        let prev_angles: Vec<f64> = if row_idx > 0 {
            optimized[row_idx - 1]
                .pattern
                .iter()
                .filter(|s| s.stitch_type != StitchType::SC)
                .map(|s| s.angular_position)
                .collect()
        } else {
            vec![]
        };

        // Run simulated annealing
        angles = simulated_annealing(&mut angles, &prev_angles, &mut rng);

        // Create new pattern with optimized positions
        let mut new_pattern = Vec::with_capacity(row.total_stitches);
        let mut special_idx = 0;

        for stitch in &row.pattern {
            if stitch.stitch_type == StitchType::SC {
                new_pattern.push(stitch.clone());
            } else {
                let optimized_angle = angles[special_idx];
                let optimized_index =
                    ((optimized_angle / (2.0 * PI)) * row.total_stitches as f64).round() as usize;
                let optimized_index = optimized_index.min(row.total_stitches - 1);

                new_pattern.push(StitchInstruction {
                    stitch_type: stitch.stitch_type,
                    angular_position: optimized_angle,
                    stitch_index: optimized_index,
                });

                special_idx += 1;
            }
        }

        // Sort by angular position
        new_pattern.sort_by(|a, b| a.angular_position.partial_cmp(&b.angular_position).unwrap());

        // Update stitch indices
        for (idx, stitch) in new_pattern.iter_mut().enumerate() {
            stitch.stitch_index = idx;
        }

        optimized.push(Row {
            row_number: row.row_number,
            total_stitches: row.total_stitches,
            pattern: new_pattern,
        });
    }

    optimized
}

/// Simulated annealing optimization
fn simulated_annealing(
    angles: &mut Vec<f64>,
    prev_angles: &[f64],
    rng: &mut ChaCha8Rng,
) -> Vec<f64> {
    if angles.is_empty() {
        return vec![];
    }

    let n = angles.len();
    let mut current = angles.clone();
    let mut best = current.clone();
    let mut best_energy = energy(&best, prev_angles);

    let mut temperature = 1.0;
    let cooling_rate = 0.95;
    let iterations = 1000;

    for _ in 0..iterations {
        // Perturb 1-2 angles
        let mut candidate = current.clone();
        let num_perturb = if rng.gen_bool(0.5) { 1 } else { 2 };

        for _ in 0..num_perturb.min(n) {
            let idx = rng.gen_range(0..n);
            let delta = rng.gen_range(-0.1..0.1);
            candidate[idx] = (candidate[idx] + delta).rem_euclid(2.0 * PI);
        }

        let current_energy = energy(&current, prev_angles);
        let candidate_energy = energy(&candidate, prev_angles);

        // Accept or reject
        let delta_e = candidate_energy - current_energy;
        if delta_e < 0.0 || rng.gen::<f64>() < (-delta_e / temperature).exp() {
            current = candidate;

            if candidate_energy < best_energy {
                best = current.clone();
                best_energy = candidate_energy;
            }
        }

        temperature *= cooling_rate;
    }

    best
}

/// Energy function: negative log likelihood of uniform distribution
fn energy(angles: &[f64], prev_angles: &[f64]) -> f64 {
    let n = angles.len();
    if n <= 1 {
        return 0.0;
    }

    let mut e = 0.0;

    // Repulsion term: minimize clustering
    for i in 0..n {
        for j in (i + 1)..n {
            let diff = (angles[j] - angles[i]).abs();
            let dist = diff.min(2.0 * PI - diff);
            // Add small epsilon to prevent log(0)
            e -= (dist.abs() + 1e-10).ln();
        }
    }

    // Staggering term: offset from previous row
    if !prev_angles.is_empty() {
        let lambda = 0.5; // Weight for staggering constraint
        for &angle in angles {
            let mut min_dist = 2.0 * PI;
            for &prev_angle in prev_angles {
                let diff = (angle - prev_angle).abs();
                let dist = diff.min(2.0 * PI - diff);
                min_dist = min_dist.min(dist);
            }
            // Penalty if too close to previous row's stitches
            e += lambda * (-min_dist).exp();
        }
    }

    e
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_row(row_number: usize, total_stitches: usize, inc_count: usize) -> Row {
        let mut pattern = Vec::new();

        // Simple uniform distribution
        let inc_spacing = total_stitches / inc_count;

        for i in 0..total_stitches {
            let stitch_type = if i % inc_spacing == 0 && pattern.len() < inc_count {
                StitchType::INC
            } else {
                StitchType::SC
            };

            let angle = 2.0 * PI * i as f64 / total_stitches as f64;

            pattern.push(StitchInstruction {
                stitch_type,
                angular_position: angle,
                stitch_index: i,
            });
        }

        Row {
            row_number,
            total_stitches,
            pattern,
        }
    }

    #[test]
    fn test_optimize_no_special_stitches() {
        let rows = vec![create_test_row(1, 12, 0)];
        let optimized = optimize_stitch_placement(&rows);

        assert_eq!(optimized.len(), 1);
        assert_eq!(optimized[0].pattern.len(), 12);
    }

    #[test]
    fn test_optimize_preserves_stitch_count() {
        let rows = vec![create_test_row(1, 18, 6)];
        let optimized = optimize_stitch_placement(&rows);

        assert_eq!(optimized.len(), 1);
        assert_eq!(optimized[0].total_stitches, 18);

        let inc_count = optimized[0]
            .pattern
            .iter()
            .filter(|s| s.stitch_type == StitchType::INC)
            .count();
        assert_eq!(inc_count, 6);
    }

    #[test]
    fn test_energy_function() {
        // Evenly spaced angles should have lower energy
        let even = vec![0.0, PI / 3.0, 2.0 * PI / 3.0, PI, 4.0 * PI / 3.0, 5.0 * PI / 3.0];
        let clustered = vec![0.0, 0.1, 0.2, PI, PI + 0.1, PI + 0.2];

        let e_even = energy(&even, &[]);
        let e_clustered = energy(&clustered, &[]);

        assert!(e_even < e_clustered);
    }
}
