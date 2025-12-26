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

    /// Parameterize a 3D mesh into 2D UV coordinates using a coupled LSCM solver.
    pub fn parameterize(&self, mesh: &MeshData) -> Result<Vec<[f32; 2]>> {
        let n = mesh.vertices.len();
        let f = mesh.faces.len();
        
        if n < 3 {
            anyhow::bail!("Mesh must have at least 3 vertices for parameterization.");
        }

        // Find two distant vertices to "pin" and prevent the trivial (collapsed) solution
        let (pin0, pin1) = self.find_pin_vertices(mesh);

        // We build a system Ax = b where x = [u0...un, v0...vn]^T (length 2n)
        // Each face contributes 2 Cauchy-Riemann equations (rows)
        // Total rows: 2 * face_count + 4 (for pinning)
        let mut coo = CooMatrix::new(2 * f + 4, 2 * n);
        let mut b = DVector::zeros(2 * f + 4);
        
        let mut row = 0;
        for face in &mesh.faces {
            let i = face.indices[0] as usize;
            let j = face.indices[1] as usize;
            let k = face.indices[2] as usize;

            let p_i = mesh.vertices[i].position;
            let p_j = mesh.vertices[j].position;
            let p_k = mesh.vertices[k].position;

            // 1. Compute local 2D coordinates for the triangle vertices
            // Vector e_ij is the local X-axis
            let e_ij = [p_j[0] - p_i[0], p_j[1] - p_i[1], p_j[2] - p_i[2]];
            let len_ij = (e_ij[0].powi(2) + e_ij[1].powi(2) + e_ij[2].powi(2)).sqrt();
            if len_ij < 1e-10 { continue; }
            
            let unit_x = [e_ij[0] / len_ij, e_ij[1] / len_ij, e_ij[2] / len_ij];
            let e_ik = [p_k[0] - p_i[0], p_k[1] - p_i[1], p_k[2] - p_i[2]];
            
            // Local 2D coordinates: (x0, y0)=(0,0), (x1, y1)=(len_ij, 0), (x2, y2)
            let x0 = 0.0; let y0 = 0.0;
            let x1 = len_ij; let y1 = 0.0;
            
            // x2 is the projection of e_ik onto unit_x
            let x2 = e_ik[0] * unit_x[0] + e_ik[1] * unit_x[1] + e_ik[2] * unit_x[2];
            
            // y2 is the height of the triangle
            let cross = [
                unit_x[1] * e_ik[2] - unit_x[2] * e_ik[1],
                unit_x[2] * e_ik[0] - unit_x[0] * e_ik[2],
                unit_x[0] * e_ik[1] - unit_x[1] * e_ik[0],
            ];
            let y2 = (cross[0].powi(2) + cross[1].powi(2) + cross[2].powi(2)).sqrt();
            
            if y2 < 1e-10 { continue; }

            // 2. Form Cauchy-Riemann weights
            // For a triangle Area A, the partial derivatives are weighted by 1/sqrt(A)
            // to ensure the squared norm of the system matches the conformal energy.
            let sqrt_area = (y2 * len_ij * 0.5).sqrt();
            let w = 1.0 / (y2 * len_ij); // Base weight for gradient computation

            // Derivatives of shape functions
            let du_x = [(y1 - y2) * w, (y2 - y0) * w, (y0 - y1) * w];
            let du_y = [(x2 - x1) * w, (x0 - x2) * w, (x1 - x0) * w];

            // Equation 1: du/dx - dv/dy = 0
            coo.push(row, i, (du_x[0] * sqrt_area) as f64);
            coo.push(row, j, (du_x[1] * sqrt_area) as f64);
            coo.push(row, k, (du_x[2] * sqrt_area) as f64);
            coo.push(row, n + i, (-du_y[0] * sqrt_area) as f64);
            coo.push(row, n + j, (-du_y[1] * sqrt_area) as f64);
            coo.push(row, n + k, (-du_y[2] * sqrt_area) as f64);

            // Equation 2: du/dy + dv/dx = 0
            coo.push(row + 1, i, (du_y[0] * sqrt_area) as f64);
            coo.push(row + 1, j, (du_y[1] * sqrt_area) as f64);
            coo.push(row + 1, k, (du_y[2] * sqrt_area) as f64);
            coo.push(row + 1, n + i, (du_x[0] * sqrt_area) as f64);
            coo.push(row + 1, n + j, (du_x[1] * sqrt_area) as f64);
            coo.push(row + 1, n + k, (du_x[2] * sqrt_area) as f64);

            row += 2;
        }

        // 3. Pin two vertices to fix rotation/scale/translation
        // Pin 0: (u, v) = (0, 0)
        coo.push(row, pin0, 1.0);
        b[row] = 0.0;
        coo.push(row + 1, n + pin0, 1.0);
        b[row + 1] = 0.0;
        
        // Pin 1: (u, v) = (1, 0)
        coo.push(row + 2, pin1, 1.0);
        b[row + 2] = 1.0;
        coo.push(row + 3, n + pin1, 1.0);
        b[row + 3] = 0.0;

        // 4. Solve the normal equations (A^T A) x = A^T b
        let a_sparse = CscMatrix::from(&coo);
        let at = a_sparse.transpose();
        let ata = &at * &a_sparse;
        let atb = &at * &b;

        let solution = self.solve_cg(&ata, &atb, 2 * n)?;

        // 5. Extract UV results
        let mut uv_coords = vec![[0.0, 0.0]; n];
        for i in 0..n {
            uv_coords[i] = [solution[i] as f32, solution[n + i] as f32];
        }

        Ok(uv_coords)
    }

    fn find_pin_vertices(&self, mesh: &MeshData) -> (usize, usize) {
        let mut max_dist = 0.0;
        let mut farthest = 1;
        let p0 = mesh.vertices[0].position;
        
        for (i, v) in mesh.vertices.iter().enumerate().skip(1) {
            let dist = (v.position[0] - p0[0]).powi(2) + 
                       (v.position[1] - p0[1]).powi(2) + 
                       (v.position[2] - p0[2]).powi(2);
            if dist > max_dist {
                max_dist = dist;
                farthest = i;
            }
        }
        (0, farthest)
    }

    fn solve_cg(&self, a: &CscMatrix<f64>, b: &DVector<f64>, dim: usize) -> Result<Vec<f64>> {
        let mut x = DVector::zeros(dim);
        let mut r = b - a * &x;
        let mut p = r.clone();
        let mut rs_old = r.dot(&r);

        for _ in 0..2000 { // Max iterations
            let ap = a * &p;
            let alpha = rs_old / p.dot(&ap);
            x += alpha * &p;
            r -= alpha * &ap;
            
            let rs_new = r.dot(&r);
            if rs_new.sqrt() < 1e-8 {
                break;
            }
            p = &r + (rs_new / rs_old) * &p;
            rs_old = rs_new;
        }

        Ok(x.as_slice().to_vec())
    }
}

impl Default for LSCMParameterizer {
    fn default() -> Self {
        Self::new()
    }
}
