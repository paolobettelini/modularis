use audience_api::Audience;
use cell_menu_api::{CellMenuId, CellMenuOpenIntent, CellMenuOpenRequested};
use generated_block_registry::BlockId;
use inventory_core_api::{
    InventoryLayout, InventorySectionId, InventorySectionLayout, InventorySectionRole,
};
use server_chunk_world_api::{ResidentChunkKey, ServerChunkWorld};
use voxel_math_api::BlockPos;

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

pub fn crafting_table_menu_id(world: &ResidentChunkKey, position: BlockPos) -> CellMenuId {
    CellMenuId::new(crafting_table_identity(world, position))
}

pub fn crafting_table_audience(world: &ResidentChunkKey, position: BlockPos) -> Audience {
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

fn crafting_table_identity(world: &ResidentChunkKey, position: BlockPos) -> String {
    format!(
        "demo:crafting-table:{}:{}:{}:{}:{}",
        world.instance, world.provider, position.x, position.y, position.z
    )
}
