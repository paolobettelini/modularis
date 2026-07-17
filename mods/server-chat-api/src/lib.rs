use audience_api::Audience;
use bevy::prelude::*;
use player_network_message_types::PlayerId;

pub const MAX_CHAT_INPUT_BYTES: usize = 512;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerChatSet {
    Receive,
    Route,
    ExecuteCommands,
    ApplyGameplay,
    Publish,
    Sync,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerChatInputReceived {
    pub player_id: PlayerId,
    pub text: String,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct PublishServerChatMessage {
    pub audience: Audience,
    pub text: String,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClearServerPlayerChatRequested {
    pub player_id: PlayerId,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerCommandRequested {
    pub player_id: PlayerId,
    pub input: String,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerCommandSuggestionsRequested {
    pub player_id: PlayerId,
    pub request_id: u64,
    pub input: String,
    pub cursor: usize,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerCommandSuggestionsReady {
    pub player_id: PlayerId,
    pub request_id: u64,
    pub suggestions: Vec<String>,
}

pub trait ServerChatApi: Send + Sync + 'static {}
