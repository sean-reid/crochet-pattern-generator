use anyhow::Result;
use crate::pattern::types::CrochetPattern;
use super::diagram::DiagramGenerator;

pub struct InstructionGenerator {
    _private: (),
}

impl InstructionGenerator {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn generate_instructions(&self, mut pattern: CrochetPattern) -> Result<CrochetPattern> {
        // Generate SVG diagram
        let diagram_gen = DiagramGenerator::new();
        pattern.diagram = Some(diagram_gen.generate(&pattern)?);

        Ok(pattern)
    }
}

impl Default for InstructionGenerator {
    fn default() -> Self {
        Self::new()
    }
}
