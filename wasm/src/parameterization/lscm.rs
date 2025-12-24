use anyhow::Result;
use crate::mesh::types::MeshData;
use nalgebra_sparse::{CooMatrix, CscMatrix};
use nalgebra::DVector;

pub struct LSCMParameterizer {
    _private: (),
}

impl LSCMParameterizer {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn parameterize(&self, mesh: &MeshData) -> Result<Vec<[f32; 2]>> {
        let n = mesh.vertices.len();
        
        if n < 3 {
            anyhow::bail!("Mesh must have at least 3 vertices");
        }

        let (pin0, pin1) = self.find_pin_vertices(mesh);

        let mut triplets_u = Vec::new();
        let mut triplets_v = Vec::new();
        
        let mut row = 0;
        
        for face in &mesh.faces {
            let v0 = mesh.vertices[face.indices[0] as usize].position;
            let v1 = mesh.vertices[face.indices[1] as usize].position;
            let v2 = mesh.vertices[face.indices[2] as usize].position;

            let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

            let len1 = (e1[0] * e1[0] + e1[1] * e1[1] + e1[2] * e1[2]).sqrt();
            if len1 < 1e-10 {
                continue;
            }

            let x0 = 0.0;
            let y0 = 0.0;
            let x1 = len1;
            let y1 = 0.0;

            let dot = (e1[0] * e2[0] + e1[1] * e2[1] + e1[2] * e2[2]) / len1;
            let cross = [
                e1[1] * e2[2] - e1[2] * e2[1],
                e1[2] * e2[0] - e1[0] * e2[2],
                e1[0] * e2[1] - e1[1] * e2[0],
            ];
            let cross_len = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
            
            let x2 = dot;
            let y2 = cross_len / len1;

            if y2.abs() < 1e-10 {
                continue;
            }

            let dx1 = (x1 - x0) as f64;
            let dy1 = (y1 - y0) as f64;
            let dx2 = (x2 - x0) as f64;
            let dy2 = (y2 - y0) as f64;

            let area = dx1 * dy2 - dx2 * dy1;
            if area.abs() < 1e-10 {
                continue;
            }

            let w = 1.0 / area;

            let i0 = face.indices[0] as usize;
            let i1 = face.indices[1] as usize;
            let i2 = face.indices[2] as usize;

            triplets_u.push((row, i0, -dy2 * w));
            triplets_u.push((row, i1, dy2 * w));
            triplets_u.push((row + 1, i0, dx2 * w));
            triplets_u.push((row + 1, i1, -dx2 * w));

            triplets_v.push((row, i0, dy1 * w));
            triplets_v.push((row, i2, -dy1 * w));
            triplets_v.push((row + 1, i0, -dx1 * w));
            triplets_v.push((row + 1, i2, dx1 * w));

            row += 2;
        }

        let u_coords = self.solve_lscm_system(n, &triplets_u, pin0, pin1, 0.0, 1.0)?;
        let v_coords = self.solve_lscm_system(n, &triplets_v, pin0, pin1, 0.0, 0.0)?;

        let mut uv_coords = vec![[0.0, 0.0]; n];
        for i in 0..n {
            uv_coords[i] = [u_coords[i], v_coords[i]];
        }

        Ok(uv_coords)
    }

    fn find_pin_vertices(&self, mesh: &MeshData) -> (usize, usize) {
        let mut max_dist = 0.0;
        let mut farthest = 1;
        
        let p0 = mesh.vertices[0].position;
        
        for (i, v) in mesh.vertices.iter().enumerate().skip(1) {
            let dx = v.position[0] - p0[0];
            let dy = v.position[1] - p0[1];
            let dz = v.position[2] - p0[2];
            let dist = dx * dx + dy * dy + dz * dz;
            
            if dist > max_dist {
                max_dist = dist;
                farthest = i;
            }
        }
        
        (0, farthest)
    }

    fn solve_lscm_system(
        &self,
        n: usize,
        triplets: &[(usize, usize, f64)],
        pin0: usize,
        pin1: usize,
        val0: f64,
        val1: f64,
    ) -> Result<Vec<f32>> {
        let max_row = triplets.iter().map(|(r, _, _)| *r).max().unwrap_or(0) + 1;
        
        let mut coo = CooMatrix::new(max_row + 2, n);
        
        for &(row, col, val) in triplets {
            coo.push(row, col, val);
        }
        
        coo.push(max_row, pin0, 1.0);
        coo.push(max_row + 1, pin1, 1.0);
        
        let a = CscMatrix::from(&coo);
        
        let mut b = DVector::zeros(max_row + 2);
        b[max_row] = val0;
        b[max_row + 1] = val1;
        
        let at = a.transpose();
        let ata = &at * &a;
        let atb = &at * &b;
        
        let x = self.solve_cg(&ata, &atb, n)?;
        
        Ok(x.iter().map(|&v| v as f32).collect())
    }

    fn solve_cg(&self, a: &CscMatrix<f64>, b: &DVector<f64>, n: usize) -> Result<Vec<f64>> {
        let mut x = vec![0.0; n];
        let mut r = b.clone();
        let mut p = r.clone();
        let mut rs_old = r.dot(&r);
        
        for _ in 0..1000 {
            let ap = a * &DVector::from_vec(p.clone().data.as_vec().clone());
            let alpha = rs_old / p.dot(&ap);
            
            for i in 0..n {
                x[i] += alpha * p[i];
                r[i] -= alpha * ap[i];
            }
            
            let rs_new = r.dot(&r);
            if rs_new.sqrt() < 1e-10 {
                break;
            }
            
            let beta = rs_new / rs_old;
            for i in 0..n {
                p[i] = r[i] + beta * p[i];
            }
            
            rs_old = rs_new;
        }
        
        Ok(x)
    }
}

impl Default for LSCMParameterizer {
    fn default() -> Self {
        Self::new()
    }
}
