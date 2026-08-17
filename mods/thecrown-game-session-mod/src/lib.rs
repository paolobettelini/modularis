use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{
    AuthenticateTransferReceived, ClientBoundMessage, NetworkMessageSet,
};
use network_protocol_mod::NetworkProtocolMod;
use network_transport_events_mod::{NetworkTransportEventsMod, ServerTransportDisconnected};
use patchwork_game_auth_api::{ServerAuthenticatedAccounts, ServerPatchworkPlayerJoined};
use patchwork_game_auth_events_mod::PatchworkGameAuthEventsMod;
use server_kick_api::{ServerKickApi, ServerKickRequested, ServerKickTarget};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_admission_api::{ServerJoinCandidate, ServerPlayerAdmissionRule, ServerPlayerAdmissionRules};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, RwLock}};
use thecrown_game_config_api::{TheCrownGameConfig, TheCrownGameConfigApi};
use thecrown_game_relay_api::{
    AuthenticateRelayTransfer, NotifyRelayPlayerQuit, RelayExecuteTransfer,
    RelayTransferAuthenticationFinished, TheCrownGameRelayApi, TheCrownRelayRequestIds,
};
use thecrown_game_session_api::{
    TheCrownGamePlayerAdmitted, TheCrownGamePlayerSession, TheCrownGamePlayerSessions,
    TheCrownGameSessionApi, TransferTheCrownPlayer,
};
use thecrown_network_message_types::{TransferAuthenticated, TransferPlayer};
use thecrown_protocol::{PlayerIdentity, TransferPacketData};
use tokio::task::JoinHandle;
use uuid::Uuid;

#[derive(Clone)]
struct ApprovedTransfer {
    player: PlayerIdentity,
    transfer: TransferPacketData,
}

#[derive(Resource, Clone, Default)]
struct ApprovedTransfers(Arc<RwLock<HashMap<SocketAddr, ApprovedTransfer>>>);

#[derive(Clone)]
struct RequiresRelayTransfer(ApprovedTransfers);

impl ServerPlayerAdmissionRule for RequiresRelayTransfer {
    fn validate(&self, candidate: &ServerJoinCandidate, _online: &[player_network_message_types::NetworkPlayer]) -> Result<(), String> {
        self.0.0.read().expect("approved transfer map poisoned")
            .contains_key(&candidate.address)
            .then_some(())
            .ok_or_else(|| "A valid one-use TheCrown Relay transfer is required".to_owned())
    }
}

pub struct TheCrownGameSessionMod;

