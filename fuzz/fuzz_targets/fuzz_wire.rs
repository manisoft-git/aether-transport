#![no_main]
use libfuzzer_sys::fuzz_target;
use aether_wire::PacketHeader;

fuzz_target!(|data: &[u8]| {
    // Attempt zero-copy parsing and verify absence of panics / memory violations
    if let Ok(header) = PacketHeader::from_bytes(data) {
        let mut buf = [0u8; 16];
        header.write_bytes(&mut buf);
        let parsed_back = PacketHeader::from_bytes(&buf).unwrap();
        assert_eq!(header.window_id, parsed_back.window_id);
        assert_eq!(header.symbol_id, parsed_back.symbol_id);
    }
});
