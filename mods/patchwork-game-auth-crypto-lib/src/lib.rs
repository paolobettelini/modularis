use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, Payload},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hkdf::Hkdf;
use rand_core::OsRng;
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;
use x25519_dalek::{PublicKey, StaticSecret};

pub const AUTH_PROTOCOL_VERSION: u16 = 1;
pub const TRANSCRIPT_DOMAIN: &[u8] = b"patchwork-game-handshake-v1";

const C2S_KEY_INFO: &[u8] = b"patchwork-c2s-key-v1";
const S2C_KEY_INFO: &[u8] = b"patchwork-s2c-key-v1";
const C2S_IV_INFO: &[u8] = b"patchwork-c2s-iv-v1";
const S2C_IV_INFO: &[u8] = b"patchwork-s2c-iv-v1";
const GCM_TAG_BYTES: usize = 16;

#[derive(Debug, Error)]
pub enum AuthCryptoError {
    #[error("invalid UUID in authentication transcript: {0}")]
    InvalidUuid(#[from] uuid::Error),
    #[error("server id is too long for the authentication transcript")]
    ServerIdTooLong,
    #[error("X25519 produced the forbidden all-zero shared secret")]
    AllZeroSharedSecret,
    #[error("HKDF could not derive authentication key material")]
    Hkdf,
    #[error("authentication sequence counter is exhausted")]
    SequenceExhausted,
    #[error("encrypted authentication frame is malformed")]
    InvalidCiphertext,
    #[error("AES-GCM authentication failed")]
    AuthenticationFailed,
}

/// One-use X25519 private key. Its debug representation never exposes bytes.
pub struct EphemeralPrivateKey(StaticSecret);

impl std::fmt::Debug for EphemeralPrivateKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EphemeralPrivateKey([REDACTED])")
    }
}

impl EphemeralPrivateKey {
    pub fn generate() -> (Self, [u8; 32]) {
        let private = StaticSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&private).to_bytes();
        (Self(private), public)
    }

    pub fn agree(&self, peer_public: [u8; 32]) -> Result<[u8; 32], AuthCryptoError> {
        let shared = self
            .0
            .diffie_hellman(&PublicKey::from(peer_public))
            .to_bytes();
        if shared.iter().all(|byte| *byte == 0) {
            return Err(AuthCryptoError::AllZeroSharedSecret);
        }
        Ok(shared)
    }
}

