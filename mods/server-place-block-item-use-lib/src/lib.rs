use block_edit_events_api::ServerBlockPlaced;
use block_state_api::BlockState;
use block_manager_api::BlockManagerApi;
use block_shape_api::BlockShapeService;
use inventory_events_api::{HeldItemUseDispatched, ItemUseSucceeded};
use item_use_api::ItemUseTarget;
use player_gravity_api::gravity_up;
use server_block_interaction_rules_api::ServerBlockInteractionRules;
use server_chunk_world_api::{ServerChunkWorld, WorldEditError};
use server_player_gravity_api::ServerPlayerGravities;
use server_player_hitbox_api::ServerPlayerHitboxes;
use server_player_registry_api::ServerPlayerRegistry;

#[derive(Debug, Clone)]
pub struct PlaceBlockItemOutcome {
    pub placed: ServerBlockPlaced,
    pub succeeded: ItemUseSucceeded,
}

/// Applies the reusable place-block item mechanic for one dispatched use.
///
/// The function includes the vanilla reach and player-overlap checks but does
/// not decide when it runs. Custom servers can gate it by scope, mode,
/// permissions or any other state before calling it.
pub fn prepare_block_placement<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    players: &ServerPlayerRegistry,
    gravities: &ServerPlayerGravities,
    hitboxes: &ServerPlayerHitboxes,
    rules: &ServerBlockInteractionRules,
    shapes: &BlockShapeService,
    item_use: &HeldItemUseDispatched,
) -> Result<Option<server_block_placement_api::PendingBlockPlacement>, WorldEditError> {
    let Some(place_block) = item_use.item.metadata.place_block else {
        return Ok(None);
    };
    let ItemUseTarget::Block { hit, adjacent, .. } = item_use.target else {
        return Ok(None);
    };
    if hit.frame!=adjacent.frame || (hit.local.x as i64-adjacent.local.x as i64).abs()
        +(hit.local.y as i64-adjacent.local.y as i64).abs()
        +(hit.local.z as i64-adjacent.local.z as i64).abs()!=1 {return Ok(None);}
    let Some(actor) = players.player(item_use.player_id) else {
        return Ok(None);
    };
    let Some(key) = world.resident_key_for_player(actor.id,adjacent.chunk()) else { return Ok(None); };
    let Some(pose) = world.frames().transform(&key.scope(),adjacent.frame) else { return Ok(None); };
    if !rules.player_can_reach_in_frame(
        actor.position,
        gravity_up(gravities.gravity(actor.id)),
        hitboxes.hitbox(actor.id).eye_height,
        adjacent,
        pose,
    ) {
        return Ok(None);
    }
    let Some(scope) = world
        .resident_key_for_player(item_use.player_id, adjacent.chunk())
        .map(|key| key.scope())
    else {
        return Ok(None);
    };
    let placed_shape = shapes.shape(&BlockState::new(place_block.block));
    let occupied_by_visible_player = B::is_solid(place_block.block)
        && players.players().iter().any(|player| {
            let hitbox = hitboxes.hitbox(player.id);
            let player_min = [
                player.position[0] - hitbox.radius,
                player.position[1],
                player.position[2] - hitbox.radius,
            ];
            let player_max = [
                player.position[0] + hitbox.radius,
                player.position[1] + hitbox.height,
                player.position[2] + hitbox.radius,
            ];
            world
                .resident_key_for_player(player.id, adjacent.chunk())
                .is_some_and(|key| key.scope() == scope)
                && placed_shape.boxes().iter().any(|bounds| {
                    let player = voxel_frame_geometry_lib::OrientedVoxelBox::new(
                        voxel_frame_api::VoxelBounds { min: player_min.map(f64::from), max: player_max.map(f64::from) },
                        voxel_frame_api::VoxelFrameTransform::IDENTITY);
                    let block = voxel_frame_geometry_lib::OrientedVoxelBox::new(voxel_frame_geometry_lib::block_bounds(adjacent.local,*bounds),pose);
                    player.overlaps(block)
                })
        });
    if occupied_by_visible_player {
        return Ok(None);
    }

    Ok(Some(server_block_placement_api::PendingBlockPlacement { item_use: item_use.clone(), position: adjacent, block: BlockState::new(place_block.block), allowed: true }))
}

pub fn apply_block_placement(world:&ServerChunkWorld, intent:&server_block_placement_api::PendingBlockPlacement)->Result<Option<PlaceBlockItemOutcome>,WorldEditError> {
    if !intent.allowed { return Ok(None); }
    let item_use=&intent.item_use;
    let mutation=world.place_block_for_player(item_use.player_id,intent.position,intent.block.clone())?;
    Ok(Some(PlaceBlockItemOutcome {
        placed: ServerBlockPlaced { player_id:item_use.player_id, scope:mutation.scope, position:mutation.position, block:mutation.current, replaced:mutation.previous },
        succeeded: ItemUseSucceeded { player_id:item_use.player_id, cell:item_use.cell.clone(), item_before_use:item_use.item.clone() },
    }))
}

/// Explicit custom-orchestration convenience; additional validators can instead
/// be called between prepare_block_placement and apply_block_placement.
pub fn try_place_block_item<B:BlockManagerApi>(
    world:&ServerChunkWorld,players:&ServerPlayerRegistry,gravities:&ServerPlayerGravities,
    hitboxes:&ServerPlayerHitboxes,rules:&ServerBlockInteractionRules,shapes:&BlockShapeService,item_use:&HeldItemUseDispatched
)->Result<Option<PlaceBlockItemOutcome>,WorldEditError> {
    match prepare_block_placement::<B>(world,players,gravities,hitboxes,rules,shapes,item_use)? {
        Some(intent)=>apply_block_placement(world,&intent), None=>Ok(None)
    }
}
