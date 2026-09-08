//! CRC-16-CCITT implementation for packet payload integrity verification.

const POLYNOMIAL: u16 = 0x1021;

const fn make_table() -> [u16; 256] {
    let mut table = [0u16; 256];
    let mut i = 0;
    while i < 256 {
        let mut curr = (i as u16) << 8;
        let mut j = 0;
        while j < 8 {
            if (curr & 0x8000) != 0 {
                curr = (curr << 1) ^ POLYNOMIAL;
            } else {
                curr <<= 1;
            }
            j += 1;
        }
        table[i] = curr;
        i += 1;
    }
    table
}

static CRC_TABLE: [u16; 256] = make_table();

/// Calculates the CRC-16-CCITT checksum over a slice with an initial value of 0xFFFF.
pub fn crc16_ccitt(data: &[u8]) -> u16 {
    let mut crc = 0xFFFFu16;
    for &byte in data {
        let table_idx = ((crc >> 8) as u8 ^ byte) as usize;
        crc = (crc << 8) ^ CRC_TABLE[table_idx];
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16_known_vector() {
        let data = b"123456789";
        // Standard CRC16-CCITT (0xFFFF init) produces 0x29B1
        let res = crc16_ccitt(data);
        assert_eq!(res, 0x29B1);
    }
}
