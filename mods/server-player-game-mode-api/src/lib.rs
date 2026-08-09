use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMode { Creative, Survival, Adventure }

impl GameMode {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "creative" => Some(Self::Creative),
            "survival" => Some(Self::Survival),
            "adventure" => Some(Self::Adventure),
            _ => None,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Creative => "creative",
            Self::Survival => "survival",
            Self::Adventure => "adventure",
        }
    }
}

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

pub trait ServerPlayerGameModeApi: Send + Sync + 'static {}
