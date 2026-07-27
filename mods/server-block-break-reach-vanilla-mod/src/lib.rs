use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{PendingBlockBreaks, ServerBlockEditSet};
use block_edit_events_mod::BlockEditEventsMod;
use server_block_break_reach_lib::block_break_is_in_reach;
use server_block_interaction_rules_api::{
    ServerBlockInteractionRules, ServerBlockInteractionRulesApi,
};
use server_player_gravity_api::{ServerPlayerGravities, ServerPlayerGravityApi};
use server_player_hitbox_api::{
    ServerPlayerHitboxApi, ServerPlayerHitboxSet, ServerPlayerHitboxes,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerBlockBreakReachVanillaMod;

impl ServerBlockBreakReachVanillaMod {
    pub fn init<
        G: ServerPlayerGravityApi,
        H: ServerPlayerHitboxApi,
        R: ServerBlockInteractionRulesApi,
        P: ServerPlayerRegistryApi,
    >(
        bevy: &mut BevyMod,
        _events: &mut BlockEditEventsMod,
        _gravity: &mut G,
        _hitbox: &mut H,
        _rules: &mut R,
        _players: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            validate_block_break_reach
                .in_set(ServerBlockEditSet::Validate)
                .after(ServerPlayerHitboxSet),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn validate_block_break_reach(
    players: Res<ServerPlayerRegistry>,
    gravities: Res<ServerPlayerGravities>,
    hitboxes: Res<ServerPlayerHitboxes>,
    rules: Res<ServerBlockInteractionRules>,
    mut pending: ResMut<PendingBlockBreaks>,
) {
    for request in &mut pending.breaks {
        request.allowed = block_break_is_in_reach(&players, &gravities, &hitboxes, *rules, request);
    }
}
