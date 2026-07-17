use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chat_api::{
    PublishServerChatMessage, ServerChatApi, ServerChatSet, ServerCommandRequested,
    ServerCommandSuggestionsReady, ServerCommandSuggestionsRequested,
};
use server_command_api::{
    CommandPlayer, ServerCommandApi, ServerCommandRegistry, ServerCommandSource,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerCommandBrigadierMod;

impl ServerCommandBrigadierMod {
    pub fn init<C: ServerChatApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _players: &mut P,
    ) -> Self {
        bevy.app
            .init_resource::<ServerCommandRegistry>()
            .add_systems(
                Update,
                (execute_commands, complete_commands).in_set(ServerChatSet::ExecuteCommands),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerCommandApi for ServerCommandBrigadierMod {}

fn execute_commands(
    mut requests: MessageReader<ServerCommandRequested>,
    commands: Res<ServerCommandRegistry>,
    players: Res<ServerPlayerRegistry>,
    mut output: MessageWriter<PublishServerChatMessage>,
) {
    for request in requests.read() {
        let Some(source) = command_source(request.player_id, &players) else {
            continue;
        };
        if let Err(error) = commands.execute(&request.input, source) {
            output.write(PublishServerChatMessage {
                audience: Audience::personal(request.player_id),
                text: format!("Command error: {error}"),
            });
        }
    }
}

fn complete_commands(
    mut requests: MessageReader<ServerCommandSuggestionsRequested>,
    commands: Res<ServerCommandRegistry>,
    players: Res<ServerPlayerRegistry>,
    mut ready: MessageWriter<ServerCommandSuggestionsReady>,
) {
    for request in requests.read() {
        let Some(source) = command_source(request.player_id, &players) else {
            continue;
        };
        ready.write(ServerCommandSuggestionsReady {
            player_id: request.player_id,
            request_id: request.request_id,
            suggestions: commands.suggestions(&request.input, request.cursor, source),
        });
    }
}

fn command_source(
    player_id: player_network_message_types::PlayerId,
    registry: &ServerPlayerRegistry,
) -> Option<ServerCommandSource> {
    let player = registry.player(player_id)?;
    let online_players = registry
        .players()
        .into_iter()
        .map(|player| CommandPlayer {
            id: player.id,
            name: player.name,
        })
        .collect();
    Some(ServerCommandSource {
        player_id,
        player_name: player.name.clone(),
        online_players,
    })
}
