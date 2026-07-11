use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use client_settings_api::{SettingsApi, SettingsStore};
use generated_client_settings_registry::SettingKey;
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
    pub fn init<S: SettingsApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _settings: &mut S,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
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

fn connect(mut commands: Commands, settings: Res<SettingsStore>) {
    let address = settings
        .get_string(SettingKey::NetworkServerAddress)
        .unwrap_or("127.0.0.1:9999");
    let server: SocketAddr = address
        .parse()
        .unwrap_or_else(|error| panic!("invalid server address '{address}': {error}"));
    let socket = UdpSocket::bind("0.0.0.0:0")
        .unwrap_or_else(|error| panic!("failed to bind client UDP socket: {error}"));
    socket
        .connect(server)
        .unwrap_or_else(|error| panic!("failed to connect UDP socket to {server}: {error}"));
    socket
        .set_nonblocking(true)
        .expect("failed to make client UDP socket nonblocking");
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
