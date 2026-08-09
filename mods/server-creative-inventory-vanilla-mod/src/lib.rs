use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use creative_inventory_events_api::CreativeItemTakeRequested;
use creative_inventory_events_mod::CreativeInventoryEventsMod;
use generated_permission_registry::PermissionId;
use inventory_events_api::{InventoryServerSet, InventorySetCellRequested};
use inventory_events_mod::InventoryEventsMod;
use item_manager_api::ItemManagerApi;
use server_give_item_lib::vanilla_creative_item;
use server_inventory_api::{ServerInventories, ServerInventoryApi};
use server_player_permission_api::{ServerPlayerPermissionApi, ServerPlayerPermissions};
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct ServerCreativeInventoryVanillaMod<I, B>(PhantomData<(I, B)>);

impl<I: ItemManagerApi, B: BlockManagerApi> ServerCreativeInventoryVanillaMod<I, B> {
    pub fn init<S: ServerInventoryApi, P: ServerPlayerPermissionApi>(
        bevy: &mut BevyMod,
        _creative_events: &mut CreativeInventoryEventsMod,
        _inventory: &mut S,
        _permissions: &mut P,
        _inventory_events: &mut InventoryEventsMod,
        _items: &mut I,
        _blocks: &mut B,
        _metadata: &mut item_metadata_registry_codegen::ItemMetadataRegistryCodegenMod,
        _quantity: &mut item_quantity_meta::ItemQuantityMetaMod,
        _place_block: &mut item_place_block_meta::ItemPlaceBlockMetaMod,
        _igniter: &mut item_portal_igniter_meta::ItemPortalIgniterMetaMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_creative_item_takes::<I, B>.in_set(InventoryServerSet::Validate),
        );
        Self(PhantomData)
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn apply_creative_item_takes<I: ItemManagerApi, B: BlockManagerApi>(
    permissions: Res<ServerPlayerPermissions>,
    inventories: Res<ServerInventories>,
    mut requests: MessageReader<CreativeItemTakeRequested>,
    mut sets: MessageWriter<InventorySetCellRequested>,
) {
    for request in requests.read() {
        if !permissions.has(request.player_id, PermissionId::Privileged) { continue; }
        let Some(inventory) = inventories.get(request.player_id) else { continue; };
        if !inventory.inventory.layout.contains(&request.to) { continue; }
        sets.write(InventorySetCellRequested {
            player_id: request.player_id,
            cell: request.to.clone(),
            item: Some(vanilla_creative_item::<I, B>(request.item)),
        });
    }
}
