use anyhow::Result;
use crate::pattern::types::CrochetPattern;

pub struct PatternFormatter {
    _private: (),
}

impl PatternFormatter {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn to_text(&self, pattern: &CrochetPattern) -> Result<String> {
        let mut text = String::new();

        text.push_str("CROCHET PATTERN\n");
        text.push_str("===============\n\n");
        text.push_str(&format!("Total stitches: {}\n", pattern.metadata.stitch_count));
        text.push_str(&format!("Total rows: {}\n", pattern.metadata.row_count));
        text.push_str(&format!("Estimated time: {}\n", pattern.metadata.estimated_time));
        text.push_str(&format!("Yarn needed: {}\n\n", pattern.metadata.yarn_estimate));

        text.push_str("INSTRUCTIONS\n");
        text.push_str("============\n\n");

        for row in &pattern.instructions.rows {
            text.push_str(&format!("Row {}: ", row.number));
            
            let instructions: Vec<String> = row.stitches
                .iter()
                .map(|sg| sg.instruction.clone())
                .collect();
            
            text.push_str(&instructions.join(", "));
            text.push_str(&format!(" ({})\n", row.total_stitches));
        }

        Ok(text)
    }

    pub fn to_svg(&self, pattern: &CrochetPattern) -> Result<String> {
        pattern.diagram.clone().ok_or_else(|| anyhow::anyhow!("No diagram available"))
    }
}

impl Default for PatternFormatter {
    fn default() -> Self {
        Self::new()
    }
}
