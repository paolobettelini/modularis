use generated_block_registry::BlockId;
use player_network_message_types::NetworkPlayer;
use portal_api::{PortalAxis, PortalFrame};
use server_chunk_world_api::ServerChunkWorld;
use server_dimension_api::{Dimension, RequestPlayerDimensionChange, ServerDimensions};
use server_player_hitbox_api::ServerPlayerHitboxes;
use server_portal_api::ServerPortals;
use voxel_math_api::BlockPos;
use world_instance_api::WorldScopeId;

pub const DEFAULT_PORTAL_COOLDOWN_SECONDS: f64 = 1.5;

#[derive(Debug, Clone)]
pub struct PendingReturnPortal {
    pub expected_dimension: Dimension,
    pub source_dimension: Dimension,
    pub source_position: [f32; 3],
    pub frame_block: BlockId,
    pub color: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct PortalTravelDecision {
    pub request: RequestPlayerDimensionChange,
    pub pending_return: Option<PendingReturnPortal>,
}

pub fn detect_portal_travel(
    world: &ServerChunkWorld,
    dimensions: &ServerDimensions,
    hitboxes: &ServerPlayerHitboxes,
    portals: &ServerPortals,
    player: &NetworkPlayer,
) -> Option<PortalTravelDecision> {
    let block = BlockPos::new(
        player.position[0].floor() as i32,
        player.position[1].floor() as i32,
        player.position[2].floor() as i32,
    );
    let scope = world
        .resident_key_for_player(player.id, block.chunk())?
        .scope();
    let hitbox = hitboxes.hitbox(player.id);
    let portal = portals.in_scope(&scope).find(|portal| {
        portal
            .frame
            .contains_player(player.position, hitbox.radius, hitbox.height)
    })?;
    let source_dimension = dimensions.dimension_id_for(player.id)?;
    let pending_return = portal
        .destination_position
        .is_none()
        .then(|| PendingReturnPortal {
            expected_dimension: portal.destination,
            source_dimension,
            source_position: portal.frame.safe_position_beside(),
            frame_block: portal.frame_block,
            color: portal.color,
        });
    Some(PortalTravelDecision {
        request: RequestPlayerDimensionChange {
            player_id: player.id,
            target: portal.destination,
            position: portal.destination_position,
        },
        pending_return,
    })
}

pub fn return_portal_exists(
    portals: &ServerPortals,
    scope: &WorldScopeId,
    pending: &PendingReturnPortal,
) -> bool {
    portals.in_scope(scope).any(|portal| {
        portal.frame_block == pending.frame_block
            && portal.destination == pending.source_dimension
            && portal.destination_position == Some(pending.source_position)
    })
}

pub fn find_return_portal_frame(
    portals: &ServerPortals,
    scope: &WorldScopeId,
    spawn: BlockPos,
) -> Option<PortalFrame> {
    (0..16)
        .map(|slot| PortalFrame {
            origin: BlockPos::new(
                spawn.x - 2 + (slot % 4) * 6,
                spawn.y,
                spawn.z + 3 + (slot / 4) * 6,
            ),
            axis: PortalAxis::X,
        })
        .find(|candidate| {
            !portals
                .in_scope(scope)
                .any(|portal| portal.frame == *candidate)
        })
}
