use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use inventory_events_api::{HeldItemUseDispatched, InventoryServerSet, ItemUseSucceeded};
use inventory_events_mod::InventoryEventsMod;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_dimension_api::{ServerDimensionApi, ServerDimensions};
use server_portal_api::{
    ServerPortalApi, ServerPortalOpened, ServerPortalRules, ServerPortalSet, ServerPortals,
};
use server_portal_ignite_lib::evaluate_portal_ignition;
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct ServerPortalIgniteVanillaMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ServerPortalIgniteVanillaMod<B> {
    pub fn init<P: ServerPortalApi, W: ServerChunkWorldApi, D: ServerDimensionApi>(
        bevy: &mut BevyMod,
        _inventory: &mut InventoryEventsMod,
        _portals: &mut P,
        _world: &mut W,
        _dimensions: &mut D,
        _blocks: &mut B,
        _igniter: &mut item_portal_igniter_meta::ItemPortalIgniterMetaMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            ignite_portals::<B>
                .in_set(InventoryServerSet::ApplyWorldEffects)
                .in_set(ServerPortalSet::Ignite),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn ignite_portals<B: BlockManagerApi>(
    world: Res<ServerChunkWorld>,
    dimensions: Res<ServerDimensions>,
    rules: Res<ServerPortalRules>,
    mut portals: ResMut<ServerPortals>,
    mut uses: MessageReader<HeldItemUseDispatched>,
    mut opened: MessageWriter<ServerPortalOpened>,
    mut succeeded: MessageWriter<ItemUseSucceeded>,
) {
    for item_use in uses.read() {
        let Some(ignition) = evaluate_portal_ignition::<B>(&world, &dimensions, &rules, item_use)
        else {
            continue;
        };
        if portals.insert(ignition.portal.clone()) {
            opened.write(ServerPortalOpened {
                player_id: item_use.player_id,
                portal: ignition.portal,
            });
        }
        succeeded.write(ignition.succeeded);
    }
}
