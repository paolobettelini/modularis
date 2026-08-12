use bevy::prelude::*;
use generated_permission_registry::PermissionId;
use player_network_message_types::PlayerId;
use std::collections::HashMap;

pub use generated_game_mode_registry::{
    GameMode, all_game_modes, from_str as game_mode_from_str, id as game_mode_id,
    short_id as game_mode_short_id,
};

#[derive(Resource, Default)]
pub struct ServerPlayerGameModes(HashMap<PlayerId, GameMode>);

impl ServerPlayerGameModes {
    pub fn get(&self, player_id: PlayerId) -> Option<GameMode> { self.0.get(&player_id).copied() }
    pub fn set(&mut self, player_id: PlayerId, mode: GameMode) -> Option<GameMode> { self.0.insert(player_id, mode) }
    pub fn remove(&mut self, player_id: PlayerId) { self.0.remove(&player_id); }
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPlayerGameMode { pub player_id: PlayerId, pub mode: GameMode }

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerPlayerGameModeChanged {
    pub player_id: PlayerId,
    pub previous: Option<GameMode>,
    pub mode: GameMode,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerGameModeSet { Apply, ApplyPolicy }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerPlayerGameModePolicy {
    pub clear_permissions: Vec<PermissionId>,
    pub permission_changes: Vec<(PermissionId, bool)>,
    pub permission_denials: Vec<(PermissionId, bool)>,
    pub outline_enabled: Option<bool>,
}

#[derive(Resource, Default)]
pub struct ServerPlayerGameModePolicies(HashMap<GameMode, ServerPlayerGameModePolicy>);

impl ServerPlayerGameModePolicies {
    pub fn register(&mut self, mode: GameMode, policy: ServerPlayerGameModePolicy) {
        assert!(self.0.insert(mode, policy).is_none(), "duplicate policy for game mode '{}'", mode.id());
    }

    pub fn get(&self, mode: GameMode) -> Option<&ServerPlayerGameModePolicy> { self.0.get(&mode) }
}

pub trait ServerPlayerGameModeApi: Send + Sync + 'static {}
