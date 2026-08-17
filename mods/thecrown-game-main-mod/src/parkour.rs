use audience_api::Audience;
use bevy::prelude::*;
use block_edit_events_api::{ServerBlockBroken, ServerBlockPlaced};
use block_state_api::BlockState;
use generated_block_registry::BlockId;
use generated_sound_registry::SoundId;
use parkour_gameplay_lib::{ParkourBlockEdit, ParkourConfig, ParkourRun, ParkourUpdate};
use player_network_message_types::PlayerId;
use server_chat_api::PublishServerChatMessage;
use server_kick_api::{ServerKickRequested, ServerKickTarget};
use server_chunk_provider_api::ChunkProviderId;
use server_chunk_routing_api::ServerChunkRoute;
use server_chunk_world_api::{BlockMutation, ServerChunkWorld};
use server_player_gravity_api::SetServerPlayerGravity;
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_registry_api::ServerPlayerMovementApplied;
use server_player_scale_api::SetServerPlayerScale;
use server_player_world_api::RequestServerPlayerWorldChange;
use server_scope_api::{
    ScopeFacetId, ScopeNodeDescriptor, ScopeNodeId, ServerPlayerScopeChanged, ServerScopes,
};
use server_scope_world_api::ServerScopeWorlds;
use server_sound_api::{PlayServerSound, SoundPlayback};
use std::collections::{HashMap, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use thecrown_game_relay_api::{
    RelayParkourRecordLoaded, RelayParkourRecordSubmitted, RequestRelayParkourRecord,
    SubmitRelayParkourRecord, TheCrownRelayRequestIds,
};
use thecrown_game_session_api::TheCrownGamePlayerAdmitted;
use thecrown_protocol::{GameInstanceSpec, GameMode};
use thecrown_world_template_api::TheCrownWorldTemplates;
use world_instance_api::WorldInstanceId;

use crate::{hub, instance_policy::InstancePlayerPolicy};

const THECROWN_SCOPE: &str = "thecrown";

#[derive(Component, Debug, Clone)]
pub struct TheCrownInstanceEntity { pub instance_id: String, pub mode: GameMode }

#[derive(Component, Debug, Clone, Copy)]
pub struct TheCrownPlayerArena { pub player_id: PlayerId }

pub struct InstanceRecord {
    pub spec: GameInstanceSpec,
    pub scope: ScopeNodeId,
    pub world: Option<WorldInstanceId>,
    pub spawn: [f32; 3],
    pub player_policy: InstancePlayerPolicy,
    pub players: HashSet<PlayerId>,
}

pub struct PlayerRecord {
    pub instance_id: String,
    pub scope: ScopeNodeId,
    pub world: WorldInstanceId,
    pub arena_entity: Option<Entity>,
    pub identity: uuid::Uuid,
    pub parkour_record: Option<i32>,
    pub submitted_record: i32,
    pub pending_completed_score: Option<i32>,
}

#[derive(Resource)]
pub struct TheCrownRuntime {
    root: Option<ScopeNodeId>,
    pub instances: HashMap<String, InstanceRecord>,
    pub players: HashMap<PlayerId, PlayerRecord>,
    config: ParkourConfig,
    next_admission_nonce: u64,
}

impl Default for TheCrownRuntime {
    fn default() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or_default();
        Self {
            root: None,
            instances: HashMap::new(),
            players: HashMap::new(),
            config: ParkourConfig::default(),
            next_admission_nonce: seed,
        }
    }
}

impl TheCrownRuntime {
    fn take_admission_nonce(&mut self) -> u64 {
        let nonce = self.next_admission_nonce;
        self.next_admission_nonce = self.next_admission_nonce.wrapping_add(1);
        nonce
    }
}

pub fn setup_root(mut commands: Commands, scopes: Res<ServerScopes>, mut runtime: ResMut<TheCrownRuntime>) {
    let root = ScopeNodeId::new(THECROWN_SCOPE);
    scopes.spawn(&mut commands, ScopeNodeDescriptor::child(THECROWN_SCOPE, scopes.root()))
        .expect("the TheCrown root scope must be unique");
    runtime.root = Some(root);
    info!("TheCrown game server started with no logical instances");
}

