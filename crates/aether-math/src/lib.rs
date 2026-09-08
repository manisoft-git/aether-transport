//! `aether-math`: Core Galois Field arithmetic and Gaussian solver routines.

pub mod gf256;
pub mod matrix;
pub mod simd;

pub use gf256::{gf_add, gf_div, gf_inv, gf_mul, gf_sub, GF256};
pub use matrix::GaussianSolver;
pub use simd::{vec_mul_add, vec_xor};
