//! In-place incremental Gaussian-Jordan matrix solver over GF(2^8).
//!
//! Maintains an augmented matrix in row echelon form as packets arrive,
//! tracking linear rank and solving immediately once full rank K is reached.

use crate::gf256::{gf_inv, gf_mul};
use crate::simd::{vec_mul_add, vec_xor};

#[derive(Clone, Debug)]
pub struct GaussianSolver {
    k: usize,
    symbol_size: usize,
    matrix: Vec<u8>,        // k * k coefficient matrix in row-major order
    payloads: Vec<u8>,      // k * symbol_size augmented payload matrix
    row_pivot_col: Vec<Option<usize>>, // Pivot column index for each row in echelon form
    rank: usize,
}

impl GaussianSolver {
    /// Creates a new solver expecting K source symbols of length symbol_size bytes.
    pub fn new(k: usize, symbol_size: usize) -> Self {
        Self {
            k,
            symbol_size,
            matrix: vec![0u8; k * k],
            payloads: vec![0u8; k * symbol_size],
            row_pivot_col: vec![None; k],
            rank: 0,
        }
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn is_solved(&self) -> bool {
        self.rank == self.k
    }

    /// Ingests an incoming coded packet.
    /// Returns `true` if the packet was innovative (increased rank), or `false` if redundant.
    pub fn add_row(&mut self, coeffs: &[u8], payload: &[u8]) -> bool {
        assert_eq!(coeffs.len(), self.k);
        assert_eq!(payload.len(), self.symbol_size);

        if self.is_solved() {
            return false;
        }

        let mut row_coeffs = coeffs.to_vec();
        let mut row_payload = payload.to_vec();

        // Reduce incoming row against existing echelon rows
        for i in 0..self.rank {
            if let Some(pivot_col) = self.row_pivot_col[i] {
                let leading_val = row_coeffs[pivot_col];
                if leading_val != 0 {
                    // Eliminate leading_val in row_coeffs using row i
                    let existing_coeffs = self.get_row_coeffs(i);
                    let existing_payload = self.get_row_payload(i);

                    // row_coeffs ^= leading_val * existing_coeffs
                    for c in 0..self.k {
                        row_coeffs[c] ^= gf_mul(existing_coeffs[c], leading_val);
                    }
                    // row_payload ^= leading_val * existing_payload
                    vec_mul_add(&mut row_payload, existing_payload, leading_val);
                }
            }
        }

        // Find the new pivot column
        let mut pivot_col = None;
        for c in 0..self.k {
            if row_coeffs[c] != 0 {
                pivot_col = Some(c);
                break;
            }
        }

        let pivot = match pivot_col {
            Some(c) => c,
            None => {
                // Linearly dependent row (all zeros)
                return false;
            }
        };

        // Normalize the pivot row so that leading coefficient is 1
        let pivot_val = row_coeffs[pivot];
        if pivot_val != 1 {
            let inv = gf_inv(pivot_val);
            for c in 0..self.k {
                row_coeffs[c] = gf_mul(row_coeffs[c], inv);
            }
            // Scale payload
            for b in 0..self.symbol_size {
                row_payload[b] = gf_mul(row_payload[b], inv);
            }
        }

        // Back-substitute against earlier rows to maintain Reduced Row Echelon Form (RREF)
        for i in 0..self.rank {
            let existing_lead = self.matrix[i * self.k + pivot];
            if existing_lead != 0 {
                for c in 0..self.k {
                    let val = gf_mul(row_coeffs[c], existing_lead);
                    self.matrix[i * self.k + c] ^= val;
                }
                let existing_payload = self.get_row_payload_mut(i);
                vec_mul_add(existing_payload, &row_payload, existing_lead);
            }
        }

        // Insert new row
        let new_row_idx = self.rank;
        let start_c = new_row_idx * self.k;
        self.matrix[start_c..start_c + self.k].copy_from_slice(&row_coeffs);

        let start_p = new_row_idx * self.symbol_size;
        self.payloads[start_p..start_p + self.symbol_size].copy_from_slice(&row_payload);

        self.row_pivot_col[new_row_idx] = Some(pivot);
        self.rank += 1;

        // If we reached full rank, perform full diagonal alignment if needed
        if self.rank == self.k {
            self.finalize_solution();
        }

        true
    }

    fn finalize_solution(&mut self) {
        // Ensure rows are sorted by pivot column index (0..k)
        for target_col in 0..self.k {
            let mut found_row = None;
            for r in target_col..self.k {
                if self.row_pivot_col[r] == Some(target_col) {
                    found_row = Some(r);
                    break;
                }
            }
            if let Some(r) = found_row {
                if r != target_col {
                    // Swap row r and target_col
                    self.swap_rows(r, target_col);
                }
            }
        }
    }

    fn swap_rows(&mut self, r1: usize, r2: usize) {
        let k = self.k;
        let s = self.symbol_size;

        for c in 0..k {
            self.matrix.swap(r1 * k + c, r2 * k + c);
        }
        for b in 0..s {
            self.payloads.swap(r1 * s + b, r2 * s + b);
        }
        self.row_pivot_col.swap(r1, r2);
    }

    fn get_row_coeffs(&self, row: usize) -> &[u8] {
        let start = row * self.k;
        &self.matrix[start..start + self.k]
    }

    fn get_row_payload(&self, row: usize) -> &[u8] {
        let start = row * self.symbol_size;
        &self.payloads[start..start + self.symbol_size]
    }

    fn get_row_payload_mut(&mut self, row: usize) -> &mut [u8] {
        let start = row * self.symbol_size;
        &mut self.payloads[start..start + self.symbol_size]
    }

    /// Returns the reconstructed source symbols if full rank is achieved.
    pub fn reconstructed(&self) -> Option<Vec<Vec<u8>>> {
        if !self.is_solved() {
            return None;
        }

        let mut res = Vec::with_capacity(self.k);
        for i in 0..self.k {
            res.push(self.get_row_payload(i).to_vec());
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systematic_identity_solve() {
        const K: usize = 3;
        const S: usize = 4;
        let mut solver = GaussianSolver::new(K, S);

        let p1 = [1, 2, 3, 4];
        let p2 = [5, 6, 7, 8];
        let p3 = [9, 10, 11, 12];

        assert!(solver.add_row(&[1, 0, 0], &p1));
        assert!(solver.add_row(&[0, 1, 0], &p2));
        assert!(solver.add_row(&[0, 0, 1], &p3));

        assert!(solver.is_solved());
        let res = solver.reconstructed().unwrap();
        assert_eq!(res[0], p1);
        assert_eq!(res[1], p2);
        assert_eq!(res[2], p3);
    }

    #[test]
    fn test_coded_linear_combination_solve() {
        const K: usize = 2;
        const S: usize = 2;
        let mut solver = GaussianSolver::new(K, S);

        // Source symbols: s1 = [10, 20], s2 = [30, 40]
        // c1 = 1*s1 + 2*s2 = [10 ^ 2*30, 20 ^ 2*40]
        // c2 = 3*s1 + 1*s2
        let s1 = [10, 20];
        let s2 = [30, 40];

        let c1_0 = 10 ^ gf_mul(30, 2);
        let c1_1 = 20 ^ gf_mul(40, 2);
        let c1 = [c1_0, c1_1];

        let c2_0 = gf_mul(10, 3) ^ 30;
        let c2_1 = gf_mul(20, 3) ^ 40;
        let c2 = [c2_0, c2_1];

        assert!(solver.add_row(&[1, 2], &c1));
        assert_eq!(solver.rank(), 1);
        assert!(solver.add_row(&[3, 1], &c2));
        assert!(solver.is_solved());

        let out = solver.reconstructed().unwrap();
        assert_eq!(out[0], s1);
        assert_eq!(out[1], s2);
    }
}
