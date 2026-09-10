//! Fulcrum Network Coder: Outer GF(2) expansion + Inner sliding-window GF(2^8) RLNC.

use aether_math::matrix::GaussianSolver;
use aether_math::simd::{vec_mul_add, vec_xor};
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoderError {
    #[error("Insufficient degrees of freedom: rank {rank} < required {required}")]
    InsufficientRank { rank: usize, required: usize },
    #[error("Symbol size mismatch: expected {expected}, got {got}")]
    SizeMismatch { expected: usize, got: usize },
}

/// Fulcrum Encoder instance for a single generation of K symbols.
#[derive(Clone, Debug)]
pub struct FulcrumEncoder {
    k: usize,
    r: usize,
    symbol_size: usize,
    source_symbols: Vec<Vec<u8>>,
    expansion_symbols: Vec<Vec<u8>>,
}

impl FulcrumEncoder {
    /// Creates an encoder for K source symbols and r expansion symbols.
    pub fn new(source_symbols: Vec<Vec<u8>>, r: usize) -> Self {
        let k = source_symbols.len();
        assert!(k > 0, "Source symbols cannot be empty");
        let symbol_size = source_symbols[0].len();
        for s in &source_symbols {
            assert_eq!(s.len(), symbol_size, "Symbols must have identical length");
        }

        // Generate r expansion symbols over GF(2) via sparse XOR
        let mut expansion_symbols = Vec::with_capacity(r);
        for exp_idx in 0..r {
            let mut exp = vec![0u8; symbol_size];
            for (src_idx, src) in source_symbols.iter().enumerate() {
                // Sparse deterministic selection: include src if (src_idx + exp_idx) % 2 == 0
                if (src_idx + exp_idx) % 2 == 0 || src_idx == exp_idx {
                    vec_xor(&mut exp, src);
                }
            }
            expansion_symbols.push(exp);
        }

        Self {
            k,
            r,
            symbol_size,
            source_symbols,
            expansion_symbols,
        }
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn r(&self) -> usize {
        self.r
    }

    pub fn symbol_size(&self) -> usize {
        self.symbol_size
    }

    /// Returns a reference to the systematic source symbol at index `i`.
    pub fn get_systematic(&self, i: usize) -> Option<&[u8]> {
        self.source_symbols.get(i).map(|v| v.as_slice())
    }

    /// Generates an inner RLNC repair symbol over GF(2^8) from intermediate symbols (K + r)
    /// using coefficients derived from `seed`.
    pub fn generate_repair(&self, seed: u16, out: &mut [u8]) {
        assert_eq!(out.len(), self.symbol_size);
        out.fill(0);

        let total_intermediate = self.k + self.r;
        let mut rng = StdRng::seed_from_u64(seed as u64);

        // Linear combination over GF(2^8)
        for i in 0..total_intermediate {
            let mut coeff = (rng.next_u32() & 0xFF) as u8;
            if coeff == 0 {
                coeff = 1; // Ensure non-zero participation
            }

            if i < self.k {
                vec_mul_add(out, &self.source_symbols[i], coeff);
            } else {
                vec_mul_add(out, &self.expansion_symbols[i - self.k], coeff);
            }
        }
    }
}

/// Fulcrum Decoder instance accumulating systematic and repair symbols.
#[derive(Debug)]
pub struct FulcrumDecoder {
    k: usize,
    r: usize,
    symbol_size: usize,
    systematic_received: Vec<Option<Vec<u8>>>,
    solver: GaussianSolver,
    systematic_count: usize,
}

impl FulcrumDecoder {
    pub fn new(k: usize, r: usize, symbol_size: usize) -> Self {
        Self {
            k,
            r,
            symbol_size,
            systematic_received: vec![None; k],
            solver: GaussianSolver::new(k + r, symbol_size),
            systematic_count: 0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.systematic_count == self.k || self.solver.is_solved()
    }

    pub fn degrees_of_freedom(&self) -> usize {
        if self.systematic_count == self.k {
            self.k
        } else {
            self.solver.rank()
        }
    }

    /// Ingests a systematic symbol (symbol_id < K).
    pub fn ingest_systematic(&mut self, symbol_id: usize, data: &[u8]) -> bool {
        if symbol_id >= self.k || data.len() != self.symbol_size {
            return false;
        }

        if self.systematic_received[symbol_id].is_none() {
            self.systematic_received[symbol_id] = Some(data.to_vec());
            self.systematic_count += 1;

            // Also feed into inner solver as unit vector
            let mut coeffs = vec![0u8; self.k + self.r];
            coeffs[symbol_id] = 1;
            self.solver.add_row(&coeffs, data);
            true
        } else {
            false
        }
    }

    /// Ingests an inner coded repair symbol derived from seed.
    pub fn ingest_repair(&mut self, seed: u16, data: &[u8]) -> bool {
        if data.len() != self.symbol_size || self.is_complete() {
            return false;
        }

        let total = self.k + self.r;
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let mut coeffs = vec![0u8; total];
        for coeff in coeffs.iter_mut() {
            let mut c = (rng.next_u32() & 0xFF) as u8;
            if c == 0 {
                c = 1;
            }
            *coeff = c;
        }

        self.solver.add_row(&coeffs, data)
    }

    /// Reconstructs the original K source symbols.
    pub fn reconstruct(&self) -> Result<Vec<Vec<u8>>, CoderError> {
        // Fast path: all systematic received directly
        if self.systematic_count == self.k {
            let mut res = Vec::with_capacity(self.k);
            for opt in &self.systematic_received {
                res.push(opt.as_ref().unwrap().clone());
            }
            return Ok(res);
        }

        // Fallback: Gaussian solver solution
        if self.solver.is_solved() {
            let intermediate = self.solver.reconstructed().ok_or(CoderError::InsufficientRank {
                rank: self.solver.rank(),
                required: self.k + self.r,
            })?;
            // First K intermediate symbols are the source symbols!
            return Ok(intermediate[0..self.k].to_vec());
        }

        Err(CoderError::InsufficientRank {
            rank: self.degrees_of_freedom(),
            required: self.k,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fulcrum_all_systematic() {
        let s1 = vec![1, 2, 3, 4];
        let s2 = vec![5, 6, 7, 8];
        let s3 = vec![9, 10, 11, 12];
        let source = vec![s1.clone(), s2.clone(), s3.clone()];

        let _encoder = FulcrumEncoder::new(source.clone(), 1);
        let mut decoder = FulcrumDecoder::new(3, 1, 4);

        assert!(decoder.ingest_systematic(0, &s1));
        assert!(decoder.ingest_systematic(1, &s2));
        assert!(decoder.ingest_systematic(2, &s3));

        assert!(decoder.is_complete());
        let recovered = decoder.reconstruct().unwrap();
        assert_eq!(recovered, source);
    }

    #[test]
    fn test_fulcrum_recovery_from_repair_packets() {
        let s1 = vec![10, 20, 30, 40];
        let s2 = vec![50, 60, 70, 80];
        let source = vec![s1.clone(), s2.clone()];

        let encoder = FulcrumEncoder::new(source.clone(), 1);
        let mut decoder = FulcrumDecoder::new(2, 1, 4);

        // Systematic packet 0 is received, packet 1 is lost!
        assert!(decoder.ingest_systematic(0, &s1));

        // Inject repair packets until reconstructed
        let mut repair = vec![0u8; 4];
        for seed in 1..20 {
            encoder.generate_repair(seed, &mut repair);
            decoder.ingest_repair(seed, &repair);
            if decoder.is_complete() {
                break;
            }
        }

        assert!(decoder.is_complete());
        let recovered = decoder.reconstruct().unwrap();
        assert_eq!(recovered, source);
    }
}
