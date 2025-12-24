use crate::mesh::types::MeshData;

/// Voronoi diagram computation for mesh processing
/// Can be used for remeshing, spacing optimization, etc.
pub struct VoronoiDiagram {
    sites: Vec<[f32; 2]>,
    cells: Vec<VoronoiCell>,
}

#[derive(Debug, Clone)]
pub struct VoronoiCell {
    pub site_index: usize,
    pub neighbors: Vec<usize>,
    pub area: f32,
}

impl VoronoiDiagram {
    pub fn new() -> Self {
        Self {
            sites: Vec::new(),
            cells: Vec::new(),
        }
    }

    /// Compute Voronoi diagram from mesh vertex positions
    pub fn from_mesh(mesh: &MeshData) -> Self {
        let mut sites = Vec::new();
        
        // Project vertices to 2D (use existing UV coords if available)
        for vertex in &mesh.vertices {
            sites.push(vertex.uv);
        }
        
        let mut diagram = Self {
            sites,
            cells: Vec::new(),
        };
        
        diagram.compute();
        diagram
    }

    /// Compute Voronoi diagram from 2D sites
    pub fn from_sites(sites: Vec<[f32; 2]>) -> Self {
        let mut diagram = Self {
            sites,
            cells: Vec::new(),
        };
        
        diagram.compute();
        diagram
    }

    /// Compute the Voronoi diagram
    fn compute(&mut self) {
        if self.sites.is_empty() {
            return;
        }

        // Simple nearest-neighbor based approach
        // For production, would use Fortune's algorithm or Bowyer-Watson
        self.cells.clear();
        
        for i in 0..self.sites.len() {
            let mut neighbors = Vec::new();
            
            // Find potential neighbors by checking proximity
            for j in 0..self.sites.len() {
                if i == j {
                    continue;
                }
                
                if self.are_neighbors(i, j) {
                    neighbors.push(j);
                }
            }
            
            // Estimate cell area (simplified)
            let area = self.estimate_cell_area(i);
            
            self.cells.push(VoronoiCell {
                site_index: i,
                neighbors,
                area,
            });
        }
    }

    /// Check if two sites are neighbors in Voronoi diagram
    fn are_neighbors(&self, i: usize, j: usize) -> bool {
        let site_i = self.sites[i];
        let site_j = self.sites[j];
        
        let dist_ij = self.distance(site_i, site_j);
        
        // Check if any other site is between them
        for k in 0..self.sites.len() {
            if k == i || k == j {
                continue;
            }
            
            let site_k = self.sites[k];
            let dist_ik = self.distance(site_i, site_k);
            let dist_jk = self.distance(site_j, site_k);
            
            // If k is much closer to either site, i and j likely aren't neighbors
            if dist_ik < dist_ij * 0.6 || dist_jk < dist_ij * 0.6 {
                // Check if k is roughly between i and j
                let dot = (site_k[0] - site_i[0]) * (site_j[0] - site_i[0]) +
                         (site_k[1] - site_i[1]) * (site_j[1] - site_i[1]);
                
                if dot > 0.0 && dot < dist_ij * dist_ij {
                    return false;
                }
            }
        }
        
        // Threshold for neighbor distance
        dist_ij < self.average_site_spacing() * 2.0
    }

    /// Estimate cell area using nearest neighbors
    fn estimate_cell_area(&self, site_index: usize) -> f32 {
        let site = self.sites[site_index];
        
        // Find closest neighbor
        let mut min_dist = f32::INFINITY;
        for (j, other_site) in self.sites.iter().enumerate() {
            if j == site_index {
                continue;
            }
            
            let dist = self.distance(site, *other_site);
            min_dist = min_dist.min(dist);
        }
        
        // Approximate cell area as square of half the distance to nearest neighbor
        let radius = min_dist * 0.5;
        std::f32::consts::PI * radius * radius
    }

    /// Calculate distance between two 2D points
    fn distance(&self, p1: [f32; 2], p2: [f32; 2]) -> f32 {
        let dx = p2[0] - p1[0];
        let dy = p2[1] - p1[1];
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate average spacing between sites
    fn average_site_spacing(&self) -> f32 {
        if self.sites.len() < 2 {
            return 1.0;
        }

        let mut total_dist = 0.0;
        let mut count = 0;

        for i in 0..self.sites.len().min(10) {
            let mut min_dist = f32::INFINITY;
            for j in 0..self.sites.len() {
                if i == j {
                    continue;
                }
                let dist = self.distance(self.sites[i], self.sites[j]);
                min_dist = min_dist.min(dist);
            }
            total_dist += min_dist;
            count += 1;
        }

        if count > 0 {
            total_dist / count as f32
        } else {
            1.0
        }
    }

    /// Get the cells of the diagram
    pub fn cells(&self) -> &[VoronoiCell] {
        &self.cells
    }

    /// Get the sites
    pub fn sites(&self) -> &[[f32; 2]] {
        &self.sites
    }
}

impl Default for VoronoiDiagram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voronoi_creation() {
        let sites = vec![
            [0.0, 0.0],
            [1.0, 0.0],
            [0.5, 1.0],
        ];
        
        let diagram = VoronoiDiagram::from_sites(sites);
        assert_eq!(diagram.cells().len(), 3);
    }

    #[test]
    fn test_distance() {
        let diagram = VoronoiDiagram::new();
        let dist = diagram.distance([0.0, 0.0], [3.0, 4.0]);
        assert!((dist - 5.0).abs() < 0.001);
    }
}
