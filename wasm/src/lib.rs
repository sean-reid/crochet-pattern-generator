use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

mod algorithms;
mod bindings;
mod instruction;
mod loader;
mod mesh;
mod parameterization;
mod pattern;
mod stitch;
mod utils;

pub use bindings::*;

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    utils::log("Crochet Pattern Generator WASM module initialized");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrochetConfig {
    pub stitches_per_inch: f32,
    pub rows_per_inch: f32,
    pub yarn_weight: String,
    pub hook_size_mm: f32,
    pub target_width_inches: Option<f32>,
    pub target_height_inches: Option<f32>,
    pub construction_type: String,
    pub max_distortion: f32,
    pub simplify_mesh: bool,
    pub target_stitch_count: Option<u32>,
}

impl Default for CrochetConfig {
    fn default() -> Self {
        Self {
            stitches_per_inch: 5.0,
            rows_per_inch: 5.5,
            yarn_weight: "worsted".to_string(),
            hook_size_mm: 5.0,
            target_width_inches: None,
            target_height_inches: None,
            construction_type: "flat".to_string(),
            max_distortion: 0.3,
            simplify_mesh: true,
            target_stitch_count: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub success: bool,
    pub pattern: Option<pattern::types::CrochetPattern>,
    pub error: Option<String>,
    pub warnings: Vec<String>,
}
