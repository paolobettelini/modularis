use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{ServerBlockBroken, ServerBlockEditSet, ServerBlockPlaced};
use block_edit_events_mod::BlockEditEventsMod;
use block_instance_api::BlockInstance;
use generated_block_registry::BlockId;
use parkour_gameplay_lib::{ParkourBlockEdit, ParkourConfig, ParkourRun, ParkourUpdate};
use player_network_message_types::PlayerId;
use server_chat_api::{
    PublishServerChatMessage, ServerChatApi, ServerChatInputReceived, ServerChatSet,
};
use server_chunk_provider_api::ChunkProviderId;
use server_chunk_routing_api::ServerChunkRoute;
use server_chunk_world_api::{BlockMutation, ServerChunkWorld, ServerChunkWorldApi};
use server_player_chat_lib::player_chat_message;
use server_player_lifecycle_events_api::{ServerPlayerJoined, ServerPlayerLeft, ServerPlayerReady};
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{
    ServerPlayerMovementApplied, ServerPlayerMovementSet, ServerPlayerRegistry,
    ServerPlayerRegistryApi, ServerPlayerSessionSet,
};
use server_player_world_api::{
    RequestServerPlayerWorldChange, ServerPlayerWorldApi, ServerPlayerWorldSet,
};
use server_scope_api::{
    ScopeFacetId, ScopeNodeDescriptor, ScopeNodeId, ServerPlayerScopeChanged, ServerScopeApi,
    ServerScopeSet, ServerScopes,
};
use server_scope_world_api::{ServerScopeWorldApi, ServerScopeWorlds};
use std::collections::HashMap;
use tokio::task::JoinHandle;
use world_instance_api::WorldInstanceId;

const THECROWN_SCOPE: &str = "thecrown";
const PREWARMED_PARKOUR_INSTANCES: usize = 2;
const PLAYERS_PER_PARKOUR_INSTANCE: usize = 16;

