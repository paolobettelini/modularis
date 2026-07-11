use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use client_settings_api::{SettingsApi, SettingsStore};
use generated_client_settings_registry::SettingKey;
use generated_network_messages::{ClientBoundMessage, ClientPacketReceived, NetworkMessageSet};
use network_framing_api::{drain_frames, encode_frame, flush_queued_frames, read_available};
use network_protocol_mod::NetworkProtocolMod;
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

impl ClientNetworkApi for ClientTcpNetwork {}

fn connect(mut commands: Commands, settings: Res<SettingsStore>) {
    let address = settings
        .get_string(SettingKey::NetworkServerAddress)
        .unwrap_or("127.0.0.1:9999");
    let server: SocketAddr = address
        .parse()
        .unwrap_or_else(|error| panic!("invalid server address '{address}': {error}"));
    let stream = TcpStream::connect(server)
        .unwrap_or_else(|error| panic!("failed to connect TCP socket to {server}: {error}"));
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
    commands.insert_resource(ClientNetworkSender::new(move |message| {
        let bytes = message
            .encode_cbor()
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
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
    info!("client TCP connected to {server}");
}

fn disconnect(mut commands: Commands) {
    commands.remove_resource::<ClientNetworkSender>();
    commands.remove_resource::<ClientTcpConnection>();
}

fn receive_packets(
    mut commands: Commands,
    connection: Option<ResMut<ClientTcpConnection>>,
    mut received: MessageWriter<ClientPacketReceived>,
) {
    let Some(mut connection) = connection else {
        return;
    };
    let connection = &mut *connection;

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
        commands.remove_resource::<ClientNetworkSender>();
        commands.remove_resource::<ClientTcpConnection>();
        return;
    }

    match read_available(&mut connection.reader, &mut connection.read_buffer) {
        Ok(true) => {}
        Ok(false) => {
            warn!("server {} closed TCP connection", connection.server);
            commands.remove_resource::<ClientNetworkSender>();
            commands.remove_resource::<ClientTcpConnection>();
            return;
        }
        Err(error) => {
            warn!("client TCP receive failed: {error}");
            commands.remove_resource::<ClientNetworkSender>();
            commands.remove_resource::<ClientTcpConnection>();
            return;
        }
    }

    let frames = match drain_frames(&mut connection.read_buffer) {
        Ok(frames) => frames,
        Err(error) => {
            warn!("client TCP framing error: {error}");
            commands.remove_resource::<ClientNetworkSender>();
            commands.remove_resource::<ClientTcpConnection>();
            return;
        }
    };
    for frame in frames {
        match ClientBoundMessage::decode_cbor(&frame) {
            Ok(message) => {
                received.write(ClientPacketReceived(message));
            }
            Err(error) => warn!(
                "discarding invalid TCP packet from {}: {error}",
                connection.server
            ),
        }
    }
}
