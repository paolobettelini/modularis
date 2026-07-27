use bevy::prelude::*;
use player_network_message_types::PlayerId;
use server_chunk_routing_api::ServerChunkRoute;
use server_scope_api::{ScopeFacetId, ScopeNodeId, ServerScopes};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Default)]
struct ScopeWorldState {
    default_route: Option<ServerChunkRoute>,
    routes: HashMap<ScopeNodeId, ServerChunkRoute>,
}

/// Associates world routes with scope nodes.
///
/// A child inherits the nearest route in its ancestry. This permits, for
/// example, one parent node to own a shared map while child nodes isolate chat
/// or entity visibility. Binding a route to a child overrides only that
/// subtree.
#[derive(Resource, Clone, Default)]
pub struct ServerScopeWorlds(Arc<RwLock<ScopeWorldState>>);

impl ServerScopeWorlds {
    pub fn set_default_route(&self, route: ServerChunkRoute) {
        self.0
            .write()
            .expect("server scope worlds lock poisoned")
            .default_route = Some(route);
    }

    pub fn default_route(&self) -> Option<ServerChunkRoute> {
        self.0
            .read()
            .expect("server scope worlds lock poisoned")
            .default_route
            .clone()
    }

    pub fn bind(
        &self,
        scopes: &ServerScopes,
        scope: ScopeNodeId,
        route: ServerChunkRoute,
    ) -> Result<Option<ServerChunkRoute>, server_scope_api::ScopeError> {
        scopes.add_facet(&scope, ScopeFacetId::world())?;
        Ok(self
            .0
            .write()
            .expect("server scope worlds lock poisoned")
            .routes
            .insert(scope, route))
    }

    pub fn unbind(&self, scope: &ScopeNodeId) -> Option<ServerChunkRoute> {
        self.0
            .write()
            .expect("server scope worlds lock poisoned")
            .routes
            .remove(scope)
    }

    pub fn route_bound_to(&self, scope: &ScopeNodeId) -> Option<ServerChunkRoute> {
        self.0
            .read()
            .expect("server scope worlds lock poisoned")
            .routes
            .get(scope)
            .cloned()
    }

    pub fn route_for_scope(
        &self,
        scopes: &ServerScopes,
        scope: &ScopeNodeId,
    ) -> Option<ServerChunkRoute> {
        let owner = scopes.resolve_facet(scope, &ScopeFacetId::world())?;
        self.route_bound_to(&owner)
    }

    pub fn route_for_player(
        &self,
        scopes: &ServerScopes,
        player_id: PlayerId,
    ) -> Option<ServerChunkRoute> {
        let scope = scopes.player_scope(player_id)?;
        self.route_for_scope(scopes, &scope)
    }
}

pub trait ServerScopeWorldApi: Send + Sync + 'static {}
