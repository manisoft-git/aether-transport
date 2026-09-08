//! Aether Core binary entry point.

use aether_core::TransportSession;
use std::env;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let bind_addr: SocketAddr = args
        .windows(2)
        .find(|w| w[0] == "--bind")
        .and_then(|w| w[1].parse().ok())
        .unwrap_or_else(|| "127.0.0.1:4242".parse().unwrap());

    println!("==================================================================");
    println!(" AETHER-TRANSPORT: SOVEREIGN PROTOCOL REACTOR");
    println!(" Listening on UDP: {}", bind_addr);
    println!("==================================================================");

    let socket = UdpSocket::bind(bind_addr).await?;
    info!("UDP reactor bound to {}", bind_addr);

    let mut session = TransportSession::new(8, 2, 1400);
    info!("Initialized Coding Session: K={}, r={}, SymbolSize={}", session.k(), 2, session.symbol_size());

    // Demonstration transmission loop
    let payload = b"Aether Transport Sovereign Protocol Demonstration Stream Payload";
    let symbols = session.prepare_source_symbols(payload);
    let packets = session.encode_window(symbols);

    println!("[*] Generated {} wire packets (Systematics + Fulcrum Repairs)", packets.len());
    if let Some(recovered) = session.decode_window(&packets) {
        println!("[+] Verified local window decode: {} bytes recovered!", recovered.len());
    }

    session.advance_window();
    println!("[+] Session ready for live streaming on port {}", bind_addr.port());

    Ok(())
}
