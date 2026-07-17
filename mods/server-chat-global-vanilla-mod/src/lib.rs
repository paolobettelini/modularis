use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chat_api::{
    PublishServerChatMessage, ServerChatApi, ServerChatInputReceived, ServerChatSet,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerChatGlobalVanillaMod;

impl ServerChatGlobalVanillaMod {
    pub fn init<C: ServerChatApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _players: &mut P,
    ) -> Self {
        bevy.app
            .add_systems(Update, publish_global_chat.in_set(ServerChatSet::Publish));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn publish_global_chat(
    mut inputs: MessageReader<ServerChatInputReceived>,
    players: Res<ServerPlayerRegistry>,
    mut messages: MessageWriter<PublishServerChatMessage>,
) {
    for input in inputs.read().filter(|input| !input.text.starts_with('/')) {
        let Some(player) = players.player(input.player_id) else {
            continue;
        };
        messages.write(PublishServerChatMessage {
            audience: Audience::everyone(),
            text: format!("[{}] {}", player.name, input.text),
        });
    }
}
