use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_permission_registry::PermissionId;
use game_mode_registry_codegen::GameModeRegistryCodegenMod;
use server_chat_api::{PublishServerChatMessage, ServerChatApi, ServerChatSet};
use server_command_api::{
    CommandPlayer, ServerCommandApi, ServerCommandRegistry, ServerCommandSource,
    brigadier::{
        arguments::string_argument_type::{get_string, greedy_string},
        builder::{argument_builder::ArgumentBuilder, literal_argument_builder::literal, required_argument_builder::Argument},
        context::CommandContext,
        suggestion::{SuggestionProvider, Suggestions, SuggestionsBuilder},
    },
    split_player_prefix,
};
use server_player_game_mode_api::{
    GameMode, ServerPlayerGameModeApi, ServerPlayerGameModeSet, SetPlayerGameMode,
    all_game_modes,
};
use server_player_permission_api::{ServerPlayerPermissionApi, ServerPlayerPermissions};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
struct GameModeInvocation { source: u64, arguments: String }

#[derive(Resource, Clone, Default)]
struct GameModeCommandQueue(Arc<Mutex<Vec<GameModeInvocation>>>);

pub struct ServerCommandGameModeVanillaMod;

impl ServerCommandGameModeVanillaMod {
    pub fn init<C: ServerCommandApi, H: ServerChatApi, P: ServerPlayerRegistryApi, R: ServerPlayerPermissionApi, G: ServerPlayerGameModeApi>(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _players: &mut P,
        _permissions: &mut R,
        _game_modes: &mut G,
        _game_modes_codegen: &mut GameModeRegistryCodegenMod,
    ) -> Self {
        let queue = GameModeCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_game_mode_commands
                .in_set(ServerChatSet::ApplyGameplay)
                .before(ServerPlayerGameModeSet::Apply),
        );
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn register_command(commands: &ServerCommandRegistry, queue: &GameModeCommandQueue) {
    let queue = queue.0.clone();
    let arguments = ArgumentBuilder::new(
        Argument::<ServerCommandSource>::new(
            "arguments",
            Arc::new(greedy_string()),
            Some(Arc::new(GameModeSuggestions)),
        ).into(),
    ).executes(move |context: &CommandContext<ServerCommandSource>| {
        queue.lock().expect("game mode command queue lock poisoned").push(GameModeInvocation {
            source: context.source.player_id,
            arguments: get_string(context, "arguments").unwrap_or_default(),
        });
        1
    });
    commands.register_restricted(
        "gamemode",
        PermissionId::CanChangeOwnGameMode,
        literal("gamemode").then(arguments),
    );
}

struct GameModeSuggestions;

impl SuggestionProvider<ServerCommandSource> for GameModeSuggestions {
    fn get_suggestions(&self, context: CommandContext<ServerCommandSource>, mut builder: SuggestionsBuilder) -> Suggestions {
        let remaining = builder.remaining().trim_start().to_owned();
        let lower = remaining.to_ascii_lowercase();
        for mode in all_game_modes() {
            if mode.short_id().starts_with(&lower) { builder = builder.suggest(mode.short_id()); }
        }
        if context.source.has_permission(PermissionId::Privileged) {
            if let Some((player, mode_prefix)) = split_player_prefix(&remaining, &context.source.online_players) {
                if !mode_prefix.is_empty() || remaining.len() != remaining.trim_end().len() {
                    for mode in all_game_modes() {
                        if mode.short_id().starts_with(&mode_prefix.to_ascii_lowercase()) {
                            builder = builder.suggest(&format!("{} {}", player.name, mode.short_id()));
                        }
                    }
                    return builder.build();
                }
            }
            if !remaining.chars().any(char::is_whitespace) {
                for player in &context.source.online_players {
                    if player.name.to_ascii_lowercase().starts_with(&lower) { builder = builder.suggest(&player.name); }
                }
            }
        }
        builder.build()
    }
}

fn apply_game_mode_commands(
    queue: Res<GameModeCommandQueue>,
    players: Res<ServerPlayerRegistry>,
    permissions: Res<ServerPlayerPermissions>,
    mut changes: MessageWriter<SetPlayerGameMode>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations = std::mem::take(&mut *queue.0.lock().expect("game mode command queue lock poisoned"));
    let online = players.players().into_iter().map(|player| CommandPlayer { id: player.id, name: player.name }).collect::<Vec<_>>();
    for invocation in invocations {
        let parsed = parse_game_mode(invocation.source, &invocation.arguments, &online);
        let Ok((target, mode)) = parsed else {
            feedback(invocation.source, parsed.unwrap_err(), &mut chat);
            continue;
        };
        if target != invocation.source && !permissions.has(invocation.source, PermissionId::Privileged) {
            feedback(invocation.source, "This command is not available".to_string(), &mut chat);
            continue;
        }
        changes.write(SetPlayerGameMode { player_id: target, mode });
        let name = players.player(target).map(|player| player.name.as_str()).unwrap_or("unknown player");
        feedback(invocation.source, format!("Set {name} to {} mode", mode.short_id()), &mut chat);
    }
}

fn parse_game_mode(source: u64, arguments: &str, online: &[CommandPlayer]) -> Result<(u64, GameMode), String> {
    let arguments = arguments.trim();
    if let Some(mode) = GameMode::parse(arguments) { return Ok((source, mode)); }
    if let Some((player, remainder)) = split_player_prefix(arguments, online)
        && let Some(mode) = GameMode::parse(remainder.trim())
    {
        return Ok((player.id, mode));
    }
    let modes = all_game_modes().iter().map(|mode| mode.short_id()).collect::<Vec<_>>().join("|");
    Err(format!("Usage: /gamemode [player] <{modes}>"))
}

fn feedback(player_id: u64, text: String, chat: &mut MessageWriter<PublishServerChatMessage>) {
    chat.write(PublishServerChatMessage { audience: Audience::personal(player_id), text });
}
