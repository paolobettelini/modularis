use block_edit_events_api::PendingBlockBreak;
use player_gravity_api::gravity_up;
use server_block_interaction_rules_api::ServerBlockInteractionRules;
use server_player_gravity_api::ServerPlayerGravities;
use server_player_hitbox_api::ServerPlayerHitboxes;
use server_player_registry_api::ServerPlayerRegistry;

pub fn block_break_is_in_reach(
    players: &ServerPlayerRegistry,
    gravities: &ServerPlayerGravities,
    hitboxes: &ServerPlayerHitboxes,
    rules: ServerBlockInteractionRules,
    request: &PendingBlockBreak,
) -> bool {
    request.allowed
        && players.player(request.player_id).is_some_and(|player| {
            rules.player_can_reach_from_eye(
                player.position,
                gravity_up(gravities.gravity(player.id)),
                hitboxes.hitbox(player.id).eye_height,
                request.position,
            )
        })
}
