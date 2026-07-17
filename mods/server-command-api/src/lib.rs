pub use azalea_brigadier as brigadier;

use azalea_brigadier::{
    builder::argument_builder::ArgumentBuilder,
    command_dispatcher::CommandDispatcher,
    context::CommandContext,
    suggestion::{SuggestionProvider, Suggestions, SuggestionsBuilder},
};
use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandPlayer {
    pub id: PlayerId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ServerCommandSource {
    pub player_id: PlayerId,
    pub player_name: String,
    pub online_players: Vec<CommandPlayer>,
}

pub fn player_with_name(players: &[CommandPlayer], name: &str) -> Option<CommandPlayer> {
    players
        .iter()
        .find(|player| player.name.eq_ignore_ascii_case(name.trim()))
        .cloned()
}

pub fn split_player_prefix<'a>(
    input: &'a str,
    players: &[CommandPlayer],
) -> Option<(CommandPlayer, &'a str)> {
    let input = input.trim_start();
    players
        .iter()
        .filter_map(|player| {
            let prefix = input.get(..player.name.len())?;
            if !prefix.eq_ignore_ascii_case(&player.name) {
                return None;
            }
            let remainder = input.get(player.name.len()..)?;
            if !remainder.is_empty() && !remainder.chars().next().is_some_and(char::is_whitespace) {
                return None;
            }
            Some((player.clone(), remainder.trim_start()))
        })
        .max_by_key(|(player, _)| player.name.len())
}

pub struct OnlinePlayerSuggestions;

impl SuggestionProvider<ServerCommandSource> for OnlinePlayerSuggestions {
    fn get_suggestions(
        &self,
        context: CommandContext<ServerCommandSource>,
        mut builder: SuggestionsBuilder,
    ) -> Suggestions {
        let remaining = builder.remaining_lowercase().to_string();
        for player in &context.source.online_players {
            if player.name.to_lowercase().starts_with(&remaining) {
                builder = builder.suggest(&player.name);
            }
        }
        builder.build()
    }
}

#[derive(Resource, Clone)]
pub struct ServerCommandRegistry {
    dispatcher: Arc<RwLock<CommandDispatcher<ServerCommandSource>>>,
}

impl Default for ServerCommandRegistry {
    fn default() -> Self {
        Self {
            dispatcher: Arc::new(RwLock::new(CommandDispatcher::new())),
        }
    }
}

impl ServerCommandRegistry {
    pub fn register(&self, command: ArgumentBuilder<ServerCommandSource>) {
        self.dispatcher
            .write()
            .expect("server command registry lock poisoned")
            .register(command);
    }

    pub fn execute(&self, input: &str, source: ServerCommandSource) -> Result<i32, String> {
        self.dispatcher
            .read()
            .expect("server command registry lock poisoned")
            .execute(input, source)
            .map_err(|error| error.message())
    }

    pub fn suggestions(
        &self,
        input: &str,
        cursor: usize,
        source: ServerCommandSource,
    ) -> Vec<String> {
        let Some(command) = input.strip_prefix('/') else {
            return Vec::new();
        };
        let command_cursor = cursor.saturating_sub(1).min(command.len());
        if !command.is_char_boundary(command_cursor) {
            return Vec::new();
        }
        let dispatcher = self
            .dispatcher
            .read()
            .expect("server command registry lock poisoned");
        let parsed = dispatcher.parse(command.to_string().into(), source);
        CommandDispatcher::get_completion_suggestions_with_cursor(parsed, command_cursor)
            .list()
            .iter()
            .map(|suggestion| format!("/{}", suggestion.apply(command)))
            .collect()
    }
}

pub trait ServerCommandApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;
    use brigadier::{builder::literal_argument_builder::literal, context::CommandContext};

    fn source() -> ServerCommandSource {
        ServerCommandSource {
            player_id: 7,
            player_name: "Player7".to_string(),
            online_players: Vec::new(),
        }
    }

    #[test]
    fn registered_commands_execute_and_complete() {
        let registry = ServerCommandRegistry::default();
        let command: ArgumentBuilder<ServerCommandSource> =
            literal("ping").executes(|_context: &CommandContext<ServerCommandSource>| 7);
        registry.register(command);

        assert_eq!(registry.execute("ping", source()), Ok(7));
        assert_eq!(registry.suggestions("/p", 2, source()), vec!["/ping"]);
    }

    #[test]
    fn longest_player_name_prefix_wins() {
        let players = vec![
            CommandPlayer {
                id: 1,
                name: "Player".to_string(),
            },
            CommandPlayer {
                id: 2,
                name: "Player Two".to_string(),
            },
        ];

        let (player, remainder) = split_player_prefix("Player Two 4.0", &players).unwrap();
        assert_eq!(player.id, 2);
        assert_eq!(remainder, "4.0");
    }
}
