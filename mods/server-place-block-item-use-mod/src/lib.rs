use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::ServerBlockPlaced;
use block_edit_events_mod::BlockEditEventsMod;
use inventory_events_api::{HeldItemUseDispatched, InventoryServerSet, ItemUseSucceeded};
use inventory_events_mod::InventoryEventsMod;
use item_use_api::ItemUseTarget;
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_hitbox_api::player_intersects_block;
use server_block_interaction_rules_api::{
    ServerBlockInteractionRules, ServerBlockInteractionRulesApi,
};
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerPlaceBlockItemUseMod;

impl ServerPlaceBlockItemUseMod {
    pub fn init<
        W: ServerChunkWorldApi,
        P: ServerPlayerRegistryApi,
        G: PlayerGravityApi,
        R: ServerBlockInteractionRulesApi,
    >(
        bevy: &mut BevyMod,
        _inventory_events: &mut InventoryEventsMod,
        _block_events: &mut BlockEditEventsMod,
        _world: &mut W,
        _players: &mut P,
        _gravity: &mut G,
        _rules: &mut R,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_place_block_item.in_set(InventoryServerSet::ApplyWorldEffects),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_place_block_item(
    world: Res<ServerChunkWorld>,
    players: Res<ServerPlayerRegistry>,
    gravity: Res<Gravity>,
    rules: Res<ServerBlockInteractionRules>,
    mut uses: MessageReader<HeldItemUseDispatched>,
    mut placed: MessageWriter<ServerBlockPlaced>,
    mut succeeded: MessageWriter<ItemUseSucceeded>,
) {
    for item_use in uses.read() {
        let Some(place_block) = item_use.item.metadata.place_block else {
            continue;
        };
        let ItemUseTarget::Block { adjacent, .. } = item_use.target else {
            continue;
        };
        let Some(actor) = players.player(item_use.player_id) else {
            continue;
        };
        if !rules.player_can_reach(actor.position, gravity.up(), adjacent) {
            continue;
        }
        let Some(scope) = world
            .resident_key_for_player(item_use.player_id, adjacent.chunk())
            .map(|key| key.scope())
        else {
            continue;
        };
        let occupied_by_visible_player = players.players().iter().any(|player| {
            world
                .resident_key_for_player(player.id, adjacent.chunk())
                .is_some_and(|key| key.scope() == scope)
                && player_intersects_block(player.position, adjacent)
        });
        if occupied_by_visible_player {
            continue;
        }
        match world.place_block_for_player(item_use.player_id, adjacent, place_block.block) {
            Ok(mutation) => {
                placed.write(ServerBlockPlaced {
                    player_id: item_use.player_id,
                    scope: mutation.scope,
                    position: mutation.position,
                    block: mutation.current,
                    replaced: mutation.previous,
                });
                succeeded.write(ItemUseSucceeded {
                    player_id: item_use.player_id,
                    cell: item_use.cell.clone(),
                    item_before_use: item_use.item.clone(),
                });
            }
            Err(error) => debug!("ignored place-block item use: {error:?}"),
        }
    }
}