pub fn random_nonce_32() -> [u8; 32] {
    use rand_core::RngCore;

    let mut nonce = [0_u8; 32];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn canonical_transcript(
    protocol_version: u16,
    handshake_id: &str,
    server_id: &str,
    server_public_key: &[u8; 32],
    client_public_key: &[u8; 32],
    server_nonce: &[u8; 32],
    client_nonce: &[u8; 32],
) -> Result<Vec<u8>, AuthCryptoError> {
    let handshake_id = Uuid::parse_str(handshake_id)?;
    let server_id_bytes = server_id.as_bytes();
    let server_id_len =
        u16::try_from(server_id_bytes.len()).map_err(|_| AuthCryptoError::ServerIdTooLong)?;
    let mut transcript =
        Vec::with_capacity(TRANSCRIPT_DOMAIN.len() + 2 + 16 + 2 + server_id_bytes.len() + 32 * 4);
    transcript.extend_from_slice(TRANSCRIPT_DOMAIN);
    transcript.extend_from_slice(&protocol_version.to_be_bytes());
    transcript.extend_from_slice(handshake_id.as_bytes());
    transcript.extend_from_slice(&server_id_len.to_be_bytes());
    transcript.extend_from_slice(server_id_bytes);
    transcript.extend_from_slice(server_public_key);
    transcript.extend_from_slice(client_public_key);
    transcript.extend_from_slice(server_nonce);
    transcript.extend_from_slice(client_nonce);
    Ok(transcript)
}

pub fn transcript_hash(transcript: &[u8]) -> [u8; 32] {
    Sha256::digest(transcript).into()
}

pub fn constant_time_eq_32(left: &[u8; 32], right: &[u8; 32]) -> bool {
    left.iter()
        .zip(right)
        .fold(0_u8, |difference, (left, right)| {
            difference | (left ^ right)
        })
        == 0
}

pub fn encode_base64_32(value: &[u8; 32]) -> String {
    URL_SAFE_NO_PAD.encode(value)
}

pub fn decode_base64_32(value: &str) -> Result<[u8; 32], AuthCryptoError> {
    let decoded = URL_SAFE_NO_PAD
        .decode(value)
        .map_err(|_| AuthCryptoError::InvalidCiphertext)?;
    decoded
        .try_into()
        .map_err(|_| AuthCryptoError::InvalidCiphertext)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionRole {
    Client,
    Server,
}

#[derive(Clone, Copy)]
struct KeyMaterial {
    c2s_key: [u8; 32],
    s2c_key: [u8; 32],
    c2s_iv: [u8; 12],
    s2c_iv: [u8; 12],
}

impl KeyMaterial {
    fn derive(
        shared_secret: &[u8; 32],
        transcript_hash: &[u8; 32],
    ) -> Result<Self, AuthCryptoError> {
        let hkdf = Hkdf::<Sha256>::new(Some(transcript_hash), shared_secret);
        let mut material = Self {
            c2s_key: [0; 32],
            s2c_key: [0; 32],
            c2s_iv: [0; 12],
            s2c_iv: [0; 12],
        };
        hkdf.expand(C2S_KEY_INFO, &mut material.c2s_key)
            .map_err(|_| AuthCryptoError::Hkdf)?;
        hkdf.expand(S2C_KEY_INFO, &mut material.s2c_key)
            .map_err(|_| AuthCryptoError::Hkdf)?;
        hkdf.expand(C2S_IV_INFO, &mut material.c2s_iv)
            .map_err(|_| AuthCryptoError::Hkdf)?;
        hkdf.expand(S2C_IV_INFO, &mut material.s2c_iv)
            .map_err(|_| AuthCryptoError::Hkdf)?;
        Ok(material)
    }
}

struct DirectionalCipher {
    key: [u8; 32],
    base_iv: [u8; 12],
    direction: u8,
    sequence: u64,
}

impl DirectionalCipher {
    fn new(key: [u8; 32], base_iv: [u8; 12], direction: u8) -> Self {
        Self {
            key,
            base_iv,
            direction,
            sequence: 0,
        }
    }

    fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, AuthCryptoError> {
        self.ensure_sequence_available()?;
        let ciphertext_length = plaintext
            .len()
            .checked_add(GCM_TAG_BYTES)
            .and_then(|length| u32::try_from(length).ok())
            .ok_or(AuthCryptoError::InvalidCiphertext)?;
        let aad = aad(
            AUTH_PROTOCOL_VERSION,
            self.direction,
            self.sequence,
            ciphertext_length,
        );
        let nonce = nonce(self.base_iv, self.sequence);
        let cipher =
            Aes256Gcm::new_from_slice(&self.key).map_err(|_| AuthCryptoError::InvalidCiphertext)?;
        let encrypted = cipher
            .encrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: plaintext,
                    aad: &aad,
                },
            )
            .map_err(|_| AuthCryptoError::AuthenticationFailed)?;
        self.sequence += 1;
        Ok(encrypted)
    }

    fn decrypt_candidate(&self, ciphertext: &[u8]) -> Result<Vec<u8>, AuthCryptoError> {
        self.ensure_sequence_available()?;
        let ciphertext_length =
            u32::try_from(ciphertext.len()).map_err(|_| AuthCryptoError::InvalidCiphertext)?;
        if ciphertext.len() < GCM_TAG_BYTES {
            return Err(AuthCryptoError::InvalidCiphertext);
        }
        let aad = aad(
            AUTH_PROTOCOL_VERSION,
            self.direction,
            self.sequence,
            ciphertext_length,
        );
        let nonce = nonce(self.base_iv, self.sequence);
        let cipher =
            Aes256Gcm::new_from_slice(&self.key).map_err(|_| AuthCryptoError::InvalidCiphertext)?;
        cipher
            .decrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: ciphertext,
                    aad: &aad,
                },
            )
            .map_err(|_| AuthCryptoError::AuthenticationFailed)
    }

    fn commit(&mut self) -> Result<(), AuthCryptoError> {
        self.ensure_sequence_available()?;
        self.sequence += 1;
        Ok(())
    }

    fn ensure_sequence_available(&self) -> Result<(), AuthCryptoError> {
        if self.sequence == u64::MAX {
            Err(AuthCryptoError::SequenceExhausted)
        } else {
            Ok(())
        }
    }
}

