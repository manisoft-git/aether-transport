//! Lightweight subspace homomorphic vector MAC for Byzantine pollution defense.
//!
//! Enforces that linear combinations formed by the network or sender are validated
//! prior to entering the Gaussian elimination matrix solver.

use aether_math::gf256::gf_mul;

#[derive(Clone, Debug)]
pub struct SubspaceMac {
    key: Vec<u8>,
}

impl SubspaceMac {
    /// Creates a new subspace MAC evaluator with the given pseudo-random key.
    pub fn new(key: &[u8]) -> Self {
        assert!(!key.is_empty(), "Key cannot be empty");
        Self { key: key.to_vec() }
    }

    /// Computes the homomorphic tag for a payload vector in GF(2^8).
    /// tag = sum_i(key[i % key_len] * payload[i]) in GF(2^8)
    pub fn compute_tag(&self, payload: &[u8]) -> u16 {
        let mut acc_low: u8 = 0;
        let mut acc_high: u8 = 0;
        let key_len = self.key.len();

        for (i, &byte) in payload.iter().enumerate() {
            let k_low = self.key[i % key_len];
            let k_high = self.key[(i + 13) % key_len];
            acc_low ^= gf_mul(byte, k_low);
            acc_high ^= gf_mul(byte, k_high);
        }

        u16::from_le_bytes([acc_low, acc_high])
    }

    /// Verifies if a linear combination of tags matches the tag of the combined payload.
    pub fn verify_combination(
        coeffs: &[u8],
        source_tags: &[u16],
        combined_tag: u16,
    ) -> bool {
        assert_eq!(coeffs.len(), source_tags.len());

        let mut expected_low: u8 = 0;
        let mut expected_high: u8 = 0;

        for (&coeff, &tag) in coeffs.iter().zip(source_tags.iter()) {
            if coeff == 0 {
                continue;
            }
            let [t_low, t_high] = tag.to_le_bytes();
            expected_low ^= gf_mul(t_low, coeff);
            expected_high ^= gf_mul(t_high, coeff);
        }

        let expected = u16::from_le_bytes([expected_low, expected_high]);
        expected == combined_tag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homomorphic_tag_linearity() {
        let key = b"aether-subspace-mac-key-12345678";
        let mac = SubspaceMac::new(key);

        let p1 = [10u8, 20, 30, 40];
        let p2 = [50u8, 60, 70, 80];

        let tag1 = mac.compute_tag(&p1);
        let tag2 = mac.compute_tag(&p2);

        // Linear combination: c = 3 * p1 ^ 5 * p2
        let mut c = [0u8; 4];
        for i in 0..4 {
            c[i] = gf_mul(p1[i], 3) ^ gf_mul(p2[i], 5);
        }

        let tag_c = mac.compute_tag(&c);

        assert!(SubspaceMac::verify_combination(
            &[3, 5],
            &[tag1, tag2],
            tag_c
        ));

        // Corrupted tag should fail
        assert!(!SubspaceMac::verify_combination(
            &[3, 5],
            &[tag1, tag2],
            tag_c ^ 0x00FF
        ));
    }
}