#[derive(Component, Debug, Clone)]
pub struct TheCrownParkourInstance {
    pub id: u64,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct TheCrownPlayerArena {
    pub player_id: PlayerId,
}

#[derive(Debug, Clone)]
struct ParkourInstanceRecord {
    id: u64,
    scope: ScopeNodeId,
    players: usize,
}

#[derive(Debug, Clone)]
struct PlayerArenaRecord {
    instance_id: u64,
    scope: ScopeNodeId,
    world: WorldInstanceId,
    entity: Entity,
}

#[derive(Resource)]
struct TheCrownRuntime {
    root: Option<ScopeNodeId>,
    instances: Vec<ParkourInstanceRecord>,
    players: HashMap<PlayerId, PlayerArenaRecord>,
    next_instance_id: u64,
    config: ParkourConfig,
}

impl Default for TheCrownRuntime {
    fn default() -> Self {
        Self {
            root: None,
            instances: Vec::new(),
            players: HashMap::new(),
            next_instance_id: 1,
            config: ParkourConfig::default(),
        }
    }
}

pub struct TheCrownMainMod;

impl TheCrownMainMod {
    pub fn init<
        S: ServerScopeApi,
        SW: ServerScopeWorldApi,
        W: ServerChunkWorldApi,
        P: ServerPlayerRegistryApi,
        PW: ServerPlayerWorldApi,
        C: ServerChatApi,
    >(
        bevy: &mut BevyMod,
        _scopes_api: &mut S,
        _scope_worlds_api: &mut SW,
        _world_api: &mut W,
        _players_api: &mut P,
        _player_world_api: &mut PW,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _chat_api: &mut C,
        _block_edits: &mut BlockEditEventsMod,
    ) -> Self {
        bevy.app
            .init_resource::<TheCrownRuntime>()
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
                    .before(ServerBlockEditSet::Sync),
            )
            .add_systems(Update, publish_instance_chat.in_set(ServerChatSet::Publish))
            .add_systems(
                Update,
                cleanup_left_players
                    .after(ServerPlayerSessionSet::Cleanup)
                    .before(ServerScopeSet::Cleanup),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn setup_thecrown(
    mut commands: Commands,
    scopes: Res<ServerScopes>,
    mut runtime: ResMut<TheCrownRuntime>,
) {
    let root = ScopeNodeId::new(THECROWN_SCOPE);
    scopes
        .spawn(
            &mut commands,
            ScopeNodeDescriptor::child(THECROWN_SCOPE, scopes.root()),
        )
        .expect("the TheCrown root scope must be unique");
    runtime.root = Some(root);
    for _ in 0..PREWARMED_PARKOUR_INSTANCES {
        spawn_parkour_instance(&mut commands, &scopes, &mut runtime);
    }
}

fn assign_joined_players(
    mut commands: Commands,
    scopes: Res<ServerScopes>,
    scope_worlds: Res<ServerScopeWorlds>,
    world: Res<ServerChunkWorld>,
    time: Res<Time>,
    mut runtime: ResMut<TheCrownRuntime>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut scope_changes: MessageWriter<ServerPlayerScopeChanged>,
    mut world_changes: MessageWriter<RequestServerPlayerWorldChange>,
) {
    for joined in joined.read() {
        if runtime.players.contains_key(&joined.player_id) {
            continue;
        }
        let instance_index =
            choose_or_spawn_instance(&mut commands, &scopes, &mut runtime, joined.player_id);
        let instance_id = runtime.instances[instance_index].id;
        let parent = runtime.instances[instance_index].scope.clone();
        runtime.instances[instance_index].players += 1;

        let arena_scope = ScopeNodeId::new(format!(
            "thecrown:parkour-{instance_id}:player-{}",
            joined.player_id
        ));
        let entity = scopes
            .spawn(
                &mut commands,
                ScopeNodeDescriptor::child(arena_scope.0.clone(), parent)
                    .with_facet(ScopeFacetId::visibility()),
            )
            .expect("a player arena scope must be unique");
        commands.entity(entity).insert(TheCrownPlayerArena {
            player_id: joined.player_id,
        });

        let world_id = WorldInstanceId::new(format!(
            "thecrown:parkour-{instance_id}:player-{}",
            joined.player_id
        ));
        scope_worlds
            .bind(
                &scopes,
                arena_scope.clone(),
                ServerChunkRoute {
                    instance: world_id.clone(),
                    provider: ChunkProviderId::primary(),
                },
            )
            .expect("the arena scope was created above");
        let previous = scopes
            .assign_player(joined.player_id, arena_scope.clone())
            .expect("the arena scope was created above");
        scope_changes.write(ServerPlayerScopeChanged {
            player_id: joined.player_id,
            previous,
            current: Some(arena_scope.clone()),
        });

        let mut run = ParkourRun::new(parkour_seed(instance_id, joined.player_id));
        let initial = run.reset(&runtime.config, time.elapsed_secs_f64());
        apply_parkour_edits(&world, joined.player_id, &initial.edits, None, None);
        commands.entity(entity).insert(run);
        let spawn = initial
            .teleport
            .expect("a parkour reset always returns a spawn");
        world_changes.write(RequestServerPlayerWorldChange {
            player_id: joined.player_id,
            world: world_id.clone(),
            position: spawn,
        });
        runtime.players.insert(
            joined.player_id,
            PlayerArenaRecord {
                instance_id,
                scope: arena_scope,
                world: world_id,
                entity,
            },
        );
    }
}

fn welcome_ready_players(
    mut ready: MessageReader<ServerPlayerReady>,
    mut messages: MessageWriter<PublishServerChatMessage>,
) {
    for player in ready.read() {
        messages.write(PublishServerChatMessage {
            audience: Audience::personal(player.player_id),
            text: "Welcome to TheCrown Parkour. Reach the next block without falling!".to_string(),
        });
    }
}

fn progress_parkour(
    world: Res<ServerChunkWorld>,
    time: Res<Time>,
    runtime: Res<TheCrownRuntime>,
    mut runs: Query<&mut ParkourRun>,
    mut movements: MessageReader<ServerPlayerMovementApplied>,
    mut broken: MessageWriter<ServerBlockBroken>,
    mut placed: MessageWriter<ServerBlockPlaced>,
    mut world_changes: MessageWriter<RequestServerPlayerWorldChange>,
    mut messages: MessageWriter<PublishServerChatMessage>,
) {
    for movement in movements.read() {
        let Some(arena) = runtime.players.get(&movement.player_id) else {
            continue;
        };
        let Ok(mut run) = runs.get_mut(arena.entity) else {
            continue;
        };
        let update =
            run.observe_position(&runtime.config, movement.position, time.elapsed_secs_f64());
        if update.edits.is_empty() && update.teleport.is_none() {
            continue;
        }
        apply_parkour_edits(
            &world,
            movement.player_id,
            &update.edits,
            Some(&mut broken),
            Some(&mut placed),
        );
        if let Some(position) = update.teleport {
            world_changes.write(RequestServerPlayerWorldChange {
                player_id: movement.player_id,
                world: arena.world.clone(),
                position,
            });
        }
        publish_score(movement.player_id, &update, &mut messages);

        // TODO(audio): publish a domain-level parkour checkpoint sound event
        // once the demo has a generic server-to-client sound contract.
    }
}

fn publish_score(
    player_id: PlayerId,
    update: &ParkourUpdate,
    messages: &mut MessageWriter<PublishServerChatMessage>,
) {
    if !update.score_changed {
        return;
    }
    messages.write(PublishServerChatMessage {
        audience: Audience::personal(player_id),
        text: format!("Current score: {} (combo: {})", update.score, update.combo),
    });
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

fn cleanup_left_players(
    mut commands: Commands,
    scopes: Res<ServerScopes>,
    scope_worlds: Res<ServerScopeWorlds>,
    world: Res<ServerChunkWorld>,
    mut runtime: ResMut<TheCrownRuntime>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for left in left.read() {
        let Some(arena) = runtime.players.remove(&left.player_id) else {
            continue;
        };
        scope_worlds.unbind(&arena.scope);
        world.discard_instance(&arena.world);
        if let Ok(removed) = scopes.remove_subtree(&arena.scope) {
            for entity in removed.entities {
                commands.entity(entity).try_despawn();
            }
        }
        if let Some(instance) = runtime
            .instances
            .iter_mut()
            .find(|instance| instance.id == arena.instance_id)
        {
            instance.players = instance.players.saturating_sub(1);
        }
        remove_idle_dynamic_instance(&mut commands, &scopes, &mut runtime, arena.instance_id);
    }
}

fn choose_or_spawn_instance(
    commands: &mut Commands,
    scopes: &ServerScopes,
    runtime: &mut TheCrownRuntime,
    player_id: PlayerId,
) -> usize {
    let candidates = runtime
        .instances
        .iter()
        .enumerate()
        .filter_map(|(index, instance)| {
            (instance.players < PLAYERS_PER_PARKOUR_INSTANCE).then_some(index)
        })
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        spawn_parkour_instance(commands, scopes, runtime);
        return runtime.instances.len() - 1;
    }
    candidates[mix_player_id(player_id) as usize % candidates.len()]
}

fn spawn_parkour_instance(
    commands: &mut Commands,
    scopes: &ServerScopes,
    runtime: &mut TheCrownRuntime,
) {
    let id = runtime.next_instance_id;
    runtime.next_instance_id += 1;
    let scope = ScopeNodeId::new(format!("thecrown:parkour-{id}"));
    let parent = runtime
        .root
        .clone()
        .expect("TheCrown root must exist before its instances");
    let entity = scopes
        .spawn(
            commands,
            ScopeNodeDescriptor::child(scope.0.clone(), parent).with_facet(ScopeFacetId::chat()),
        )
        .expect("parkour instance ids are monotonic");
    commands
        .entity(entity)
        .insert(TheCrownParkourInstance { id });
    runtime.instances.push(ParkourInstanceRecord {
        id,
        scope,
        players: 0,
    });
}

fn remove_idle_dynamic_instance(
    commands: &mut Commands,
    scopes: &ServerScopes,
    runtime: &mut TheCrownRuntime,
    instance_id: u64,
) {
    if runtime.instances.len() <= PREWARMED_PARKOUR_INSTANCES {
        return;
    }
    let Some(index) = runtime
        .instances
        .iter()
        .position(|instance| instance.id == instance_id && instance.players == 0)
    else {
        return;
    };
    let instance = runtime.instances.remove(index);
    if let Ok(removed) = scopes.remove_subtree(&instance.scope) {
        for entity in removed.entities {
            commands.entity(entity).try_despawn();
        }
    }
}

fn apply_parkour_edits(
    world: &ServerChunkWorld,
    player_id: PlayerId,
    edits: &[ParkourBlockEdit],
    mut broken: Option<&mut MessageWriter<ServerBlockBroken>>,
    mut placed: Option<&mut MessageWriter<ServerBlockPlaced>>,
) {
    for edit in edits {
        let Ok(mutation) =
            world.set_block_for_player(player_id, edit.position, BlockInstance::new(edit.block))
        else {
            continue;
        };
        if mutation.previous == mutation.current {
            continue;
        }
        publish_block_mutation(player_id, mutation, &mut broken, &mut placed);
    }
}

fn publish_block_mutation(
    player_id: PlayerId,
    mutation: BlockMutation,
    broken: &mut Option<&mut MessageWriter<ServerBlockBroken>>,
    placed: &mut Option<&mut MessageWriter<ServerBlockPlaced>>,
) {
    if mutation.current.block == BlockId::Air {
        if let Some(broken) = broken.as_mut() {
            broken.write(ServerBlockBroken {
                player_id,
                scope: mutation.scope,
                position: mutation.position,
                previous: mutation.previous,
            });
        }
    } else if let Some(placed) = placed.as_mut() {
        placed.write(ServerBlockPlaced {
            player_id,
            scope: mutation.scope,
            position: mutation.position,
            block: mutation.current,
            replaced: mutation.previous,
        });
    }
}

fn parkour_seed(instance_id: u64, player_id: PlayerId) -> u64 {
    0x5448_4543_524f_574e_u64
        ^ instance_id.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ player_id.wrapping_mul(0xbf58_476d_1ce4_e5b9)
}

fn mix_player_id(player_id: PlayerId) -> u64 {
    let mut value = player_id.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}
