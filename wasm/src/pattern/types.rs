use serde::{Deserialize, Serialize};
use crate::stitch::{Stitch, StitchType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrochetPattern {
    pub metadata: PatternMetadata,
    pub stitches: Vec<Stitch>,
    pub instructions: PatternInstructions,
    pub diagram: Option<String>,  // SVG diagram
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMetadata {
    pub stitch_count: usize,
    pub row_count: usize,
    pub estimated_time: String,
    pub yarn_estimate: String,
    pub dimensions: Dimensions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternInstructions {
    pub rows: Vec<RowInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowInstruction {
    pub number: u32,
    pub stitches: Vec<StitchGroup>,
    pub total_stitches: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StitchGroup {
    pub count: usize,
    pub stitch_type: StitchType,
    pub instruction: String,
}
