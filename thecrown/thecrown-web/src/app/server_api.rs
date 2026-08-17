use leptos::prelude::*;
use thecrown_protocol::{GameMode, PlayerStatus, RELAY_SUBJECT, RelayPacket, ServerEntry};
use uuid::Uuid;

use super::models::{NetworkOverview, PlayerProfile, TransferRequestResult, ViewerProfile};

#[cfg(feature = "server")]
fn server_error(message: impl Into<String>) -> ServerFnError {
    ServerFnError::new(message.into())
}

#[cfg(feature = "server")]
async fn relay_request(packet: RelayPacket) -> Result<RelayPacket, ServerFnError> {
    let state = expect_context::<crate::state::WebState>();
    state
        .nats
        .request::<_, RelayPacket, _>(RELAY_SUBJECT, &packet)
        .await
        .map_err(|error| server_error(format!("relay request failed: {error:#}")))
}

#[cfg(feature = "server")]
async fn player_status(player_uuid: Uuid) -> Result<PlayerStatus, ServerFnError> {
    match relay_request(RelayPacket::GetPlayerStatus { player_uuid }).await? {
        RelayPacket::ServePlayerStatus { status } => Ok(status),
        other => Err(server_error(format!(
            "relay returned an unexpected packet for player status: {other:?}"
        ))),
    }
}

#[cfg(feature = "server")]
async fn authenticated_player_uuid() -> Result<Option<Uuid>, ServerFnError> {
    use actix_web::HttpRequest;
    use leptos_actix::extract;

    let request: HttpRequest = extract()
        .await
        .map_err(|error| server_error(format!("failed to read request: {error}")))?;
    let Some(uuid_cookie) = request.cookie("player_uuid") else {
        return Ok(None);
    };
    let Some(token_cookie) = request.cookie("token") else {
        return Ok(None);
    };
    let player_uuid = match Uuid::parse_str(uuid_cookie.value()) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(None),
    };

    let state = expect_context::<crate::state::WebState>();
    let db = state.db.clone();
    let token = token_cookie.value().to_owned();
    let valid = tokio::task::spawn_blocking(move || db.user_has_token(player_uuid, &token))
        .await
        .map_err(|error| server_error(format!("session validation task failed: {error}")))?
        .map_err(|error| server_error(format!("session validation failed: {error}")))?;

    Ok(valid.then_some(player_uuid))
}

#[server]
pub async fn get_network_overview() -> Result<NetworkOverview, ServerFnError> {
    let online_players = match relay_request(RelayPacket::GetOnlinePlayers).await? {
        RelayPacket::ServeOnlinePlayers { count } => count,
        other => {
            return Err(server_error(format!(
                "relay returned an unexpected online-player response: {other:?}"
            )));
        }
    };

    let mut hubs = match relay_request(RelayPacket::GetLobbyServers).await? {
        RelayPacket::ServeLobbyServers { servers } => servers,
        other => {
            return Err(server_error(format!(
                "relay returned an unexpected hub-server response: {other:?}"
            )));
        }
    };

    let mut parkour = match relay_request(RelayPacket::GetModeServers {
        mode: GameMode::Parkour,
    })
    .await?
    {
        RelayPacket::ServeModeServers { servers } => servers,
        other => {
            return Err(server_error(format!(
                "relay returned an unexpected mode-server response: {other:?}"
            )));
        }
    };

    hubs.sort_by(|left, right| left.instance_id.cmp(&right.instance_id));
    parkour.sort_by(|left, right| left.instance_id.cmp(&right.instance_id));

    let physical_servers = hubs
        .iter()
        .chain(parkour.iter())
        .map(|entry| entry.server_id.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let active_instances = hubs.len() + parkour.len();

    Ok(NetworkOverview {
        online_players,
        physical_servers,
        active_instances,
        hubs,
        parkour,
    })
}

#[server]
pub async fn get_current_viewer() -> Result<Option<ViewerProfile>, ServerFnError> {
    let Some(player_uuid) = authenticated_player_uuid().await? else {
        return Ok(None);
    };

    let state = expect_context::<crate::state::WebState>();
    let db = state.db.clone();
    let player = tokio::task::spawn_blocking(move || db.get_player(player_uuid))
        .await
        .map_err(|error| server_error(format!("player database task failed: {error}")))?
        .map_err(|error| server_error(format!("failed to load player: {error}")))?;
    let Some(player) = player else {
        return Ok(None);
    };

    Ok(Some(ViewerProfile {
        uuid: player.uuid.hyphenated().to_string(),
        username: player.username,
        status: player_status(player.uuid).await?,
    }))
}

#[server]
pub async fn get_player_profile(identity: String) -> Result<Option<PlayerProfile>, ServerFnError> {
    let identity = identity.trim().to_owned();
    if identity.is_empty() || identity.len() > 128 {
        return Ok(None);
    }

    let state = expect_context::<crate::state::WebState>();
    let db = state.db.clone();
    let identity_for_db = identity.clone();
    let player = tokio::task::spawn_blocking(move || {
        if let Ok(player_uuid) = Uuid::parse_str(&identity_for_db) {
            db.get_player(player_uuid)
        } else {
            db.get_player_by_username(&identity_for_db)
        }
    })
    .await
    .map_err(|error| server_error(format!("player lookup task failed: {error}")))?
    .map_err(|error| server_error(format!("failed to look up player: {error}")))?;

    let Some(player) = player else {
        return Ok(None);
    };
    let status = player_status(player.uuid).await?;

    Ok(Some(PlayerProfile {
        uuid: player.uuid.hyphenated().to_string(),
        username: player.username,
        parkour_record: player.parkour_record,
        status,
    }))
}

#[server]
pub async fn request_transfer_to_instance(
    instance_id: String,
) -> Result<TransferRequestResult, ServerFnError> {
    let instance_id = instance_id.trim().to_owned();
    if instance_id.is_empty() || instance_id.len() > 96 {
        return Ok(TransferRequestResult {
            accepted: false,
            message: "Invalid instance ID.".to_owned(),
        });
    }

    let Some(player_uuid) = authenticated_player_uuid().await? else {
        return Ok(TransferRequestResult {
            accepted: false,
            message: "Log in from Modularis before requesting a transfer.".to_owned(),
        });
    };

    let state = expect_context::<crate::state::WebState>();
    state
        .nats
        .publish(
            RELAY_SUBJECT,
            &RelayPacket::TrySendPlayerToSpecificServer {
                player_uuid,
                instance_id: instance_id.clone(),
            },
        )
        .await
        .map_err(|error| server_error(format!("failed to send transfer request: {error:#}")))?;

    Ok(TransferRequestResult {
        accepted: true,
        message: format!("Transfer request sent for {instance_id}."),
    })
}

pub fn status_label(status: &PlayerStatus) -> &'static str {
    match status {
        PlayerStatus::Offline => "Offline",
        PlayerStatus::Online { .. } => "Online",
    }
}

pub fn status_class(status: &PlayerStatus) -> &'static str {
    match status {
        PlayerStatus::Offline => "status-pill offline",
        PlayerStatus::Online { .. } => "status-pill online",
    }
}

pub fn entry_mode_label(entry: &ServerEntry) -> &'static str {
    match entry.mode {
        GameMode::Hub => "Hub",
        GameMode::Parkour => "Parkour",
    }
}
