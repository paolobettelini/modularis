use serde::{Deserialize, Serialize};
use thecrown_protocol::TransferPacketData;

/// Tells a Modularis client to reconnect to a destination selected by Relay.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferPlayer {
    pub transfer: TransferPacketData,
}

/// Proves the intended Relay destination before the ordinary game JoinRequest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthenticateTransfer {
    pub transfer: TransferPacketData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferAuthenticated;

