use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
use crate::{CrochetConfig, ProcessingResult, utils};
use crate::loader::gltf_parser::GltfLoader;
use crate::mesh::processing::MeshProcessor;
use crate::parameterization::lscm::LSCMParameterizer;
use crate::stitch::grid_generator::StitchGridGenerator;
use crate::pattern::optimizer::PatternOptimizer;
use crate::instruction::generator::InstructionGenerator;

/// Main entry point: Load and validate a GLTF/GLB file
#[wasm_bindgen]
pub async fn load_model(data: &[u8]) -> Result<JsValue, JsValue> {
    utils::log("Loading GLTF/GLB model...");
    
    let loader = GltfLoader::new();
    match loader.load_from_bytes(data) {
        Ok(mesh_data) => {
            utils::log(&format!("Model loaded: {} vertices, {} faces", 
                mesh_data.vertices.len(), 
                mesh_data.faces.len()
            ));
            
            let result = serde_json::json!({
                "success": true,
                "vertices": mesh_data.vertices.len(),
                "faces": mesh_data.faces.len(),
                "bounds": {
                    "min": mesh_data.bounds.min,
                    "max": mesh_data.bounds.max,
                }
            });
            
            to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Err(e) => {
            utils::log_error(&format!("Failed to load model: {}", e));
            Err(JsValue::from_str(&e.to_string()))
        }
    }
}

/// Process mesh and generate crochet pattern
#[wasm_bindgen]
pub async fn generate_pattern(
    model_data: &[u8],
    config_js: JsValue,
) -> Result<JsValue, JsValue> {
    utils::log("Starting pattern generation...");
    
    // Parse configuration
    let config: CrochetConfig = from_value(config_js)
        .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;
    
    // Load model
    let loader = GltfLoader::new();
    let mut mesh = loader.load_from_bytes(model_data)
        .map_err(|e| JsValue::from_str(&format!("Failed to load model: {}", e)))?;
    
    utils::log("Model loaded, processing mesh...");
    
    // Process mesh (simplification, validation, analysis)
    let processor = MeshProcessor::new();
    processor.process(&mut mesh, &config)
        .map_err(|e| JsValue::from_str(&format!("Mesh processing failed: {}", e)))?;
    
    utils::log("Mesh processed, computing parameterization...");
    
    // Parameterize surface (UV mapping)
    let parameterizer = LSCMParameterizer::new();
    let uv_coords = parameterizer.parameterize(&mesh)
        .map_err(|e| JsValue::from_str(&format!("Parameterization failed: {}", e)))?;
    
    utils::log("Parameterization complete, generating stitch grid...");
    
    // Generate stitch grid
    let stitch_generator = StitchGridGenerator::new(config.clone());
    let stitch_grid = stitch_generator.generate(&mesh, &uv_coords)
        .map_err(|e| JsValue::from_str(&format!("Stitch generation failed: {}", e)))?;
    
    utils::log("Stitch grid generated, optimizing pattern...");
    
    // Optimize pattern (row grouping, construction order)
    let optimizer = PatternOptimizer::new(config.clone());
    let pattern = optimizer.optimize(stitch_grid)
        .map_err(|e| JsValue::from_str(&format!("Pattern optimization failed: {}", e)))?;
    
    utils::log("Pattern optimized, generating instructions...");
    
    // Generate human-readable instructions
    let instruction_gen = InstructionGenerator::new();
    let final_pattern = instruction_gen.generate_instructions(pattern)
        .map_err(|e| JsValue::from_str(&format!("Instruction generation failed: {}", e)))?;
    
    utils::log(&format!(
        "Pattern complete: {} stitches, {} rows",
        final_pattern.metadata.stitch_count,
        final_pattern.metadata.row_count
    ));
    
    let result = ProcessingResult {
        success: true,
        pattern: Some(final_pattern),
        error: None,
        warnings: vec![],
    };
    
    to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get mesh statistics for display
#[wasm_bindgen]
pub fn get_mesh_info(data: &[u8]) -> Result<JsValue, JsValue> {
    let loader = GltfLoader::new();
    match loader.load_from_bytes(data) {
        Ok(mesh_data) => {
            let info = serde_json::json!({
                "vertices": mesh_data.vertices.len(),
                "faces": mesh_data.faces.len(),
                "bounds": {
                    "min": mesh_data.bounds.min,
                    "max": mesh_data.bounds.max,
                    "size": mesh_data.bounds.size(),
                }
            });
            to_value(&info).map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Err(e) => Err(JsValue::from_str(&e.to_string()))
    }
}

/// Validate GLTF/GLB file
#[wasm_bindgen]
pub fn validate_model(data: &[u8]) -> Result<JsValue, JsValue> {
    use crate::loader::validation::ModelValidator;
    
    let loader = GltfLoader::new();
    let validator = ModelValidator::new();
    
    match loader.load_from_bytes(data) {
        Ok(mesh_data) => {
            match validator.validate(&mesh_data) {
                Ok(warnings) => {
                    let result = serde_json::json!({
                        "valid": true,
                        "warnings": warnings,
                    });
                    to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
                }
                Err(e) => {
                    let result = serde_json::json!({
                        "valid": false,
                        "error": e.to_string(),
                    });
                    to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
                }
            }
        }
        Err(e) => {
            let result = serde_json::json!({
                "valid": false,
                "error": e.to_string(),
            });
            to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}

/// Export pattern to various formats
#[wasm_bindgen]
pub fn export_pattern(pattern_js: JsValue, format: &str) -> Result<JsValue, JsValue> {
    use crate::pattern::types::CrochetPattern;
    use crate::instruction::formatter::PatternFormatter;
    
    let pattern: CrochetPattern = from_value(pattern_js)
        .map_err(|e| JsValue::from_str(&format!("Invalid pattern: {}", e)))?;
    
    let formatter = PatternFormatter::new();
    
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&pattern)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(JsValue::from_str(&json))
        }
        "svg" => {
            let svg = formatter.to_svg(&pattern)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(JsValue::from_str(&svg))
        }
        "text" => {
            let text = formatter.to_text(&pattern)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(JsValue::from_str(&text))
        }
        _ => Err(JsValue::from_str(&format!("Unknown format: {}", format)))
    }
}
