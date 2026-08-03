use block_edit_events_api::ServerBlockPlaced;
use block_instance_api::BlockInstance;
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
pub fn try_place_block_item<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    players: &ServerPlayerRegistry,
    gravities: &ServerPlayerGravities,
    hitboxes: &ServerPlayerHitboxes,
    rules: &ServerBlockInteractionRules,
    shapes: &BlockShapeService,
    item_use: &HeldItemUseDispatched,
) -> Result<Option<PlaceBlockItemOutcome>, WorldEditError> {
    let Some(place_block) = item_use.item.metadata.place_block else {
        return Ok(None);
    };
    let ItemUseTarget::Block { adjacent, .. } = item_use.target else {
        return Ok(None);
    };
    let Some(actor) = players.player(item_use.player_id) else {
        return Ok(None);
    };
    if !rules.player_can_reach_from_eye(
        actor.position,
        gravity_up(gravities.gravity(actor.id)),
        hitboxes.hitbox(actor.id).eye_height,
        adjacent,
    ) {
        return Ok(None);
    }
    let Some(scope) = world
        .resident_key_for_player(item_use.player_id, adjacent.chunk())
        .map(|key| key.scope())
    else {
        return Ok(None);
    };
    let placed_shape = shapes.shape(&BlockInstance::new(place_block.block));
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
                    let block_min = [
                        adjacent.x as f32 + bounds.min.x,
                        adjacent.y as f32 + bounds.min.y,
                        adjacent.z as f32 + bounds.min.z,
                    ];
                    let block_max = [
                        adjacent.x as f32 + bounds.max.x,
                        adjacent.y as f32 + bounds.max.y,
                        adjacent.z as f32 + bounds.max.z,
                    ];
                    overlaps(player_min[0], player_max[0], block_min[0], block_max[0])
                        && overlaps(player_min[1], player_max[1], block_min[1], block_max[1])
                        && overlaps(player_min[2], player_max[2], block_min[2], block_max[2])
                })
        });
    if occupied_by_visible_player {
        return Ok(None);
    }

    let mutation = world.place_block_for_player(item_use.player_id, adjacent, place_block.block)?;
    Ok(Some(PlaceBlockItemOutcome {
        placed: ServerBlockPlaced {
            player_id: item_use.player_id,
            scope: mutation.scope,
            position: mutation.position,
            block: mutation.current,
            replaced: mutation.previous,
        },
        succeeded: ItemUseSucceeded {
            player_id: item_use.player_id,
            cell: item_use.cell.clone(),
            item_before_use: item_use.item.clone(),
        },
    }))
}

fn overlaps(a_min: f32, a_max: f32, b_min: f32, b_max: f32) -> bool {
    a_min < b_max && a_max > b_min
}
