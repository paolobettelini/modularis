use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_scale_api::{
    ServerPlayerScaleApi, ServerPlayerScaleChanged, ServerPlayerScaleSet, ServerPlayerScales,
    SetServerPlayerScale,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerScaleStateMod;

impl ServerPlayerScaleStateMod {
    pub fn init(bevy: &mut BevyMod, _lifecycle: &mut ServerPlayerLifecycleEventsMod) -> Self {
        bevy.app
            .init_resource::<ServerPlayerScales>()
            .add_message::<SetServerPlayerScale>()
            .add_message::<ServerPlayerScaleChanged>()
            .configure_sets(
                Update,
                (ServerPlayerScaleSet::Apply, ServerPlayerScaleSet::Sync).chain(),
            )
            .add_systems(
                Update,
                apply_scale_changes.in_set(ServerPlayerScaleSet::Apply),
            )
            .add_systems(Update, remove_left_players);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerScaleApi for ServerPlayerScaleStateMod {}

fn apply_scale_changes(
    mut scales: ResMut<ServerPlayerScales>,
    mut requests: MessageReader<SetServerPlayerScale>,
    mut changed: MessageWriter<ServerPlayerScaleChanged>,
) {
    for request in requests.read() {
        if !request.scale.is_finite() || request.scale <= 0.0 {
            continue;
        }
        if scales.set(request.player_id, request.scale) {
            changed.write(ServerPlayerScaleChanged {
                player_id: request.player_id,
                scale: request.scale,
            });
        }
    }
}

fn remove_left_players(
    mut scales: ResMut<ServerPlayerScales>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for player in left.read() {
        scales.remove(player.player_id);
    }
}
