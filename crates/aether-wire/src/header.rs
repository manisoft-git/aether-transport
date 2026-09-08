//! 16-byte zero-copy wire header definition and serialization.

use thiserror::Error;

pub const WIRE_HEADER_SIZE: usize = 16;

pub const FLAG_FIN: u16 = 0x0001;
pub const FLAG_PROBE: u16 = 0x0002;
pub const FLAG_CELL_TELEMETRY: u16 = 0x0004;
pub const FLAG_MULTIPATH_PRIMARY: u16 = 0x0008;
pub const FLAG_MULTIPATH_SECONDARY: u16 = 0x0010;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum WireError {
    #[error("Buffer too small for wire header: required {required}, provided {provided}")]
    BufferTooSmall { required: usize, provided: usize },
    #[error("CRC-16 validation failed: expected {expected:#06x}, computed {computed:#06x}")]
    CrcMismatch { expected: u16, computed: u16 },
    #[error("Subspace authentication tag mismatch")]
    AuthenticationFailed,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[repr(C, packed)]
pub struct PacketHeader {
    pub window_id: u32,
    pub symbol_id: u16,
    pub window_size: u16,
    pub seed: u16,
    pub flags: u16,
    pub crc16: u16,
    pub auth_tag: u16,
}

impl PacketHeader {
    /// Zero-copy parsing of the 16-byte wire header from a byte slice.
    pub fn from_bytes(slice: &[u8]) -> Result<Self, WireError> {
        if slice.len() < WIRE_HEADER_SIZE {
            return Err(WireError::BufferTooSmall {
                required: WIRE_HEADER_SIZE,
                provided: slice.len(),
            });
        }

        let window_id = u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]);
        let symbol_id = u16::from_le_bytes([slice[4], slice[5]]);
        let window_size = u16::from_le_bytes([slice[6], slice[7]]);
        let seed = u16::from_le_bytes([slice[8], slice[9]]);
        let flags = u16::from_le_bytes([slice[10], slice[11]]);
        let crc16 = u16::from_le_bytes([slice[12], slice[13]]);
        let auth_tag = u16::from_le_bytes([slice[14], slice[15]]);

        Ok(Self {
            window_id,
            symbol_id,
            window_size,
            seed,
            flags,
            crc16,
            auth_tag,
        })
    }

    /// Serializes the header into a mutable buffer of at least 16 bytes.
    pub fn write_bytes(&self, slice: &mut [u8]) {
        assert!(slice.len() >= WIRE_HEADER_SIZE);
        slice[0..4].copy_from_slice(&self.window_id.to_le_bytes());
        slice[4..6].copy_from_slice(&self.symbol_id.to_le_bytes());
        slice[6..8].copy_from_slice(&self.window_size.to_le_bytes());
        slice[8..10].copy_from_slice(&self.seed.to_le_bytes());
        slice[10..12].copy_from_slice(&self.flags.to_le_bytes());
        slice[12..14].copy_from_slice(&self.crc16.to_le_bytes());
        slice[14..16].copy_from_slice(&self.auth_tag.to_le_bytes());
    }

    pub fn is_systematic(&self) -> bool {
        self.symbol_id < self.window_size
    }

    pub fn is_repair(&self) -> bool {
        self.symbol_id >= self.window_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_roundtrip() {
        let hdr = PacketHeader {
            window_id: 1042,
            symbol_id: 3,
            window_size: 16,
            seed: 0xCAFE,
            flags: FLAG_CELL_TELEMETRY | FLAG_MULTIPATH_PRIMARY,
            crc16: 0x1234,
            auth_tag: 0x5678,
        };

        let mut buf = [0u8; 16];
        hdr.write_bytes(&mut buf);

        let parsed = PacketHeader::from_bytes(&buf).unwrap();
        assert_eq!(hdr, parsed);
        assert!(parsed.is_systematic());
        assert!(!parsed.is_repair());
    }
}
