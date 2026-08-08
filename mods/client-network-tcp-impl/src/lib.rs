use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_config_api::ClientConfigApi;
use client_game_state_api::{GameState, GameStateApi, GameStateCommand};
use client_network_api::{ClientConnectionTarget, ClientNetworkApi, ClientNetworkSender};
use generated_network_messages::{ClientBoundMessage, ClientPacketReceived, NetworkMessageSet};
use network_frame_security_api::ClientFrameSecurity;
use network_frame_security_state_mod::NetworkFrameSecurityStateMod;
use network_framing_api::{drain_next_frame, encode_frame, flush_queued_frames, read_available};
use network_protocol_mod::NetworkProtocolMod;
use network_transport_events_mod::{
    ClientTransportConnected, ClientTransportDisconnectRequested, ClientTransportDisconnected,
    NetworkTransportEventsMod,
};
use std::{
    collections::VecDeque,
    net::{SocketAddr, TcpStream},
    sync::{Arc, Mutex},
};
use tokio::task::JoinHandle;

#[derive(Resource)]
struct ClientTcpConnection {
    reader: TcpStream,
    writer: TcpStream,
    outbox: Arc<Mutex<VecDeque<Vec<u8>>>>,
    read_buffer: Vec<u8>,
    pending_write: Vec<u8>,
    pending_offset: usize,
    server: SocketAddr,
}

pub struct ClientTcpNetwork;

