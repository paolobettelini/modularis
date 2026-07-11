use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{NetworkMessageSet, ServerBoundMessage, ServerPacketReceived};
use network_protocol_mod::NetworkProtocolMod;
use server_bevy_runner_mod::ServerBevyRunnerMod;
use server_config_api::ServerConfigApi;
use server_network_api::{ServerNetworkApi, ServerNetworkSender};
use std::{net::UdpSocket, sync::Arc};
use tokio::task::JoinHandle;

#[derive(Resource)]
struct ServerUdpSocket(Arc<UdpSocket>);

pub struct ServerUdpNetwork;

impl ServerUdpNetwork {
    pub fn init<C: ServerConfigApi>(
        bevy: &mut BevyMod,
        _config: &mut C,
        _protocol: &mut NetworkProtocolMod,
        _runner: &mut ServerBevyRunnerMod,
    ) -> Self {
        let address = C::bind_address();
        let socket = UdpSocket::bind(address).unwrap_or_else(|error| {
            panic!("failed to bind server UDP socket at {address}: {error}")
        });
        socket
            .set_nonblocking(true)
            .expect("failed to make server UDP socket nonblocking");
        let socket = Arc::new(socket);
        let sender_socket = socket.clone();
        bevy.app
            .insert_resource(ServerUdpSocket(socket.clone()))
            .insert_resource(ServerNetworkSender::new(move |address, message| {
                let bytes = message
                    .encode_cbor()
                    .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
                sender_socket.send_to(&bytes, address).map(|_| ())
            }))
            .add_systems(
                Update,
                receive_packets.in_set(NetworkMessageSet::ReceivePackets),
            );
        info!("server listening on {address}");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerNetworkApi for ServerUdpNetwork {}

fn receive_packets(
    socket: Res<ServerUdpSocket>,
    mut received: MessageWriter<ServerPacketReceived>,
) {
    let mut buffer = [0_u8; 65_507];
    loop {
        match socket.0.recv_from(&mut buffer) {
            Ok((length, source)) => match ServerBoundMessage::decode_cbor(&buffer[..length]) {
                Ok(message) => {
                    received.write(ServerPacketReceived { source, message });
                }
                Err(error) => warn!("discarding invalid packet from {source}: {error}"),
            },
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(error) => {
                warn!("server UDP receive failed: {error}");
                break;
            }
        }
    }
}
