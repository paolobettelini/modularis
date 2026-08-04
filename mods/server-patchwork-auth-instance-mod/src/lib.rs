use bevy::prelude::*;
use bevy_mod::BevyMod;
use patchwork_game_auth_http_lib::{PatchworkAuthBackend, ServerInstanceCredentials};
use server_bevy_runner_mod::ServerBevyRunnerMod;
use std::{
    sync::{Arc, RwLock, mpsc},
    thread::JoinHandle as ThreadJoinHandle,
    time::Duration,
};
use tokio::task::JoinHandle;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(120);
const REGISTRATION_RETRY_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Clone)]
enum InstanceStatus {
    Starting,
    Active {
        credentials: ServerInstanceCredentials,
        generation: u64,
    },
    Unavailable(String),
}

#[derive(Resource, Clone)]
pub struct ServerPatchworkInstanceState(Arc<RwLock<InstanceStatus>>);

#[derive(Resource, Clone)]
pub struct ServerPatchworkAuthBackend(pub PatchworkAuthBackend);

impl ServerPatchworkInstanceState {
    fn starting() -> Self {
        Self(Arc::new(RwLock::new(InstanceStatus::Starting)))
    }

    pub fn credentials(&self) -> Option<(ServerInstanceCredentials, u64)> {
        match &*self.0.read().expect("Patchwork instance state poisoned") {
            InstanceStatus::Active {
                credentials,
                generation,
            } => Some((credentials.clone(), *generation)),
            _ => None,
        }
    }

    pub fn unavailable_reason(&self) -> Option<String> {
        match &*self.0.read().expect("Patchwork instance state poisoned") {
            InstanceStatus::Unavailable(reason) => Some(reason.clone()),
            _ => None,
        }
    }

    fn set(&self, status: InstanceStatus) {
        *self.0.write().expect("Patchwork instance state poisoned") = status;
    }
}

enum InstanceWorkerCommand {
    Shutdown,
}

#[derive(Resource)]
struct ServerPatchworkInstanceWorker {
    commands: mpsc::Sender<InstanceWorkerCommand>,
    thread: Option<ThreadJoinHandle<()>>,
}

impl Drop for ServerPatchworkInstanceWorker {
    fn drop(&mut self) {
        let _ = self.commands.send(InstanceWorkerCommand::Shutdown);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

pub struct ServerPatchworkAuthInstanceMod;

impl ServerPatchworkAuthInstanceMod {
    pub fn init(bevy: &mut BevyMod, _runner: &mut ServerBevyRunnerMod) -> Self {
        let state = ServerPatchworkInstanceState::starting();
        let backend = std::env::var("BACKEND_ADDR")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| "BACKEND_ADDR is not set".to_owned())
            .and_then(|address| {
                PatchworkAuthBackend::new(address).map_err(|error| error.to_string())
            });

        match backend {
            Ok(backend) => {
                let (commands, receiver) = mpsc::channel();
                let worker_state = state.clone();
                let worker_backend = backend.clone();
                let thread = std::thread::Builder::new()
                    .name("patchwork-server-instance".to_owned())
                    .spawn(move || instance_worker(worker_backend, worker_state, receiver))
                    .expect("failed to start Patchwork server instance worker");
                bevy.app
                    .insert_resource(ServerPatchworkAuthBackend(backend.clone()))
                    .insert_resource(state)
                    .insert_resource(ServerPatchworkInstanceWorker {
                        commands,
                        thread: Some(thread),
                    });
                info!("starting Patchwork server instance registration");
            }
            Err(reason) => {
                state.set(InstanceStatus::Unavailable(reason.clone()));
                bevy.app.insert_resource(state);
                warn!("Patchwork server authentication unavailable: {reason}");
            }
        }
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn instance_worker(
    backend: PatchworkAuthBackend,
    state: ServerPatchworkInstanceState,
    commands: mpsc::Receiver<InstanceWorkerCommand>,
) {
    let mut generation = 0_u64;
    let mut active: Option<ServerInstanceCredentials> = None;
    loop {
        if active.is_none() {
            match backend.create_server_instance() {
                Ok(credentials) => {
                    generation = generation.saturating_add(1);
                    info!(
                        "Patchwork server instance registered as {}",
                        credentials.server_id()
                    );
                    state.set(InstanceStatus::Active {
                        credentials: credentials.clone(),
                        generation,
                    });
                    active = Some(credentials);
                }
                Err(error) => {
                    let reason = error.to_string();
                    warn!("Patchwork server instance registration failed: {reason}");
                    state.set(InstanceStatus::Unavailable(reason));
                    match commands.recv_timeout(REGISTRATION_RETRY_INTERVAL) {
                        Ok(InstanceWorkerCommand::Shutdown)
                        | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        Err(mpsc::RecvTimeoutError::Timeout) => continue,
                    }
                }
            }
        }

        match commands.recv_timeout(HEARTBEAT_INTERVAL) {
            Ok(InstanceWorkerCommand::Shutdown) | Err(mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let Some(credentials) = active.as_ref() else {
                    continue;
                };
                match backend.heartbeat_server_instance(credentials) {
                    Ok(expires_in) => {
                        info!("Patchwork server instance heartbeat renewed (lease {expires_in}s)")
                    }
                    Err(error) => {
                        let reason = error.to_string();
                        warn!("Patchwork server instance heartbeat failed: {reason}");
                        state.set(InstanceStatus::Unavailable(reason));
                        active = None;
                    }
                }
            }
        }
    }

    if let Some(credentials) = active {
        if let Err(error) = backend.close_server_instance(&credentials) {
            warn!("could not close Patchwork server instance cleanly: {error}");
        } else {
            info!("Patchwork server instance closed cleanly");
        }
    }
}
