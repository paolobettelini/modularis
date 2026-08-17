mod commands;
mod hub;
mod instance_policy;
mod parkour;

use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::ServerBlockEditSet;
use block_edit_events_mod::BlockEditEventsMod;
use server_chat_api::{
    PublishServerChatMessage, ServerChatApi, ServerChatInputReceived, ServerChatSet,
};
use server_chunk_world_api::ServerChunkWorldApi;
use server_command_api::ServerCommandApi;
use server_external_link_api::ServerExternalLinkApi;
use server_kick_api::ServerKickApi;
use server_player_chat_lib::player_chat_message;
use server_player_gravity_api::{ServerPlayerGravityApi, ServerPlayerGravitySet};
use server_player_registry_api::{
    ServerPlayerMovementSet, ServerPlayerRegistry, ServerPlayerRegistryApi,
    ServerPlayerRelocationSet, ServerPlayerSessionSet,
};
use server_player_scale_api::{ServerPlayerScaleApi, ServerPlayerScaleSet};
use server_player_world_api::{ServerPlayerWorldApi, ServerPlayerWorldSet};
use server_scope_api::{ScopeFacetId, ServerScopeApi, ServerScopeSet, ServerScopes};
use server_scope_world_api::ServerScopeWorldApi;
use server_sound_api::{ServerSoundApi, ServerSoundSet};
use thecrown_game_relay_api::{RelayStartInstance, RelayStopInstance, TheCrownGameRelayApi};
use thecrown_game_session_api::TheCrownGameSessionApi;
use thecrown_world_template_api::TheCrownWorldTemplateApi;
use tokio::task::JoinHandle;

pub struct TheCrownGameMainMod;

impl TheCrownGameMainMod {
    #[allow(clippy::too_many_arguments)]
    pub fn init<
        R: TheCrownGameRelayApi,
        TS: TheCrownGameSessionApi,
        T: TheCrownWorldTemplateApi,
        S: ServerScopeApi,
        SW: ServerScopeWorldApi,
        W: ServerChunkWorldApi,
        P: ServerPlayerRegistryApi,
        K: ServerKickApi,
        PW: ServerPlayerWorldApi,
        G: ServerPlayerGravityApi,
        Z: ServerPlayerScaleApi,
        C: ServerChatApi,
        M: ServerCommandApi,
        A: ServerSoundApi,
        E: ServerExternalLinkApi,
    >(
        bevy: &mut BevyMod,
        _relay: &mut R,
        _sessions: &mut TS,
        _templates: &mut T,
        _scopes: &mut S,
        _scope_worlds: &mut SW,
        _world: &mut W,
        _players: &mut P,
        _kick: &mut K,
        _player_worlds: &mut PW,
        _gravity: &mut G,
        _scale: &mut Z,
        _chat: &mut C,
        _commands: &mut M,
        _sound: &mut A,
        _external_links: &mut E,
        _block_edits: &mut BlockEditEventsMod,
    ) -> Self {
        commands::register(&mut bevy.app);
        bevy.app
            .init_resource::<parkour::TheCrownRuntime>()
            .add_systems(Startup, parkour::setup_root)
            .add_systems(Update, (
                apply_instance_commands,
                parkour::assign_admitted_players
                    .after(apply_instance_commands)
                    .before(ServerPlayerGravitySet::Apply)
                    .before(ServerPlayerScaleSet::Apply)
                    .in_set(ServerPlayerSessionSet::Initialize)
                    .in_set(ServerPlayerWorldSet::Request),
                parkour::progress_parkour
                    .after(ServerPlayerMovementSet::Apply)
                    .before(ServerPlayerRelocationSet::Apply)
                    .before(ServerPlayerMovementSet::Sync)
                    .before(ServerBlockEditSet::Sync)
                    .in_set(ServerSoundSet::Publish),
                parkour::cleanup_left_players
                    .after(ServerPlayerSessionSet::Cleanup)
                    .before(ServerScopeSet::Cleanup),
                publish_instance_chat.in_set(ServerChatSet::Publish),
                commands::apply_command_requests.in_set(ServerChatSet::ApplyGameplay),
                commands::apply_relay_results,
                commands::deliver_relay_whispers,
                parkour::apply_record_results,
            ));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn apply_instance_commands(
    mut commands: Commands,
    scopes: Res<ServerScopes>,
    scope_worlds: Res<server_scope_world_api::ServerScopeWorlds>,
    templates: Res<thecrown_world_template_api::TheCrownWorldTemplates>,
    world: Res<server_chunk_world_api::ServerChunkWorld>,
    mut runtime: ResMut<parkour::TheCrownRuntime>,
    mut starts: MessageReader<RelayStartInstance>,
    mut stops: MessageReader<RelayStopInstance>,
) {
    for start in starts.read() {
        if let Err(error) = parkour::start_instance(
            &mut commands, &scopes, &scope_worlds, &templates, &mut runtime, &start.instance,
        ) { warn!("could not start Relay instance {}: {error}", start.instance.instance_id); }
    }
    for stop in stops.read() {
        if let Err(error) = parkour::stop_instance(
            &mut commands, &scopes, &scope_worlds, &templates, &world, &mut runtime, &stop.instance_id,
        ) { warn!("could not stop Relay instance {}: {error}", stop.instance_id); }
    }
}

fn publish_instance_chat(
    scopes: Res<ServerScopes>,
    players: Res<ServerPlayerRegistry>,
    mut inputs: MessageReader<ServerChatInputReceived>,
    mut messages: MessageWriter<PublishServerChatMessage>,
) {
    for input in inputs.read().filter(|input| !input.text.starts_with('/')) {
        let Some(chat_scope) = scopes.resolve_player_facet(input.player_id, &ScopeFacetId::chat()) else { continue; };
        if let Some(message) = player_chat_message(
            &players, input.player_id, &input.text, Audience::shared(chat_scope.0),
        ) { messages.write(message); }
    }
}
