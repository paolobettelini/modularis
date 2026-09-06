use audience_api::Audience;
use cell_menu_api::{CellMenuId, CellMenuOpenIntent, CellMenuOpenRequested};
use generated_block_registry::BlockId;
use inventory_core_api::{
    InventoryLayout, InventorySectionId, InventorySectionLayout, InventorySectionRole,
};
use server_chunk_world_api::{ResidentChunkKey, ServerChunkWorld};
use voxel_frame_api::VoxelBlockAddress;

pub const CRAFTING_TABLE_MENU_KIND: &str = "demo:crafting-table";

/// Builds a crafting-table menu request if this particular intent points to a
/// valid crafting table.
///
/// The caller chooses when and for whom to invoke it. The vanilla glue invokes
/// it for every matching intent; a custom server may add permissions, phases,
/// teams, quests or arbitrary scope checks before calling it.
pub fn crafting_table_open_request(
    world: &ServerChunkWorld,
    intent: &CellMenuOpenIntent,
) -> Option<CellMenuOpenRequested> {
    if intent.kind != CRAFTING_TABLE_MENU_KIND {
        return None;
    }
    let anchor = intent.anchor?;
    if !world
        .block_for_player(intent.player_id, anchor)
        .is_some_and(|block| block.block == BlockId::CraftingTable)
    {
        return None;
    }
    let world_key = world.resident_key_for_player(intent.player_id, anchor.chunk())?;
    Some(CellMenuOpenRequested {
        player_id: intent.player_id,
        menu_id: crafting_table_menu_id(&world_key, anchor),
        title: "Crafting Table".to_string(),
        audience: crafting_table_audience(&world_key, anchor),
        layout: crafting_table_layout(),
    })
}

/// Opt-in reach policy for callers that use ordinary player interaction.
pub fn crafting_table_open_request_in_reach(
    world:&ServerChunkWorld, intent:&CellMenuOpenIntent,
    players:&server_player_registry_api::ServerPlayerRegistry,
    gravities:&server_player_gravity_api::ServerPlayerGravities,
    hitboxes:&server_player_hitbox_api::ServerPlayerHitboxes,
    rules:server_block_interaction_rules_api::ServerBlockInteractionRules,
)->Option<CellMenuOpenRequested> {
    let anchor=intent.anchor?;
    let player=players.player(intent.player_id)?;
    let key=world.resident_key_for_player(player.id,anchor.chunk())?;
    let pose=world.frames().transform(&key.scope(),anchor.frame)?;
    if !rules.player_can_reach_in_frame(player.position,player_gravity_api::gravity_up(gravities.gravity(player.id)),hitboxes.hitbox(player.id).eye_height,anchor,pose) {return None;}
    crafting_table_open_request(world,intent)
}

pub fn crafting_table_menu_id(world: &ResidentChunkKey, position: VoxelBlockAddress) -> CellMenuId {
    CellMenuId::new(crafting_table_identity(world, position))
}

pub fn crafting_table_audience(world: &ResidentChunkKey, position: VoxelBlockAddress) -> Audience {
    Audience::shared(crafting_table_identity(world, position))
}

pub fn crafting_table_layout() -> InventoryLayout {
    InventoryLayout {
        sections: vec![InventorySectionLayout {
            id: InventorySectionId::new("crafting"),
            role: InventorySectionRole::Storage,
            columns: 3,
            cells: 9,
        }],
    }
}

fn crafting_table_identity(world: &ResidentChunkKey, position: VoxelBlockAddress) -> String {
    format!(
        "demo:crafting-table:{}:{}:{}:{}:{}:{}:{}:{}",
        world.instance.to_string().len(),world.instance, world.provider.to_string().len(),world.provider, position.frame, position.local.x, position.local.y, position.local.z
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn same_local_table_in_different_frames_has_distinct_menu_identity() {
        let key=ResidentChunkKey{
            instance:world_instance_api::WorldInstanceId::new("test:world"),
            provider:server_chunk_provider_api::ChunkProviderId::primary(),
            frame:voxel_frame_api::VoxelFrameId::new(),position:voxel_math_api::ChunkPos::new(0,0,0),
        };
        let first=VoxelBlockAddress::new(key.frame,voxel_math_api::BlockPos::new(1,2,3));
        let second=VoxelBlockAddress::new(voxel_frame_api::VoxelFrameId::new(),first.local);
        assert_ne!(crafting_table_menu_id(&key,first),crafting_table_menu_id(&key,second));
    }
}
