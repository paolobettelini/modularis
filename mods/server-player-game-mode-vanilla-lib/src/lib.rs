use generated_permission_registry::PermissionId;
use server_player_game_mode_api::GameMode;

pub const VANILLA_GAME_MODE_PERMISSION_OWNER: &str = "vanilla:game-mode";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VanillaGameModePolicy {
    pub clear_permissions: Vec<PermissionId>,
    pub permission_changes: Vec<(PermissionId, bool)>,
    pub outline_enabled: bool,
}

/// Pure vanilla policy. Custom servers may invoke this conditionally, alter
/// its output, or ignore it while retaining the generic state APIs.
pub fn vanilla_game_mode_policy(mode: GameMode) -> VanillaGameModePolicy {
    match mode {
        GameMode::Creative => VanillaGameModePolicy {
            clear_permissions: Vec::new(),
            permission_changes: vec![
                (PermissionId::Privileged, true),
                (PermissionId::CanInteract, false),
            ],
            outline_enabled: true,
        },
        GameMode::Survival => VanillaGameModePolicy {
            clear_permissions: vec![PermissionId::Privileged],
            permission_changes: vec![(PermissionId::CanInteract, true)],
            outline_enabled: true,
        },
        GameMode::Adventure => VanillaGameModePolicy {
            clear_permissions: vec![PermissionId::Privileged, PermissionId::CanInteract],
            permission_changes: Vec::new(),
            outline_enabled: false,
        },
    }
}
