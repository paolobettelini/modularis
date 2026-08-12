use bevy::prelude::*;
use server_chunk_world_api::ResidentChunkKey;

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerChunkSent { pub player_id: u64, pub key: ResidentChunkKey }

