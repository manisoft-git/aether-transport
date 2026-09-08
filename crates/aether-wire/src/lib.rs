//! `aether-wire`: Zero-copy network framing and CRC16 routines.

pub mod crc16;
pub mod frame;
pub mod header;

pub use crc16::crc16_ccitt;
pub use frame::PacketFrame;
pub use header::{
    PacketHeader, WireError, FLAG_CELL_TELEMETRY, FLAG_FIN, FLAG_MULTIPATH_PRIMARY,
    FLAG_MULTIPATH_SECONDARY, FLAG_PROBE, WIRE_HEADER_SIZE,
};
