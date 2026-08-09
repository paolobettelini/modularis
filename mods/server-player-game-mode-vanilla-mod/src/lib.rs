use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_game_mode_api::{
    ServerPlayerGameModeApi, ServerPlayerGameModeChanged, ServerPlayerGameModeSet,
};
use server_player_game_mode_vanilla_lib::{
    VANILLA_GAME_MODE_PERMISSION_OWNER, vanilla_game_mode_policy,
};
use server_player_outline_api::{
    ServerPlayerOutlineApi, ServerPlayerOutlineSet, SetPlayerOutline,
};
use server_player_permission_api::{
    ClearPlayerPermissionGrants, ServerPlayerPermissionApi, ServerPlayerPermissionSet,
    SetPlayerPermission,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerGameModeVanillaMod;

impl ServerPlayerGameModeVanillaMod {
    pub fn init<G: ServerPlayerGameModeApi, P: ServerPlayerPermissionApi, O: ServerPlayerOutlineApi>(
        bevy: &mut BevyMod,
        _game_modes: &mut G,
        _permissions: &mut P,
        _outline: &mut O,
    ) -> Self {
        bevy.app
            .add_systems(
                Update,
                apply_vanilla_policy
                    .in_set(ServerPlayerGameModeSet::ApplyPolicy)
                    .before(ServerPlayerPermissionSet::Apply)
                    .before(ServerPlayerOutlineSet::Apply),
            );
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn apply_vanilla_policy(
    mut changes: MessageReader<ServerPlayerGameModeChanged>,
    mut clear_permissions: MessageWriter<ClearPlayerPermissionGrants>,
    mut permissions: MessageWriter<SetPlayerPermission>,
    mut outlines: MessageWriter<SetPlayerOutline>,
) {
    for change in changes.read() {
        let policy = vanilla_game_mode_policy(change.mode);
        for permission in policy.clear_permissions {
            clear_permissions.write(ClearPlayerPermissionGrants {
                player_id: change.player_id,
                permission,
            });
        }
        for (permission, enabled) in policy.permission_changes {
            permissions.write(SetPlayerPermission {
                player_id: change.player_id,
                owner: VANILLA_GAME_MODE_PERMISSION_OWNER.to_string(),
                permission,
                enabled,
            });
        }
        outlines.write(SetPlayerOutline { player_id: change.player_id, enabled: policy.outline_enabled });
    }
}
