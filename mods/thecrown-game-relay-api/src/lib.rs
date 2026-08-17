use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::{net::SocketAddr, sync::{Arc, atomic::{AtomicU64, Ordering}}};
use thecrown_protocol::{
    DataTransferResult, GameInstanceSpec, ParkourRecordUpdate, PlayerIdentity,
    TransferPacketData,
};
use uuid::Uuid;

#[derive(Resource, Clone, Default)]
pub struct TheCrownRelayRequestIds(Arc<AtomicU64>);

impl TheCrownRelayRequestIds {
    pub fn next(&self) -> u64 { self.0.fetch_add(1, Ordering::Relaxed) }
}

#[derive(Message, Debug, Clone)]
pub struct RelayStartInstance { pub instance: GameInstanceSpec }

#[derive(Message, Debug, Clone)]
pub struct RelayStopInstance { pub instance_id: String }

#[derive(Message, Debug, Clone)]
pub struct AuthenticateRelayTransfer {
    pub request_id: u64,
    pub source: SocketAddr,
    pub player: PlayerIdentity,
    pub transfer: TransferPacketData,
}

#[derive(Message, Debug, Clone)]
pub struct RelayTransferAuthenticationFinished {
    pub request_id: u64,
    pub source: SocketAddr,
    pub player: PlayerIdentity,
    pub transfer: TransferPacketData,
    pub accepted: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RelayTransferDestination {
    Lobby,
    Mode(thecrown_protocol::GameMode),
    Instance(String),
}

#[derive(Message, Debug, Clone)]
pub struct RequestRelayPlayerTransfer {
    pub request_id: u64,
    pub player_id: PlayerId,
    pub player_uuid: Uuid,
    pub destination: RelayTransferDestination,
}

#[derive(Message, Debug, Clone)]
pub struct RelayPlayerTransferFinished {
    pub request_id: u64,
    pub player_id: PlayerId,
    pub result: Result<DataTransferResult, String>,
}

#[derive(Message, Debug, Clone)]
pub struct NotifyRelayPlayerQuit {
    pub player_uuid: Uuid,
    pub instance_id: String,
}

#[derive(Message, Debug, Clone)]
pub struct RequestRelayWhisper {
    pub request_id: u64,
    pub sender_id: PlayerId,
    pub sender_uuid: Uuid,
    pub target_username: String,
    pub message: String,
}

#[derive(Message, Debug, Clone)]
pub struct RelayWhisperFinished {
    pub request_id: u64,
    pub sender_id: PlayerId,
    pub delivered: bool,
    pub error: Option<String>,
}

#[derive(Message, Debug, Clone)]
pub struct RelayWhisperReceived {
    pub sender: PlayerIdentity,
    pub target_uuid: Uuid,
    pub message: String,
}

#[derive(Message, Debug, Clone)]
pub struct RelayExecuteTransfer {
    pub player_uuid: Uuid,
    pub transfer: TransferPacketData,
}

#[derive(Message, Debug, Clone)]
pub struct RequestTheCrownWebLogin {
    pub request_id: u64,
    pub player_id: PlayerId,
    pub player_uuid: Uuid,
}

#[derive(Message, Debug, Clone)]
pub struct TheCrownWebLoginFinished {
    pub request_id: u64,
    pub player_id: PlayerId,
    pub result: Result<String, String>,
}

#[derive(Message, Debug, Clone)]
pub struct RequestRelayParkourRecord {
    pub request_id: u64,
    pub player_id: PlayerId,
    pub player_uuid: Uuid,
}

#[derive(Message, Debug, Clone)]
pub struct RelayParkourRecordLoaded {
    pub request_id: u64,
    pub player_id: PlayerId,
    pub result: Result<i32, String>,
}

#[derive(Message, Debug, Clone)]
pub struct SubmitRelayParkourRecord {
    pub request_id: u64,
    pub player_id: PlayerId,
    pub player_uuid: Uuid,
    pub score: i32,
}

#[derive(Message, Debug, Clone)]
pub struct RelayParkourRecordSubmitted {
    pub request_id: u64,
    pub player_id: PlayerId,
    pub submitted_score: i32,
    pub result: Result<ParkourRecordUpdate, String>,
}

pub trait TheCrownGameRelayApi: Send + Sync + 'static {}
