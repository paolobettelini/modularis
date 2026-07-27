use audience_api::Audience;
use player_network_message_types::PlayerId;
use server_chat_api::PublishServerChatMessage;
use server_player_registry_api::ServerPlayerRegistry;

/// Builds the conventional player chat line without choosing its audience.
///
/// A global vanilla policy, a scoped minigame policy and a custom monolithic
/// server can therefore reuse formatting while retaining full routing control.
pub fn player_chat_message(
    players: &ServerPlayerRegistry,
    player_id: PlayerId,
    text: &str,
    audience: Audience,
) -> Option<PublishServerChatMessage> {
    let player = players.player(player_id)?;
    Some(PublishServerChatMessage {
        audience,
        text: format!("[{}] {text}", player.name),
    })
}
