use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
use crate::{CrochetConfig, ProcessingResult, utils};
use crate::loader::gltf_parser::GltfLoader;
use crate::mesh::processing::MeshProcessor;
use crate::mesh::analysis::MeshAnalyzer;
use crate::mesh::types::{MeshData, Vertex, Face}; // Added types for cutting logic
use crate::parameterization::lscm::LSCMParameterizer;
use crate::parameterization::seam_placement::SeamPlacer; // Added for cutting
use crate::stitch::grid_generator::StitchGridGenerator;
use crate::stitch::type_classifier::StitchTypeClassifier;
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
            let error_msg = format!("Failed to load model: {}", e);
            utils::log_error(&error_msg);
            
            let result = serde_json::json!({
                "success": false,
                "error": error_msg
            });
            
            to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
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
        .map_err(|e| {
            let error_msg = format!("Invalid config: {}", e);
            utils::log_error(&error_msg);
            JsValue::from_str(&error_msg)
        })?;
    
    // Load model
    let loader = GltfLoader::new();
    let mut mesh = match loader.load_from_bytes(model_data) {
        Ok(m) => m,
        Err(e) => {
            let error_msg = format!("Failed to load model: {}. Ensure the file is a valid GLB (binary GLTF) file.", e);
            utils::log_error(&error_msg);
            
            let result = ProcessingResult {
                success: false,
                pattern: None,
                error: Some(error_msg),
                warnings: vec![],
            };
            
            return to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()));
        }
    };
    
    utils::log("Model loaded, processing mesh...");
    
    // 1. Process mesh (simplification, validation, normalization)
    let processor = MeshProcessor::new();
    if let Err(e) = processor.process(&mut mesh, &config) {
        let error_msg = format!("Mesh processing failed: {}", e);
        utils::log_error(&error_msg);
        
        let result = ProcessingResult {
            success: false,
            pattern: None,
            error: Some(error_msg),
            warnings: vec![],
        };
        
        return to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()));
    }

    // 2. CRITICAL FIX: Cut Mesh for Flattening
    // We must find a seam to "open" closed surfaces like spheres or cubes so they can be flattened correctly.
    utils::log("Placing seams for surface flattening...");
    let seam_placer = SeamPlacer::new();
    if let Ok(seam) = seam_placer.place_seam(&mesh) {
        mesh = apply_topological_cut(&mesh, &seam);
    }

    // 3. Analyze mesh curvature
    utils::log("Analyzing surface curvature...");
    let analyzer = MeshAnalyzer::new();
    if let Err(e) = analyzer.compute_curvature(&mut mesh) {
        utils::log_warn(&format!("Curvature analysis partial: {}", e));
    }
    
    utils::log("Computing surface parameterization (UV mapping)...");
    
    // 4. Parameterize surface (UV mapping)
    let parameterizer = LSCMParameterizer::new();
    let uv_coords = match parameterizer.parameterize(&mesh) {
        Ok(coords) => coords,
        Err(e) => {
            let error_msg = format!("Parameterization failed: {}. Try simplifying the mesh.", e);
            utils::log_error(&error_msg);
            
            let result = ProcessingResult {
                success: false,
                pattern: None,
                error: Some(error_msg),
                warnings: vec![],
            };
            
            return to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()));
        }
    };
    
    utils::log("Generating stitch grid...");
    
    // 5. Generate initial stitch grid
    let stitch_generator = StitchGridGenerator::new(config.clone());
    let mut stitch_grid = match stitch_generator.generate(&mesh, &uv_coords) {
        Ok(grid) => grid,
        Err(e) => {
            let error_msg = format!("Stitch generation failed: {}", e);
            utils::log_error(&error_msg);
            
            let result = ProcessingResult {
                success: false,
                pattern: None,
                error: Some(error_msg),
                warnings: vec![],
            };
            
            return to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()));
        }
    };

    // 6. Classify stitch types (inc/dec)
    utils::log("Classifying stitch types...");
    let classifier = StitchTypeClassifier::new();
    classifier.classify(&mut stitch_grid, &mesh);
    
    utils::log("Optimizing pattern instructions...");
    
    // 7. Optimize pattern
    let optimizer = PatternOptimizer::new(config.clone());
    let pattern = match optimizer.optimize(stitch_grid) {
        Ok(p) => p,
        Err(e) => {
            let error_msg = format!("Pattern optimization failed: {}", e);
            utils::log_error(&error_msg);
            
            let result = ProcessingResult {
                success: false,
                pattern: None,
                error: Some(error_msg),
                warnings: vec![],
            };
            
            return to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()));
        }
    };
    
    // 8. Generate final instructions
    let instruction_gen = InstructionGenerator::new();
    let final_pattern = match instruction_gen.generate_instructions(pattern) {
        Ok(p) => p,
        Err(e) => {
            let error_msg = format!("Instruction generation failed: {}", e);
            utils::log_error(&error_msg);
            
            let result = ProcessingResult {
                success: false,
                pattern: None,
                error: Some(error_msg),
                warnings: vec![],
            };
            
            return to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()));
        }
    };
    
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

/// Helper to duplicate vertices along a seam to "open" the mesh for parameterization
fn apply_topological_cut(mesh: &MeshData, seam: &[(u32, u32)]) -> MeshData {
    let mut new_vertices = mesh.vertices.clone();
    let mut new_faces = mesh.faces.clone();
    let mut vertex_map: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();

    for &(v0, v1) in seam {
        for &v_idx in &[v0, v1] {
            if !vertex_map.contains_key(&v_idx) {
                let new_idx = new_vertices.len() as u32;
                new_vertices.push(mesh.vertices[v_idx as usize].clone());
                vertex_map.insert(v_idx, new_idx);
            }
        }

        // Adjust faces to use duplicated vertices on one side of the cut
        for face in &mut new_faces {
            for idx in &mut face.indices {
                if *idx == v0 {
                    *idx = *vertex_map.get(&v0).unwrap();
                }
            }
        }
    }

    MeshData {
        vertices: new_vertices,
        faces: new_faces,
        bounds: mesh.bounds,
    }
}

/// Get mesh statistics for display
#[wasm_bindgen]
pub fn get_mesh_info(data: &[u8]) -> Result<JsValue, JsValue> {
    let loader = GltfLoader::new();
    match loader.load_from_bytes(data) {
        Ok(mesh_data) => {
            let info = serde_json::json!({
                "success": true,
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
        Err(e) => {
            let error_msg = format!("Failed to get mesh info: {}. Ensure the file is a valid GLB file.", e);
            utils::log_error(&error_msg);
            
            let info = serde_json::json!({
                "success": false,
                "error": error_msg
            });
            to_value(&info).map_err(|e| JsValue::from_str(&e.to_string()))
        }
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
            let error_msg = format!("Failed to load model for validation: {}. Ensure the file is a valid GLB file.", e);
            let result = serde_json::json!({
                "valid": false,
                "error": error_msg,
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
