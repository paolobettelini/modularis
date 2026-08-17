use bevy::prelude::*;
use bevy_mod::BevyMod;
use futures_util::StreamExt;
use std::{sync::{Mutex, mpsc}, thread::JoinHandle as ThreadJoinHandle, time::Duration};
use thecrown_common::nats::NatsClient;
use thecrown_game_config_api::{TheCrownGameConfig, TheCrownGameConfigApi};
use thecrown_game_relay_api::*;
use thecrown_protocol::{
    GameServerPacket, RelayPacket, WebPacket, RELAY_SUBJECT, WEB_SUBJECT, game_server_subject,
};
use tokio::{runtime::Builder, sync::mpsc as async_mpsc, task::JoinHandle};

enum WorkerCommand {
    Authenticate(AuthenticateRelayTransfer),
    Transfer(RequestRelayPlayerTransfer),
    Quit(NotifyRelayPlayerQuit),
    Whisper(RequestRelayWhisper),
    WebLogin(RequestTheCrownWebLogin),
    LoadParkourRecord(RequestRelayParkourRecord),
    SubmitParkourRecord(SubmitRelayParkourRecord),
    Shutdown,
}

enum WorkerEvent {
    Connected,
    Game(GameServerPacket),
    Auth(RelayTransferAuthenticationFinished),
    Transfer(RelayPlayerTransferFinished),
    Whisper(RelayWhisperFinished),
    WebLogin(TheCrownWebLoginFinished),
    ParkourRecordLoaded(RelayParkourRecordLoaded),
    ParkourRecordSubmitted(RelayParkourRecordSubmitted),
    Error(String),
}

#[derive(Resource)]
struct RelayWorker {
    commands: async_mpsc::UnboundedSender<WorkerCommand>,
    events: Mutex<mpsc::Receiver<WorkerEvent>>,
    thread: Option<ThreadJoinHandle<()>>,
}

