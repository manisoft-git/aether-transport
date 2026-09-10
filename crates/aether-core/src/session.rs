//! Transport session state machine and window coordinator.

use aether_coder::{FulcrumDecoder, FulcrumEncoder};
use aether_congestion::CongestionArbiter;
use aether_crypto::SubspaceMac;
use aether_wire::{PacketFrame, PacketHeader, WIRE_HEADER_SIZE};
use std::time::Duration;
use tracing::info;

#[derive(Debug)]
pub struct TransportSession {
    k: usize,
    r: usize,
    symbol_size: usize,
    current_window_id: u32,
    arbiter: CongestionArbiter,
    subspace_mac: SubspaceMac,
}

impl TransportSession {
    pub fn new(k: usize, r: usize, symbol_size: usize) -> Self {
        Self {
            k,
            r,
            symbol_size,
            current_window_id: 1,
            arbiter: CongestionArbiter::new(Duration::from_millis(50), 50.0),
            subspace_mac: SubspaceMac::new(b"aether-transport-sovereign-key-32"),
        }
    }

    pub fn window_id(&self) -> u32 {
        self.current_window_id
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn symbol_size(&self) -> usize {
        self.symbol_size
    }

    /// Splits input data into K source symbols.
    pub fn prepare_source_symbols(&self, data: &[u8]) -> Vec<Vec<u8>> {
        let mut symbols = Vec::with_capacity(self.k);
        let mut offset = 0;

        for _ in 0..self.k {
            let mut sym = vec![0u8; self.symbol_size];
            if offset < data.len() {
                let to_copy = (data.len() - offset).min(self.symbol_size);
                sym[..to_copy].copy_from_slice(&data[offset..offset + to_copy]);
                offset += to_copy;
            }
            symbols.push(sym);
        }
        symbols
    }

    /// Encodes a window of source symbols into wire packets (systematics + repairs).
    pub fn encode_window(&self, symbols: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let encoder = FulcrumEncoder::new(symbols, self.r);
        let mut packets = Vec::new();

        // 1. Systematic Packets
        for i in 0..self.k {
            let payload = encoder.get_systematic(i).unwrap();
            let auth_tag = self.subspace_mac.compute_tag(payload);

            let header = PacketHeader {
                window_id: self.current_window_id,
                symbol_id: i as u16,
                window_size: self.k as u16,
                seed: 0,
                flags: 0,
                crc16: 0,
                auth_tag,
            };

            let mut out = vec![0u8; WIRE_HEADER_SIZE + self.symbol_size];
            PacketFrame::serialize(header, payload, &mut out);
            packets.push(out);
        }

        // 2. Proactive Fulcrum Repair Packets based on arbiter redundancy ratio
        let num_repairs = self.r.max((self.k as f64 * self.arbiter.redundancy_ratio()).ceil() as usize);
        for rep_idx in 0..num_repairs {
            let seed = (self.current_window_id as u16).wrapping_mul(101) + (rep_idx as u16) + 1;
            let mut payload = vec![0u8; self.symbol_size];
            encoder.generate_repair(seed, &mut payload);
            let auth_tag = self.subspace_mac.compute_tag(&payload);

            let header = PacketHeader {
                window_id: self.current_window_id,
                symbol_id: (self.k + rep_idx) as u16,
                window_size: self.k as u16,
                seed,
                flags: 0,
                crc16: 0,
                auth_tag,
            };

            let mut out = vec![0u8; WIRE_HEADER_SIZE + self.symbol_size];
            PacketFrame::serialize(header, &payload, &mut out);
            packets.push(out);
        }

        packets
    }

    /// Ingests incoming wire packets into the decoder.
    pub fn decode_window(&self, packets: &[Vec<u8>]) -> Option<Vec<u8>> {
        let mut decoder = FulcrumDecoder::new(self.k, self.r, self.symbol_size);

        for pkt in packets {
            if let Ok(frame) = PacketFrame::parse_and_verify(pkt) {
                if frame.header.is_systematic() {
                    decoder.ingest_systematic(frame.header.symbol_id as usize, frame.payload);
                } else {
                    decoder.ingest_repair(frame.header.seed, frame.payload);
                }

                if decoder.is_complete() {
                    if let Ok(recovered) = decoder.reconstruct() {
                        let mut flattened = Vec::with_capacity(self.k * self.symbol_size);
                        for sym in recovered {
                            flattened.extend_from_slice(&sym);
                        }
                        return Some(flattened);
                    }
                }
            }
        }
        None
    }

    pub fn advance_window(&mut self) {
        self.current_window_id += 1;
        info!("Advanced to Window ID: {}", self.current_window_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_session_roundtrip() {
        let session = TransportSession::new(4, 0, 64);
        let test_data = b"Hello, Aether Sovereign Sub-IP Transport Protocol running on Rust!";

        let symbols = session.prepare_source_symbols(test_data);
        let wire_packets = session.encode_window(symbols);

        // Receive 4 packets with drops (2 systematic + 2 repair packets)
        let session_with_repairs = TransportSession::new(4, 2, 64);
        let packets_with_repairs = session_with_repairs.encode_window(session_with_repairs.prepare_source_symbols(test_data));

        // Test systematic fast-path decode
        let systematic_received = vec![wire_packets[0].clone(), wire_packets[1].clone(), wire_packets[2].clone(), wire_packets[3].clone()];
        let decoded = session.decode_window(&systematic_received).expect("Decoding systematics should succeed");
        assert_eq!(&decoded[..test_data.len()], test_data);

        // Test repair recovery with full intermediate rank (4 source + 2 repairs = 6 packets)
        let decoded_repair = session_with_repairs.decode_window(&packets_with_repairs).expect("Decoding window should succeed");
        assert_eq!(&decoded_repair[..test_data.len()], test_data);
    }
}
