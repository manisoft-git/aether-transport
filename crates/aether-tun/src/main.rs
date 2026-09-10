//! Aether TUN daemon binary.
//! Transparent OS packet capture and Fulcrum wireless tunneling daemon.

use aether_core::TransportSession;
use aether_tun::TunDevice;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

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

    println!("==================================================================");
    println!(" AETHER-TUN: VIRTUAL OS PACKET TUNNEL DAEMON");
    println!(" Virtual Interface:    {}", tun_name);
    println!(" Remote Core Gateway:  {}", remote_addr);
    println!(" Local UDP Bind:       {}", bind_addr);
    println!(" Coding Engine:        K={}, r={}, SymbolSize={} bytes", k, r, symbol_size);
    println!("==================================================================");

    let mut tun = match TunDevice::create(&tun_name) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[!] Failed to initialize TUN device: {:?}", e);
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    };

    let socket = UdpSocket::bind(bind_addr).await?;
    info!("Tunnel UDP socket bound to {}", bind_addr);

    if tun.is_fallback() {
        println!("[!] Running in userspace virtual queue fallback mode.");
        println!("[i] Note: To attach to the physical Linux kernel network stack, run with root privileges:");
        println!("    sudo cargo run --release -p aether-tun -- --name {} --remote {}", tun_name, remote_addr);
        println!("==================================================================");
        println!("[+] Injecting synthetic IP test packets across virtual tunnel to gateway...");

        let session = TransportSession::new(k, r, symbol_size);
        let mut ping_seq = 0u32;

        loop {
            ping_seq += 1;
            let mock_ip_packet = format!("AETHER_IP_PACKET_SEQ_{}_DATA_STREAM_PAYLOAD", ping_seq);
            let symbols = session.prepare_source_symbols(mock_ip_packet.as_bytes());
            let packets = session.encode_window(symbols);

            for pkt in &packets {
                let _ = socket.send_to(pkt, remote_addr).await;
            }

            println!("  [TUN-TX] Dispatched Window {} ({} packets) via virtual tunnel to {}", ping_seq, packets.len(), remote_addr);
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    } else {
        println!("[+] Real Linux TUN interface '{}' operational!", tun_name);
        println!("[+] Route traffic with: ip route add default dev {}", tun_name);

        let mut read_buf = vec![0u8; symbol_size * k];
        let session = TransportSession::new(k, r, symbol_size);

        loop {
            match tun.read_packet(&mut read_buf) {
                Ok(n) if n > 0 => {
                    let symbols = session.prepare_source_symbols(&read_buf[..n]);
                    let packets = session.encode_window(symbols);
                    for pkt in packets {
                        let _ = socket.send_to(&pkt, remote_addr).await;
                    }
                }
                _ => {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}

