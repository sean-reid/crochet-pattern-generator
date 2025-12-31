use wasm_bindgen::prelude::*;
use crochet_core::generator::generate_pattern;
use crochet_types::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Generate a crochet pattern from JSON input
#[wasm_bindgen]
pub fn generate_pattern_from_json(
    profile_json: &str,
    config_json: &str,
) -> std::result::Result<String, String> {
    // Parse inputs
    let profile: ProfileCurve = serde_json::from_str(profile_json)
        .map_err(|e| format!("Failed to parse profile: {}", e))?;

    let config: AmigurumiConfig = serde_json::from_str(config_json)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    // Generate pattern
    let pattern = generate_pattern(&profile, &config)
        .map_err(|e| e.to_string())?;

    // Serialize result
    serde_json::to_string(&pattern)
        .map_err(|e| format!("Failed to serialize pattern: {}", e))
}

/// Validate a profile curve
#[wasm_bindgen]
pub fn validate_profile(profile_json: &str) -> std::result::Result<String, String> {
    let profile: ProfileCurve = serde_json::from_str(profile_json)
        .map_err(|e| format!("Failed to parse profile: {}", e))?;

    if profile.segments.is_empty() {
        return Err("Profile has no segments".to_string());
    }

    // Check continuity
    for i in 1..profile.segments.len() {
        let prev_end = profile.segments[i - 1].end;
        let curr_start = profile.segments[i].start;
        let dist = prev_end.distance_to(&curr_start);
        
        if dist > 1e-6 {
            return Err(format!(
                "Discontinuity between segments {} and {}: distance = {}",
                i - 1, i, dist
            ));
        }
    }

    Ok("Profile is valid".to_string())
}

/// Validate a configuration
#[wasm_bindgen]
pub fn validate_config(config_json: &str) -> std::result::Result<String, String> {
    let config: AmigurumiConfig = serde_json::from_str(config_json)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    if config.total_height_cm <= 0.0 {
        return Err("Height must be positive".to_string());
    }

    if config.yarn.gauge_stitches_per_cm <= 0.0 {
        return Err("Gauge stitches per cm must be positive".to_string());
    }

    if config.yarn.gauge_rows_per_cm <= 0.0 {
        return Err("Gauge rows per cm must be positive".to_string());
    }

    Ok("Configuration is valid".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pattern_from_json() {
        let profile_json = r#"{
            "segments": [{
                "start": {"x": 2.0, "y": 0.0},
                "control1": {"x": 2.0, "y": 3.33},
                "control2": {"x": 2.0, "y": 6.67},
                "end": {"x": 2.0, "y": 10.0}
            }],
            "start_radius": 2.0,
            "end_radius": 2.0
        }"#;

        let config_json = r#"{
            "total_height_cm": 10.0,
            "yarn": {
                "gauge_stitches_per_cm": 3.0,
                "gauge_rows_per_cm": 3.0,
                "recommended_hook_size_mm": 3.5
            }
        }"#;

        let result = generate_pattern_from_json(profile_json, config_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_profile() {
        let valid_json = r#"{
            "segments": [{
                "start": {"x": 2.0, "y": 0.0},
                "control1": {"x": 2.0, "y": 3.0},
                "control2": {"x": 2.0, "y": 7.0},
                "end": {"x": 2.0, "y": 10.0}
            }],
            "start_radius": 2.0,
            "end_radius": 2.0
        }"#;

        let result = validate_profile(valid_json);
        assert!(result.is_ok());

        let invalid_json = r#"{
            "segments": [],
            "start_radius": 2.0,
            "end_radius": 2.0
        }"#;

        let result = validate_profile(invalid_json);
        assert!(result.is_err());
    }
}
