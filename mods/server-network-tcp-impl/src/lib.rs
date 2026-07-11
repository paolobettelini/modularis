use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{NetworkMessageSet, ServerBoundMessage, ServerPacketReceived};
use network_framing_api::{drain_frames, encode_frame, flush_queued_frames, read_available};
use network_protocol_mod::NetworkProtocolMod;
use server_bevy_runner_mod::ServerBevyRunnerMod;
use server_config_api::ServerConfigApi;
use server_network_api::{ServerNetworkApi, ServerNetworkSender};
use std::{
    collections::{HashMap, VecDeque},
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
}

pub struct ServerTcpNetwork;

impl ServerTcpNetwork {
    pub fn init<C: ServerConfigApi>(
        bevy: &mut BevyMod,
        _config: &mut C,
        _protocol: &mut NetworkProtocolMod,
        _runner: &mut ServerBevyRunnerMod,
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
        bevy.app
            .insert_resource(ServerTcpSocket {
                listener,
                outboxes,
                clients: HashMap::new(),
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
    mut received: MessageWriter<ServerPacketReceived>,
) {
    accept_connections(&mut socket);

    let clients = socket.clients.keys().copied().collect::<Vec<_>>();
    let mut disconnected = Vec::new();
    for address in clients {
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
        if let Err(error) = flush_result {
            warn!("server TCP send to {address} failed: {error}");
            disconnected.push(address);
            continue;
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
        let frames = match drain_frames(&mut client.read_buffer) {
            Ok(frames) => frames,
            Err(error) => {
                warn!("server TCP framing error from {address}: {error}");
                disconnected.push(address);
                continue;
            }
        };
        for frame in frames {
            match ServerBoundMessage::decode_cbor(&frame) {
                Ok(message) => {
                    received.write(ServerPacketReceived {
                        source: address,
                        message,
                    });
                }
                Err(error) => warn!("discarding invalid TCP packet from {address}: {error}"),
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
        network.remove_client(address);
        info!("client TCP disconnected: {address}");
    }
}

fn accept_connections(socket: &mut ServerTcpSocket) {
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
