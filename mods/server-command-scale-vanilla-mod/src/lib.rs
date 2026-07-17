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
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use server_player_scale_api::{ServerPlayerScaleApi, ServerPlayerScaleSet, SetServerPlayerScale};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

const MIN_SCALE: f32 = 0.05;
const MAX_SCALE: f32 = 20.0;

#[derive(Debug, Clone)]
struct ScaleInvocation {
    source: player_network_message_types::PlayerId,
    arguments: String,
}

#[derive(Resource, Clone, Default)]
struct ScaleCommandQueue(Arc<Mutex<Vec<ScaleInvocation>>>);

pub struct ServerCommandScaleVanillaMod;

impl ServerCommandScaleVanillaMod {
    pub fn init<
        C: ServerCommandApi,
        H: ServerChatApi,
        P: ServerPlayerRegistryApi,
        S: ServerPlayerScaleApi,
    >(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _players: &mut P,
        _scale: &mut S,
    ) -> Self {
        let queue = ScaleCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_scale_commands
                .in_set(ServerChatSet::ApplyGameplay)
                .before(ServerPlayerScaleSet::Apply),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command(commands: &ServerCommandRegistry, queue: &ScaleCommandQueue) {
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
            .expect("scale command queue lock poisoned")
            .push(ScaleInvocation {
                source: context.source.player_id,
                arguments: get_string(context, "arguments").unwrap_or_default(),
            });
        1
    });
    let command: ArgumentBuilder<ServerCommandSource> = literal("setscale").then(arguments);
    commands.register(command);
}

fn apply_scale_commands(
    queue: Res<ScaleCommandQueue>,
    players: Res<ServerPlayerRegistry>,
    mut changes: MessageWriter<SetServerPlayerScale>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations =
        std::mem::take(&mut *queue.0.lock().expect("scale command queue lock poisoned"));
    let online = players
        .players()
        .into_iter()
        .map(|player| server_command_api::CommandPlayer {
            id: player.id,
            name: player.name,
        })
        .collect::<Vec<_>>();
    for invocation in invocations {
        match parse_scale_arguments(invocation.source, &invocation.arguments, &online) {
            Ok((target, scale)) => {
                changes.write(SetServerPlayerScale {
                    player_id: target,
                    scale,
                });
                let target_name = players
                    .player(target)
                    .map(|player| player.name.as_str())
                    .unwrap_or("unknown player");
                publish_feedback(
                    invocation.source,
                    format!("Scale set to {scale} for {target_name}"),
                    &mut chat,
                );
            }
            Err(error) => publish_feedback(invocation.source, error, &mut chat),
        }
    }
}

fn parse_scale_arguments(
    source: player_network_message_types::PlayerId,
    arguments: &str,
    online: &[server_command_api::CommandPlayer],
) -> Result<(player_network_message_types::PlayerId, f32), String> {
    if let Ok(scale) = parse_scale(arguments) {
        return Ok((source, scale));
    }
    let Some((target, remainder)) = split_player_prefix(arguments, online) else {
        return Err(usage());
    };
    Ok((target.id, parse_scale(remainder)?))
}

fn parse_scale(value: &str) -> Result<f32, String> {
    let scale = value.trim().parse::<f32>().map_err(|_| usage())?;
    if !scale.is_finite() || !(MIN_SCALE..=MAX_SCALE).contains(&scale) {
        return Err(format!("Scale must be between {MIN_SCALE} and {MAX_SCALE}"));
    }
    Ok(scale)
}

fn usage() -> String {
    "Usage: /setscale [player] <scale>".to_string()
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
    fn parses_self_and_named_player_scale() {
        let online = [server_command_api::CommandPlayer {
            id: 2,
            name: "Player Two".to_string(),
        }];
        assert_eq!(parse_scale_arguments(1, "2.5", &online), Ok((1, 2.5)));
        assert_eq!(
            parse_scale_arguments(1, "Player Two 0.5", &online),
            Ok((2, 0.5))
        );
    }
}
