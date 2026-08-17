use thecrown_protocol::WebPacket;

use thecrown_web::state::WebState;

pub async fn handle_web_packet(state: &WebState, packet: WebPacket) -> Option<WebPacket> {
    match packet {
        WebPacket::GenerateAuthToken { player_uuid } => {
            let token = state.generate_auth_token(player_uuid).await;
            Some(WebPacket::ServeAuthToken { token })
        }
        WebPacket::ServeAuthToken { .. } => {
            log::warn!("web backend received response-only WebPacket::ServeAuthToken");
            None
        }
    }
}
