use bevy_mod::BevyMod;
use server_chunk_provider_api::ChunkViewer;
use server_chunk_routing_api::{ServerChunkRouter, ServerChunkRoutingApi};
use server_scope_api::{ServerScopeApi, ServerScopes};
use server_scope_world_api::{ServerScopeWorldApi, ServerScopeWorlds};
use tokio::task::JoinHandle;

pub struct ServerChunkRoutingScopesMod;

impl ServerChunkRoutingScopesMod {
    pub fn init<S: ServerScopeApi, W: ServerScopeWorldApi>(
        bevy: &mut BevyMod,
        _scopes_api: &mut S,
        _worlds_api: &mut W,
    ) -> Self {
        let scopes = bevy.app.world().resource::<ServerScopes>().clone();
        let worlds = bevy.app.world().resource::<ServerScopeWorlds>().clone();
        bevy.app.insert_resource(ServerChunkRouter::new(
            move |viewer, _position| match viewer {
                ChunkViewer::Server => worlds.default_route(),
                ChunkViewer::Player(player_id) => worlds.route_for_player(&scopes, player_id),
            },
        ));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerChunkRoutingApi for ServerChunkRoutingScopesMod {}
