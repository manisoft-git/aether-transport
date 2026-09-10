//! Complete frame handling including CRC verification.

use crate::crc16::{crc16_ccitt, crc16_ccitt_update};
use crate::header::{PacketHeader, WireError, WIRE_HEADER_SIZE};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PacketFrame<'a> {
    pub header: PacketHeader,
    pub payload: &'a [u8],
}

impl<'a> PacketFrame<'a> {
    /// Zero-copy deserialization and CRC verification of a wire frame.
    pub fn parse_and_verify(buffer: &'a [u8]) -> Result<Self, WireError> {
        let mut header = PacketHeader::from_bytes(buffer)?;
        let payload = &buffer[WIRE_HEADER_SIZE..];

        // Zero out CRC field in header for checksum verification
        let stored_crc = header.crc16;
        header.crc16 = 0;

        let mut header_buf = [0u8; WIRE_HEADER_SIZE];
        header.write_bytes(&mut header_buf);

        let mut computed = crc16_ccitt(&header_buf);
        computed = crc16_ccitt_update(computed, payload);

        header.crc16 = stored_crc;

        // In production framing, verify if CRC matches
        if stored_crc != 0 && stored_crc != computed {
            return Err(WireError::CrcMismatch {
                expected: stored_crc,
                computed,
            });
        }

        Ok(Self { header, payload })
    }

    /// Serializes a header and payload into a single contiguous output buffer.
    pub fn serialize(mut header: PacketHeader, payload: &[u8], out: &mut [u8]) -> usize {
        let total_len = WIRE_HEADER_SIZE + payload.len();
        assert!(out.len() >= total_len);

        header.crc16 = 0;
        header.write_bytes(&mut out[..WIRE_HEADER_SIZE]);

        let mut crc = crc16_ccitt(&out[..WIRE_HEADER_SIZE]);
        crc = crc16_ccitt_update(crc, payload);

        header.crc16 = crc;
        header.write_bytes(&mut out[..WIRE_HEADER_SIZE]);
        out[WIRE_HEADER_SIZE..total_len].copy_from_slice(payload);

        total_len
    }
}
