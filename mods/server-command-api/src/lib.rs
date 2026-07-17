pub use azalea_brigadier as brigadier;

use azalea_brigadier::{
    builder::argument_builder::ArgumentBuilder, command_dispatcher::CommandDispatcher,
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
}