pub fn start_instance(
    commands: &mut Commands,
    scopes: &ServerScopes,
    scope_worlds: &ServerScopeWorlds,
    templates: &TheCrownWorldTemplates,
    runtime: &mut TheCrownRuntime,
    spec: &GameInstanceSpec,
) -> Result<(), String> {
    if runtime.instances.contains_key(&spec.instance_id) { return Ok(()); }
    let parent = runtime.root.clone().ok_or_else(|| "TheCrown root is not ready".to_owned())?;
    let scope = ScopeNodeId::new(format!("thecrown:instance:{}", spec.instance_id));
    let descriptor = match spec.mode {
        GameMode::Hub => ScopeNodeDescriptor::child(scope.0.clone(), parent)
            .with_facet(ScopeFacetId::chat()).with_facet(ScopeFacetId::visibility()),
        GameMode::Parkour => ScopeNodeDescriptor::child(scope.0.clone(), parent)
            .with_facet(ScopeFacetId::chat()),
    };
    let entity = scopes.spawn(commands, descriptor).map_err(|error| error.to_string())?;
    commands.entity(entity).insert(TheCrownInstanceEntity { instance_id: spec.instance_id.clone(), mode: spec.mode });

    let (world, spawn) = match spec.mode {
        GameMode::Hub => {
            let world = WorldInstanceId::new(format!("thecrown:hub:{}", spec.instance_id));
            let (spawn, chunks) = hub::load_hub_world()?;
            templates.install_chunks(world.clone(), chunks);
            scope_worlds.bind(scopes, scope.clone(), ServerChunkRoute {
                instance: world.clone(), provider: ChunkProviderId::primary(),
            }).map_err(|error| error.to_string())?;
            (Some(world), spawn)
        }
        GameMode::Parkour => (None, [0.5, 42.0, 0.5]),
    };
    runtime.instances.insert(spec.instance_id.clone(), InstanceRecord {
        spec: spec.clone(),
        scope,
        world,
        spawn,
        player_policy: InstancePlayerPolicy::for_mode(spec.mode),
        players: HashSet::new(),
    });
    info!("started TheCrown {} instance '{}'", spec.mode, spec.instance_id);
    Ok(())
}

