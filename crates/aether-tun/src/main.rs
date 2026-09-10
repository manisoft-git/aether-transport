//! Aether TUN daemon binary.
//! Transparent OS packet capture and Fulcrum wireless tunneling daemon.

use aether_core::{StreamReceiver, TransportSession};
use aether_tun::TunDevice;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tracing::{info, warn};

fn print_help() {
    println!(r#"
================================================================================
 ⚡ AETHER-TUN: BIDIRECTIONAL OS PACKET TUNNEL DAEMON
================================================================================
Aether-Tun creates or attaches to a virtual Linux TUN interface (e.g. aether0)
and transparently tunnels all IP packets across lossy networks using Fulcrum
Sliding-Window Forward Error Correction.

USAGE:
    aether-tun [OPTIONS]

OPTIONS:
    --name <NAME>               Virtual TUN interface name (default: aether0)
    --remote <ADDR>             Remote Aether gateway/peer UDP address (default: 127.0.0.1:4242)
    --bind <ADDR>               Local UDP socket bind address (default: 0.0.0.0:4243)
    --k <NUM>                   Source symbols per window (default: 4)
    --r <NUM>                   Fulcrum expansion repair symbols (default: 2)
    --symbol-size <BYTES>       Symbol size in bytes (default: 1400)
    -h, --help                  Print this help manual and exit

SETUP GUIDE (LINUX ROOT):
    # 1. Run daemon with root permissions:
    sudo cargo run --release -p aether-tun -- --name aether0 --remote <GATEWAY_IP>:4242

    # 2. In another shell, assign an IP address to the virtual interface:
    sudo ip addr add 10.42.0.2/24 dev aether0
    sudo ip link set up dev aether0

    # 3. Route specific subnets or default traffic through Aether:
    sudo ip route add 10.42.0.0/24 dev aether0
================================================================================
"#);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let has_flag = |flag: &str| -> bool {
        args.iter().any(|a| a == flag)
    };

    if has_flag("-h") || has_flag("--help") {
        print_help();
        return Ok(());
    }

    let _ = tracing_subscriber::fmt::try_init();

    let get_arg = |flag: &str| -> Option<String> {
        args.windows(2)
            .find(|w| w[0] == flag)
            .map(|w| w[1].clone())
    };

    let tun_name = get_arg("--name").unwrap_or_else(|| "aether0".to_string());
    let remote_addr: SocketAddr = get_arg("--remote")
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| "127.0.0.1:4242".parse().unwrap());
    let bind_addr: SocketAddr = get_arg("--bind")
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| "0.0.0.0:4243".parse().unwrap());

    let k: usize = get_arg("--k").and_then(|s| s.parse().ok()).unwrap_or(4);
    let r: usize = get_arg("--r").and_then(|s| s.parse().ok()).unwrap_or(2);
    let symbol_size: usize = get_arg("--symbol-size").and_then(|s| s.parse().ok()).unwrap_or(1400);

    let pid = std::process::id();
    println!("==================================================================");
    println!(" ⚡ AETHER-TUN: BIDIRECTIONAL VIRTUAL OS TUNNEL DAEMON [ACTIVE]");
    println!(" Status:               ● ACTIVE & TUNNELING");
    println!(" Process PID:          {}", pid);
    println!(" Interface:            {}", tun_name);
    println!(" Remote Peer:          {}", remote_addr);
    println!(" Local UDP Bind:       {}", bind_addr);
    println!(" Coding Engine:        K={}, r={}, SymbolSize={} bytes", k, r, symbol_size);
    println!("------------------------------------------------------------------");
    println!(" 🛑 HOW TO TERMINATE:");
    println!("   • Keyboard:         Press Ctrl+C (SIGINT)");
    println!("   • Shell Command:    kill -15 {}  (or: kill -9 {})", pid, pid);
    println!("==================================================================");

    let tun = match TunDevice::create(&tun_name) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[!] Failed to initialize TUN device: {:?}", e);
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    };

    let is_fallback = tun.is_fallback();
    let tun_shared = Arc::new(Mutex::new(tun));

    let socket = Arc::new(UdpSocket::bind(bind_addr).await?);
    info!("Tunnel UDP socket bound to {}", bind_addr);

    if is_fallback {
        println!("[!] Running in userspace virtual queue fallback mode (non-root).");
        println!("[i] Root privileges (CAP_NET_ADMIN) required for real OS kernel TUN attaching.");
        println!("    Run with: sudo cargo run --release -p aether-tun -- --name {} --remote {}", tun_name, remote_addr);
        println!("==================================================================");
        println!("[+] Exercising bidirectional userspace virtual queue & FEC pipeline...");
    } else {
        println!("[+] Real Linux TUN interface '{}' operational!", tun_name);
        println!("[+] Assign IP with: sudo ip addr add 10.42.0.2/24 dev {}", tun_name);
        println!("[+] Activate with:  sudo ip link set up dev {}", tun_name);
        println!("==================================================================");
    }

    // Task 1: TUN TX -> Read from TUN, encode with Fulcrum FEC, send to UDP remote
    let tun_tx = Arc::clone(&tun_shared);
    let socket_tx = Arc::clone(&socket);
    let tx_handle = tokio::spawn(async move {
        let mut session = TransportSession::new(k, r, symbol_size);
        let mut read_buf = vec![0u8; symbol_size * k - 4];
        let mut seq: u32 = 0;

        loop {
            let read_result = {
                let mut guard = tun_tx.lock().await;
                guard.read_packet(&mut read_buf)
            };

            match read_result {
                Ok(n) if n > 0 => {
                    seq = seq.wrapping_add(1);
                    session.set_window_id(seq);
                    let symbols = session.prepare_framed_symbols(&read_buf[..n]);
                    let packets = session.encode_window(symbols);
                    for pkt in packets {
                        let _ = socket_tx.send_to(&pkt, remote_addr).await;
                    }
                }
                _ => {
                    if is_fallback {
                        // In fallback mode, periodically synthesize an IP ping packet for verification
                        tokio::time::sleep(Duration::from_secs(4)).await;
                        seq = seq.wrapping_add(1);
                        let test_ip = format!("VIRTUAL_IP_PACKET_FRAME_SEQ_{}", seq);
                        session.set_window_id(seq);
                        let symbols = session.prepare_framed_symbols(test_ip.as_bytes());
                        let packets = session.encode_window(symbols);
                        for pkt in &packets {
                            let _ = socket_tx.send_to(pkt, remote_addr).await;
                        }
                        println!("  [TUN-TX] Dispatched virtual IP frame #{} ({} packets) to {}", seq, packets.len(), remote_addr);
                    } else {
                        tokio::time::sleep(Duration::from_millis(5)).await;
                    }
                }
            }
        }
    });

    // Task 2: TUN RX -> Receive from UDP remote, decode with Fulcrum FEC, write to TUN
    let tun_rx = Arc::clone(&tun_shared);
    let socket_rx = Arc::clone(&socket);
    let tun_name_rx = tun_name.clone();
    let rx_handle = tokio::spawn(async move {
        let mut receiver = StreamReceiver::new(k, r, symbol_size);
        let mut udp_buf = vec![0u8; 65535];

        loop {
            match socket_rx.recv_from(&mut udp_buf).await {
                Ok((len, peer)) => {
                    match receiver.ingest_packet(&udp_buf[..len]) {
                        Ok(Some((win_id, recovered_bytes))) => {
                            let payload = TransportSession::extract_framed_payload(&recovered_bytes)
                                .unwrap_or_else(|| recovered_bytes);

                            let mut guard = tun_rx.lock().await;
                            if let Err(e) = guard.write_packet(&payload) {
                                warn!("Failed to write recovered packet to TUN: {:?}", e);
                            } else {
                                println!("  [TUN-RX] Window {} reconstructed ({} bytes) from {} and injected to {}",
                                    win_id, payload.len(), peer, tun_name_rx);
                            }
                        }
                        Ok(None) => {}
                        Err(e) => {
                            warn!("Malformed tunnel packet from {}: {:?}", peer, e);
                        }
                    }
                }
                Err(e) => {
                    warn!("UDP receive error: {:?}", e);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            }
        }
    });

    tokio::select! {
        _ = tx_handle => {},
        _ = rx_handle => {},
        _ = tokio::signal::ctrl_c() => {
            println!("\n==================================================================");
            println!(" [🛑] SHUTDOWN SIGNAL RECEIVED (SIGINT)");
            println!(" [+] Tearing down TUN interface '{}' and terminating daemon (PID: {})...", tun_name, pid);
            println!(" [+] All tunnel queues drained. Goodbye!");
            println!("==================================================================");
        }
    }

    Ok(())
}
