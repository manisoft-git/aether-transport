//! `aether-tun`: Userspace TUN device driver for transparent IP tunneling.

pub mod tun;

pub use tun::{TunDevice, TunError};
