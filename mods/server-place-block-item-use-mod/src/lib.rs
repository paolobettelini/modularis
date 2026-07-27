use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::ServerBlockPlaced;
use block_edit_events_mod::BlockEditEventsMod;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShapeApi, BlockShapeService};
use inventory_events_api::{HeldItemUseDispatched, InventoryServerSet, ItemUseSucceeded};
use inventory_events_mod::InventoryEventsMod;
use server_block_interaction_rules_api::{
    ServerBlockInteractionRules, ServerBlockInteractionRulesApi,
};
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_place_block_item_use_lib::try_place_block_item;
use server_player_gravity_api::{ServerPlayerGravities, ServerPlayerGravityApi};
use server_player_hitbox_api::{
    ServerPlayerHitboxApi, ServerPlayerHitboxSet, ServerPlayerHitboxes,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct ServerPlaceBlockItemUseMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ServerPlaceBlockItemUseMod<B> {
    pub fn init<
        W: ServerChunkWorldApi,
        P: ServerPlayerRegistryApi,
        G: ServerPlayerGravityApi,
        HB: ServerPlayerHitboxApi,
        R: ServerBlockInteractionRulesApi,
        H: BlockShapeApi,
    >(
        bevy: &mut BevyMod,
        _inventory_events: &mut InventoryEventsMod,
        _block_events: &mut BlockEditEventsMod,
        _world: &mut W,
        _players: &mut P,
        _gravity: &mut G,
        _hitbox: &mut HB,
        _rules: &mut R,
        _blocks: &mut B,
        _shapes: &mut H,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_place_block_item::<B>
                .in_set(InventoryServerSet::ApplyWorldEffects)
                .after(ServerPlayerHitboxSet),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_place_block_item<B: BlockManagerApi>(
    world: Res<ServerChunkWorld>,
    players: Res<ServerPlayerRegistry>,
    gravities: Res<ServerPlayerGravities>,
    hitboxes: Res<ServerPlayerHitboxes>,
    rules: Res<ServerBlockInteractionRules>,
    shapes: Res<BlockShapeService>,
    mut uses: MessageReader<HeldItemUseDispatched>,
    mut placed: MessageWriter<ServerBlockPlaced>,
    mut succeeded: MessageWriter<ItemUseSucceeded>,
) {
    for item_use in uses.read() {
        match try_place_block_item::<B>(
            &world, &players, &gravities, &hitboxes, &rules, &shapes, item_use,
        ) {
            Ok(Some(outcome)) => {
                placed.write(outcome.placed);
                succeeded.write(outcome.succeeded);
            }
            Ok(None) => {}
            Err(error) => debug!("ignored place-block item use: {error:?}"),
        }
    }
}
