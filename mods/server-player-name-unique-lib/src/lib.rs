use server_player_admission_api::{ServerJoinCandidate, ServerPlayerAdmissionRule};

#[derive(Debug, Clone, Copy, Default)]
pub struct UniquePlayerNameRule;

impl ServerPlayerAdmissionRule for UniquePlayerNameRule {
    fn validate(
        &self,
        candidate: &ServerJoinCandidate,
        online: &[player_network_message_types::NetworkPlayer],
    ) -> Result<(), String> {
        validate_unique_player_name(candidate, online)
    }
}

pub fn validate_unique_player_name(
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
