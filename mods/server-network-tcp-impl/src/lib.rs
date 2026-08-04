use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{NetworkMessageSet, ServerBoundMessage, ServerPacketReceived};
use network_frame_security_api::ServerFrameSecurity;
use network_frame_security_state_mod::NetworkFrameSecurityStateMod;
use network_framing_api::{drain_next_frame, encode_frame, flush_queued_frames, read_available};
use network_protocol_mod::NetworkProtocolMod;
use network_transport_events_mod::{
    NetworkTransportEventsMod, ServerTransportConnected, ServerTransportDisconnectRequested,
    ServerTransportDisconnected,
};
use server_bevy_runner_mod::ServerBevyRunnerMod;
use server_config_api::ServerConfigApi;
use server_network_api::{ServerNetworkApi, ServerNetworkSender};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex, RwLock},
};
use tokio::task::JoinHandle;

type TcpOutbox = Arc<Mutex<VecDeque<Vec<u8>>>>;

struct ServerTcpClient {
    reader: TcpStream,
    writer: TcpStream,
    outbox: TcpOutbox,
    read_buffer: Vec<u8>,
    pending_write: Vec<u8>,
    pending_offset: usize,
}

#[derive(Resource)]
struct ServerTcpSocket {
    listener: TcpListener,
    outboxes: Arc<RwLock<HashMap<SocketAddr, TcpOutbox>>>,
    clients: HashMap<SocketAddr, ServerTcpClient>,
    disconnect_after_flush: HashSet<SocketAddr>,
}

pub struct ServerTcpNetwork;

impl ServerTcpNetwork {
    pub fn init<C: ServerConfigApi>(
        bevy: &mut BevyMod,
        _config: &mut C,
        _protocol: &mut NetworkProtocolMod,
        _runner: &mut ServerBevyRunnerMod,
        _security_state: &mut NetworkFrameSecurityStateMod,
        _transport_events: &mut NetworkTransportEventsMod,
    ) -> Self {
        let address = C::bind_address();
        let listener = TcpListener::bind(address).unwrap_or_else(|error| {
            panic!("failed to bind server TCP socket at {address}: {error}")
        });
        listener
            .set_nonblocking(true)
            .expect("failed to make server TCP listener nonblocking");
        let outboxes = Arc::new(RwLock::new(HashMap::<SocketAddr, TcpOutbox>::new()));
        let sender_outboxes = outboxes.clone();
        let frame_security = bevy.app.world().resource::<ServerFrameSecurity>().clone();
        let sender_security = frame_security.clone();
        bevy.app
            .insert_resource(ServerTcpSocket {
                listener,
                outboxes,
                clients: HashMap::new(),
                disconnect_after_flush: HashSet::new(),
            })
            .insert_resource(ServerNetworkSender::new(move |address, message| {
                let outbox = sender_outboxes
                    .read()
                    .expect("server TCP outboxes lock poisoned")
                    .get(&address)
                    .cloned()
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::NotConnected,
                            format!("client {address} is not connected"),
                        )
                    })?;
                let bytes = message
                    .encode_cbor()
                    .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
                let bytes = sender_security.encode(address, &bytes)?;
                let frame = encode_frame(&bytes)?;
                outbox
                    .lock()
                    .expect("server TCP outbox lock poisoned")
                    .push_back(frame);
                Ok(())
            }))
            .add_systems(
                Update,
                receive_packets.in_set(NetworkMessageSet::ReceivePackets),
            );
        info!("server TCP listening on {address}");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerNetworkApi for ServerTcpNetwork {}

