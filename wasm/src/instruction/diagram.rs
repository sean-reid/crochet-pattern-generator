use anyhow::Result;
use crate::pattern::types::CrochetPattern;

pub struct DiagramGenerator {
    _private: (),
}

impl DiagramGenerator {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn generate(&self, pattern: &CrochetPattern) -> Result<String> {
        let width = 800;
        let height = 600;

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height
        );

        // Draw stitches
        let stitch_radius = 3.0;
        for stitch in &pattern.stitches {
            let x = stitch.position_2d[0] * width as f32;
            let y = stitch.position_2d[1] * height as f32;

            let color = match stitch.stitch_type {
                crate::stitch::StitchType::SingleCrochet => "#4A90E2",
                crate::stitch::StitchType::HalfDoubleCrochet => "#7ED321",
                crate::stitch::StitchType::DoubleCrochet => "#F5A623",
                crate::stitch::StitchType::Increase => "#E94B3C",
                crate::stitch::StitchType::Decrease => "#9013FE",
                crate::stitch::StitchType::ChainStitch => "#8B572A",
            };

            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="{}" />"#,
                x, y, stitch_radius, color
            ));
        }

        svg.push_str("</svg>");

        Ok(svg)
    }
}

impl Default for DiagramGenerator {
    fn default() -> Self {
        Self::new()
    }
}
