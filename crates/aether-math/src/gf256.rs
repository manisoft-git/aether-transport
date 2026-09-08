//! Finite field GF(2^8) arithmetic constructed modulo P(x) = x^8 + x^4 + x^3 + x + 1 (0x11D).

/// The primitive irreducible polynomial for GF(2^8) in hexadecimal.
pub const PRIMITIVE_POLYNOMIAL: u16 = 0x11D;

/// Compile-time calculation of exponentiation and logarithm tables.
const fn generate_tables() -> ([u8; 256], [u8; 256]) {
    let mut exp = [0u8; 256];
    let mut log = [0u8; 256];

    let mut val: u16 = 1;
    let mut i = 0;
    while i < 255 {
        exp[i] = val as u8;
        log[val as usize] = i as u8;

        val <<= 1;
        if (val & 0x100) != 0 {
            val ^= PRIMITIVE_POLYNOMIAL;
        }
        i += 1;
    }
    // Repeat for cyclic rollover
    exp[255] = exp[0];
    log[0] = 0; // Convention: log(0) is undefined, mapped to 0

    (exp, log)
}

pub static TABLES: ([u8; 256], [u8; 256]) = generate_tables();
pub static EXP_TABLE: &[u8; 256] = &TABLES.0;
pub static LOG_TABLE: &[u8; 256] = &TABLES.1;

/// Addition in GF(2^8) is bitwise XOR.
#[inline(always)]
pub const fn gf_add(a: u8, b: u8) -> u8 {
    a ^ b
}

/// Subtraction in GF(2^8) is identical to addition.
#[inline(always)]
pub const fn gf_sub(a: u8, b: u8) -> u8 {
    a ^ b
}

/// Multiplication in GF(2^8) using lookup tables.
#[inline(always)]
pub fn gf_mul(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 {
        return 0;
    }
    let log_a = LOG_TABLE[a as usize] as usize;
    let log_b = LOG_TABLE[b as usize] as usize;
    let sum = (log_a + log_b) % 255;
    EXP_TABLE[sum]
}

/// Multiplicative inverse in GF(2^8). Returns 0 for a == 0.
#[inline(always)]
pub fn gf_inv(a: u8) -> u8 {
    if a == 0 {
        return 0;
    }
    let log_a = LOG_TABLE[a as usize] as usize;
    let inv_log = 255 - log_a;
    EXP_TABLE[inv_log % 255]
}

/// Division in GF(2^8): a / b = a * inv(b).
#[inline(always)]
pub fn gf_div(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 {
        return 0;
    }
    let log_a = LOG_TABLE[a as usize] as usize;
    let log_b = LOG_TABLE[b as usize] as usize;
    let diff = (log_a + 255 - log_b) % 255;
    EXP_TABLE[diff]
}

/// Type-safe wrapper for a GF(2^8) element with standard algebraic operators.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GF256(pub u8);

impl GF256 {
    pub const ZERO: GF256 = GF256(0);
    pub const ONE: GF256 = GF256(1);

    #[inline(always)]
    pub fn new(val: u8) -> Self {
        GF256(val)
    }

    #[inline(always)]
    pub fn inv(self) -> Self {
        GF256(gf_inv(self.0))
    }
}

impl core::ops::Add for GF256 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        GF256(gf_add(self.0, rhs.0))
    }
}

impl core::ops::AddAssign for GF256 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = gf_add(self.0, rhs.0);
    }
}

impl core::ops::Sub for GF256 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        GF256(gf_sub(self.0, rhs.0))
    }
}

impl core::ops::Mul for GF256 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        GF256(gf_mul(self.0, rhs.0))
    }
}

impl core::ops::MulAssign for GF256 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = gf_mul(self.0, rhs.0);
    }
}

impl core::ops::Div for GF256 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        GF256(gf_div(self.0, rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_axioms() {
        // Multiplicative identity
        for a in 0..=255u8 {
            assert_eq!(gf_mul(a, 1), a);
            assert_eq!(gf_mul(1, a), a);
            assert_eq!(gf_mul(a, 0), 0);
        }

        // Inverses
        for a in 1..=255u8 {
            let inv = gf_inv(a);
            assert_ne!(inv, 0);
            assert_eq!(gf_mul(a, inv), 1, "a = {}, inv = {}", a, inv);
        }

        // Associativity and Distributivity
        let a = 0x57;
        let b = 0x83;
        let c = 0x13;
        assert_eq!(gf_mul(a, gf_add(b, c)), gf_add(gf_mul(a, b), gf_mul(a, c)));
    }
}
