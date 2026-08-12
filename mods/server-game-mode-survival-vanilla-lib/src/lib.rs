use generated_permission_registry::PermissionId;
use server_player_game_mode_api::ServerPlayerGameModePolicy;

pub fn survival_policy() -> ServerPlayerGameModePolicy {
    ServerPlayerGameModePolicy {
        clear_permissions: Vec::new(),
        permission_changes: vec![(PermissionId::CanInteract, true)],
        permission_denials: vec![(PermissionId::CanInteract, false)],
        outline_enabled: Some(true),
    }
}
