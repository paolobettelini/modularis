use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use inventory_events_api::{
    InventoryResetRequested, InventorySyncRequested, InventoryValidationSet,
};
use inventory_events_mod::InventoryEventsMod;
use item_manager_api::ItemManagerApi;
use server_inventory_api::{ServerInventories, ServerInventoryApi};
use server_inventory_default_loadout_lib::vanilla_default_reset;
use server_inventory_layout_api::ServerInventoryLayoutApi;
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::ServerPlayerSessionSet;
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct ServerInventoryDefaultLoadoutMod<L, I, B>(PhantomData<(L, I, B)>);

impl<L: ServerInventoryLayoutApi, I: ItemManagerApi, B: BlockManagerApi>
    ServerInventoryDefaultLoadoutMod<L, I, B>
{
    pub fn init<S: ServerInventoryApi>(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _layout: &mut L,
        _items: &mut I,
        _blocks: &mut B,
        _metadata: &mut item_metadata_registry_codegen::ItemMetadataRegistryCodegenMod,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _inventories: &mut S,
        _portal_igniter: &mut item_portal_igniter_meta::ItemPortalIgniterMetaMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            ensure_default_loadout::<L, I, B>
                .in_set(InventoryValidationSet::Initialize)
                .in_set(ServerPlayerSessionSet::Initialize),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn ensure_default_loadout<L: ServerInventoryLayoutApi, I: ItemManagerApi, B: BlockManagerApi>(
    inventories: Res<ServerInventories>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut syncs: MessageReader<InventorySyncRequested>,
    mut resets: MessageWriter<InventoryResetRequested>,
) {
    for event in joined.read() {
        resets.write(vanilla_default_reset::<L, I, B>(event.player_id));
    }
    for event in syncs.read() {
        if inventories.get(event.player_id).is_none() {
            resets.write(vanilla_default_reset::<L, I, B>(event.player_id));
        }
    }
}
