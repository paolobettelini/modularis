use audience_api::{Audience, AudienceMemberId};
use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::sync::Arc;

pub trait ResolveServerAudience: Send + Sync + 'static {
    fn resolve(&self, audience: &Audience, online: &[AudienceMemberId]) -> Vec<PlayerId>;
}

#[derive(Resource, Clone)]
pub struct ServerAudienceResolver(Arc<dyn ResolveServerAudience>);

impl ServerAudienceResolver {
    pub fn new(resolver: impl ResolveServerAudience) -> Self {
        Self(Arc::new(resolver))
    }

    pub fn resolve(&self, audience: &Audience, online: &[PlayerId]) -> Vec<PlayerId> {
        self.0.resolve(audience, online)
    }
}

pub trait ServerAudienceApi: Send + Sync + 'static {}
