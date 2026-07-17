use audience_api::{Audience, AudienceMemberId};
use bevy_mod::BevyMod;
use server_audience_api::{ResolveServerAudience, ServerAudienceApi, ServerAudienceResolver};
use tokio::task::JoinHandle;

pub struct ServerAudienceBasicImpl;

impl ServerAudienceBasicImpl {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .insert_resource(ServerAudienceResolver::new(BasicAudienceResolver));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerAudienceApi for ServerAudienceBasicImpl {}

struct BasicAudienceResolver;

impl ResolveServerAudience for BasicAudienceResolver {
    fn resolve(&self, audience: &Audience, online: &[AudienceMemberId]) -> Vec<AudienceMemberId> {
        match audience {
            Audience::Everyone | Audience::Shared(_) => online.to_vec(),
            Audience::Personal(member) => online
                .contains(member)
                .then_some(*member)
                .into_iter()
                .collect(),
        }
    }
}