impl Drop for RelayWorker {
    fn drop(&mut self) {
        let _ = self.commands.send(WorkerCommand::Shutdown);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

pub struct TheCrownGameRelayNatsMod;

impl TheCrownGameRelayNatsMod {
    pub fn init<C: TheCrownGameConfigApi>(
        bevy: &mut BevyMod,
        _config_api: &mut C,
    ) -> Self {
        let config = bevy.app.world().resource::<TheCrownGameConfig>().clone();
        let (command_tx, command_rx) = async_mpsc::unbounded_channel();
        let (event_tx, event_rx) = mpsc::channel();
        let thread = std::thread::Builder::new()
            .name("thecrown-game-relay".to_owned())
            .spawn(move || relay_worker(config, command_rx, event_tx))
            .expect("failed to start TheCrown relay worker");

        bevy.app
            .insert_resource(RelayWorker {
                commands: command_tx,
                events: Mutex::new(event_rx),
                thread: Some(thread),
            })
            .init_resource::<TheCrownRelayRequestIds>()
            .add_message::<RelayStartInstance>()
            .add_message::<RelayStopInstance>()
            .add_message::<AuthenticateRelayTransfer>()
            .add_message::<RelayTransferAuthenticationFinished>()
            .add_message::<RequestRelayPlayerTransfer>()
            .add_message::<RelayPlayerTransferFinished>()
            .add_message::<NotifyRelayPlayerQuit>()
            .add_message::<RequestRelayWhisper>()
            .add_message::<RelayWhisperFinished>()
            .add_message::<RelayWhisperReceived>()
            .add_message::<RelayExecuteTransfer>()
            .add_message::<RequestTheCrownWebLogin>()
            .add_message::<TheCrownWebLoginFinished>()
            .add_message::<RequestRelayParkourRecord>()
            .add_message::<RelayParkourRecordLoaded>()
            .add_message::<SubmitRelayParkourRecord>()
            .add_message::<RelayParkourRecordSubmitted>()
            .add_systems(Update, (submit_requests, poll_worker_events).chain());
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl TheCrownGameRelayApi for TheCrownGameRelayNatsMod {}

#[allow(clippy::too_many_arguments)]
fn submit_requests(
    worker: Res<RelayWorker>,
    mut auth: MessageReader<AuthenticateRelayTransfer>,
    mut transfers: MessageReader<RequestRelayPlayerTransfer>,
    mut quits: MessageReader<NotifyRelayPlayerQuit>,
    mut whispers: MessageReader<RequestRelayWhisper>,
    mut web: MessageReader<RequestTheCrownWebLogin>,
    mut record_loads: MessageReader<RequestRelayParkourRecord>,
    mut record_submissions: MessageReader<SubmitRelayParkourRecord>,
) {
    for request in auth.read() { let _ = worker.commands.send(WorkerCommand::Authenticate(request.clone())); }
    for request in transfers.read() { let _ = worker.commands.send(WorkerCommand::Transfer(request.clone())); }
    for request in quits.read() { let _ = worker.commands.send(WorkerCommand::Quit(request.clone())); }
    for request in whispers.read() { let _ = worker.commands.send(WorkerCommand::Whisper(request.clone())); }
    for request in web.read() { let _ = worker.commands.send(WorkerCommand::WebLogin(request.clone())); }
    for request in record_loads.read() { let _ = worker.commands.send(WorkerCommand::LoadParkourRecord(request.clone())); }
    for request in record_submissions.read() { let _ = worker.commands.send(WorkerCommand::SubmitParkourRecord(request.clone())); }
}

#[allow(clippy::too_many_arguments)]
fn poll_worker_events(
    worker: Res<RelayWorker>,
    mut starts: MessageWriter<RelayStartInstance>,
    mut stops: MessageWriter<RelayStopInstance>,
    mut auth: MessageWriter<RelayTransferAuthenticationFinished>,
    mut transfers: MessageWriter<RelayPlayerTransferFinished>,
    mut whisper_results: MessageWriter<RelayWhisperFinished>,
    mut whispers: MessageWriter<RelayWhisperReceived>,
    mut execute: MessageWriter<RelayExecuteTransfer>,
    mut web: MessageWriter<TheCrownWebLoginFinished>,
    mut record_loads: MessageWriter<RelayParkourRecordLoaded>,
    mut record_submissions: MessageWriter<RelayParkourRecordSubmitted>,
) {
    let receiver = worker.events.lock().expect("relay event channel poisoned");
    while let Ok(event) = receiver.try_recv() {
        match event {
            WorkerEvent::Connected => info!("TheCrown game server registered with Relay"),
            WorkerEvent::Game(GameServerPacket::StartInstance { instance }) => { starts.write(RelayStartInstance { instance }); }
            WorkerEvent::Game(GameServerPacket::StopInstance { instance_id }) => { stops.write(RelayStopInstance { instance_id }); }
            WorkerEvent::Game(GameServerPacket::WhisperCommand { sender, target_uuid, message }) => { whispers.write(RelayWhisperReceived { sender, target_uuid, message }); }
            WorkerEvent::Game(GameServerPacket::ExecuteTransfer { player_uuid, transfer }) => { execute.write(RelayExecuteTransfer { player_uuid, transfer }); }
            WorkerEvent::Auth(value) => { auth.write(value); }
            WorkerEvent::Transfer(value) => { transfers.write(value); }
            WorkerEvent::Whisper(value) => { whisper_results.write(value); }
            WorkerEvent::WebLogin(value) => { web.write(value); }
            WorkerEvent::ParkourRecordLoaded(value) => { record_loads.write(value); }
            WorkerEvent::ParkourRecordSubmitted(value) => { record_submissions.write(value); }
            WorkerEvent::Error(error) => warn!("TheCrown relay worker: {error}"),
        }
    }
}

fn relay_worker(
    config: TheCrownGameConfig,
    commands: async_mpsc::UnboundedReceiver<WorkerCommand>,
    events: mpsc::Sender<WorkerEvent>,
) {
    let runtime = Builder::new_current_thread().enable_all().build()
        .expect("failed to create TheCrown relay Tokio runtime");
    if let Err(error) = runtime.block_on(run_relay_worker(config, commands, events.clone())) {
        let _ = events.send(WorkerEvent::Error(error));
    }
}

async fn run_relay_worker(
    config: TheCrownGameConfig,
    mut commands: async_mpsc::UnboundedReceiver<WorkerCommand>,
    events: mpsc::Sender<WorkerEvent>,
) -> Result<(), String> {
    let nats = NatsClient::connect(
        &config.nats_url,
        Duration::from_millis(config.request_timeout_ms),
    ).await.map_err(|error| error.to_string())?;
    let subject = game_server_subject(&config.server_id);

    // Subscription must exist before registration: Relay immediately delivers
    // the initial StartInstance commands as part of registration.
    let mut subscription = nats.subscribe(subject.clone()).await.map_err(|error| error.to_string())?;
    nats.publish(RELAY_SUBJECT, &RelayPacket::RegisterServer {
        server_id: config.server_id.clone(),
        address: config.public_address.clone(),
        port: config.public_port,
    }).await.map_err(|error| error.to_string())?;
    let _ = events.send(WorkerEvent::Connected);

    loop {
        tokio::select! {
            command = commands.recv() => {
                let Some(command) = command else { break; };
                if handle_command(&nats, &config, command, &events).await { break; }
            }
            message = subscription.next() => {
                let Some(message) = message else {
                    let _ = events.send(WorkerEvent::Error("game-server NATS subscription ended".to_owned()));
                    break;
                };
                match NatsClient::decode::<GameServerPacket>(&message) {
                    Ok(packet) => { let _ = events.send(WorkerEvent::Game(packet)); }
                    Err(error) => { let _ = events.send(WorkerEvent::Error(format!("invalid game-server packet: {error:#}"))); }
                }
            }
        }
    }

    let _ = nats.publish(RELAY_SUBJECT, &RelayPacket::UnregisterServer {
        server_id: config.server_id,
    }).await;
    Ok(())
}

async fn handle_command(
    nats: &NatsClient,
    config: &TheCrownGameConfig,
    command: WorkerCommand,
    events: &mpsc::Sender<WorkerEvent>,
) -> bool {
    match command {
        WorkerCommand::Shutdown => return true,
        WorkerCommand::Authenticate(request) => {
            let result = nats.request::<_, RelayPacket, _>(RELAY_SUBJECT, &RelayPacket::AuthUserJoin {
                player_uuid: request.player.uuid,
                server_id: config.server_id.clone(),
                instance_id: request.transfer.instance_id.clone(),
                cookie: request.transfer.cookie.clone(),
            }).await;
            let (accepted, error) = match result {
                Ok(RelayPacket::ServeAuthResult { value }) => (value, None),
                Ok(packet) => (false, Some(format!("unexpected relay auth response: {packet:?}"))),
                Err(error) => (false, Some(error.to_string())),
            };
            let _ = events.send(WorkerEvent::Auth(RelayTransferAuthenticationFinished {
                request_id: request.request_id,
                source: request.source,
                player: request.player,
                transfer: request.transfer,
                accepted,
                error,
            }));
        }
        WorkerCommand::Transfer(request) => {
            let packet = match request.destination {
                RelayTransferDestination::Lobby => RelayPacket::PlayerEnterLobby { player_uuid: request.player_uuid },
                RelayTransferDestination::Mode(mode) => RelayPacket::PlayerEnterMode { player_uuid: request.player_uuid, mode },
                RelayTransferDestination::Instance(instance_id) => RelayPacket::PlayerEnterSpecificServer { player_uuid: request.player_uuid, instance_id },
            };
            let result = match nats.request::<_, RelayPacket, _>(RELAY_SUBJECT, &packet).await {
                Ok(RelayPacket::ServePlayerTransfer { data }) => Ok(data),
                Ok(packet) => Err(format!("unexpected transfer response: {packet:?}")),
                Err(error) => Err(error.to_string()),
            };
            let _ = events.send(WorkerEvent::Transfer(RelayPlayerTransferFinished {
                request_id: request.request_id,
                player_id: request.player_id,
                result,
            }));
        }
        WorkerCommand::Quit(request) => {
            if let Err(error) = nats.publish(RELAY_SUBJECT, &RelayPacket::PlayerQuit {
                player_uuid: request.player_uuid,
                server_id: config.server_id.clone(),
                instance_id: request.instance_id,
            }).await {
                let _ = events.send(WorkerEvent::Error(format!("failed to publish PlayerQuit: {error:#}")));
            }
        }
        WorkerCommand::Whisper(request) => {
            let result = nats.request::<_, RelayPacket, _>(RELAY_SUBJECT, &RelayPacket::WhisperCommandByName {
                sender_uuid: request.sender_uuid,
                target_username: request.target_username,
                message: request.message,
            }).await;
            let (delivered, error) = match result {
                Ok(RelayPacket::WhisperCommandResponse { status }) => (status, None),
                Ok(packet) => (false, Some(format!("unexpected whisper response: {packet:?}"))),
                Err(error) => (false, Some(error.to_string())),
            };
            let _ = events.send(WorkerEvent::Whisper(RelayWhisperFinished {
                request_id: request.request_id,
                sender_id: request.sender_id,
                delivered,
                error,
            }));
        }
        WorkerCommand::WebLogin(request) => {
            let result = match nats.request::<_, WebPacket, _>(WEB_SUBJECT, &WebPacket::GenerateAuthToken {
                player_uuid: request.player_uuid,
            }).await {
                Ok(WebPacket::ServeAuthToken { token }) => Ok(token),
                Ok(packet) => Err(format!("unexpected web response: {packet:?}")),
                Err(error) => Err(error.to_string()),
            };
            let _ = events.send(WorkerEvent::WebLogin(TheCrownWebLoginFinished {
                request_id: request.request_id,
                player_id: request.player_id,
                result,
            }));
        }
        WorkerCommand::LoadParkourRecord(request) => {
            let result = match nats.request::<_, RelayPacket, _>(
                RELAY_SUBJECT,
                &RelayPacket::GetParkourRecord { player_uuid: request.player_uuid },
            ).await {
                Ok(RelayPacket::ServeParkourRecord { record: Some(record) }) => Ok(record),
                Ok(RelayPacket::ServeParkourRecord { record: None }) => Err("player record does not exist".to_owned()),
                Ok(packet) => Err(format!("unexpected parkour record response: {packet:?}")),
                Err(error) => Err(error.to_string()),
            };
            let _ = events.send(WorkerEvent::ParkourRecordLoaded(RelayParkourRecordLoaded {
                request_id: request.request_id,
                player_id: request.player_id,
                result,
            }));
        }
        WorkerCommand::SubmitParkourRecord(request) => {
            let result = match nats.request::<_, RelayPacket, _>(
                RELAY_SUBJECT,
                &RelayPacket::SubmitParkourRecord {
                    player_uuid: request.player_uuid,
                    score: request.score,
                },
            ).await {
                Ok(RelayPacket::ServeParkourRecordUpdate { update: Some(update) }) => Ok(update),
                Ok(RelayPacket::ServeParkourRecordUpdate { update: None }) => Err("player record does not exist".to_owned()),
                Ok(packet) => Err(format!("unexpected parkour record update response: {packet:?}")),
                Err(error) => Err(error.to_string()),
            };
            let _ = events.send(WorkerEvent::ParkourRecordSubmitted(RelayParkourRecordSubmitted {
                request_id: request.request_id,
                player_id: request.player_id,
                submitted_score: request.score,
                result,
            }));
        }
    }
    false
}
