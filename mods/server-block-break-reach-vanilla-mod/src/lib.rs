use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{PendingBlockBreaks, ServerBlockEditSet};
use block_edit_events_mod::BlockEditEventsMod;
use player_gravity_api::{Gravity, PlayerGravityApi};
use server_block_interaction_rules_api::{
    ServerBlockInteractionRules, ServerBlockInteractionRulesApi,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerBlockBreakReachVanillaMod;

impl ServerBlockBreakReachVanillaMod {
    pub fn init<
        G: PlayerGravityApi,
        R: ServerBlockInteractionRulesApi,
        P: ServerPlayerRegistryApi,
    >(
        bevy: &mut BevyMod,
        _events: &mut BlockEditEventsMod,
        _gravity: &mut G,
        _rules: &mut R,
        _players: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            validate_block_break_reach.in_set(ServerBlockEditSet::Validate),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn validate_block_break_reach(
    players: Res<ServerPlayerRegistry>,
    gravity: Res<Gravity>,
    rules: Res<ServerBlockInteractionRules>,
    mut pending: ResMut<PendingBlockBreaks>,
) {
    for request in &mut pending.breaks {
        if !request.allowed {
            continue;
        }
        request.allowed = players.player(request.player_id).is_some_and(|player| {
            rules.player_can_reach(player.position, gravity.up(), request.position)
        });
    }
}
