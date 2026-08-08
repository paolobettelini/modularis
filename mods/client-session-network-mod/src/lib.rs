use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use client_session_api::{ClientSession, ClientSessionApi};
use generated_network_messages::{JoinAcceptedReceived, NetworkMessageSet, ServerBoundMessage};
use network_protocol_mod::NetworkProtocolMod;
use patchwork_game_auth_api::ClientPatchworkJoinGate;
use session_network_message_types::{JoinRequest, LeaveRequest};
use tokio::task::JoinHandle;

#[derive(Resource, Default)]
struct PendingJoin(bool);

pub struct ClientSessionNetworkMod;

impl ClientSessionNetworkMod {
    pub fn init<N: ClientNetworkApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _network: &mut N,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .init_resource::<ClientSession>()
            .init_resource::<PendingJoin>()
            .add_systems(OnEnter(GameState::InGame), begin_join)
            .add_systems(
                Update,
                (
                    send_pending_join,
                    accept_join.after(NetworkMessageSet::DispatchPackets),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), leave_server);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientSessionApi for ClientSessionNetworkMod {}

fn begin_join(mut pending: ResMut<PendingJoin>, mut session: ResMut<ClientSession>) {
    pending.0 = true;
    session.player_id = None;
    session.disconnect_reason = None;
}

fn send_pending_join(
    sender: Option<Res<ClientNetworkSender>>,
    auth_gate: Option<Res<ClientPatchworkJoinGate>>,
    mut pending: ResMut<PendingJoin>,
) {
    if !pending.0 {
        return;
    }
    let Some(sender) = sender else {
        return;
    };
    if auth_gate.as_ref().is_some_and(|gate| !gate.may_join()) {
        return;
    }
    if sender
        .send(&ServerBoundMessage::JoinRequest(JoinRequest))
        .is_ok()
    {
        pending.0 = false;
    }
}

fn accept_join(
    mut accepted: MessageReader<JoinAcceptedReceived>,
    mut session: ResMut<ClientSession>,
) {
    for accepted in accepted.read() {
        session.player_id = Some(accepted.0.player_id);
        session.disconnect_reason = None;
        info!("joined server as player {}", accepted.0.player_id);
    }
}

fn leave_server(sender: Option<Res<ClientNetworkSender>>, mut session: ResMut<ClientSession>) {
    if let Some(sender) = sender {
        let _ = sender.send(&ServerBoundMessage::LeaveRequest(LeaveRequest));
    }
    session.player_id = None;
}
