use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{CellMenuOpenIntent, CellMenuOpenRequested, CellMenuServerSet};
use cell_menu_events_mod::CellMenuEventsMod;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_crafting_table_menu_lib::crafting_table_open_request_in_reach;
use generated_permission_registry::PermissionId;
use server_player_permission_api::{ServerPlayerPermissionApi, ServerPlayerPermissions};
use tokio::task::JoinHandle;

pub struct ServerCraftingTableMenuVanillaMod;

impl ServerCraftingTableMenuVanillaMod {
    pub fn init<W: ServerChunkWorldApi, P: ServerPlayerPermissionApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _world: &mut W,
        _permissions: &mut P,
        _players:&mut impl server_player_registry_api::ServerPlayerRegistryApi,
        _gravity:&mut impl server_player_gravity_api::ServerPlayerGravityApi,
        _hitboxes:&mut impl server_player_hitbox_api::ServerPlayerHitboxApi,
        _rules:&mut impl server_block_interaction_rules_api::ServerBlockInteractionRulesApi,
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
    permissions: Res<ServerPlayerPermissions>,
    players:Res<server_player_registry_api::ServerPlayerRegistry>,
    gravities:Res<server_player_gravity_api::ServerPlayerGravities>,
    hitboxes:Res<server_player_hitbox_api::ServerPlayerHitboxes>,
    rules:Res<server_block_interaction_rules_api::ServerBlockInteractionRules>,
    mut intents: MessageReader<CellMenuOpenIntent>,
    mut opens: MessageWriter<CellMenuOpenRequested>,
) {
    for intent in intents.read() {
        if !permissions.has(intent.player_id, PermissionId::CanInteract) { continue; }
        if let Some(request) = crafting_table_open_request_in_reach(&world,intent,&players,&gravities,&hitboxes,*rules) {
            opens.write(request);
        }
    }
}
