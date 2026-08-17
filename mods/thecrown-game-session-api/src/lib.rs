use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::collections::HashMap;
use thecrown_protocol::{GameMode, PlayerIdentity, TransferPacketData};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TheCrownGamePlayerSession {
    pub identity: PlayerIdentity,
    pub instance_id: String,
    pub mode: GameMode,
}

#[derive(Resource, Default)]
pub struct TheCrownGamePlayerSessions {
    by_player: HashMap<PlayerId, TheCrownGamePlayerSession>,
    by_uuid: HashMap<Uuid, PlayerId>,
}

impl TheCrownGamePlayerSessions {
    pub fn insert(&mut self, player_id: PlayerId, session: TheCrownGamePlayerSession) {
        self.by_uuid.insert(session.identity.uuid, player_id);
        self.by_player.insert(player_id, session);
    }
    pub fn get(&self, player_id: PlayerId) -> Option<&TheCrownGamePlayerSession> { self.by_player.get(&player_id) }
    pub fn player_for_uuid(&self, uuid: Uuid) -> Option<PlayerId> { self.by_uuid.get(&uuid).copied() }
    pub fn remove(&mut self, player_id: PlayerId) -> Option<TheCrownGamePlayerSession> {
        let session = self.by_player.remove(&player_id)?;
        self.by_uuid.remove(&session.identity.uuid);
        Some(session)
    }
}

#[derive(Message, Debug, Clone)]
pub struct TheCrownGamePlayerAdmitted {
    pub player_id: PlayerId,
    pub session: TheCrownGamePlayerSession,
}

#[derive(Message, Debug, Clone)]
pub struct TransferTheCrownPlayer {
    pub player_id: PlayerId,
    pub transfer: TransferPacketData,
}

pub trait TheCrownGameSessionApi: Send + Sync + 'static {}
