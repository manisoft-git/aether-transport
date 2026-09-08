//! `aether-crypto`: Byzantine pollution defense via homomorphic MACs and session encryption.

pub mod handshake;
pub mod subspace;

pub use handshake::{CryptoError, SessionCrypto};
pub use subspace::SubspaceMac;
