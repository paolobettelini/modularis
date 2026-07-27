use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{
    ServerPlayerRegistry, ServerPlayerRegistryApi, ServerPlayerSessionSet,
};
use server_player_world_api::{
    RequestServerPlayerWorldChange, ServerPlayerWorldApi, ServerPlayerWorldChanged,
    ServerPlayerWorldSet, ServerPlayerWorlds,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerWorldStateMod;

impl ServerPlayerWorldStateMod {
    pub fn init<P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _players: &mut P,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
    ) -> Self {
        bevy.app
            .init_resource::<ServerPlayerWorlds>()
            .add_message::<RequestServerPlayerWorldChange>()
            .add_message::<ServerPlayerWorldChanged>()
            .configure_sets(
                Update,
                (
                    ServerPlayerWorldSet::Request,
                    ServerPlayerWorldSet::Apply,
                    ServerPlayerWorldSet::Sync,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                apply_world_changes
                    .in_set(ServerPlayerWorldSet::Apply)
                    .in_set(ServerPlayerSessionSet::Initialize),
            )
            .add_systems(
                Update,
                cleanup_left_players.after(ServerPlayerSessionSet::Cleanup),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerWorldApi for ServerPlayerWorldStateMod {}

fn apply_world_changes(
    worlds: Res<ServerPlayerWorlds>,
    mut players: ResMut<ServerPlayerRegistry>,
    mut requests: MessageReader<RequestServerPlayerWorldChange>,
    mut changed: MessageWriter<ServerPlayerWorldChanged>,
) {
    for request in requests.read() {
        if players
            .set_player_position(request.player_id, request.position)
            .is_none()
        {
            warn!(
                "cannot move unknown player {} to world '{}'",
                request.player_id, request.world
            );
            continue;
        }
        let previous = worlds.set(request.player_id, request.world.clone());
        changed.write(ServerPlayerWorldChanged {
            player_id: request.player_id,
            previous,
            current: request.world.clone(),
            position: request.position,
        });
    }
}

fn cleanup_left_players(
    worlds: Res<ServerPlayerWorlds>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for player in left.read() {
        worlds.remove(player.player_id);
    }
}
