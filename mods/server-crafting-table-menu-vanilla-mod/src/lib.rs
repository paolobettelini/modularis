use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{CellMenuOpenIntent, CellMenuOpenRequested, CellMenuServerSet};
use cell_menu_events_mod::CellMenuEventsMod;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_crafting_table_menu_lib::crafting_table_open_request;
use tokio::task::JoinHandle;

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
        if let Some(request) = crafting_table_open_request(&world, intent) {
            opens.write(request);
        }
    }
}
