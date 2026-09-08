//! `aether-xdp`: eBPF/XDP kernel-bypass driver and userspace interface.

pub mod loader;

pub use loader::{XdpEngine, XdpError};
