//! X25519 ephemeral key exchange and ChaCha20-Poly1305 AEAD session encryption.

use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use rand::rngs::OsRng;
use thiserror::Error;
use x25519_dalek::{EphemeralSecret, PublicKey};

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Cryptographic decryption failed: authentication tag mismatch or corrupted ciphertext")]
    DecryptionFailed,
    #[error("Handshake key exchange failed")]
    KeyExchangeFailed,
}

pub struct SessionCrypto {
    cipher: ChaCha20Poly1305,
    shared_secret: [u8; 32],
}

impl SessionCrypto {
    /// Generates an ephemeral X25519 keypair for initiating a handshake.
    pub fn generate_ephemeral() -> (EphemeralSecret, PublicKey) {
        let secret = EphemeralSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&secret);
        (secret, public)
    }

    /// Derives session cipher from local ephemeral secret and peer public key.
    pub fn from_key_exchange(local_secret: EphemeralSecret, peer_public: &PublicKey) -> Self {
        let shared = local_secret.diffie_hellman(peer_public);
        let shared_bytes: [u8; 32] = *shared.as_bytes();
        let key = Key::from_slice(&shared_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        Self {
            cipher,
            shared_secret: shared_bytes,
        }
    }

    pub fn shared_secret(&self) -> &[u8; 32] {
        &self.shared_secret
    }

    /// Encrypts plaintext using ChaCha20-Poly1305 with a 96-bit nonce.
    pub fn encrypt(&self, nonce_counter: u64, plaintext: &[u8]) -> Vec<u8> {
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[0..8].copy_from_slice(&nonce_counter.to_le_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);

        self.cipher
            .encrypt(nonce, plaintext)
            .expect("Encryption failure")
    }

    /// Decrypts ciphertext using ChaCha20-Poly1305.
    pub fn decrypt(&self, nonce_counter: u64, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[0..8].copy_from_slice(&nonce_counter.to_le_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);

        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x25519_chacha20_handshake_roundtrip() {
        // Alice generates keypair
        let (alice_secret, alice_public) = SessionCrypto::generate_ephemeral();
        // Bob generates keypair
        let (bob_secret, bob_public) = SessionCrypto::generate_ephemeral();

        let alice_session = SessionCrypto::from_key_exchange(alice_secret, &bob_public);
        let bob_session = SessionCrypto::from_key_exchange(bob_secret, &alice_public);

        assert_eq!(alice_session.shared_secret(), bob_session.shared_secret());

        let msg = b"Aether Protocol Session Handshake Secure Payload";
        let encrypted = alice_session.encrypt(1, msg);
        let decrypted = bob_session.decrypt(1, &encrypted).unwrap();

        assert_eq!(msg.to_vec(), decrypted);
    }
}
