use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chat_api::{PublishServerChatMessage, ServerChatApi, ServerChatSet};
use server_command_api::{
    CommandPlayer, ServerCommandApi, ServerCommandRegistry, ServerCommandSource,
    brigadier::{
        arguments::string_argument_type::{get_string, greedy_string},
        builder::{
            argument_builder::ArgumentBuilder, literal_argument_builder::literal,
            required_argument_builder::Argument,
        },
        context::CommandContext,
        suggestion::{SuggestionProvider, Suggestions, SuggestionsBuilder},
    },
    player_with_name, split_player_prefix,
};
use server_dimension_api::{
    RequestPlayerDimensionChange, ServerDimensionApi, ServerDimensionSet, ServerDimensions,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

const MAX_COORDINATE: f32 = 30_000_000.0;

#[derive(Debug, Clone)]
struct TeleportInvocation {
    source: player_network_message_types::PlayerId,
    arguments: String,
}

#[derive(Resource, Clone, Default)]
struct TeleportCommandQueue(Arc<Mutex<Vec<TeleportInvocation>>>);

#[derive(Debug, Clone, Copy, PartialEq)]
enum TeleportDestination {
    Coordinates(Vec3),
    Player(player_network_message_types::PlayerId),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ParsedTeleport {
    subject: player_network_message_types::PlayerId,
    destination: TeleportDestination,
}

pub struct ServerCommandTeleportVanillaMod;

impl ServerCommandTeleportVanillaMod {
    pub fn init<
        C: ServerCommandApi,
        H: ServerChatApi,
        P: ServerPlayerRegistryApi,
        D: ServerDimensionApi,
    >(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _players: &mut P,
        _dimensions: &mut D,
    ) -> Self {
        let queue = TeleportCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_teleport_commands
                .in_set(ServerChatSet::ApplyGameplay)
                .before(ServerDimensionSet::Apply),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command(commands: &ServerCommandRegistry, queue: &TeleportCommandQueue) {
    let queue = queue.0.clone();
    let arguments = ArgumentBuilder::new(
        Argument::<ServerCommandSource>::new(
            "arguments",
            Arc::new(greedy_string()),
            Some(Arc::new(TeleportPlayerSuggestions)),
        )
        .into(),
    )
    .executes(move |context: &CommandContext<ServerCommandSource>| {
        queue
            .lock()
            .expect("teleport command queue lock poisoned")
            .push(TeleportInvocation {
                source: context.source.player_id,
                arguments: get_string(context, "arguments").unwrap_or_default(),
            });
        1
    });
    let command: ArgumentBuilder<ServerCommandSource> = literal("teleport").then(arguments);
    commands.register(command);
}

struct TeleportPlayerSuggestions;

impl SuggestionProvider<ServerCommandSource> for TeleportPlayerSuggestions {
    fn get_suggestions(
        &self,
        context: CommandContext<ServerCommandSource>,
        mut builder: SuggestionsBuilder,
    ) -> Suggestions {
        let remaining = builder.remaining().trim_start();
        if let Some((subject, second)) =
            split_player_prefix(remaining, &context.source.online_players)
            && !second.is_empty()
        {
            let second_lower = second.to_lowercase();
            for player in &context.source.online_players {
                if player.name.to_lowercase().starts_with(&second_lower) {
                    builder = builder.suggest(&format!("{} {}", subject.name, player.name));
                }
            }
            return builder.build();
        }
        let remaining_lower = remaining.to_lowercase();
        for player in &context.source.online_players {
            if player.name.to_lowercase().starts_with(&remaining_lower) {
                builder = builder.suggest(&player.name);
            }
        }
        builder.build()
    }
}

fn apply_teleport_commands(
    queue: Res<TeleportCommandQueue>,
    players: Res<ServerPlayerRegistry>,
    dimensions: Res<ServerDimensions>,
    mut requests: MessageWriter<RequestPlayerDimensionChange>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations = std::mem::take(
        &mut *queue
            .0
            .lock()
            .expect("teleport command queue lock poisoned"),
    );
    let online = command_players(&players);
    for invocation in invocations {
        let parsed = match parse_teleport(invocation.source, &invocation.arguments, &online) {
            Ok(parsed) => parsed,
            Err(error) => {
                publish_feedback(invocation.source, error, &mut chat);
                continue;
            }
        };
        let Some(subject) = players.player(parsed.subject) else {
            publish_feedback(invocation.source, "Player not found".to_string(), &mut chat);
            continue;
        };
        let destination = match parsed.destination {
            TeleportDestination::Coordinates(position) => dimensions
                .dimension_id_for(subject.id)
                .map(|dimension| (dimension, position)),
            TeleportDestination::Player(destination_id) => {
                players.player(destination_id).and_then(|destination| {
                    dimensions
                        .dimension_id_for(destination.id)
                        .map(|dimension| (dimension, Vec3::from_array(destination.position)))
                })
            }
        };
        let Some((dimension, position)) = destination else {
            publish_feedback(
                invocation.source,
                "Destination is unavailable".to_string(),
                &mut chat,
            );
            continue;
        };
        requests.write(RequestPlayerDimensionChange {
            player_id: subject.id,
            target: dimension,
            position: Some(position.to_array()),
        });
        publish_feedback(
            invocation.source,
            format!(
                "Teleported {} to ({:.3}, {:.3}, {:.3}) in {}",
                subject.name,
                position.x,
                position.y,
                position.z,
                generated_dimension_registry::id(dimension)
            ),
            &mut chat,
        );
    }
}

fn parse_teleport(
    source: player_network_message_types::PlayerId,
    arguments: &str,
    online: &[CommandPlayer],
) -> Result<ParsedTeleport, String> {
    if let Ok(position) = parse_coordinates(arguments) {
        return Ok(ParsedTeleport {
            subject: source,
            destination: TeleportDestination::Coordinates(position),
        });
    }
    let Some((first, remainder)) = split_player_prefix(arguments, online) else {
        return Err(usage());
    };
    if remainder.is_empty() {
        return Ok(ParsedTeleport {
            subject: source,
            destination: TeleportDestination::Player(first.id),
        });
    }
    if let Ok(position) = parse_coordinates(remainder) {
        return Ok(ParsedTeleport {
            subject: first.id,
            destination: TeleportDestination::Coordinates(position),
        });
    }
    let Some(second) = player_with_name(online, remainder) else {
        return Err(usage());
    };
    Ok(ParsedTeleport {
        subject: first.id,
        destination: TeleportDestination::Player(second.id),
    })
}

fn parse_coordinates(value: &str) -> Result<Vec3, String> {
    let coordinates = value
        .split_whitespace()
        .map(str::parse::<f32>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| usage())?;
    let [x, y, z] = coordinates.as_slice() else {
        return Err(usage());
    };
    let position = Vec3::new(*x, *y, *z);
    if !position.is_finite()
        || position
            .to_array()
            .into_iter()
            .any(|coordinate| coordinate.abs() > MAX_COORDINATE)
    {
        return Err(format!(
            "Coordinates must be finite and between -{MAX_COORDINATE} and {MAX_COORDINATE}"
        ));
    }
    Ok(position)
}

fn command_players(players: &ServerPlayerRegistry) -> Vec<CommandPlayer> {
    players
        .players()
        .into_iter()
        .map(|player| CommandPlayer {
            id: player.id,
            name: player.name,
        })
        .collect()
}

fn usage() -> String {
    "Usage: /teleport <x> <y> <z>, /teleport <player>, /teleport <player> <x> <y> <z>, or /teleport <player> <player>".to_string()
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

    fn players() -> Vec<CommandPlayer> {
        vec![
            CommandPlayer {
                id: 1,
                name: "Player1".to_string(),
            },
            CommandPlayer {
                id: 2,
                name: "Player 2".to_string(),
            },
        ]
    }

    #[test]
    fn parses_all_supported_teleport_forms() {
        let players = players();
        assert_eq!(
            parse_teleport(1, "1 2 3", &players),
            Ok(ParsedTeleport {
                subject: 1,
                destination: TeleportDestination::Coordinates(Vec3::new(1.0, 2.0, 3.0)),
            })
        );
        assert_eq!(
            parse_teleport(1, "Player 2", &players),
            Ok(ParsedTeleport {
                subject: 1,
                destination: TeleportDestination::Player(2),
            })
        );
        assert_eq!(
            parse_teleport(2, "Player1 4 5 6", &players),
            Ok(ParsedTeleport {
                subject: 1,
                destination: TeleportDestination::Coordinates(Vec3::new(4.0, 5.0, 6.0)),
            })
        );
        assert_eq!(
            parse_teleport(2, "Player1 Player 2", &players),
            Ok(ParsedTeleport {
                subject: 1,
                destination: TeleportDestination::Player(2),
            })
        );
    }
}
