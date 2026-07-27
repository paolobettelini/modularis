use bevy_mod::BevyMod;
use server_scope_api::ServerScopeApi;
use server_scope_world_api::{ServerScopeWorldApi, ServerScopeWorlds};
use tokio::task::JoinHandle;

pub struct ServerScopeWorldStateMod;

impl ServerScopeWorldStateMod {
    pub fn init<S: ServerScopeApi>(bevy: &mut BevyMod, _scopes: &mut S) -> Self {
        bevy.app.init_resource::<ServerScopeWorlds>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerScopeWorldApi for ServerScopeWorldStateMod {}