/// Per-connection directional AES state. It must never be reused for another
/// connection, including a transfer to another server.
pub struct SecureChannel {
    outbound: DirectionalCipher,
    inbound: DirectionalCipher,
}

impl std::fmt::Debug for SecureChannel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SecureChannel")
            .field("outbound_sequence", &self.outbound.sequence)
            .field("inbound_sequence", &self.inbound.sequence)
            .finish_non_exhaustive()
    }
}

impl SecureChannel {
    pub fn derive(
        role: ConnectionRole,
        shared_secret: &[u8; 32],
        transcript_hash: &[u8; 32],
    ) -> Result<Self, AuthCryptoError> {
        let material = KeyMaterial::derive(shared_secret, transcript_hash)?;
        let (outbound, inbound) = match role {
            ConnectionRole::Client => (
                DirectionalCipher::new(material.c2s_key, material.c2s_iv, 0),
                DirectionalCipher::new(material.s2c_key, material.s2c_iv, 1),
            ),
            ConnectionRole::Server => (
                DirectionalCipher::new(material.s2c_key, material.s2c_iv, 1),
                DirectionalCipher::new(material.c2s_key, material.c2s_iv, 0),
            ),
        };
        Ok(Self { outbound, inbound })
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, AuthCryptoError> {
        self.outbound.encrypt(plaintext)
    }

    /// Decrypt without advancing the receive counter. Call `commit_inbound`
    /// only after the plaintext has also been decoded and accepted.
    pub fn decrypt_candidate(&self, ciphertext: &[u8]) -> Result<Vec<u8>, AuthCryptoError> {
        self.inbound.decrypt_candidate(ciphertext)
    }

    pub fn commit_inbound(&mut self) -> Result<(), AuthCryptoError> {
        self.inbound.commit()
    }
}

fn nonce(base_iv: [u8; 12], sequence: u64) -> [u8; 12] {
    let mut encoded = [0_u8; 12];
    encoded[4..].copy_from_slice(&sequence.to_be_bytes());
    let mut nonce = base_iv;
    for (nonce, sequence) in nonce.iter_mut().zip(encoded) {
        *nonce ^= sequence;
    }
    nonce
}

fn aad(protocol_version: u16, direction: u8, sequence: u64, length: u32) -> [u8; 15] {
    let mut aad = [0_u8; 15];
    aad[..2].copy_from_slice(&protocol_version.to_be_bytes());
    aad[2] = direction;
    aad[3..11].copy_from_slice(&sequence.to_be_bytes());
    aad[11..].copy_from_slice(&length.to_be_bytes());
    aad
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_and_server_derive_compatible_directional_channels() {
        let (client_private, client_public) = EphemeralPrivateKey::generate();
        let (server_private, server_public) = EphemeralPrivateKey::generate();
        let client_shared = client_private.agree(server_public).unwrap();
        let server_shared = server_private.agree(client_public).unwrap();
        assert_eq!(client_shared, server_shared);

        let hash = [7_u8; 32];
        let mut client =
            SecureChannel::derive(ConnectionRole::Client, &client_shared, &hash).unwrap();
        let mut server =
            SecureChannel::derive(ConnectionRole::Server, &server_shared, &hash).unwrap();

        let ciphertext = client.encrypt(b"hello").unwrap();
        assert_eq!(server.decrypt_candidate(&ciphertext).unwrap(), b"hello");
        server.commit_inbound().unwrap();

        let reply = server.encrypt(b"world").unwrap();
        assert_eq!(client.decrypt_candidate(&reply).unwrap(), b"world");
        client.commit_inbound().unwrap();
    }
}
