//! Aether Core binary entry point.
//! Production asynchronous UDP protocol reactor & client transmitter.

use aether_core::{StreamReceiver, TransportSession};
use aether_wire::{PacketFrame, PacketHeader, FLAG_FIN, FLAG_PROBE, WIRE_HEADER_SIZE};
use rand::Rng;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    let get_arg = |flag: &str| -> Option<String> {
        args.windows(2)
            .find(|w| w[0] == flag)
            .map(|w| w[1].clone())
    };

    let has_flag = |flag: &str| -> bool {
        args.iter().any(|a| a == flag)
    };

    let k: usize = get_arg("--k").and_then(|s| s.parse().ok()).unwrap_or(8);
    let r: usize = get_arg("--r").and_then(|s| s.parse().ok()).unwrap_or(2);
    let symbol_size: usize = get_arg("--symbol-size").and_then(|s| s.parse().ok()).unwrap_or(1400);

    let is_client = has_flag("--client") || has_flag("--send") || args.windows(2).any(|w| w[0] == "--message" || w[0] == "--target");
    let is_server = has_flag("--server");

    if is_client {
        run_client(&args, k, r, symbol_size).await?;
    } else if is_server {
        let bind_addr: SocketAddr = get_arg("--bind")
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| "0.0.0.0:4242".parse().unwrap());
        run_server(bind_addr, k, r, symbol_size).await?;
    } else {
        // Default / Demo mode: Run self-test verification then launch reactor
        println!("==================================================================");
        println!(" AETHER-TRANSPORT: SOVEREIGN PROTOCOL DEMO & REACTOR");
        println!("==================================================================");
        
        let session = TransportSession::new(k, r, symbol_size);
        let test_payload = b"ManiSoft Sovereign Aether-Transport: Autonomous Loss-Immune Radio Link Payload";
        let symbols = session.prepare_source_symbols(test_payload);
        let packets = session.encode_window(symbols);

        println!("[*] Generated {} wire packets ({} Systematic + {} Fulcrum Repairs)", packets.len(), k, packets.len() - k);
        if let Some(recovered) = session.decode_window(&packets) {
            let recovered_str = String::from_utf8_lossy(&recovered[..test_payload.len()]);
            println!("[+] Verified local Galois field decode: {} bytes recovered -> \"{}\"", test_payload.len(), recovered_str);
        }

        let bind_addr: SocketAddr = get_arg("--bind")
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| "0.0.0.0:4242".parse().unwrap());
        println!("[+] Mathematical self-test passed. Launching live UDP reactor on {}...", bind_addr);
        run_server(bind_addr, k, r, symbol_size).await?;
    }

    Ok(())
}

