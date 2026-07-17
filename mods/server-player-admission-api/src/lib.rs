use bevy::prelude::*;
use player_network_message_types::NetworkPlayer;
use std::{net::SocketAddr, sync::Arc};

#[derive(Debug, Clone)]
pub struct ServerJoinCandidate {
    pub address: SocketAddr,
    pub name: String,
}

pub trait ServerPlayerAdmissionRule: Send + Sync + 'static {
    fn validate(
        &self,
        candidate: &ServerJoinCandidate,
        online: &[NetworkPlayer],
    ) -> Result<(), String>;
}

#[derive(Resource, Default, Clone)]
pub struct ServerPlayerAdmissionRules(Vec<Arc<dyn ServerPlayerAdmissionRule>>);

impl ServerPlayerAdmissionRules {
    pub fn register(&mut self, rule: impl ServerPlayerAdmissionRule) {
        self.0.push(Arc::new(rule));
    }

    pub fn validate(
        &self,
        candidate: &ServerJoinCandidate,
        online: &[NetworkPlayer],
    ) -> Result<(), String> {
        for rule in &self.0 {
            rule.validate(candidate, online)?;
        }
        Ok(())
    }
}
