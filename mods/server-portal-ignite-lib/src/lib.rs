use block_manager_api::BlockManagerApi;
use inventory_events_api::{HeldItemUseDispatched, ItemUseSucceeded};
use item_use_api::ItemUseTarget;
use portal_api::find_ignitable_frame;
use server_chunk_world_api::ServerChunkWorld;
use server_dimension_api::{Dimension, ServerDimensions};
use server_portal_api::{ActivePortal, ServerPortalRules};

#[derive(Debug, Clone)]
pub struct PortalIgnition {
    pub portal: ActivePortal,
    pub succeeded: ItemUseSucceeded,
}

/// Evaluates one portal-igniter use without registering or broadcasting it.
///
/// The vanilla glue accepts every valid result. A custom server can inspect
/// scope, player state or destination and decide whether to insert it.
pub fn evaluate_portal_ignition<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    dimensions: &ServerDimensions,
    rules: &ServerPortalRules,
    item_use: &HeldItemUseDispatched,
) -> Option<PortalIgnition> {
    item_use.item.metadata.portal_igniter.as_ref()?;
    let ItemUseTarget::Block { hit, adjacent, .. } = item_use.target else {
        return None;
    };
    let hit_block = world.block_for_player(item_use.player_id, hit)?;
    let rule = rules.for_frame_block(hit_block.block)?;
    let frame = find_ignitable_frame(
        adjacent,
        |position| {
            world
                .block_for_player(item_use.player_id, position)
                .is_some_and(|block| block.block == rule.frame_block)
        },
        |position| {
            world
                .block_for_player(item_use.player_id, position)
                .is_some_and(|block| B::is_air(block.block))
        },
    )?;
    let scope = world
        .resident_key_for_player(item_use.player_id, frame.origin.chunk())?
        .scope();
    let current = dimensions
        .dimension_id_for(item_use.player_id)
        .unwrap_or(Dimension::Overworld);
    Some(PortalIgnition {
        portal: ActivePortal {
            scope,
            frame,
            frame_block: rule.frame_block,
            destination: rule.destination_from(current),
            destination_position: None,
            color: rule.color,
        },
        succeeded: ItemUseSucceeded {
            player_id: item_use.player_id,
            cell: item_use.cell.clone(),
            item_before_use: item_use.item.clone(),
        },
    })
}
