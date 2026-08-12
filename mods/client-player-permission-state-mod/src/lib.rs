use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_player_permission_api::{
    ClientPlayerPermissionApi, ClientPlayerPermissions, ClientPlayerPermissionsChanged,
    ClientPlayerPermissionSet,
};
use tokio::task::JoinHandle;

pub struct ClientPlayerPermissionStateMod;

impl ClientPlayerPermissionStateMod {
    pub fn init<G: GameStateApi>(
        bevy: &mut BevyMod,
        _codegen: &mut permission_registry_codegen::PermissionRegistryCodegenMod,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<ClientPlayerPermissions>()
            .add_message::<ClientPlayerPermissionsChanged>()
            .configure_sets(
                Update,
                (ClientPlayerPermissionSet::Receive, ClientPlayerPermissionSet::React).chain(),
            )
            .add_systems(OnExit(GameState::InGame), clear_permissions);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ClientPlayerPermissionApi for ClientPlayerPermissionStateMod {}

fn clear_permissions(mut permissions: ResMut<ClientPlayerPermissions>) {
    permissions.clear();
}
