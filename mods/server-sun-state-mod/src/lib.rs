use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_sun_api::{ServerSunApi, ServerSunChanged, ServerSunSet, ServerSunState, SetServerSun};
use tokio::task::JoinHandle;

pub struct ServerSunStateMod;

impl ServerSunStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ServerSunState>()
            .add_message::<SetServerSun>()
            .add_message::<ServerSunChanged>()
            .configure_sets(Update, (ServerSunSet::Apply, ServerSunSet::Sync).chain())
            .add_systems(Update, apply_sun_settings.in_set(ServerSunSet::Apply));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerSunApi for ServerSunStateMod {}

fn apply_sun_settings(
    mut requests: MessageReader<SetServerSun>,
    mut state: ResMut<ServerSunState>,
    mut changed: MessageWriter<ServerSunChanged>,
) {
    for request in requests.read() {
        if state.current == Some(request.settings) {
            continue;
        }
        let previous = state.current.replace(request.settings);
        changed.write(ServerSunChanged {
            previous,
            current: request.settings,
        });
    }
}
