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
    split_player_prefix,
};
use server_player_flight_speed_api::{
    ServerPlayerFlightSpeedApi, ServerPlayerFlightSpeedSet, SetServerPlayerFlightSpeed,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

const MAX_FLIGHT_SPEED_MULTIPLIER: f32 = 100.0;

#[derive(Debug, Clone)]
struct FlightSpeedInvocation {
    source: player_network_message_types::PlayerId,
    arguments: String,
}

#[derive(Resource, Clone, Default)]
struct FlightSpeedCommandQueue(Arc<Mutex<Vec<FlightSpeedInvocation>>>);

pub struct ServerCommandFlightSpeedVanillaMod;

impl ServerCommandFlightSpeedVanillaMod {
    pub fn init<
        C: ServerCommandApi,
        H: ServerChatApi,
        P: ServerPlayerRegistryApi,
        S: ServerPlayerFlightSpeedApi,
    >(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _players: &mut P,
        _speed: &mut S,
    ) -> Self {
        let queue = FlightSpeedCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_flight_speed_commands
                .in_set(ServerChatSet::ApplyGameplay)
                .before(ServerPlayerFlightSpeedSet::Apply),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command(commands: &ServerCommandRegistry, queue: &FlightSpeedCommandQueue) {
    let queue = queue.0.clone();
    let arguments = ArgumentBuilder::new(
        Argument::<ServerCommandSource>::new(
            "arguments",
            Arc::new(greedy_string()),
            Some(Arc::new(OnlinePlayerSuggestions)),
        )
        .into(),
    )
    .executes(move |context: &CommandContext<ServerCommandSource>| {
        queue
            .lock()
            .expect("flight speed command queue lock poisoned")
            .push(FlightSpeedInvocation {
                source: context.source.player_id,
                arguments: get_string(context, "arguments").unwrap_or_default(),
            });
        1
    });
    let command: ArgumentBuilder<ServerCommandSource> = literal("flightspeed").then(arguments);
    commands.register(command);
}

fn apply_flight_speed_commands(
    queue: Res<FlightSpeedCommandQueue>,
    players: Res<ServerPlayerRegistry>,
    mut changes: MessageWriter<SetServerPlayerFlightSpeed>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations = std::mem::take(
        &mut *queue
            .0
            .lock()
            .expect("flight speed command queue lock poisoned"),
    );
    let online = players
        .players()
        .into_iter()
        .map(|player| server_command_api::CommandPlayer {
            id: player.id,
            name: player.name,
        })
        .collect::<Vec<_>>();
    for invocation in invocations {
        match parse_arguments(invocation.source, &invocation.arguments, &online) {
            Ok((target, multiplier)) => {
                changes.write(SetServerPlayerFlightSpeed {
                    player_id: target,
                    multiplier,
                });
                let target_name = players
                    .player(target)
                    .map(|player| player.name.as_str())
                    .unwrap_or("unknown player");
                publish_feedback(
                    invocation.source,
                    format!("Flight speed set to {multiplier} for {target_name}"),
                    &mut chat,
                );
            }
            Err(error) => publish_feedback(invocation.source, error, &mut chat),
        }
    }
}

fn parse_arguments(
    source: player_network_message_types::PlayerId,
    arguments: &str,
    online: &[server_command_api::CommandPlayer],
) -> Result<(player_network_message_types::PlayerId, f32), String> {
    if let Ok(multiplier) = parse_multiplier(arguments) {
        return Ok((source, multiplier));
    }
    let Some((target, remainder)) = split_player_prefix(arguments, online) else {
        return Err(usage());
    };
    Ok((target.id, parse_multiplier(remainder)?))
}

fn parse_multiplier(value: &str) -> Result<f32, String> {
    let multiplier = value
        .trim()
        .parse::<f32>()
        .map_err(|_| "Flight speed must be a number".to_string())?;
    if !multiplier.is_finite() || !(0.0..=MAX_FLIGHT_SPEED_MULTIPLIER).contains(&multiplier) {
        return Err(format!(
            "Flight speed must be between 0 and {MAX_FLIGHT_SPEED_MULTIPLIER}"
        ));
    }
    Ok(multiplier)
}

fn usage() -> String {
    "Usage: /flightspeed <amount> or /flightspeed <player> <amount>".to_string()
}

fn publish_feedback(
    player_id: player_network_message_types::PlayerId,
    text: String,
    chat: &mut MessageWriter<PublishServerChatMessage>,
) {
    chat.write(PublishServerChatMessage {
        audience: Audience::personal(player_id),
        text,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_self_and_named_player_flight_speed() {
        let online = [server_command_api::CommandPlayer {
            id: 2,
            name: "Player Two".to_string(),
        }];
        assert_eq!(parse_arguments(1, "2", &online), Ok((1, 2.0)));
        assert_eq!(parse_arguments(1, "Player Two 3.5", &online), Ok((2, 3.5)));
    }
}
