use bevy_mod::BevyMod;
use server_player_admission_api::{
    ServerJoinCandidate, ServerPlayerAdmissionRule, ServerPlayerAdmissionRules,
};
use server_player_registry_api::ServerPlayerRegistryApi;
use tokio::task::JoinHandle;

pub struct ServerPlayerNameUniqueVanillaMod;

impl ServerPlayerNameUniqueVanillaMod {
    pub fn init<P: ServerPlayerRegistryApi>(bevy: &mut BevyMod, _players: &mut P) -> Self {
        bevy.app
            .world_mut()
            .resource_mut::<ServerPlayerAdmissionRules>()
            .register(UniquePlayerNameRule);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct UniquePlayerNameRule;

impl ServerPlayerAdmissionRule for UniquePlayerNameRule {
    fn validate(
        &self,
        candidate: &ServerJoinCandidate,
        online: &[player_network_message_types::NetworkPlayer],
    ) -> Result<(), String> {
        if online
            .iter()
            .any(|player| player.name.eq_ignore_ascii_case(&candidate.name))
        {
            Err(format!(
                "A player named '{}' is already connected",
                candidate.name
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use player_network_message_types::NetworkPlayer;
    use std::net::{Ipv4Addr, SocketAddr};

    fn candidate(name: &str) -> ServerJoinCandidate {
        ServerJoinCandidate {
            address: SocketAddr::from((Ipv4Addr::LOCALHOST, 9999)),
            name: name.to_string(),
        }
    }

    fn online(name: &str) -> NetworkPlayer {
        NetworkPlayer {
            id: 1,
            name: name.to_string(),
            position: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    #[test]
    fn rejects_case_insensitive_duplicate_names() {
        let online = [online("Player42")];
        assert!(
            UniquePlayerNameRule
                .validate(&candidate("player42"), &online)
                .is_err()
        );
        assert!(
            UniquePlayerNameRule
                .validate(&candidate("Player7"), &online)
                .is_ok()
        );
    }
}
