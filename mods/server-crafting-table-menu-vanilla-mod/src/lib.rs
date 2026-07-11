use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{CellMenuId, CellMenuOpenIntent, CellMenuOpenRequested, CellMenuServerSet};
use cell_menu_events_mod::CellMenuEventsMod;
use generated_block_registry::BlockId;
use inventory_core_api::{
    InventoryLayout, InventorySectionId, InventorySectionLayout, InventorySectionRole,
};
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;

const CRAFTING_TABLE_KIND: &str = "demo:crafting-table";

pub struct ServerCraftingTableMenuVanillaMod;

impl ServerCraftingTableMenuVanillaMod {
    pub fn init<W: ServerChunkWorldApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _world: &mut W,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            open_crafting_table_menu.in_set(CellMenuServerSet::Validate),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn open_crafting_table_menu(
    world: Res<ServerChunkWorld>,
    mut intents: MessageReader<CellMenuOpenIntent>,
    mut opens: MessageWriter<CellMenuOpenRequested>,
) {
    for intent in intents.read() {
        if intent.kind != CRAFTING_TABLE_KIND {
            continue;
        }
        let Some(anchor) = intent.anchor else {
            continue;
        };
        if !world
            .block_for_player(intent.player_id, anchor)
            .is_some_and(|block| block.block == BlockId::CraftingTable)
        {
            continue;
        }
        let Some(world_key) = world.resident_key_for_player(intent.player_id, anchor.chunk())
        else {
            continue;
        };
        opens.write(CellMenuOpenRequested {
            player_id: intent.player_id,
            menu_id: crafting_table_menu_id(&world_key, anchor),
            title: "Crafting Table".to_string(),
            audience: Audience::shared(format!(
                "demo:crafting-table:{}:{}:{}:{}:{}",
                world_key.instance, world_key.provider, anchor.x, anchor.y, anchor.z
            )),
            layout: crafting_table_layout(),
        });
    }
}

fn crafting_table_menu_id(
    world: &server_chunk_world_api::ResidentChunkKey,
    position: BlockPos,
) -> CellMenuId {
    CellMenuId::new(format!(
        "demo:crafting-table:{}:{}:{}:{}:{}",
        world.instance, world.provider, position.x, position.y, position.z
    ))
}

fn crafting_table_layout() -> InventoryLayout {
    InventoryLayout {
        sections: vec![InventorySectionLayout {
            id: InventorySectionId::new("crafting"),
            role: InventorySectionRole::Storage,
            columns: 3,
            cells: 9,
        }],
    }
}
