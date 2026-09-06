use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{
    PendingBlockBreaks, ServerBlockBreakRequested, ServerBlockEditSet,
};
use block_edit_events_mod::BlockEditEventsMod;
use generated_permission_registry::PermissionId;
use server_block_breaking_events_api::{ServerBlockBreakingSet, ServerValidatedBlockBreak};
use server_block_edit_world_lib::allow_block_break;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_player_game_mode_api::{ServerPlayerGameModeApi, ServerPlayerGameModes};
use server_player_permission_api::{ServerPlayerPermissionApi, ServerPlayerPermissions};
use tokio::task::JoinHandle;

pub struct ServerBlockBreakingEventsMod;
impl ServerBlockBreakingEventsMod {
    pub fn init<W: ServerChunkWorldApi, P: ServerPlayerPermissionApi, G: ServerPlayerGameModeApi>(
        bevy: &mut BevyMod,
        _events: &mut BlockEditEventsMod,
        _game_modes: &mut G,
        _permissions: &mut P,
        _world: &mut W,
    ) -> Self {
        bevy.app.add_message::<ServerValidatedBlockBreak>()
            .configure_sets(Update, (ServerBlockBreakingSet::Dispatch, ServerBlockBreakingSet::ApplyEffects).chain())
            .add_systems(Update, collect_break_requests.in_set(ServerBlockEditSet::Collect))
            .add_systems(Update, dispatch_validated_breaks
                .in_set(ServerBlockEditSet::Apply)
                .in_set(ServerBlockBreakingSet::Dispatch));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn collect_break_requests(
    permissions: Res<ServerPlayerPermissions>,
    mut requests: MessageReader<ServerBlockBreakRequested>,
    mut pending: ResMut<PendingBlockBreaks>,
) {
    for request in requests.read() {
        if permissions.has(request.player_id, PermissionId::CanInteract) {
            pending.breaks.push(allow_block_break(request));
        } else {
            warn!(
                "rejected block break from player {} at {:?}: missing CanInteract permission",
                request.player_id, request.position
            );
        }
    }
}

fn dispatch_validated_breaks(
    world: Res<ServerChunkWorld>,
    game_modes: Res<ServerPlayerGameModes>,
    mut pending: ResMut<PendingBlockBreaks>,
    mut validated: MessageWriter<ServerValidatedBlockBreak>,
    mut last_logged_target: Local<std::collections::HashMap<u64, voxel_frame_api::VoxelBlockAddress>>,
) {
    for request in std::mem::take(&mut pending.breaks) {
        if !request.allowed {
            warn!(
                "rejected block break from player {} at {:?}: an interaction validator denied it",
                request.player_id, request.position
            );
            continue;
        }
        let Some(mode) = game_modes.get(request.player_id) else {
            warn!(
                "rejected block break from player {} at {:?}: no game mode is assigned",
                request.player_id, request.position
            );
            continue;
        };
        let Some(key) = world.resident_key_for_player(request.player_id, request.position.chunk()) else {
            warn!(
                "rejected block break from player {} at {:?}: no resident chunk is visible in the player's world scope",
                request.player_id, request.position
            );
            continue;
        };
        let Some(block) = world.block_for_player(request.player_id, request.position) else {
            warn!(
                "rejected block break from player {} at {:?}: the scoped world provider returned no block",
                request.player_id, request.position
            );
            continue;
        };
        if last_logged_target.get(&request.player_id) != Some(&request.position) {
            info!(
                "validated block break from player {} at {:?} in {:?} mode",
                request.player_id, request.position, mode
            );
            last_logged_target.insert(request.player_id, request.position);
        }
        validated.write(ServerValidatedBlockBreak { player_id: request.player_id, mode, key, position: request.position, block });
    }
}
