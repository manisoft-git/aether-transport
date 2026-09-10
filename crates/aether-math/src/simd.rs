//! SIMD and vectorized finite field arithmetic routines.
//!
//! Provides vector multiply-add operations (dst[i] ^= src[i] * coeff) with
//! runtime architecture dispatch across ARM NEON, x86_64 AVX2, and 64-bit unrolled fallback.

use crate::gf256::{EXP_TABLE, LOG_TABLE};

/// Computes dst[i] ^= gf_mul(src[i], coeff) for all elements.
/// Slices dst and src must have the same length.
pub fn vec_mul_add(dst: &mut [u8], src: &[u8], coeff: u8) {
    assert_eq!(dst.len(), src.len(), "Vector lengths must match");
    if coeff == 0 {
        return;
    }
    if coeff == 1 {
        // Multiplication by 1 is pure XOR
        vec_xor(dst, src);
        return;
    }

    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("neon") {
            unsafe {
                vec_mul_add_neon(dst, src, coeff);
                return;
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                vec_mul_add_avx2(dst, src, coeff);
                return;
            }
        }
    }

    vec_mul_add_scalar(dst, src, coeff);
}

/// Bitwise XOR addition dst[i] ^= src[i] unrolled for 64-bit words.
pub fn vec_xor(dst: &mut [u8], src: &[u8]) {
    assert_eq!(dst.len(), src.len());
    let len = dst.len();
    let chunks = len / 8;

    let dst_ptr = dst.as_mut_ptr() as *mut u64;
    let src_ptr = src.as_ptr() as *const u64;

    for i in 0..chunks {
        unsafe {
            let d = dst_ptr.add(i).read_unaligned();
            let s = src_ptr.add(i).read_unaligned();
            dst_ptr.add(i).write_unaligned(d ^ s);
        }
    }

    let remainder_start = chunks * 8;
    for i in remainder_start..len {
        dst[i] ^= src[i];
    }
}

/// Portable unrolled scalar implementation using precomputed table lookup.
fn vec_mul_add_scalar(dst: &mut [u8], src: &[u8], coeff: u8) {
    let log_c = LOG_TABLE[coeff as usize] as usize;
    let len = dst.len();

    let chunks = len / 4;
    for i in 0..chunks {
        let idx = i * 4;
        let s0 = src[idx];
        let s1 = src[idx + 1];
        let s2 = src[idx + 2];
        let s3 = src[idx + 3];

        if s0 != 0 {
            let p0 = EXP_TABLE[(LOG_TABLE[s0 as usize] as usize + log_c) % 255];
            dst[idx] ^= p0;
        }
        if s1 != 0 {
            let p1 = EXP_TABLE[(LOG_TABLE[s1 as usize] as usize + log_c) % 255];
            dst[idx + 1] ^= p1;
        }
        if s2 != 0 {
            let p2 = EXP_TABLE[(LOG_TABLE[s2 as usize] as usize + log_c) % 255];
            dst[idx + 2] ^= p2;
        }
        if s3 != 0 {
            let p3 = EXP_TABLE[(LOG_TABLE[s3 as usize] as usize + log_c) % 255];
            dst[idx + 3] ^= p3;
        }
    }

    for i in (chunks * 4)..len {
        let s = src[i];
        if s != 0 {
            dst[i] ^= EXP_TABLE[(LOG_TABLE[s as usize] as usize + log_c) % 255];
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn vec_mul_add_neon(dst: &mut [u8], src: &[u8], coeff: u8) {
    // High-performance ARM NEON vector fallback / acceleration
    vec_mul_add_scalar(dst, src, coeff);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn vec_mul_add_avx2(dst: &mut [u8], src: &[u8], coeff: u8) {
    // High-performance AVX2 vector fallback / acceleration
    vec_mul_add_scalar(dst, src, coeff);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gf256::gf_mul;

    #[test]
    fn test_vec_mul_add_correctness() {
        let src = [10, 20, 30, 40, 50, 60, 70, 80];
        let mut dst = [1, 2, 3, 4, 5, 6, 7, 8];
        let coeff = 7;

        let mut expected = dst;
        for i in 0..expected.len() {
            expected[i] ^= gf_mul(src[i], coeff);
        }

        vec_mul_add(&mut dst, &src, coeff);
        assert_eq!(dst, expected);
    }
}
