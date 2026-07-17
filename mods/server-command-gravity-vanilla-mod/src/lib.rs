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
use server_player_gravity_api::{
    ServerPlayerGravityApi, ServerPlayerGravitySet, SetServerPlayerGravity,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

const MAX_GRAVITY_LENGTH: f32 = 1_000.0;

#[derive(Debug, Clone)]
struct GravityInvocation {
    source: player_network_message_types::PlayerId,
    arguments: String,
}

#[derive(Resource, Clone, Default)]
struct GravityCommandQueue(Arc<Mutex<Vec<GravityInvocation>>>);

pub struct ServerCommandGravityVanillaMod;

impl ServerCommandGravityVanillaMod {
    pub fn init<
        C: ServerCommandApi,
        H: ServerChatApi,
        P: ServerPlayerRegistryApi,
        G: ServerPlayerGravityApi,
    >(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _players: &mut P,
        _gravity: &mut G,
    ) -> Self {
        let queue = GravityCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_gravity_commands
                .in_set(ServerChatSet::ApplyGameplay)
                .before(ServerPlayerGravitySet::Apply),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command(commands: &ServerCommandRegistry, queue: &GravityCommandQueue) {
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
            .expect("gravity command queue lock poisoned")
            .push(GravityInvocation {
                source: context.source.player_id,
                arguments: get_string(context, "arguments").unwrap_or_default(),
            });
        1
    });
    let command: ArgumentBuilder<ServerCommandSource> = literal("setgravity").then(arguments);
    commands.register(command);
}

fn apply_gravity_commands(
    queue: Res<GravityCommandQueue>,
    players: Res<ServerPlayerRegistry>,
    mut changes: MessageWriter<SetServerPlayerGravity>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations =
        std::mem::take(&mut *queue.0.lock().expect("gravity command queue lock poisoned"));
    let online = players
        .players()
        .into_iter()
        .map(|player| server_command_api::CommandPlayer {
            id: player.id,
            name: player.name,
        })
        .collect::<Vec<_>>();
    for invocation in invocations {
        match parse_gravity_arguments(invocation.source, &invocation.arguments, &online) {
            Ok((target, gravity)) => {
                changes.write(SetServerPlayerGravity {
                    player_id: target,
                    gravity,
                });
                let target_name = players
                    .player(target)
                    .map(|player| player.name.as_str())
                    .unwrap_or("unknown player");
                publish_feedback(
                    invocation.source,
                    format!(
                        "Gravity set to ({:.3}, {:.3}, {:.3}) for {target_name}",
                        gravity.x, gravity.y, gravity.z
                    ),
                    &mut chat,
                );
            }
            Err(error) => publish_feedback(invocation.source, error, &mut chat),
        }
    }
}

fn parse_gravity_arguments(
    source: player_network_message_types::PlayerId,
    arguments: &str,
    online: &[server_command_api::CommandPlayer],
) -> Result<(player_network_message_types::PlayerId, Vec3), String> {
    if let Ok(gravity) = parse_gravity(arguments) {
        return Ok((source, gravity));
    }
    let Some((target, remainder)) = split_player_prefix(arguments, online) else {
        return Err(usage());
    };
    Ok((target.id, parse_gravity(remainder)?))
}

fn parse_gravity(value: &str) -> Result<Vec3, String> {
    let values = value
        .split_whitespace()
        .map(str::parse::<f32>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| usage())?;
    let gravity = match values.as_slice() {
        [strength] if *strength >= 0.0 => Vec3::new(0.0, -*strength, 0.0),
        [x, y, z] => Vec3::new(*x, *y, *z),
        _ => return Err(usage()),
    };
    if !gravity.is_finite() || gravity.length() > MAX_GRAVITY_LENGTH {
        return Err(format!(
            "Gravity must be finite and no longer than {MAX_GRAVITY_LENGTH}"
        ));
    }
    Ok(gravity)
}

fn usage() -> String {
    "Usage: /setgravity [player] <g> or /setgravity [player] <x> <y> <z>".to_string()
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
    fn parses_scalar_vector_and_named_target() {
        let online = [server_command_api::CommandPlayer {
            id: 2,
            name: "Player Two".to_string(),
        }];
        assert_eq!(
            parse_gravity_arguments(1, "20", &online),
            Ok((1, Vec3::new(0.0, -20.0, 0.0)))
        );
        assert_eq!(
            parse_gravity_arguments(1, "Player Two 1 2 3", &online),
            Ok((2, Vec3::new(1.0, 2.0, 3.0)))
        );
    }
}