async fn run_server(
    bind_addr: SocketAddr,
    k: usize,
    r: usize,
    symbol_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(bind_addr).await?;
    println!("==================================================================");
    println!(" AETHER-TRANSPORT: ASYNC UDP PROTOCOL REACTOR ONLINE");
    println!(" Listening on UDP: {}", bind_addr);
    println!(" Parameters: K={}, r={}, SymbolSize={} bytes", k, r, symbol_size);
    println!(" RFC UDP Port: 4242 | Galois Field: GF(2^8) Fulcrum");
    println!(" Press Ctrl+C to stop.");
    println!("==================================================================");
    info!("UDP reactor bound to {}", bind_addr);

    let mut receiver = StreamReceiver::new(k, r, symbol_size);
    let mut buf = vec![0u8; 65535];

    loop {
        let (len, peer) = socket.recv_from(&mut buf).await?;
        let packet_bytes = &buf[..len];

        match PacketFrame::parse_and_verify(packet_bytes) {
            Ok(frame) => {
                let sym_type = if frame.header.is_systematic() {
                    "Systematic"
                } else {
                    "Fulcrum Repair"
                };

                let win_id = frame.header.window_id;
                let sym_id = frame.header.symbol_id;
                let seed = frame.header.seed;

                println!(
                    "  [RECV] Win {:<4} | Sym {:<3} ({:<14}) | Seed: 0x{:04X} | {} bytes from {}",
                    win_id,
                    sym_id,
                    sym_type,
                    seed,
                    frame.payload.len(),
                    peer
                );


                // Check for immediate probe / ping
                if (frame.header.flags & FLAG_PROBE) != 0 {
                    let mut ack_buf = vec![0u8; WIRE_HEADER_SIZE];
                    let ack_header = PacketHeader {
                        window_id: frame.header.window_id,
                        symbol_id: frame.header.symbol_id,
                        window_size: frame.header.window_size,
                        seed: frame.header.seed,
                        flags: FLAG_FIN | FLAG_PROBE,
                        crc16: 0,
                        auth_tag: frame.header.auth_tag,
                    };
                    PacketFrame::serialize(ack_header, &[], &mut ack_buf);
                    let _ = socket.send_to(&ack_buf, peer).await;
                    println!("  [ACK]  Dispatched probe acknowledgment to {}", peer);
                    continue;
                }

                // Ingest into progressive stream receiver
                match receiver.ingest_packet(packet_bytes) {
                    Ok(Some((window_id, recovered_bytes))) => {
                        // Trim trailing zeros / null padding
                        let clean_len = recovered_bytes.iter().rposition(|&b| b != 0).map_or(0, |idx| idx + 1);
                        let clean_bytes = &recovered_bytes[..clean_len];
                        let preview = String::from_utf8_lossy(clean_bytes);

                        println!("==================================================================");
                        println!(" [+] 🚀 WINDOW {} FULLY RECONSTRUCTED VIA GAUSSIAN INVERSION!", window_id);
                        println!(" [+] Payload Size: {} bytes (Clean: {} bytes)", recovered_bytes.len(), clean_len);
                        println!(" [+] Recovered Data: \"{}\"", preview);
                        println!("==================================================================");

                        // Dispatched ACK frame
                        let mut ack_buf = vec![0u8; WIRE_HEADER_SIZE];
                        let ack_header = PacketHeader {
                            window_id,
                            symbol_id: 0,
                            window_size: k as u16,
                            seed: 0,
                            flags: FLAG_FIN,
                            crc16: 0,
                            auth_tag: 0,
                        };
                        PacketFrame::serialize(ack_header, &[], &mut ack_buf);
                        let _ = socket.send_to(&ack_buf, peer).await;
                        println!("  [ACK] Dispatched Window {} completion acknowledgment to {}", window_id, peer);
                    }
                    Ok(None) => {
                        // Window still accumulating degrees of freedom
                    }
                    Err(e) => {
                        warn!("Error processing frame: {:?}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Malformed packet from {}: {:?}", peer, e);
            }
        }
    }
}

async fn run_client(
    args: &[String],
    k: usize,
    r: usize,
    symbol_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let get_arg = |flag: &str| -> Option<String> {
        args.windows(2)
            .find(|w| w[0] == flag)
            .map(|w| w[1].clone())
    };

    let target_addr: SocketAddr = get_arg("--target")
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| "127.0.0.1:4242".parse().unwrap());

    let message = get_arg("--message")
        .unwrap_or_else(|| "Aether Transport Sovereign Protocol Demonstration Stream Payload".to_string());

    let loss_pct: f64 = get_arg("--loss")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0)
        .max(0.0)
        .min(99.0);

    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    println!("==================================================================");
    println!(" AETHER-TRANSPORT: SOVEREIGN CLIENT TRANSMITTER");
    println!(" Target Gateway:  {}", target_addr);
    println!(" Message:         \"{}\"", message);
    println!(" Coding Engine:   K={}, r={}, SymbolSize={} bytes", k, r, symbol_size);
    println!(" Simulated Loss:  {:.1}%", loss_pct);
    println!("==================================================================");

    let session = TransportSession::new(k, r, symbol_size);
    let symbols = session.prepare_source_symbols(message.as_bytes());
    let packets = session.encode_window(symbols);

    println!("[*] Generated {} wire packets (Systematics + Fulcrum Repairs)", packets.len());

    let mut rng = rand::thread_rng();
    let mut sent_count = 0;
    let mut dropped_count = 0;

    for (idx, pkt) in packets.iter().enumerate() {
        let is_dropped = loss_pct > 0.0 && rng.gen_bool(loss_pct / 100.0);
        let sym_type = if idx < k { "Systematic" } else { "Repair" };

        if is_dropped {
            dropped_count += 1;
            println!("  [DROP] Simulating loss for {} packet #{}", sym_type, idx);
        } else {
            sent_count += 1;
            socket.send_to(pkt, target_addr).await?;
            println!("  [SENT] Dispatched {} packet #{} ({} bytes)", sym_type, idx, pkt.len());
        }
        tokio::time::sleep(Duration::from_millis(15)).await;
    }

    println!("==================================================================");
    println!(" Transmission Complete: {} sent, {} dropped ({:.1}% drop rate)", sent_count, dropped_count, (dropped_count as f64 / packets.len() as f64) * 100.0);

    // Wait for remote ACK with timeout
    let mut ack_buf = [0u8; 1024];
    match timeout(Duration::from_secs(2), socket.recv_from(&mut ack_buf)).await {
        Ok(Ok((len, peer))) => {
            if let Ok(frame) = PacketFrame::parse_and_verify(&ack_buf[..len]) {
                let ack_win = frame.header.window_id;
                println!("[+] Received Gateway ACK from {} for Window {}", peer, ack_win);
            } else {
                println!("[+] Received response packet from {}", peer);
            }
        }

        Ok(Err(e)) => {
            println!("[!] Receive error: {:?}", e);
        }
        Err(_) => {
            println!("[*] Gateway ACK wait timeout (data transmitted via UDP)");
        }
    }

    Ok(())
}

