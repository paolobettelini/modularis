use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_config_api::ClientConfigApi;
use client_game_state_api::{GameState, GameStateApi, GameStateCommand};
use client_network_api::{ClientConnectionTarget, ClientNetworkApi, ClientNetworkSender};
use generated_network_messages::{ClientBoundMessage, ClientPacketReceived, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};
use tokio::task::JoinHandle;

#[derive(Resource)]
struct ClientUdpConnection {
    socket: Arc<UdpSocket>,
    server: SocketAddr,
}

pub struct ClientUdpNetwork;

impl ClientUdpNetwork {
    pub fn init<C: ClientConfigApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _config: &mut C,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .insert_resource(ClientConnectionTarget::new(C::default_server_address()))
            .add_systems(OnEnter(GameState::InGame), connect)
            .add_systems(
                Update,
                receive_packets
                    .run_if(in_state(GameState::InGame))
                    .in_set(NetworkMessageSet::ReceivePackets),
            )
            .add_systems(OnExit(GameState::InGame), disconnect);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientNetworkApi for ClientUdpNetwork {}

fn connect(
    mut commands: Commands,
    target: Res<ClientConnectionTarget>,
    mut game_state: MessageWriter<GameStateCommand>,
) {
    let address = target.address().trim();
    if address.is_empty() {
        error!("cannot connect: server address is empty");
        game_state.write(GameStateCommand::ShowDisconnect);
        return;
    }
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(error) => {
            error!("failed to bind client UDP socket: {error}");
            game_state.write(GameStateCommand::ShowDisconnect);
            return;
        }
    };
    if let Err(error) = socket.connect(address) {
        error!("failed to connect UDP socket to '{address}': {error}");
        game_state.write(GameStateCommand::ShowDisconnect);
        return;
    }
    let server = match socket.peer_addr() {
        Ok(server) => server,
        Err(error) => {
            error!("failed to read resolved UDP peer address for '{address}': {error}");
            game_state.write(GameStateCommand::ShowDisconnect);
            return;
        }
    };
    if let Err(error) = socket.set_nonblocking(true) {
        error!("failed to make client UDP socket nonblocking: {error}");
        game_state.write(GameStateCommand::ShowDisconnect);
        return;
    }
    let socket = Arc::new(socket);
    let sender_socket = socket.clone();
    commands.insert_resource(ClientNetworkSender::new(move |message| {
        let bytes = message
            .encode_cbor()
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
        sender_socket.send(&bytes).map(|_| ())
    }));
    commands.insert_resource(ClientUdpConnection { socket, server });
    info!("client UDP connected to {server}");
}

fn disconnect(mut commands: Commands) {
    commands.remove_resource::<ClientNetworkSender>();
    commands.remove_resource::<ClientUdpConnection>();
}

fn receive_packets(
    connection: Option<Res<ClientUdpConnection>>,
    mut received: MessageWriter<ClientPacketReceived>,
) {
    let Some(connection) = connection else {
        return;
    };
    let mut buffer = [0_u8; 65_507];
    loop {
        match connection.socket.recv(&mut buffer) {
            Ok(length) => match ClientBoundMessage::decode_cbor(&buffer[..length]) {
                Ok(message) => {
                    received.write(ClientPacketReceived(message));
                }
                Err(error) => warn!(
                    "discarding invalid packet from {}: {error}",
                    connection.server
                ),
            },
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(error) => {
                warn!("client UDP receive failed: {error}");
                break;
            }
        }
    }
}
