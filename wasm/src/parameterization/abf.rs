use anyhow::Result;
use crate::mesh::types::MeshData;

pub struct ABFParameterizer {
    _private: (),
}

impl ABFParameterizer {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn parameterize(&self, _mesh: &MeshData) -> Result<Vec<[f32; 2]>> {
        // ABF++ implementation would go here
        // For now, return error suggesting LSCM
        anyhow::bail!("ABF++ parameterization not yet implemented. Use LSCM instead.")
    }
}

impl Default for ABFParameterizer {
    fn default() -> Self {
        Self::new()
    }
}
