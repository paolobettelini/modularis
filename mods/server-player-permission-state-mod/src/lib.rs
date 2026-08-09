use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_permission_api::{
    ClearPlayerPermissionGrants, ServerPlayerPermissionApi, ServerPlayerPermissionSet,
    ServerPlayerPermissions, ServerPlayerPermissionsChanged, SetPlayerPermission,
};
use server_player_registry_api::ServerPlayerSessionSet;
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ServerPlayerPermissionStateMod;

impl ServerPlayerPermissionStateMod {
    pub fn init(
        bevy: &mut BevyMod,
        _codegen: &mut permission_registry_codegen::PermissionRegistryCodegenMod,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
    ) -> Self {
        bevy.app
            .init_resource::<ServerPlayerPermissions>()
            .add_message::<SetPlayerPermission>()
            .add_message::<ClearPlayerPermissionGrants>()
            .add_message::<ServerPlayerPermissionsChanged>()
            .configure_sets(
                Update,
                (
                    ServerPlayerPermissionSet::Apply,
                    ServerPlayerPermissionSet::DeriveCapabilities,
                    ServerPlayerPermissionSet::Sync,
                )
                    .chain()
                    .after(ServerPlayerSessionSet::Initialize),
            )
            .add_systems(
                Update,
                (clear_permission_grants, apply_permission_changes)
                    .chain()
                    .in_set(ServerPlayerPermissionSet::Apply),
            )
            .add_systems(Update, remove_left_players.after(ServerPlayerPermissionSet::Sync));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ServerPlayerPermissionApi for ServerPlayerPermissionStateMod {}

fn apply_permission_changes(
    mut permissions: ResMut<ServerPlayerPermissions>,
    mut requests: MessageReader<SetPlayerPermission>,
    mut changed: MessageWriter<ServerPlayerPermissionsChanged>,
) {
    for request in requests.read() {
        let before = permissions.effective(request.player_id).into_iter().collect::<HashSet<_>>();
        if !permissions.set(request.player_id, &request.owner, request.permission, request.enabled) {
            continue;
        }
        let effective = permissions.effective(request.player_id);
        let after = effective.iter().copied().collect::<HashSet<_>>();
        changed.write(ServerPlayerPermissionsChanged {
            player_id: request.player_id,
            added: after.difference(&before).copied().collect(),
            removed: before.difference(&after).copied().collect(),
            effective,
        });
    }
}

fn clear_permission_grants(
    mut permissions: ResMut<ServerPlayerPermissions>,
    mut requests: MessageReader<ClearPlayerPermissionGrants>,
    mut changed: MessageWriter<ServerPlayerPermissionsChanged>,
) {
    for request in requests.read() {
        let before = permissions.effective(request.player_id).into_iter().collect::<HashSet<_>>();
        if !permissions.clear_grants(request.player_id, request.permission) {
            continue;
        }
        let effective = permissions.effective(request.player_id);
        let after = effective.iter().copied().collect::<HashSet<_>>();
        changed.write(ServerPlayerPermissionsChanged {
            player_id: request.player_id,
            added: after.difference(&before).copied().collect(),
            removed: before.difference(&after).copied().collect(),
            effective,
        });
    }
}

fn remove_left_players(
    mut permissions: ResMut<ServerPlayerPermissions>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for event in left.read() {
        permissions.remove_player(event.player_id);
    }
}
