use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{ServerBlockBroken, ServerBlockPlaced};
use block_edit_events_mod::BlockEditEventsMod;
use generated_block_registry::BlockId;
use player_network_message_types::PlayerId;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_dimension_api::{
    RequestPlayerDimensionChange, ServerDimensionApi, ServerDimensionSet,
    ServerPlayerDimensionChanged,
};
use server_player_hitbox_api::{ServerPlayerHitboxApi, ServerPlayerHitboxes};
use server_player_registry_api::{
    ServerPlayerMovementSet, ServerPlayerRegistry, ServerPlayerRegistryApi,
};
use server_portal_api::{
    ActivePortal, ServerPortalApi, ServerPortalOpened, ServerPortalSet, ServerPortals,
};
use server_portal_travel_lib::{
    DEFAULT_PORTAL_COOLDOWN_SECONDS, PendingReturnPortal, detect_portal_travel as decide_travel,
    find_return_portal_frame, return_portal_exists,
};
use std::collections::HashMap;
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;

#[derive(Resource, Default)]
struct PortalTravelState {
    cooldowns: HashMap<PlayerId, f64>,
    pending_returns: HashMap<PlayerId, PendingReturnPortal>,
}

pub struct ServerPortalTravelVanillaMod;

impl ServerPortalTravelVanillaMod {
    pub fn init<
        P: ServerPortalApi,
        D: ServerDimensionApi,
        W: ServerChunkWorldApi,
        R: ServerPlayerRegistryApi,
        H: ServerPlayerHitboxApi,
    >(
        bevy: &mut BevyMod,
        _block_events: &mut BlockEditEventsMod,
        _portals: &mut P,
        _dimensions: &mut D,
        _world: &mut W,
        _players: &mut R,
        _hitboxes: &mut H,
    ) -> Self {
        bevy.app.init_resource::<PortalTravelState>().add_systems(
            Update,
            (
                detect_portal_travel.after(ServerPlayerMovementSet::Apply),
                create_return_portals.after(ServerDimensionSet::Apply),
            )
                .in_set(ServerPortalSet::Travel),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn detect_portal_travel(
    time: Res<Time>,
    world: Res<ServerChunkWorld>,
    dimensions: Res<server_dimension_api::ServerDimensions>,
    players: Res<ServerPlayerRegistry>,
    hitboxes: Res<ServerPlayerHitboxes>,
    portals: Res<ServerPortals>,
    mut state: ResMut<PortalTravelState>,
    mut requests: MessageWriter<RequestPlayerDimensionChange>,
) {
    let now = time.elapsed_secs_f64();
    state
        .cooldowns
        .retain(|player_id, _| players.player(*player_id).is_some());
    state
        .pending_returns
        .retain(|player_id, _| players.player(*player_id).is_some());

    for player in players.players() {
        if state
            .cooldowns
            .get(&player.id)
            .is_some_and(|until| *until > now)
        {
            continue;
        }
        let Some(decision) = decide_travel(&world, &dimensions, &hitboxes, &portals, &player)
        else {
            continue;
        };
        if let Some(pending) = decision.pending_return {
            state.pending_returns.insert(player.id, pending);
        }
        requests.write(decision.request);
        state
            .cooldowns
            .insert(player.id, now + DEFAULT_PORTAL_COOLDOWN_SECONDS);
    }
}

fn create_return_portals(
    world: Res<ServerChunkWorld>,
    mut portals: ResMut<ServerPortals>,
    mut state: ResMut<PortalTravelState>,
    mut changes: MessageReader<ServerPlayerDimensionChanged>,
    mut opened: MessageWriter<ServerPortalOpened>,
    mut placed: MessageWriter<ServerBlockPlaced>,
    mut broken: MessageWriter<ServerBlockBroken>,
) {
    for change in changes.read() {
        let Some(pending) = state.pending_returns.remove(&change.player_id) else {
            continue;
        };
        if change.current.id != pending.expected_dimension {
            continue;
        }
        let spawn = BlockPos::new(
            change.position[0].floor() as i32,
            change.position[1].floor() as i32,
            change.position[2].floor() as i32,
        );
        let Some(scope) = world
            .resident_key_for_player(change.player_id, spawn.chunk())
            .map(|key| key.scope())
        else {
            continue;
        };
        if return_portal_exists(&portals, &scope, &pending) {
            continue;
        }
        let frame = find_return_portal_frame(&portals, &scope, spawn);
        let Some(frame) = frame else {
            warn!("no free return portal slot near the dimension spawn");
            continue;
        };
        for position in frame.required_frame_blocks() {
            if let Ok(mutation) =
                world.set_block_for_player(change.player_id, position, pending.frame_block)
            {
                placed.write(ServerBlockPlaced {
                    player_id: change.player_id,
                    scope: mutation.scope,
                    position: mutation.position,
                    block: mutation.current,
                    replaced: mutation.previous,
                });
            }
        }
        for position in frame.interior_blocks() {
            if let Ok(mutation) =
                world.set_block_for_player(change.player_id, position, BlockId::Air)
                && mutation.previous.block != BlockId::Air
            {
                broken.write(ServerBlockBroken {
                    player_id: change.player_id,
                    scope: mutation.scope,
                    position: mutation.position,
                    previous: mutation.previous,
                });
            }
        }
        let portal = ActivePortal {
            scope,
            frame,
            frame_block: pending.frame_block,
            destination: pending.source_dimension,
            destination_position: Some(pending.source_position),
            color: pending.color,
        };
        if portals.insert(portal.clone()) {
            opened.write(ServerPortalOpened {
                player_id: change.player_id,
                portal,
            });
        }
    }
}
