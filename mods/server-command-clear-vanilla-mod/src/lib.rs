use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chat_api::{ClearServerPlayerChatRequested, ServerChatApi, ServerChatSet};
use server_command_api::{
    ServerCommandApi, ServerCommandRegistry, ServerCommandSource,
    brigadier::{
        builder::{argument_builder::ArgumentBuilder, literal_argument_builder::literal},
        context::CommandContext,
    },
};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

#[derive(Resource, Clone, Default)]
struct ClearChatCommandQueue(Arc<Mutex<Vec<player_network_message_types::PlayerId>>>);

pub struct ServerCommandClearVanillaMod;

impl ServerCommandClearVanillaMod {
    pub fn init<C: ServerCommandApi, H: ServerChatApi>(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
    ) -> Self {
        let queue = ClearChatCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_clear_commands.in_set(ServerChatSet::ApplyGameplay),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command(commands: &ServerCommandRegistry, queue: &ClearChatCommandQueue) {
    let queue = queue.0.clone();
    let command: ArgumentBuilder<ServerCommandSource> =
        literal("clear").executes(move |context: &CommandContext<ServerCommandSource>| {
            queue
                .lock()
                .expect("clear chat command queue lock poisoned")
                .push(context.source.player_id);
            1
        });
    commands.register(command);
}

fn apply_clear_commands(
    queue: Res<ClearChatCommandQueue>,
    mut requests: MessageWriter<ClearServerPlayerChatRequested>,
) {
    let players = std::mem::take(
        &mut *queue
            .0
            .lock()
            .expect("clear chat command queue lock poisoned"),
    );
    for player_id in players {
        requests.write(ClearServerPlayerChatRequested { player_id });
    }
}
