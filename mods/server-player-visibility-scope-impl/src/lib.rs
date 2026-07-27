use bevy_mod::BevyMod;
use server_player_visibility_api::{ServerPlayerVisibility, ServerPlayerVisibilityApi};
use server_scope_api::{ScopeFacetId, ServerScopeApi, ServerScopes};
use tokio::task::JoinHandle;

pub struct ServerPlayerVisibilityScopeImpl;

impl ServerPlayerVisibilityScopeImpl {
    pub fn init<S: ServerScopeApi>(bevy: &mut BevyMod, _scopes_api: &mut S) -> Self {
        let scopes = bevy.app.world().resource::<ServerScopes>().clone();
        bevy.app
            .insert_resource(ServerPlayerVisibility::new(move |viewer, subject| {
                let viewer_scope =
                    scopes.resolve_player_facet(viewer.id, &ScopeFacetId::visibility());
                let subject_scope =
                    scopes.resolve_player_facet(subject.id, &ScopeFacetId::visibility());
                viewer_scope.is_some() && viewer_scope == subject_scope
            }));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerVisibilityApi for ServerPlayerVisibilityScopeImpl {}
