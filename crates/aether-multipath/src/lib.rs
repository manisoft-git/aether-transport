//! `aether-multipath`: Asymmetric delay-aware packet bonding across cellular paths.

pub mod scheduler;

pub use scheduler::{MultipathScheduler, NetworkPath};
