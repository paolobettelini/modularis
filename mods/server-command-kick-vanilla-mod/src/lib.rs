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
use server_kick_api::{ServerKickApi, ServerKickRequested, ServerKickSet, ServerKickTarget};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

const DEFAULT_KICK_REASON: &str = "Kicked by an operator";

#[derive(Debug, Clone)]
struct KickInvocation {
    source: player_network_message_types::PlayerId,
    arguments: String,
}

#[derive(Resource, Clone, Default)]
struct KickCommandQueue(Arc<Mutex<Vec<KickInvocation>>>);

pub struct ServerCommandKickVanillaMod;

impl ServerCommandKickVanillaMod {
    pub fn init<
        C: ServerCommandApi,
        H: ServerChatApi,
        P: ServerPlayerRegistryApi,
        K: ServerKickApi,
    >(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _players: &mut P,
        _kick: &mut K,
    ) -> Self {
        let queue = KickCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_kick_commands
                .in_set(ServerChatSet::ApplyGameplay)
                .before(ServerKickSet::Apply),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command(commands: &ServerCommandRegistry, queue: &KickCommandQueue) {
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
            .expect("kick command queue lock poisoned")
            .push(KickInvocation {
                source: context.source.player_id,
                arguments: get_string(context, "arguments").unwrap_or_default(),
            });
        1
    });
    let command: ArgumentBuilder<ServerCommandSource> = literal("kick").then(arguments);
    commands.register(command);
}

fn apply_kick_commands(
    queue: Res<KickCommandQueue>,
    players: Res<ServerPlayerRegistry>,
    mut requests: MessageWriter<ServerKickRequested>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations =
        std::mem::take(&mut *queue.0.lock().expect("kick command queue lock poisoned"));
    let online = players
        .players()
        .into_iter()
        .map(|player| server_command_api::CommandPlayer {
            id: player.id,
            name: player.name,
        })
        .collect::<Vec<_>>();
    for invocation in invocations {
        let Some((target, reason)) = parse_arguments(&invocation.arguments, &online) else {
            publish_feedback(
                invocation.source,
                "Usage: /kick <player> [reason]".to_string(),
                &mut chat,
            );
            continue;
        };
        requests.write(ServerKickRequested {
            target: ServerKickTarget::Player(target.id),
            reason,
        });
        if target.id != invocation.source {
            publish_feedback(
                invocation.source,
                format!("Kicked {}", target.name),
                &mut chat,
            );
        }
    }
}

fn parse_arguments(
    arguments: &str,
    online: &[server_command_api::CommandPlayer],
) -> Option<(server_command_api::CommandPlayer, String)> {
    let (target, remainder) = split_player_prefix(arguments, online)?;
    let reason = if remainder.trim().is_empty() {
        DEFAULT_KICK_REASON.to_string()
    } else {
        remainder.trim().to_string()
    };
    Some((target, reason))
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
    fn parses_optional_reason_for_names_with_spaces() {
        let online = [server_command_api::CommandPlayer {
            id: 2,
            name: "Player Two".to_string(),
        }];
        assert_eq!(
            parse_arguments("Player Two testing", &online),
            Some((online[0].clone(), "testing".to_string()))
        );
        assert_eq!(
            parse_arguments("Player Two", &online),
            Some((online[0].clone(), DEFAULT_KICK_REASON.to_string()))
        );
    }
}
