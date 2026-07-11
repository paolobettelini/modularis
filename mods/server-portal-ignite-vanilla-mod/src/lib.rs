use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use inventory_events_api::{HeldItemUseDispatched, InventoryServerSet, ItemUseSucceeded};
use inventory_events_mod::InventoryEventsMod;
use item_use_api::ItemUseTarget;
use portal_api::find_ignitable_frame;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_dimension_api::{Dimension, ServerDimensionApi, ServerDimensions};
use server_portal_api::{
    ActivePortal, ServerPortalApi, ServerPortalOpened, ServerPortalRules, ServerPortalSet,
    ServerPortals,
};
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
        if item_use.item.metadata.portal_igniter.is_none() {
            continue;
        }
        let ItemUseTarget::Block { hit, adjacent, .. } = item_use.target else {
            continue;
        };
        let Some(hit_block) = world.block_for_player(item_use.player_id, hit) else {
            continue;
        };
        let Some(rule) = rules.for_frame_block(hit_block.block) else {
            continue;
        };
        let Some(frame) = find_ignitable_frame(
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
        ) else {
            continue;
        };
        let Some(scope) = world
            .resident_key_for_player(item_use.player_id, frame.origin.chunk())
            .map(|key| key.scope())
        else {
            continue;
        };
        let current = dimensions
            .dimension_id_for(item_use.player_id)
            .unwrap_or(Dimension::Overworld);
        let portal = ActivePortal {
            scope,
            frame,
            frame_block: rule.frame_block,
            destination: rule.destination_from(current),
            destination_position: None,
            color: rule.color,
        };
        if portals.insert(portal.clone()) {
            opened.write(ServerPortalOpened {
                player_id: item_use.player_id,
                portal,
            });
        }
        succeeded.write(ItemUseSucceeded {
            player_id: item_use.player_id,
            cell: item_use.cell.clone(),
            item_before_use: item_use.item.clone(),
        });
    }
}