impl TheCrownGameSessionMod {
    #[allow(clippy::too_many_arguments)]
    pub fn init<
        C: TheCrownGameConfigApi,
        R: TheCrownGameRelayApi,
        P: ServerPlayerRegistryApi,
        E: ServerNetworkEventsApi,
        K: ServerKickApi,
    >(
        bevy: &mut BevyMod,
        _config_api: &mut C,
        _relay: &mut R,
        _players: &mut P,
        _events: &mut E,
        _kick: &mut K,
        _auth_events: &mut PatchworkGameAuthEventsMod,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _transport_events: &mut NetworkTransportEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        let approved = ApprovedTransfers::default();
        bevy.app.world_mut().resource_mut::<ServerPlayerAdmissionRules>()
            .register(RequiresRelayTransfer(approved.clone()));
        bevy.app
            .insert_resource(approved)
            .init_resource::<TheCrownGamePlayerSessions>()
            .add_message::<TheCrownGamePlayerAdmitted>()
            .add_message::<TransferTheCrownPlayer>()
            .add_systems(Update, (
                request_transfer_auth.after(NetworkMessageSet::DispatchPackets),
                finish_transfer_auth,
                bind_admitted_player,
                handle_external_transfer,
                send_player_transfer,
                cleanup_disconnected_approval,
                cleanup_left_player,
            ));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl TheCrownGameSessionApi for TheCrownGameSessionMod {}

fn request_transfer_auth(
    mut packets: MessageReader<AuthenticateTransferReceived>,
    config: Res<TheCrownGameConfig>,
    accounts: Res<ServerAuthenticatedAccounts>,
    ids: Res<TheCrownRelayRequestIds>,
    mut requests: MessageWriter<AuthenticateRelayTransfer>,
    mut kicks: MessageWriter<ServerKickRequested>,
) {
    for packet in packets.read() {
        let transfer = packet.message.transfer.clone();
        if transfer.server_id != config.server_id || transfer.port != config.public_port {
            kicks.write(ServerKickRequested { target: ServerKickTarget::Address(packet.source), reason: "Transfer destination does not match this game server".to_owned() });
            continue;
        }
        let Some(account) = accounts.account_for_address(packet.source) else {
            kicks.write(ServerKickRequested { target: ServerKickTarget::Address(packet.source), reason: "Patchwork authentication is required before Relay admission".to_owned() });
            continue;
        };
        let Ok(uuid) = Uuid::parse_str(&account.account_uuid) else {
            kicks.write(ServerKickRequested { target: ServerKickTarget::Address(packet.source), reason: "Invalid Patchwork account UUID".to_owned() });
            continue;
        };
        requests.write(AuthenticateRelayTransfer {
            request_id: ids.next(),
            source: packet.source,
            player: PlayerIdentity { uuid, username: account.nickname },
            transfer,
        });
    }
}

fn finish_transfer_auth(
    mut results: MessageReader<RelayTransferAuthenticationFinished>,
    approved: Res<ApprovedTransfers>,
    mut packets: MessageWriter<ServerPacketOut>,
    mut kicks: MessageWriter<ServerKickRequested>,
) {
    for result in results.read() {
        if result.accepted {
            approved.0.write().expect("approved transfer map poisoned").insert(result.source, ApprovedTransfer {
                player: result.player.clone(), transfer: result.transfer.clone(),
            });
            packets.write(ServerPacketOut {
                audience: ServerAudience::Address(result.source),
                message: ClientBoundMessage::TransferAuthenticated(TransferAuthenticated),
            });
            info!("Relay authenticated {} for instance {}", result.player.username, result.transfer.instance_id);
        } else {
            kicks.write(ServerKickRequested {
                target: ServerKickTarget::Address(result.source),
                reason: result.error.clone().unwrap_or_else(|| "Relay transfer was rejected or already consumed".to_owned()),
            });
        }
    }
}

fn bind_admitted_player(
    mut joined: MessageReader<ServerPatchworkPlayerJoined>,
    registry: Res<ServerPlayerRegistry>,
    approved: Res<ApprovedTransfers>,
    mut sessions: ResMut<TheCrownGamePlayerSessions>,
    mut admitted: MessageWriter<TheCrownGamePlayerAdmitted>,
    mut kicks: MessageWriter<ServerKickRequested>,
) {
    for joined in joined.read() {
        let Some(address) = registry.address_for_player(joined.player_id) else { continue; };
        let transfer = approved.0.write().expect("approved transfer map poisoned").remove(&address);
        let Some(approved) = transfer else {
            kicks.write(ServerKickRequested { target: ServerKickTarget::Player(joined.player_id), reason: "Relay admission disappeared before session initialization".to_owned() });
            continue;
        };
        if approved.player.uuid.to_string() != joined.account.account_uuid {
            kicks.write(ServerKickRequested { target: ServerKickTarget::Player(joined.player_id), reason: "Relay and Patchwork identities do not match".to_owned() });
            continue;
        }
        let session = TheCrownGamePlayerSession {
            identity: approved.player,
            instance_id: approved.transfer.instance_id,
            mode: approved.transfer.mode,
        };
        sessions.insert(joined.player_id, session.clone());
        admitted.write(TheCrownGamePlayerAdmitted { player_id: joined.player_id, session });
    }
}

fn handle_external_transfer(
    mut requests: MessageReader<RelayExecuteTransfer>,
    sessions: Res<TheCrownGamePlayerSessions>,
    mut transfers: MessageWriter<TransferTheCrownPlayer>,
) {
    for request in requests.read() {
        let Some(player_id) = sessions.player_for_uuid(request.player_uuid) else { continue; };
        transfers.write(TransferTheCrownPlayer { player_id, transfer: request.transfer.clone() });
    }
}

fn send_player_transfer(
    mut transfers: MessageReader<TransferTheCrownPlayer>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for transfer in transfers.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(transfer.player_id),
            message: ClientBoundMessage::TransferPlayer(TransferPlayer { transfer: transfer.transfer.clone() }),
        });
    }
}

fn cleanup_disconnected_approval(
    mut disconnected: MessageReader<ServerTransportDisconnected>,
    approved: Res<ApprovedTransfers>,
) {
    let mut approvals = approved.0.write().expect("approved transfer map poisoned");
    for disconnected in disconnected.read() {
        approvals.remove(&disconnected.address);
    }
}

fn cleanup_left_player(
    mut left: MessageReader<ServerPlayerLeft>,
    mut sessions: ResMut<TheCrownGamePlayerSessions>,
    mut quits: MessageWriter<NotifyRelayPlayerQuit>,
) {
    for left in left.read() {
        if let Some(session) = sessions.remove(left.player_id) {
            quits.write(NotifyRelayPlayerQuit { player_uuid: session.identity.uuid, instance_id: session.instance_id });
        }
    }
}
