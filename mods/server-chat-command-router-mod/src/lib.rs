use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chat_api::{
    ServerChatApi, ServerChatInputReceived, ServerChatSet, ServerCommandRequested,
};
use tokio::task::JoinHandle;

pub struct ServerChatCommandRouterMod;

impl ServerChatCommandRouterMod {
    pub fn init<C: ServerChatApi>(bevy: &mut BevyMod, _chat: &mut C) -> Self {
        bevy.app
            .add_systems(Update, route_commands.in_set(ServerChatSet::Route));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn route_commands(
    mut inputs: MessageReader<ServerChatInputReceived>,
    mut commands: MessageWriter<ServerCommandRequested>,
) {
    for input in inputs.read().filter(|input| input.text.starts_with('/')) {
        let command = input.text.trim_start_matches('/').trim();
        if !command.is_empty() {
            commands.write(ServerCommandRequested {
                player_id: input.player_id,
                input: command.to_string(),
            });
        }
    }
}
