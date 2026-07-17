use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, GameStateCommand};
use client_session_api::{ClientSession, ClientSessionApi};
use generated_network_messages::{KickReceived, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientKickNetworkReceiveMod;

impl ClientKickNetworkReceiveMod {
    pub fn init<S: ClientSessionApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _session: &mut S,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_kick.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_kick(
    mut kicked: MessageReader<KickReceived>,
    mut session: ResMut<ClientSession>,
    mut state: MessageWriter<GameStateCommand>,
) {
    for kicked in kicked.read() {
        warn!("server disconnected client: {}", kicked.0.reason);
        session.player_id = None;
        session.disconnect_reason = Some(kicked.0.reason.clone());
        state.write(GameStateCommand::ShowDisconnect);
    }
}
