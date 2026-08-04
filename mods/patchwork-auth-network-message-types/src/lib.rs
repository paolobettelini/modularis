use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyExchangeRequest {
    pub protocol_version: u16,
    pub handshake_id: String,
    pub server_id: String,
    pub server_public_key: [u8; 32],
    pub server_nonce: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyExchangeResponse {
    pub handshake_id: String,
    pub client_public_key: [u8; 32],
    pub client_nonce: [u8; 32],
    pub handshake_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientFinish {
    pub handshake_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginSuccess {
    pub player_session_id: String,
    pub account_uuid: String,
    pub nickname: String,
    pub admission: String,
    pub source_server_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthenticationFailed {
    pub reason: String,
}
