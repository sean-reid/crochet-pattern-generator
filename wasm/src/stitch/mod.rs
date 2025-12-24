pub mod grid_generator;
pub mod type_classifier;
pub mod connectivity;
pub mod placement_optimizer;

use serde::{Deserialize, Serialize};

/// Stitch types supported
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StitchType {
    SingleCrochet,      // sc
    HalfDoubleCrochet,  // hdc
    DoubleCrochet,      // dc
    Increase,           // inc (2 sc in same stitch)
    Decrease,           // dec (sc2tog)
    ChainStitch,        // ch
}

impl StitchType {
    pub fn abbreviation(&self) -> &'static str {
        match self {
            StitchType::SingleCrochet => "sc",
            StitchType::HalfDoubleCrochet => "hdc",
            StitchType::DoubleCrochet => "dc",
            StitchType::Increase => "inc",
            StitchType::Decrease => "dec",
            StitchType::ChainStitch => "ch",
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            StitchType::SingleCrochet => 1.0,
            StitchType::HalfDoubleCrochet => 1.5,
            StitchType::DoubleCrochet => 2.0,
            StitchType::Increase => 1.0,
            StitchType::Decrease => 1.0,
            StitchType::ChainStitch => 0.5,
        }
    }
}

/// A single stitch in the pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stitch {
    pub id: u32,
    pub stitch_type: StitchType,
    pub position_3d: [f32; 3],
    pub position_2d: [f32; 2],
    pub row: u32,
    pub connections: Vec<u32>,
}

/// Grid of stitches covering the surface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StitchGrid {
    pub stitches: Vec<Stitch>,
    pub rows: Vec<Vec<u32>>,  // Stitch IDs grouped by row
}
