//! Linux TUN virtual device driver interface.

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Error, Debug)]
pub enum TunError {
    #[error("Failed to open /dev/net/tun: {0}")]
    OpenFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tun device operation failed")]
    IoctlFailed,
}

pub struct TunDevice {
    name: String,
    file: Option<File>,
    is_virtual_fallback: bool,
    virtual_queue: std::collections::VecDeque<Vec<u8>>,
}

impl TunDevice {
    /// Creates or attaches to a TUN device named `name` (e.g. "aether0").
    /// Falls back to a virtual user-queue if permissions are insufficient.
    pub fn create(name: &str) -> Result<Self, TunError> {
        let tun_path = "/dev/net/tun";

        let file_res = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(tun_path);

        match file_res {
            Ok(file) => {
                let mut ifr: libc::ifreq = unsafe { std::mem::zeroed() };
                let ifr_name_slice = &mut ifr.ifr_name[..name.len().min(libc::IFNAMSIZ - 1)];
                for (dst, src) in ifr_name_slice.iter_mut().zip(name.as_bytes()) {
                    *dst = *src as libc::c_char;
                }

                // Flags: IFF_TUN (IP packets, no Ethernet header) + IFF_NO_PI (no packet information prefix)
                ifr.ifr_ifru.ifru_flags = (libc::IFF_TUN | libc::IFF_NO_PI) as libc::c_short;

                let fd = file.as_raw_fd();
                let res = unsafe { libc::ioctl(fd, 0x400454ca, &ifr) }; // TUNSETIFF ioctl

                if res < 0 {
                    warn!("TUNSETIFF ioctl failed (err: {}). Falling back to virtual queue mode", res);
                    Ok(Self::new_virtual(name))
                } else {
                    info!("Successfully created real Linux TUN interface '{}'", name);
                    Ok(Self {
                        name: name.to_string(),
                        file: Some(file),
                        is_virtual_fallback: false,
                        virtual_queue: std::collections::VecDeque::new(),
                    })
                }
            }
            Err(e) => {
                warn!("Cannot open {} ({}). Using virtual user-queue fallback mode", tun_path, e);
                Ok(Self::new_virtual(name))
            }
        }
    }

    fn new_virtual(name: &str) -> Self {
        Self {
            name: name.to_string(),
            file: None,
            is_virtual_fallback: true,
            virtual_queue: std::collections::VecDeque::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_fallback(&self) -> bool {
        self.is_virtual_fallback
    }

    /// Reads an IP packet from the TUN interface.
    pub fn read_packet(&mut self, buf: &mut [u8]) -> Result<usize, TunError> {
        if let Some(ref mut file) = self.file {
            let n = file.read(buf)?;
            Ok(n)
        } else {
            if let Some(pkt) = self.virtual_queue.pop_front() {
                let len = pkt.len().min(buf.len());
                buf[..len].copy_from_slice(&pkt[..len]);
                Ok(len)
            } else {
                Err(TunError::Io(std::io::Error::new(
                    std::io::ErrorKind::WouldBlock,
                    "Virtual TUN queue empty",
                )))
            }
        }
    }

    /// Writes an IP packet to the TUN interface.
    pub fn write_packet(&mut self, buf: &[u8]) -> Result<usize, TunError> {
        if let Some(ref mut file) = self.file {
            let n = file.write(buf)?;
            Ok(n)
        } else {
            self.virtual_queue.push_back(buf.to_vec());
            Ok(buf.len())
        }
    }
}
