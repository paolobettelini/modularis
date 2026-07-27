use audience_api::{Audience, AudienceMemberId};
use bevy_mod::BevyMod;
use server_audience_api::{ResolveServerAudience, ServerAudienceApi, ServerAudienceResolver};
use server_scope_api::{ScopeNodeId, ServerScopeApi, ServerScopes};
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ServerAudienceScopeImpl;

impl ServerAudienceScopeImpl {
    pub fn init<S: ServerScopeApi>(bevy: &mut BevyMod, _scopes_api: &mut S) -> Self {
        let scopes = bevy.app.world().resource::<ServerScopes>().clone();
        bevy.app
            .insert_resource(ServerAudienceResolver::new(ScopeAudienceResolver {
                scopes,
            }));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerAudienceApi for ServerAudienceScopeImpl {}

struct ScopeAudienceResolver {
    scopes: ServerScopes,
}

impl ResolveServerAudience for ScopeAudienceResolver {
    fn resolve(&self, audience: &Audience, online: &[AudienceMemberId]) -> Vec<AudienceMemberId> {
        match audience {
            Audience::Everyone => online.to_vec(),
            Audience::Personal(player_id) => online
                .contains(player_id)
                .then_some(*player_id)
                .into_iter()
                .collect(),
            Audience::Shared(id) => {
                let online = online.iter().copied().collect::<HashSet<_>>();
                self.scopes
                    .members_in_subtree(&ScopeNodeId::new(id.0.clone()))
                    .into_iter()
                    .filter(|player| online.contains(player))
                    .collect()
            }
        }
    }
}
