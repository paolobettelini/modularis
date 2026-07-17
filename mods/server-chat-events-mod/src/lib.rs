use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chat_api::{
    PublishServerChatMessage, ServerChatApi, ServerChatInputReceived, ServerChatSet,
    ServerCommandRequested, ServerCommandSuggestionsReady, ServerCommandSuggestionsRequested,
};
use tokio::task::JoinHandle;

pub struct ServerChatEventsMod;

impl ServerChatEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<ServerChatInputReceived>()
            .add_message::<PublishServerChatMessage>()
            .add_message::<ServerCommandRequested>()
            .add_message::<ServerCommandSuggestionsRequested>()
            .add_message::<ServerCommandSuggestionsReady>()
            .configure_sets(
                Update,
                (
                    ServerChatSet::Receive,
                    ServerChatSet::Route,
                    ServerChatSet::ExecuteCommands,
                    ServerChatSet::ApplyGameplay,
                    ServerChatSet::Publish,
                    ServerChatSet::Sync,
                )
                    .chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerChatApi for ServerChatEventsMod {}
