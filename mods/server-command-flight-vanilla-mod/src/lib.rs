use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chat_api::{PublishServerChatMessage, ServerChatApi, ServerChatSet};
use server_command_api::{
    OnlinePlayerSuggestions, ServerCommandApi, ServerCommandRegistry, ServerCommandSource,
    brigadier::{
        arguments::string_argument_type::{get_string, greedy_string},
        builder::{
            argument_builder::ArgumentBuilder, literal_argument_builder::literal,
            required_argument_builder::Argument,
        },
        context::CommandContext,
    },
};
use server_player_flight_api::{
    ServerPlayerFlightApi, ServerPlayerFlightCapabilities, ServerPlayerFlightSet,
    SetPlayerFlightCapability,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
struct FlightInvocation {
    source: player_network_message_types::PlayerId,
    target_name: Option<String>,
}

#[derive(Resource, Clone, Default)]
struct FlightCommandQueue(Arc<Mutex<Vec<FlightInvocation>>>);

pub struct ServerCommandFlightVanillaMod;

impl ServerCommandFlightVanillaMod {
    pub fn init<
        C: ServerCommandApi,
        H: ServerChatApi,
        P: ServerPlayerRegistryApi,
        F: ServerPlayerFlightApi,
    >(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _players: &mut P,
        _flight: &mut F,
    ) -> Self {
        let queue = FlightCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_flight_commands
                .in_set(ServerChatSet::ApplyGameplay)
                .before(ServerPlayerFlightSet::Apply),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command(commands: &ServerCommandRegistry, queue: &FlightCommandQueue) {
    let self_queue = queue.0.clone();
    let target_queue = queue.0.clone();
    let player_argument = ArgumentBuilder::new(
        Argument::<ServerCommandSource>::new(
            "player",
            Arc::new(greedy_string()),
            Some(Arc::new(OnlinePlayerSuggestions)),
        )
        .into(),
    )
    .executes(move |context: &CommandContext<ServerCommandSource>| {
        target_queue
            .lock()
            .expect("flight command queue lock poisoned")
            .push(FlightInvocation {
                source: context.source.player_id,
                target_name: get_string(context, "player"),
            });
        1
    });
    let command: ArgumentBuilder<ServerCommandSource> = literal("flight")
        .executes(move |context: &CommandContext<ServerCommandSource>| {
            self_queue
                .lock()
                .expect("flight command queue lock poisoned")
                .push(FlightInvocation {
                    source: context.source.player_id,
                    target_name: None,
                });
            1
        })
        .then(player_argument);
    commands.register(command);
}

fn apply_flight_commands(
    queue: Res<FlightCommandQueue>,
    players: Res<ServerPlayerRegistry>,
    capabilities: Res<ServerPlayerFlightCapabilities>,
    mut changes: MessageWriter<SetPlayerFlightCapability>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations =
        std::mem::take(&mut *queue.0.lock().expect("flight command queue lock poisoned"));
    for invocation in invocations {
        let target = match &invocation.target_name {
            Some(name) => players
                .players()
                .into_iter()
                .find(|player| player.name.eq_ignore_ascii_case(name)),
            None => players.player(invocation.source).cloned(),
        };
        let Some(target) = target else {
            chat.write(PublishServerChatMessage {
                audience: Audience::personal(invocation.source),
                text: "Player not found".to_string(),
            });
            continue;
        };
        let enabled = !capabilities.enabled(target.id);
        changes.write(SetPlayerFlightCapability {
            player_id: target.id,
            enabled,
        });
        chat.write(PublishServerChatMessage {
            audience: Audience::personal(invocation.source),
            text: format!(
                "Flight {} for {}",
                if enabled { "enabled" } else { "disabled" },
                target.name
            ),
        });
    }
}
