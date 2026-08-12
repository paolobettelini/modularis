use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_game_mode_api::{
    ServerPlayerGameModeApi, ServerPlayerGameModeChanged, ServerPlayerGameModePolicies,
    ServerPlayerGameModeSet,
};
use server_player_outline_api::{ServerPlayerOutlineApi, ServerPlayerOutlineSet, SetPlayerOutline};
use server_player_permission_api::{
    ClearPlayerPermissionGrants, ServerPlayerPermissionApi, ServerPlayerPermissionSet,
    SetPlayerPermission, SetPlayerPermissionDenied,
};
use tokio::task::JoinHandle;

pub const GAME_MODE_PERMISSION_OWNER: &str = "vanilla:game-mode";

pub struct ServerPlayerGameModePolicyMod;

impl ServerPlayerGameModePolicyMod {
    pub fn init<G: ServerPlayerGameModeApi, P: ServerPlayerPermissionApi, O: ServerPlayerOutlineApi>(
        bevy: &mut BevyMod,
        _game_modes: &mut G,
        _permissions: &mut P,
        _outline: &mut O,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_registered_policy
                .in_set(ServerPlayerGameModeSet::ApplyPolicy)
                .before(ServerPlayerPermissionSet::Apply)
                .before(ServerPlayerOutlineSet::Apply),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn apply_registered_policy(
    policies: Res<ServerPlayerGameModePolicies>,
    mut changes: MessageReader<ServerPlayerGameModeChanged>,
    mut clear_permissions: MessageWriter<ClearPlayerPermissionGrants>,
    mut permissions: MessageWriter<SetPlayerPermission>,
    mut denials: MessageWriter<SetPlayerPermissionDenied>,
    mut outlines: MessageWriter<SetPlayerOutline>,
) {
    for change in changes.read() {
        let Some(policy) = policies.get(change.mode) else { continue };
        for permission in &policy.clear_permissions {
            clear_permissions.write(ClearPlayerPermissionGrants {
                player_id: change.player_id,
                permission: *permission,
            });
        }
        for (permission, enabled) in &policy.permission_changes {
            permissions.write(SetPlayerPermission {
                player_id: change.player_id,
                owner: GAME_MODE_PERMISSION_OWNER.to_string(),
                permission: *permission,
                enabled: *enabled,
            });
        }
        for (permission, denied) in &policy.permission_denials {
            denials.write(SetPlayerPermissionDenied {
                player_id: change.player_id,
                owner: GAME_MODE_PERMISSION_OWNER.to_string(),
                permission: *permission,
                denied: *denied,
            });
        }
        if let Some(enabled) = policy.outline_enabled {
            outlines.write(SetPlayerOutline { player_id: change.player_id, enabled });
        }
    }
}
