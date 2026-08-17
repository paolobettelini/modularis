use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{ClientBoundMessage, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use patchwork_game_auth_api::ServerPatchworkPlayerJoined;
use patchwork_game_auth_events_mod::PatchworkGameAuthEventsMod;
use server_kick_api::{ServerKickApi, ServerKickRequested, ServerKickTarget};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use thecrown_auth_relay_api::{
    RequestTheCrownAdmission, TheCrownAdmissionFinished, TheCrownAuthRelayApi,
};
use thecrown_network_message_types::TransferPlayer;
use thecrown_protocol::{AccomodatePlayerData, PlayerIdentity};
use tokio::task::JoinHandle;
use uuid::Uuid;

pub struct TheCrownAuthMainMod;

impl TheCrownAuthMainMod {
    pub fn init<
        R: TheCrownAuthRelayApi,
        E: ServerNetworkEventsApi,
        K: ServerKickApi,
        P: ServerPlayerRegistryApi,
    >(
        bevy: &mut BevyMod,
        _relay: &mut R,
        _events: &mut E,
        _kick: &mut K,
        _players: &mut P,
        _auth_events: &mut PatchworkGameAuthEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(Update, (
            request_relay_admission,
            apply_relay_admission.after(request_relay_admission).after(NetworkMessageSet::DispatchPackets),
        ));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn request_relay_admission(
    mut joined: MessageReader<ServerPatchworkPlayerJoined>,
    mut requests: MessageWriter<RequestTheCrownAdmission>,
    mut kicks: MessageWriter<ServerKickRequested>,
) {
    for joined in joined.read() {
        match Uuid::parse_str(&joined.account.account_uuid) {
            Ok(uuid) => {
                info!("requesting TheCrown admission for {} ({uuid})", joined.account.nickname);
                requests.write(RequestTheCrownAdmission {
                    player_id: joined.player_id,
                    player: PlayerIdentity { uuid, username: joined.account.nickname.clone() },
                });
            }
            Err(error) => {
                kicks.write(ServerKickRequested {
                    target: ServerKickTarget::Player(joined.player_id),
                    reason: format!("Invalid Patchwork account UUID: {error}"),
                });
            }
        }
    }
}

fn apply_relay_admission(
    mut results: MessageReader<TheCrownAdmissionFinished>,
    players: Res<ServerPlayerRegistry>,
    mut packets: MessageWriter<ServerPacketOut>,
    mut kicks: MessageWriter<ServerKickRequested>,
) {
    for result in results.read() {
        if players.player(result.player_id).is_none() { continue; }
        match &result.result {
            Ok(AccomodatePlayerData::Join { transfer }) => {
                packets.write(ServerPacketOut {
                    audience: ServerAudience::Player(result.player_id),
                    message: ClientBoundMessage::TransferPlayer(TransferPlayer { transfer: transfer.clone() }),
                });
                info!("Relay assigned player {} to {}/{}", result.player_id, transfer.server_id, transfer.instance_id);
            }
            Ok(AccomodatePlayerData::Ban { reason, time_left_seconds }) => {
                let suffix = time_left_seconds.map(|seconds| format!(" ({seconds}s remaining)")).unwrap_or_default();
                kicks.write(ServerKickRequested { target: ServerKickTarget::Player(result.player_id), reason: format!("{reason}{suffix}") });
            }
            Ok(AccomodatePlayerData::Unavailable { reason }) => {
                kicks.write(ServerKickRequested { target: ServerKickTarget::Player(result.player_id), reason: reason.clone() });
            }
            Err(error) => {
                kicks.write(ServerKickRequested { target: ServerKickTarget::Player(result.player_id), reason: format!("TheCrown Relay unavailable: {error}") });
            }
        }
    }
}