fn receive_packets(
    mut socket: ResMut<ServerTcpSocket>,
    network: Res<ServerNetworkSender>,
    security: Res<ServerFrameSecurity>,
    mut disconnect_requests: MessageReader<ServerTransportDisconnectRequested>,
    mut connected: MessageWriter<ServerTransportConnected>,
    mut disconnected_events: MessageWriter<ServerTransportDisconnected>,
    mut received: MessageWriter<ServerPacketReceived>,
) {
    for request in disconnect_requests.read() {
        socket.disconnect_after_flush.insert(request.address);
    }
    accept_connections(&mut socket, &security, &mut connected);

    let clients = socket.clients.keys().copied().collect::<Vec<_>>();
    let mut disconnected = Vec::new();
    for address in clients {
        let disconnect_after_flush = socket.disconnect_after_flush.contains(&address);
        let Some(client) = socket.clients.get_mut(&address) else {
            disconnected.push(address);
            continue;
        };

        let flush_result = {
            let mut queued = client
                .outbox
                .lock()
                .expect("server TCP outbox lock poisoned");
            flush_queued_frames(
                &mut client.writer,
                &mut queued,
                &mut client.pending_write,
                &mut client.pending_offset,
            )
        };
        match flush_result {
            Ok(flushed) => {
                if flushed && disconnect_after_flush {
                    disconnected.push(address);
                    continue;
                }
            }
            Err(error) => {
                warn!("server TCP send to {address} failed: {error}");
                disconnected.push(address);
                continue;
            }
        }

        match read_available(&mut client.reader, &mut client.read_buffer) {
            Ok(true) => {}
            Ok(false) => {
                disconnected.push(address);
                continue;
            }
            Err(error) => {
                warn!("server TCP receive from {address} failed: {error}");
                disconnected.push(address);
                continue;
            }
        }
        while security.can_decode(address) {
            let plaintext_mode = security.is_plaintext(address);
            let frame = match drain_next_frame(&mut client.read_buffer) {
                Ok(Some(frame)) => frame,
                Ok(None) => break,
                Err(error) => {
                    warn!("server TCP framing error from {address}: {error}");
                    disconnected.push(address);
                    break;
                }
            };
            let decoded = match security.decode_candidate(address, &frame) {
                Ok(Some(decoded)) => decoded,
                Ok(None) => break,
                Err(error) => {
                    warn!("server secure frame from {address} rejected: {error}");
                    security.fail(address);
                    disconnected.push(address);
                    break;
                }
            };
            match ServerBoundMessage::decode_cbor(&decoded) {
                Ok(message) => {
                    if let Err(error) = security.commit_inbound(address) {
                        warn!("server secure sequence from {address} failed: {error}");
                        security.fail(address);
                        disconnected.push(address);
                        break;
                    }
                    received.write(ServerPacketReceived {
                        source: address,
                        message,
                    });
                }
                Err(error) => {
                    warn!("invalid TCP packet from {address}: {error}");
                    if !plaintext_mode {
                        security.fail(address);
                        disconnected.push(address);
                        break;
                    }
                }
            }
            if plaintext_mode {
                break;
            }
        }
    }

    for address in disconnected {
        socket
            .outboxes
            .write()
            .expect("server TCP outboxes lock poisoned")
            .remove(&address);
        socket.clients.remove(&address);
        socket.disconnect_after_flush.remove(&address);
        security.remove(address);
        network.remove_client(address);
        disconnected_events.write(ServerTransportDisconnected { address });
        info!("client TCP disconnected: {address}");
    }
}

fn accept_connections(
    socket: &mut ServerTcpSocket,
    security: &ServerFrameSecurity,
    connected: &mut MessageWriter<ServerTransportConnected>,
) {
    loop {
        match socket.listener.accept() {
            Ok((stream, address)) => {
                if let Err(error) = stream.set_nonblocking(true) {
                    warn!("failed to make TCP client {address} nonblocking: {error}");
                    continue;
                }
                if let Err(error) = stream.set_nodelay(true) {
                    warn!("failed to enable TCP_NODELAY for {address}: {error}");
                    continue;
                }
                let writer = match stream.try_clone() {
                    Ok(writer) => writer,
                    Err(error) => {
                        warn!("failed to clone TCP stream for {address}: {error}");
                        continue;
                    }
                };
                let outbox = Arc::new(Mutex::new(VecDeque::new()));
                socket
                    .outboxes
                    .write()
                    .expect("server TCP outboxes lock poisoned")
                    .insert(address, outbox.clone());
                socket.clients.insert(
                    address,
                    ServerTcpClient {
                        reader: stream,
                        writer,
                        outbox,
                        read_buffer: Vec::new(),
                        pending_write: Vec::new(),
                        pending_offset: 0,
                    },
                );
                security.register_plaintext(address);
                connected.write(ServerTransportConnected { address });
                info!("client TCP connected: {address}");
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(error) => {
                warn!("server TCP accept failed: {error}");
                break;
            }
        }
    }
}
