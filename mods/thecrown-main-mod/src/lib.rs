mod parkour;
mod web;

use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::ServerBlockEditSet;
use block_edit_events_mod::BlockEditEventsMod;
use parkour::{
    TheCrownRuntime, assign_joined_players, cleanup_left_players, progress_parkour, setup_thecrown,
    welcome_ready_players,
};
use server_chat_api::{
    PublishServerChatMessage, ServerChatApi, ServerChatInputReceived, ServerChatSet,
};
use server_chunk_world_api::ServerChunkWorldApi;
use server_player_chat_lib::player_chat_message;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{
    ServerPlayerMovementSet, ServerPlayerRegistry, ServerPlayerRegistryApi, ServerPlayerSessionSet,
};
use server_player_world_api::{ServerPlayerWorldApi, ServerPlayerWorldSet};
use server_scope_api::{ScopeFacetId, ServerScopeApi, ServerScopeSet, ServerScopes};
use server_scope_world_api::ServerScopeWorldApi;
use server_sound_api::{ServerSoundApi, ServerSoundSet};
use tokio::task::JoinHandle;
use web::{TheCrownDashboard, spawn_dashboard_server, sync_dashboard};

pub use parkour::{TheCrownParkourInstance, TheCrownPlayerArena};

/// The deliberately monolithic orchestration mod for the custom TheCrown
/// server.
///
/// Its internals are split into Rust modules for readability, but they remain
/// one Patchwork mod and one server policy boundary.
pub struct TheCrownMainMod;

impl TheCrownMainMod {
    pub fn init<
        S: ServerScopeApi,
        SW: ServerScopeWorldApi,
        W: ServerChunkWorldApi,
        P: ServerPlayerRegistryApi,
        PW: ServerPlayerWorldApi,
        C: ServerChatApi,
        A: ServerSoundApi,
    >(
        bevy: &mut BevyMod,
        _scopes_api: &mut S,
        _scope_worlds_api: &mut SW,
        _world_api: &mut W,
        _players_api: &mut P,
        _player_world_api: &mut PW,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _chat_api: &mut C,
        _sound_api: &mut A,
        _block_edits: &mut BlockEditEventsMod,
    ) -> Self {
        let dashboard = TheCrownDashboard::default();
        spawn_dashboard_server(dashboard.clone());

        bevy.app
            .init_resource::<TheCrownRuntime>()
            .insert_resource(dashboard)
            .add_systems(Startup, setup_thecrown)
            .add_systems(
                Update,
                assign_joined_players
                    .in_set(ServerPlayerSessionSet::Initialize)
                    .in_set(ServerPlayerWorldSet::Request),
            )
            .add_systems(
                Update,
                welcome_ready_players.after(ServerPlayerSessionSet::Sync),
            )
            .add_systems(
                Update,
                progress_parkour
                    .after(ServerPlayerMovementSet::Apply)
                    .before(ServerPlayerMovementSet::Sync)
                    .before(ServerBlockEditSet::Sync)
                    .in_set(ServerSoundSet::Publish),
            )
            .add_systems(Update, publish_instance_chat.in_set(ServerChatSet::Publish))
            .add_systems(
                Update,
                cleanup_left_players
                    .after(ServerPlayerSessionSet::Cleanup)
                    .before(ServerScopeSet::Cleanup),
            )
            .add_systems(
                Update,
                sync_dashboard
                    .after(assign_joined_players)
                    .after(cleanup_left_players),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn publish_instance_chat(
    scopes: Res<ServerScopes>,
    players: Res<ServerPlayerRegistry>,
    mut inputs: MessageReader<ServerChatInputReceived>,
    mut messages: MessageWriter<PublishServerChatMessage>,
) {
    for input in inputs.read().filter(|input| !input.text.starts_with('/')) {
        let Some(chat_scope) = scopes.resolve_player_facet(input.player_id, &ScopeFacetId::chat())
        else {
            continue;
        };
        if let Some(message) = player_chat_message(
            &players,
            input.player_id,
            &input.text,
            Audience::shared(chat_scope.0),
        ) {
            messages.write(message);
        }
    }
}
