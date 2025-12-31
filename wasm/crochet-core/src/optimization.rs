use crochet_types::{Row, StitchInstruction, StitchType};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use std::f64::consts::PI;

/// Optimize stitch placement using simulated annealing
/// 
/// In crochet, stitches must be worked sequentially around the circle.
/// This optimization adjusts WHERE special stitches (INC/DEC) are placed
/// in the sequence while maintaining the circular order.
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

        // Extract indices of special stitches in the sequence
        let special_indices: Vec<usize> = row
            .pattern
            .iter()
            .enumerate()
            .filter(|(_, s)| s.stitch_type != StitchType::SC)
            .map(|(i, _)| i)
            .collect();

        // Get previous row's special stitch positions for staggering
        let prev_special_indices: Vec<usize> = if row_idx > 0 {
            let prev_row = &optimized[row_idx - 1];
            prev_row
                .pattern
                .iter()
                .enumerate()
                .filter(|(_, s)| s.stitch_type != StitchType::SC)
                .map(|(i, _)| i)
                .collect()
        } else {
            vec![]
        };

        // Run simulated annealing to find optimal placement
        let optimized_indices = optimize_special_stitch_indices(
            &special_indices,
            &prev_special_indices,
            row.pattern.len(),
            &mut rng,
        );

        // Create new pattern with optimized positions
        let mut new_pattern = vec![StitchType::SC; row.pattern.len()];
        
        // Place special stitches at optimized positions
        let mut special_idx = 0;
        for &pos in &optimized_indices {
            new_pattern[pos] = row.pattern[special_indices[special_idx]].stitch_type;
            special_idx += 1;
        }

        // Convert to StitchInstruction vec
        let pattern_vec: Vec<StitchInstruction> = new_pattern
            .iter()
            .enumerate()
            .map(|(i, &stitch_type)| {
                let angle = 2.0 * PI * i as f64 / new_pattern.len() as f64;
                StitchInstruction {
                    stitch_type,
                    angular_position: angle,
                    stitch_index: i,
                }
            })
            .collect();

        optimized.push(Row {
            row_number: row.row_number,
            total_stitches: row.total_stitches,
            pattern: pattern_vec,
        });
    }

    optimized
}

/// Optimize the placement of special stitches within a sequential pattern
fn optimize_special_stitch_indices(
    special_indices: &[usize],
    prev_special_indices: &[usize],
    pattern_length: usize,
    rng: &mut ChaCha8Rng,
) -> Vec<usize> {
    if special_indices.is_empty() {
        return vec![];
    }

    let n = special_indices.len();
    
    // Start with evenly spaced positions
    let spacing = pattern_length as f64 / n as f64;
    let mut current: Vec<usize> = (0..n)
        .map(|i| (i as f64 * spacing).round() as usize % pattern_length)
        .collect();
    
    // If we have a previous row, offset by half spacing for staggering
    if !prev_special_indices.is_empty() && n > 0 {
        let offset = (spacing / 2.0).round() as usize;
        current = current.iter().map(|&pos| (pos + offset) % pattern_length).collect();
    }
    
    let mut best = current.clone();
    let mut best_energy = index_energy(&best, prev_special_indices, pattern_length);

    let mut temperature = 1.0;
    let cooling_rate = 0.95;
    let iterations = 500;

    for _ in 0..iterations {
        // Perturb: swap two positions or shift one
        let mut candidate = current.clone();
        
        if rng.gen_bool(0.5) && n > 1 {
            // Swap two positions
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..n);
            candidate.swap(i, j);
        } else {
            // Shift one position
            let i = rng.gen_range(0..n);
            let delta = rng.gen_range(-3..=3);
            candidate[i] = ((candidate[i] as i32 + delta).rem_euclid(pattern_length as i32)) as usize;
        }

        // Ensure no duplicates
        candidate.sort_unstable();
        candidate.dedup();
        if candidate.len() != n {
            continue; // Skip if we lost positions due to collision
        }

        let current_energy = index_energy(&current, prev_special_indices, pattern_length);
        let candidate_energy = index_energy(&candidate, prev_special_indices, pattern_length);

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

/// Energy function for index-based optimization
/// Lower energy = better distribution
fn index_energy(indices: &[usize], prev_indices: &[usize], pattern_length: usize) -> f64 {
    let n = indices.len();
    if n <= 1 {
        return 0.0;
    }

    let mut e = 0.0;

    // Repulsion term: prefer even spacing within this row
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = circular_distance(indices[i], indices[j], pattern_length);
            // Penalize clustering - stronger penalty for closer spacing
            e -= (dist as f64 + 1.0).ln();
        }
    }

    // Staggering term: offset from previous row (stronger weight)
    if !prev_indices.is_empty() {
        let lambda = 1.0; // Increased from 0.5 for stronger staggering
        for &idx in indices {
            let mut min_dist = pattern_length;
            for &prev_idx in prev_indices {
                let dist = circular_distance(idx, prev_idx, pattern_length);
                min_dist = min_dist.min(dist);
            }
            // Penalty if too close to previous row's stitches
            if min_dist < pattern_length / (indices.len() * 2) {
                // Strong penalty if within "too close" range
                e += lambda * 10.0 * (-(min_dist as f64)).exp();
            } else {
                e += lambda * (-(min_dist as f64 / 2.0)).exp();
            }
        }
    }

    e
}

/// Calculate circular distance between two indices
fn circular_distance(a: usize, b: usize, length: usize) -> usize {
    let diff = if a > b { a - b } else { b - a };
    diff.min(length - diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_row(row_number: usize, total_stitches: usize, inc_count: usize) -> Row {
        let mut pattern = Vec::new();

        if inc_count == 0 {
            // All SC
            for i in 0..total_stitches {
                let angle = 2.0 * PI * i as f64 / total_stitches as f64;
                pattern.push(StitchInstruction {
                    stitch_type: StitchType::SC,
                    angular_position: angle,
                    stitch_index: i,
                });
            }
        } else {
            // Simple uniform distribution
            let inc_spacing = total_stitches / inc_count;

            for i in 0..total_stitches {
                let stitch_type = if i % inc_spacing == 0 && pattern.iter().filter(|s| s.stitch_type == StitchType::INC).count() < inc_count {
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
        // Evenly spaced indices should have lower energy
        let even = vec![0, 5, 10, 15, 20, 25];
        let clustered = vec![0, 1, 2, 15, 16, 17];

        let e_even = index_energy(&even, &[], 30);
        let e_clustered = index_energy(&clustered, &[], 30);

        assert!(e_even < e_clustered);
    }
}
