use bevy::prelude::*;
use player_network_message_types::NetworkPlayer;
use std::{net::SocketAddr, sync::Arc};

#[derive(Debug, Clone)]
pub struct ServerJoinCandidate {
    pub address: SocketAddr,
    pub name: String,
}

pub trait ServerPlayerAdmissionRule: Send + Sync + 'static {
    fn prepare(&self, _candidate: &mut ServerJoinCandidate) -> Result<(), String> {
        Ok(())
    }

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
        candidate: &mut ServerJoinCandidate,
        online: &[NetworkPlayer],
    ) -> Result<(), String> {
        for rule in &self.0 {
            rule.prepare(candidate)?;
        }
        for rule in &self.0 {
            rule.validate(candidate, online)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RequiresPreparedName;

    impl ServerPlayerAdmissionRule for RequiresPreparedName {
        fn validate(
            &self,
            candidate: &ServerJoinCandidate,
            _online: &[NetworkPlayer],
        ) -> Result<(), String> {
            (candidate.name == "BackendName")
                .then_some(())
                .ok_or_else(|| "identity was not prepared".to_owned())
        }
    }

    struct BackendIdentity;

    impl ServerPlayerAdmissionRule for BackendIdentity {
        fn prepare(&self, candidate: &mut ServerJoinCandidate) -> Result<(), String> {
            candidate.name = "BackendName".to_owned();
            Ok(())
        }

        fn validate(
            &self,
            _candidate: &ServerJoinCandidate,
            _online: &[NetworkPlayer],
        ) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn prepares_identity_before_any_validation_rule_runs() {
        let mut rules = ServerPlayerAdmissionRules::default();
        rules.register(RequiresPreparedName);
        rules.register(BackendIdentity);
        let mut candidate = ServerJoinCandidate {
            address: "127.0.0.1:9999".parse().unwrap(),
            name: "Anonymous".to_owned(),
        };

        assert!(rules.validate(&mut candidate, &[]).is_ok());
        assert_eq!(candidate.name, "BackendName");
    }
}
