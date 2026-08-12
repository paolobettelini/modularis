use generated_permission_registry::PermissionId;
use server_player_game_mode_api::ServerPlayerGameModePolicy;

pub fn adventure_policy() -> ServerPlayerGameModePolicy {
    ServerPlayerGameModePolicy {
        clear_permissions: vec![PermissionId::CanInteract],
        permission_changes: vec![(PermissionId::CanInteract, false)],
        permission_denials: vec![(PermissionId::CanInteract, true)],
        outline_enabled: Some(false),
    }
}
