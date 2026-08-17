use bevy::prelude::*;
use bevy_mod::BevyMod;
use std::{sync::{Mutex, mpsc}, thread::JoinHandle as ThreadJoinHandle, time::Duration};
use thecrown_auth_relay_api::{RequestTheCrownAdmission, TheCrownAdmissionFinished, TheCrownAuthRelayApi};
use thecrown_common::nats::NatsClient;
use thecrown_protocol::{RelayPacket, RELAY_SUBJECT};
use tokio::{runtime::Builder, sync::mpsc as async_mpsc, task::JoinHandle};

enum WorkerCommand { Admit(RequestTheCrownAdmission), Shutdown }

#[derive(Resource)]
struct AuthRelayWorker {
    commands: async_mpsc::UnboundedSender<WorkerCommand>,
    results: Mutex<mpsc::Receiver<TheCrownAdmissionFinished>>,
    thread: Option<ThreadJoinHandle<()>>,
}

impl Drop for AuthRelayWorker {
    fn drop(&mut self) {
        let _ = self.commands.send(WorkerCommand::Shutdown);
        if let Some(thread) = self.thread.take() { let _ = thread.join(); }
    }
}

pub struct TheCrownAuthRelayNatsMod;

impl TheCrownAuthRelayNatsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        let (command_tx, mut command_rx) = async_mpsc::unbounded_channel();
        let (result_tx, result_rx) = mpsc::channel();
        let thread = std::thread::Builder::new()
            .name("thecrown-auth-relay".to_owned())
            .spawn(move || {
                let runtime = Builder::new_current_thread().enable_all().build()
                    .expect("failed to create TheCrown auth Tokio runtime");
                runtime.block_on(async move {
                    let client = match NatsClient::connect("nats://127.0.0.1:4222", Duration::from_millis(2_000)).await {
                        Ok(client) => client,
                        Err(error) => {
                            warn!("TheCrown auth could not connect to NATS: {error:#}");
                            return;
                        }
                    };
                    info!("TheCrown auth connected to Relay through NATS");
                    while let Some(command) = command_rx.recv().await {
                        match command {
                            WorkerCommand::Shutdown => break,
                            WorkerCommand::Admit(request) => {
                                let result = match client.request::<_, RelayPacket, _>(RELAY_SUBJECT, &RelayPacket::PlayerWantsToJoin { player: request.player }).await {
                                    Ok(RelayPacket::AccomodatePlayer { data }) => Ok(data),
                                    Ok(packet) => Err(format!("unexpected relay admission response: {packet:?}")),
                                    Err(error) => Err(error.to_string()),
                                };
                                let _ = result_tx.send(TheCrownAdmissionFinished { player_id: request.player_id, result });
                            }
                        }
                    }
                });
            })
            .expect("failed to start TheCrown auth relay worker");
        bevy.app
            .insert_resource(AuthRelayWorker { commands: command_tx, results: Mutex::new(result_rx), thread: Some(thread) })
            .add_message::<RequestTheCrownAdmission>()
            .add_message::<TheCrownAdmissionFinished>()
            .add_systems(Update, (submit_admissions, poll_admissions).chain());
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl TheCrownAuthRelayApi for TheCrownAuthRelayNatsMod {}

fn submit_admissions(
    worker: Res<AuthRelayWorker>,
    mut requests: MessageReader<RequestTheCrownAdmission>,
    mut results: MessageWriter<TheCrownAdmissionFinished>,
) {
    for request in requests.read() {
        if worker.commands.send(WorkerCommand::Admit(request.clone())).is_err() {
            warn!("TheCrown auth relay worker is unavailable");
            results.write(TheCrownAdmissionFinished {
                player_id: request.player_id,
                result: Err("TheCrown Relay connection is unavailable".to_owned()),
            });
        }
    }
}

fn poll_admissions(
    worker: Res<AuthRelayWorker>,
    mut results: MessageWriter<TheCrownAdmissionFinished>,
) {
    let receiver = worker.results.lock().expect("auth relay result channel poisoned");
    while let Ok(result) = receiver.try_recv() { results.write(result); }
}
