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
use server_place_block_item_use_lib::{prepare_block_placement,apply_block_placement};
use server_block_placement_api::{BlockPlacementSet,PendingBlockPlacements};
use server_player_gravity_api::{ServerPlayerGravities, ServerPlayerGravityApi};
use server_player_hitbox_api::{
    ServerPlayerHitboxApi, ServerPlayerHitboxSet, ServerPlayerHitboxes,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use generated_permission_registry::PermissionId;
use server_player_permission_api::{ServerPlayerPermissionApi, ServerPlayerPermissions};
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
        PM: ServerPlayerPermissionApi,
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
        _permissions: &mut PM,
    ) -> Self {
        bevy.app.init_resource::<PendingBlockPlacements>()
            .configure_sets(Update,(BlockPlacementSet::Collect,BlockPlacementSet::Validate,BlockPlacementSet::Apply).chain().in_set(InventoryServerSet::ApplyWorldEffects))
            .add_systems(Update,commit_placements.in_set(BlockPlacementSet::Apply))
            .add_systems(
            Update,
            apply_place_block_item::<B>
                .in_set(BlockPlacementSet::Collect)
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
    permissions: Res<ServerPlayerPermissions>,
    mut uses: MessageReader<HeldItemUseDispatched>,
    mut pending: ResMut<PendingBlockPlacements>,
) {
    for item_use in uses.read() {
        if !permissions.has(item_use.player_id, PermissionId::CanInteract) { continue; }
        match prepare_block_placement::<B>(
            &world, &players, &gravities, &hitboxes, &rules, &shapes, item_use,
        ) {
            Ok(Some(outcome)) => {
                pending.0.push(outcome);
            }
            Ok(None) => {}
            Err(error) => debug!("ignored place-block item use: {error:?}"),
        }
    }
}

fn commit_placements(world:Res<ServerChunkWorld>,mut pending:ResMut<PendingBlockPlacements>,mut placed:MessageWriter<ServerBlockPlaced>,mut succeeded:MessageWriter<ItemUseSucceeded>) {
    for intent in std::mem::take(&mut pending.0) {
        match apply_block_placement(&world,&intent) {
            Ok(Some(outcome))=>{placed.write(outcome.placed);succeeded.write(outcome.succeeded);}
            Ok(None)=>{},Err(error)=>debug!("placement rejected at commit: {error:?}"),
        }
    }
}