impl ClientTcpNetwork {
    pub fn init<C: ClientConfigApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _config: &mut C,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
        _security: &mut NetworkFrameSecurityStateMod,
        _transport_events: &mut NetworkTransportEventsMod,
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

impl ClientNetworkApi for ClientTcpNetwork {}

fn connect(
    mut commands: Commands,
    target: Res<ClientConnectionTarget>,
    security: Res<ClientFrameSecurity>,
    mut connected: MessageWriter<ClientTransportConnected>,
    mut game_state: MessageWriter<GameStateCommand>,
) {
    let address = target.address().trim();
    if address.is_empty() {
        error!("cannot connect: server address is empty");
        game_state.write(GameStateCommand::ShowDisconnect);
        return;
    }
    let stream = match TcpStream::connect(address) {
        Ok(stream) => stream,
        Err(error) => {
            error!("failed to connect TCP socket to '{address}': {error}");
            game_state.write(GameStateCommand::ShowDisconnect);
            return;
        }
    };
    let server = match stream.peer_addr() {
        Ok(server) => server,
        Err(error) => {
            error!("failed to read resolved TCP peer address for '{address}': {error}");
            game_state.write(GameStateCommand::ShowDisconnect);
            return;
        }
    };
    stream
        .set_nonblocking(true)
        .expect("failed to make client TCP socket nonblocking");
    stream
        .set_nodelay(true)
        .expect("failed to enable TCP_NODELAY on client socket");
    let writer = stream
        .try_clone()
        .expect("failed to clone client TCP stream writer");
    let outbox = Arc::new(Mutex::new(VecDeque::new()));
    let sender_outbox = outbox.clone();
    security.reset_plaintext();
    let sender_security = security.clone();
    commands.insert_resource(ClientNetworkSender::new(move |message| {
        let bytes = message
            .encode_cbor()
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
        let bytes = sender_security.encode(&bytes)?;
        let frame = encode_frame(&bytes)?;
        sender_outbox
            .lock()
            .expect("client TCP outbox lock poisoned")
            .push_back(frame);
        Ok(())
    }));
    commands.insert_resource(ClientTcpConnection {
        reader: stream,
        writer,
        outbox,
        read_buffer: Vec::new(),
        pending_write: Vec::new(),
        pending_offset: 0,
        server,
    });
    connected.write(ClientTransportConnected { server });
    info!("client TCP connected to {server}");
}

fn disconnect(
    mut commands: Commands,
    connection: Option<Res<ClientTcpConnection>>,
    security: Res<ClientFrameSecurity>,
    mut disconnected: MessageWriter<ClientTransportDisconnected>,
) {
    if let Some(connection) = connection {
        disconnected.write(ClientTransportDisconnected {
            server: connection.server,
        });
    }
    security.reset_plaintext();
    commands.remove_resource::<ClientNetworkSender>();
    commands.remove_resource::<ClientTcpConnection>();
}

fn receive_packets(
    mut commands: Commands,
    connection: Option<ResMut<ClientTcpConnection>>,
    security: Res<ClientFrameSecurity>,
    mut disconnect_requests: MessageReader<ClientTransportDisconnectRequested>,
    mut disconnected: MessageWriter<ClientTransportDisconnected>,
    mut received: MessageWriter<ClientPacketReceived>,
) {
    let Some(mut connection) = connection else {
        return;
    };
    let connection = &mut *connection;

    if disconnect_requests.read().next().is_some() {
        disconnected.write(ClientTransportDisconnected {
            server: connection.server,
        });
        security.fail();
        commands.remove_resource::<ClientNetworkSender>();
        commands.remove_resource::<ClientTcpConnection>();
        return;
    }

    let flush_result = {
        let outbox = connection.outbox.clone();
        let mut queued = outbox.lock().expect("client TCP outbox lock poisoned");
        flush_queued_frames(
            &mut connection.writer,
            &mut queued,
            &mut connection.pending_write,
            &mut connection.pending_offset,
        )
    };
    if let Err(error) = flush_result {
        warn!("client TCP send failed: {error}");
        disconnected.write(ClientTransportDisconnected {
            server: connection.server,
        });
        security.fail();
        commands.remove_resource::<ClientNetworkSender>();
        commands.remove_resource::<ClientTcpConnection>();
        return;
    }

    match read_available(&mut connection.reader, &mut connection.read_buffer) {
        Ok(true) => {}
        Ok(false) => {
            warn!("server {} closed TCP connection", connection.server);
            disconnected.write(ClientTransportDisconnected {
                server: connection.server,
            });
            security.fail();
            commands.remove_resource::<ClientNetworkSender>();
            commands.remove_resource::<ClientTcpConnection>();
            return;
        }
        Err(error) => {
            warn!("client TCP receive failed: {error}");
            disconnected.write(ClientTransportDisconnected {
                server: connection.server,
            });
            security.fail();
            commands.remove_resource::<ClientNetworkSender>();
            commands.remove_resource::<ClientTcpConnection>();
            return;
        }
    }

    while security.can_decode() {
        let plaintext_mode = security.is_plaintext();
        let frame = match drain_next_frame(&mut connection.read_buffer) {
            Ok(Some(frame)) => frame,
            Ok(None) => break,
            Err(error) => {
                warn!("client TCP framing error: {error}");
                disconnected.write(ClientTransportDisconnected {
                    server: connection.server,
                });
                security.fail();
                commands.remove_resource::<ClientNetworkSender>();
                commands.remove_resource::<ClientTcpConnection>();
                return;
            }
        };
        let decoded = match security.decode_candidate(&frame) {
            Ok(Some(decoded)) => decoded,
            Ok(None) => break,
            Err(error) => {
                warn!("client secure frame rejected: {error}");
                disconnected.write(ClientTransportDisconnected {
                    server: connection.server,
                });
                security.fail();
                commands.remove_resource::<ClientNetworkSender>();
                commands.remove_resource::<ClientTcpConnection>();
                return;
            }
        };
        match ClientBoundMessage::decode_cbor(&decoded) {
            Ok(message) => {
                if let Err(error) = security.commit_inbound() {
                    warn!("client secure sequence failed: {error}");
                    disconnected.write(ClientTransportDisconnected {
                        server: connection.server,
                    });
                    security.fail();
                    commands.remove_resource::<ClientNetworkSender>();
                    commands.remove_resource::<ClientTcpConnection>();
                    return;
                }
                received.write(ClientPacketReceived(message));
            }
            Err(error) => {
                warn!("invalid TCP packet from {}: {error}", connection.server);
                if !plaintext_mode {
                    disconnected.write(ClientTransportDisconnected {
                        server: connection.server,
                    });
                    security.fail();
                    commands.remove_resource::<ClientNetworkSender>();
                    commands.remove_resource::<ClientTcpConnection>();
                    return;
                }
            }
        }
        if plaintext_mode {
            break;
        }
    }
}