pub fn stop_instance(
    commands: &mut Commands,
    scopes: &ServerScopes,
    scope_worlds: &ServerScopeWorlds,
    templates: &TheCrownWorldTemplates,
    world: &ServerChunkWorld,
    runtime: &mut TheCrownRuntime,
    instance_id: &str,
) -> Result<(), String> {
    let Some(instance) = runtime.instances.get(instance_id) else { return Ok(()); };
    if !instance.players.is_empty() {
        return Err(format!("instance still contains {} player(s)", instance.players.len()));
    }
    let instance = runtime.instances.remove(instance_id).expect("instance checked above");
    scope_worlds.unbind(&instance.scope);
    if let Some(world_id) = &instance.world {
        templates.remove(world_id);
        world.discard_instance(world_id);
    }
    let removed = scopes.remove_subtree(&instance.scope).map_err(|error| error.to_string())?;
    for entity in removed.entities { commands.entity(entity).try_despawn(); }
    info!("stopped TheCrown instance '{instance_id}'");
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn assign_admitted_players(
    mut commands: Commands,
    scopes: Res<ServerScopes>,
    scope_worlds: Res<ServerScopeWorlds>,
    world: Res<ServerChunkWorld>,
    time: Res<Time>,
    mut runtime: ResMut<TheCrownRuntime>,
    mut admitted: MessageReader<TheCrownGamePlayerAdmitted>,
    mut scope_changes: MessageWriter<ServerPlayerScopeChanged>,
    mut world_changes: MessageWriter<RequestServerPlayerWorldChange>,
    mut gravity: MessageWriter<SetServerPlayerGravity>,
    mut scale: MessageWriter<SetServerPlayerScale>,
    mut messages: MessageWriter<PublishServerChatMessage>,
    mut kicks: MessageWriter<ServerKickRequested>,
    request_ids: Res<TheCrownRelayRequestIds>,
    mut record_loads: MessageWriter<RequestRelayParkourRecord>,
) {
    for admitted in admitted.read() {
        if runtime.players.contains_key(&admitted.player_id) { continue; }
        let Some(instance) = runtime.instances.get(&admitted.session.instance_id) else {
            warn!("player {} authenticated for unavailable instance '{}'", admitted.player_id, admitted.session.instance_id);
            kicks.write(ServerKickRequested {
                target: ServerKickTarget::Player(admitted.player_id),
                reason: "The assigned TheCrown instance is not available on this server".to_owned(),
            });
            continue;
        };
        if instance.spec.mode != admitted.session.mode {
            warn!("player {} transfer mode does not match instance", admitted.player_id);
            kicks.write(ServerKickRequested {
                target: ServerKickTarget::Player(admitted.player_id),
                reason: "The assigned TheCrown instance has an incompatible mode".to_owned(),
            });
            continue;
        }
        let mode = instance.spec.mode;
        let instance_scope = instance.scope.clone();
        let shared_world = instance.world.clone();
        let shared_spawn = instance.spawn;
        let player_policy = instance.player_policy;
        let admission_nonce = runtime.take_admission_nonce();

        let (player_scope, player_world, arena_entity, spawn) = match mode {
            GameMode::Hub => (instance_scope.clone(), shared_world.expect("Hub has a shared world"), None, shared_spawn),
            GameMode::Parkour => {
                let scope = ScopeNodeId::new(format!("thecrown:instance:{}:player:{}", admitted.session.instance_id, admitted.player_id));
                let entity = scopes.spawn(&mut commands, ScopeNodeDescriptor::child(scope.0.clone(), instance_scope)
                    .with_facet(ScopeFacetId::visibility())).expect("parkour player scope must be unique");
                commands.entity(entity).insert(TheCrownPlayerArena { player_id: admitted.player_id });
                let world_id = WorldInstanceId::new(format!("thecrown:parkour:{}:player:{}", admitted.session.instance_id, admitted.player_id));
                scope_worlds.bind(&scopes, scope.clone(), ServerChunkRoute {
                    instance: world_id.clone(), provider: ChunkProviderId::primary(),
                }).expect("parkour player scope exists");
                let mut run = ParkourRun::new(parkour_seed(&admitted.session.instance_id, admitted.player_id));
                let initial = run.reset(&runtime.config, time.elapsed_secs_f64());
                apply_parkour_edits(&world, admitted.player_id, &initial.edits, None, None);
                let spawn = initial.teleport.expect("parkour reset supplies a spawn");
                commands.entity(entity).insert(run);
                (scope, world_id, Some(entity), spawn)
            }
        };

        let previous = scopes.assign_player(admitted.player_id, player_scope.clone()).expect("player scope exists");
        scope_changes.write(ServerPlayerScopeChanged { player_id: admitted.player_id, previous, current: Some(player_scope.clone()) });
        world_changes.write(RequestServerPlayerWorldChange { player_id: admitted.player_id, world: player_world.clone(), position: spawn });
        gravity.write(SetServerPlayerGravity {
            player_id: admitted.player_id,
            gravity: player_policy.gravity,
        });
        scale.write(SetServerPlayerScale {
            player_id: admitted.player_id,
            scale: player_policy.scale_for_admission(admission_nonce),
        });
        runtime.instances.get_mut(&admitted.session.instance_id).expect("instance still exists").players.insert(admitted.player_id);
        runtime.players.insert(admitted.player_id, PlayerRecord {
            instance_id: admitted.session.instance_id.clone(), scope: player_scope, world: player_world, arena_entity,
            identity: admitted.session.identity.uuid,
            parkour_record: None,
            submitted_record: 0,
            pending_completed_score: None,
        });
        messages.write(PublishServerChatMessage { audience: Audience::personal(admitted.player_id), text: match mode {
            GameMode::Hub => format!("Welcome to TheCrown Hub {}", admitted.session.instance_id),
            GameMode::Parkour => "Welcome to TheCrown Parkour. Reach the next block without falling!".to_owned(),
        }});
        if mode == GameMode::Parkour {
            record_loads.write(RequestRelayParkourRecord {
                request_id: request_ids.next(),
                player_id: admitted.player_id,
                player_uuid: admitted.session.identity.uuid,
            });
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn progress_parkour(
    world: Res<ServerChunkWorld>, time: Res<Time>, mut runtime: ResMut<TheCrownRuntime>,
    mut runs: Query<&mut ParkourRun>, mut movements: MessageReader<ServerPlayerMovementApplied>,
    mut broken: MessageWriter<ServerBlockBroken>, mut placed: MessageWriter<ServerBlockPlaced>,
    mut world_changes: MessageWriter<RequestServerPlayerWorldChange>,
    mut messages: MessageWriter<PublishServerChatMessage>, mut sounds: MessageWriter<PlayServerSound>,
    request_ids: Res<TheCrownRelayRequestIds>,
    mut record_submissions: MessageWriter<SubmitRelayParkourRecord>,
) {
    for movement in movements.read() {
        let Some(arena) = runtime.players.get(&movement.player_id) else { continue; };
        let Some(entity) = arena.arena_entity else { continue; };
        let world_id = arena.world.clone();
        let identity = arena.identity;
        let Ok(mut run) = runs.get_mut(entity) else { continue; };
        let update = run.observe_position(&runtime.config, movement.position, time.elapsed_secs_f64());
        if update.edits.is_empty() && update.teleport.is_none() { continue; }
        apply_parkour_edits(&world, movement.player_id, &update.edits, Some(&mut broken), Some(&mut placed));
        if let Some(position) = update.teleport {
            world_changes.write(RequestServerPlayerWorldChange { player_id: movement.player_id, world: world_id, position });
        }
        publish_score(movement.player_id, &update, &mut messages);
        if let Some(completed_score) = update.completed_score {
            let Some(player) = runtime.players.get_mut(&movement.player_id) else { continue; };
            if player.parkour_record.is_none() {
                player.pending_completed_score = Some(
                    player.pending_completed_score.unwrap_or(0).max(completed_score),
                );
            } else if completed_score > player.submitted_record {
                player.submitted_record = completed_score;
                record_submissions.write(SubmitRelayParkourRecord {
                    request_id: request_ids.next(),
                    player_id: movement.player_id,
                    player_uuid: identity,
                    score: completed_score,
                });
            }
        }
        if update.score_changed && update.teleport.is_none() {
            sounds.write(PlayServerSound {
                audience: Audience::personal(movement.player_id),
                playback: SoundPlayback::new(SoundId::NoteBlockBass).with_volume(1.0)
                    .with_pitch(0.9 + (update.combo - 1) as f32 * 0.05).at(movement.position.to_array()),
            });
        }
    }
}

pub fn apply_record_results(
    mut runtime: ResMut<TheCrownRuntime>,
    request_ids: Res<TheCrownRelayRequestIds>,
    mut loaded: MessageReader<RelayParkourRecordLoaded>,
    mut submitted: MessageReader<RelayParkourRecordSubmitted>,
    mut submissions: MessageWriter<SubmitRelayParkourRecord>,
    mut messages: MessageWriter<PublishServerChatMessage>,
) {
    for result in loaded.read() {
        let Some(player) = runtime.players.get_mut(&result.player_id) else { continue; };
        match &result.result {
            Ok(record) => {
                player.parkour_record = Some(*record);
                player.submitted_record = *record;
                messages.write(PublishServerChatMessage {
                    audience: Audience::personal(result.player_id),
                    text: format!("Your parkour record is {record}."),
                });
                if let Some(completed_score) = player.pending_completed_score.take()
                    && completed_score > player.submitted_record
                {
                    player.submitted_record = completed_score;
                    submissions.write(SubmitRelayParkourRecord {
                        request_id: request_ids.next(),
                        player_id: result.player_id,
                        player_uuid: player.identity,
                        score: completed_score,
                    });
                }
            }
            Err(error) => warn!("could not load parkour record for player {}: {error}", result.player_id),
        }
    }

    for result in submitted.read() {
        let Some(player) = runtime.players.get_mut(&result.player_id) else { continue; };
        match &result.result {
            Ok(update) => {
                player.parkour_record = Some(update.current);
                player.submitted_record = player.submitted_record.max(update.current);
                if update.improved {
                    messages.write(PublishServerChatMessage {
                        audience: Audience::personal(result.player_id),
                        text: format!("New parkour record: {}!", update.current),
                    });
                }
            }
            Err(error) => {
                player.submitted_record = player.parkour_record.unwrap_or(0);
                warn!("could not save parkour record for player {}: {error}", result.player_id);
            }
        }
    }
}

pub fn cleanup_left_players(
    mut commands: Commands, scopes: Res<ServerScopes>, scope_worlds: Res<ServerScopeWorlds>,
    world: Res<ServerChunkWorld>, mut runtime: ResMut<TheCrownRuntime>, mut left: MessageReader<ServerPlayerLeft>,
) {
    for left in left.read() {
        let Some(player) = runtime.players.remove(&left.player_id) else { continue; };
        if let Some(instance) = runtime.instances.get_mut(&player.instance_id) { instance.players.remove(&left.player_id); }
        if player.arena_entity.is_some() {
            scope_worlds.unbind(&player.scope);
            world.discard_instance(&player.world);
            if let Ok(removed) = scopes.remove_subtree(&player.scope) {
                for entity in removed.entities { commands.entity(entity).try_despawn(); }
            }
        }
    }
}

fn publish_score(player_id: PlayerId, update: &ParkourUpdate, messages: &mut MessageWriter<PublishServerChatMessage>) {
    if update.score_changed {
        messages.write(PublishServerChatMessage { audience: Audience::personal(player_id), text: format!("Current score: {} (combo: {})", update.score, update.combo) });
    }
}

fn apply_parkour_edits(
    world: &ServerChunkWorld, player_id: PlayerId, edits: &[ParkourBlockEdit],
    mut broken: Option<&mut MessageWriter<ServerBlockBroken>>, mut placed: Option<&mut MessageWriter<ServerBlockPlaced>>,
) {
    for edit in edits {
        let Ok(mutation) = world.set_block_for_player(player_id, edit.position, BlockState::new(edit.block)) else { continue; };
        if mutation.previous == mutation.current { continue; }
        publish_block_mutation(player_id, mutation, &mut broken, &mut placed);
    }
}

fn publish_block_mutation(
    player_id: PlayerId, mutation: BlockMutation,
    broken: &mut Option<&mut MessageWriter<ServerBlockBroken>>, placed: &mut Option<&mut MessageWriter<ServerBlockPlaced>>,
) {
    if mutation.current.block == BlockId::Air {
        if let Some(writer) = broken.as_mut() { writer.write(ServerBlockBroken { player_id, scope: mutation.scope, position: mutation.position, previous: mutation.previous }); }
    } else if let Some(writer) = placed.as_mut() {
        writer.write(ServerBlockPlaced { player_id, scope: mutation.scope, position: mutation.position, block: mutation.current, replaced: mutation.previous });
    }
}

fn parkour_seed(instance_id: &str, player_id: PlayerId) -> u64 {
    let mut hasher = DefaultHasher::new(); instance_id.hash(&mut hasher); player_id.hash(&mut hasher); hasher.finish()
}
