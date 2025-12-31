use serde::{Deserialize, Serialize};

/// 2D point in drawing space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64, // horizontal position (radius)
    pub y: f64, // vertical position (height)
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Cubic Bézier spline segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplineSegment {
    pub start: Point2D,
    pub control1: Point2D,
    pub control2: Point2D,
    pub end: Point2D,
}

impl SplineSegment {
    /// Evaluate Bézier curve at parameter t (0 to 1)
    pub fn evaluate(&self, t: f64) -> Point2D {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        Point2D {
            x: mt3 * self.start.x
                + 3.0 * mt2 * t * self.control1.x
                + 3.0 * mt * t2 * self.control2.x
                + t3 * self.end.x,
            y: mt3 * self.start.y
                + 3.0 * mt2 * t * self.control1.y
                + 3.0 * mt * t2 * self.control2.y
                + t3 * self.end.y,
        }
    }

    /// Evaluate first derivative at parameter t
    pub fn derivative(&self, t: f64) -> Point2D {
        let t2 = t * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;

        Point2D {
            x: 3.0 * mt2 * (self.control1.x - self.start.x)
                + 6.0 * mt * t * (self.control2.x - self.control1.x)
                + 3.0 * t2 * (self.end.x - self.control2.x),
            y: 3.0 * mt2 * (self.control1.y - self.start.y)
                + 6.0 * mt * t * (self.control2.y - self.control1.y)
                + 3.0 * t2 * (self.end.y - self.control2.y),
        }
    }
}

/// Complete user-drawn profile (one side only, will be rotated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileCurve {
    pub segments: Vec<SplineSegment>,
    pub start_radius: f64, // magic circle radius at bottom
    pub end_radius: f64,   // magic circle radius at top
}

/// Physical yarn specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YarnSpec {
    pub gauge_stitches_per_cm: f64, // horizontal stitch density
    pub gauge_rows_per_cm: f64,     // vertical row density
    pub recommended_hook_size_mm: f64,
}

/// Dimensions in real-world units
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmigurumiConfig {
    pub total_height_cm: f64,
    pub yarn: YarnSpec,
}

/// Stitch type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StitchType {
    SC,     // single crochet
    INC,    // increase
    DEC,    // decrease
    INVDEC, // invisible decrease
}

impl StitchType {
    pub fn to_string(&self) -> &'static str {
        match self {
            StitchType::SC => "SC",
            StitchType::INC => "INC",
            StitchType::DEC => "DEC",
            StitchType::INVDEC => "INVDEC",
        }
    }
}

/// Stitch instruction with position
/// 
/// Represents an instruction to work into a stitch from the previous row.
/// In crochet, you work sequentially around the circle, and each instruction
/// operates on one (or more, for decreases) stitches from the previous row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StitchInstruction {
    pub stitch_type: StitchType,
    /// Angular position in the previous row (radians from 0 to 2π)
    pub angular_position: f64,
    /// Index in the instruction sequence (0 to pattern.len()-1)
    /// This is the position in the previous row where we work
    pub stitch_index: usize,
}

/// Single row instruction
/// 
/// In crochet, each row is worked INTO the stitches of the previous row.
/// - `pattern` contains instructions to execute (one per stitch from previous row)
/// - `total_stitches` is the number of stitches created by executing those instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub row_number: usize,
    /// Number of stitches CREATED by this row
    pub total_stitches: usize,
    /// Instructions to execute (length = previous row's stitch count for rows > 1)
    pub pattern: Vec<StitchInstruction>,
}

impl Row {
    /// Convert pattern to human-readable string
    pub fn pattern_string(&self) -> String {
        if self.pattern.is_empty() {
            return format!("{} SC", self.total_stitches);
        }

        let mut result = String::new();
        let mut current_type = self.pattern[0].stitch_type;
        let mut count = 1;

        for i in 1..self.pattern.len() {
            if self.pattern[i].stitch_type == current_type {
                count += 1;
            } else {
                if count > 1 {
                    result.push_str(&format!("{} {}, ", count, current_type.to_string()));
                } else {
                    result.push_str(&format!("{}, ", current_type.to_string()));
                }
                current_type = self.pattern[i].stitch_type;
                count = 1;
            }
        }

        // Add final group
        if count > 1 {
            result.push_str(&format!("{} {}", count, current_type.to_string()));
        } else {
            result.push_str(current_type.to_string());
        }

        result
    }
}

/// Pattern metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMetadata {
    pub total_rows: usize,
    pub total_stitches: usize,
    pub estimated_time_minutes: f64,
    pub yarn_length_meters: f64,
}

/// Complete generated pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrochetPattern {
    pub rows: Vec<Row>,
    pub metadata: PatternMetadata,
}

/// Error types for pattern generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternError {
    InvalidProfileCurve(String),
    InvalidConfiguration(String),
    OptimizationFailure(String),
    InternalError(String),
}

impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternError::InvalidProfileCurve(msg) => write!(f, "Invalid profile curve: {}", msg),
            PatternError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            PatternError::OptimizationFailure(msg) => write!(f, "Optimization failed: {}", msg),
            PatternError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for PatternError {}

pub type Result<T> = std::result::Result<T, PatternError>;
