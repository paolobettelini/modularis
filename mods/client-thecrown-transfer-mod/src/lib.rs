use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi, GameStateCommand};
use client_network_api::{ClientConnectionTarget, ClientNetworkApi, ClientNetworkSender};
use client_session_api::{ClientSessionApi, ClientSessionJoinGates};
use generated_network_messages::{
    NetworkMessageSet, ServerBoundMessage, TransferAuthenticatedReceived, TransferPlayerReceived,
};
use network_protocol_mod::NetworkProtocolMod;
use patchwork_game_auth_api::ClientPatchworkGameAuthenticated;
use patchwork_game_auth_events_mod::PatchworkGameAuthEventsMod;
use thecrown_network_message_types::AuthenticateTransfer;
use thecrown_protocol::TransferPacketData;
use tokio::task::JoinHandle;

const RELAY_TRANSFER_GATE: &str = "thecrown:relay-transfer";

#[derive(Resource, Default)]
struct PendingTransfer {
    destination: Option<TransferPacketData>,
    entry_address: Option<String>,
    reconnect_from_menu: bool,
    proof_sent: bool,
}

pub struct ClientTheCrownTransferMod;

impl ClientTheCrownTransferMod {
    pub fn init<N: ClientNetworkApi, S: ClientSessionApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _network: &mut N,
        _session: &mut S,
        _state: &mut G,
        _protocol: &mut NetworkProtocolMod,
        _auth_events: &mut PatchworkGameAuthEventsMod,
    ) -> Self {
        bevy.app
            .init_resource::<PendingTransfer>()
            .add_systems(
                Update,
                (
                    receive_transfer.after(NetworkMessageSet::DispatchPackets),
                    send_transfer_proof,
                    accept_transfer.after(NetworkMessageSet::DispatchPackets),
                    reconnect_from_main_menu,
                ),
            )
            .add_systems(OnEnter(GameState::Disconnected), clear_failed_transfer)
            .add_systems(OnEnter(GameState::MainMenu), restore_entry_server);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_transfer(
    mut packets: MessageReader<TransferPlayerReceived>,
    mut pending: ResMut<PendingTransfer>,
    mut target: ResMut<ClientConnectionTarget>,
    gates: Res<ClientSessionJoinGates>,
    mut state: MessageWriter<GameStateCommand>,
) {
    for packet in packets.read() {
        let transfer = packet.0.transfer.clone();
        if pending.entry_address.is_none() {
            pending.entry_address = Some(target.address().to_owned());
        }
        target.set_address(format!("{}:{}", transfer.address, transfer.port));
        gates.block(RELAY_TRANSFER_GATE);
        pending.destination = Some(transfer);
        pending.reconnect_from_menu = true;
        pending.proof_sent = false;
        state.write(GameStateCommand::BackToMainMenu);
        info!("received TheCrown transfer; reconnecting to selected game server");
    }
}

fn reconnect_from_main_menu(
    game_state: Res<State<GameState>>,
    mut pending: ResMut<PendingTransfer>,
    mut state: MessageWriter<GameStateCommand>,
) {
    if *game_state.get() == GameState::MainMenu && pending.reconnect_from_menu {
        pending.reconnect_from_menu = false;
        state.write(GameStateCommand::StartGame);
    }
}

fn send_transfer_proof(
    mut authenticated: MessageReader<ClientPatchworkGameAuthenticated>,
    sender: Option<Res<ClientNetworkSender>>,
    mut pending: ResMut<PendingTransfer>,
) {
    if authenticated.is_empty() || pending.proof_sent {
        return;
    }
    authenticated.clear();
    let (Some(sender), Some(transfer)) = (sender, pending.destination.clone()) else {
        return;
    };
    match sender.send(&ServerBoundMessage::AuthenticateTransfer(AuthenticateTransfer {
        transfer,
    })) {
        Ok(()) => pending.proof_sent = true,
        Err(error) => warn!("failed to send TheCrown transfer proof: {error}"),
    }
}

fn accept_transfer(
    mut accepted: MessageReader<TransferAuthenticatedReceived>,
    mut pending: ResMut<PendingTransfer>,
    gates: Res<ClientSessionJoinGates>,
) {
    if accepted.read().next().is_none() {
        return;
    }
    pending.destination = None;
    pending.proof_sent = false;
    gates.release(RELAY_TRANSFER_GATE);
    info!("TheCrown relay transfer authenticated");
}

fn clear_failed_transfer(
    mut pending: ResMut<PendingTransfer>,
    mut target: ResMut<ClientConnectionTarget>,
    gates: Res<ClientSessionJoinGates>,
) {
    pending.destination = None;
    pending.proof_sent = false;
    pending.reconnect_from_menu = false;
    if let Some(entry_address) = pending.entry_address.take() {
        target.set_address(entry_address);
    }
    gates.release(RELAY_TRANSFER_GATE);
}

fn restore_entry_server(
    mut pending: ResMut<PendingTransfer>,
    mut target: ResMut<ClientConnectionTarget>,
    gates: Res<ClientSessionJoinGates>,
) {
    // The first MainMenu transition is the intentional hop between Relay
    // servers. A later transition is a user disconnect and must restore the
    // entry/auth endpoint rather than reusing a consumed game-server ticket.
    if pending.reconnect_from_menu {
        return;
    }
    if let Some(entry_address) = pending.entry_address.take() {
        target.set_address(entry_address);
        pending.destination = None;
        pending.proof_sent = false;
        gates.release(RELAY_TRANSFER_GATE);
        info!("restored TheCrown entry server after leaving the game server");
    }
}
