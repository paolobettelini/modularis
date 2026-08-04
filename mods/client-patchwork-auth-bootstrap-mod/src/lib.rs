use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi, GameStateCommand};
use client_session_api::{ClientSession, ClientSessionApi};
use patchwork_game_auth_api::{
    ClientPatchworkJoinGate, ClientPatchworkProcessAuthenticated, ClientProcessAuthState,
    ClientProcessAuthStatus,
};
use patchwork_game_auth_events_mod::PatchworkGameAuthEventsMod;
use patchwork_game_auth_http_lib::{AuthHttpError, PatchworkAuthBackend, ProcessSession};
use patchwork_game_auth_pipe_lib::{AuthPipeBootstrap, read_auth_pipe_from_environment};
use std::sync::{Mutex, mpsc};
use tokio::task::JoinHandle;

#[derive(Resource, Clone)]
pub struct ClientPatchworkAuthBackend(pub PatchworkAuthBackend);

#[derive(Resource)]
struct ProcessAuthResult(Mutex<mpsc::Receiver<Result<ProcessSession, String>>>);

pub struct ClientPatchworkAuthBootstrapMod;

impl ClientPatchworkAuthBootstrapMod {
    pub fn init<S: ClientSessionApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _session: &mut S,
        _game_state: &mut G,
        _events: &mut PatchworkGameAuthEventsMod,
    ) -> Self {
        match read_auth_pipe_from_environment() {
            Ok(AuthPipeBootstrap::Anonymous) => {
                bevy.app.insert_resource(ClientProcessAuthState::new(
                    ClientProcessAuthStatus::Anonymous,
                ));
                info!("Patchwork process authentication disabled for anonymous launch");
            }
            Ok(AuthPipeBootstrap::Authenticated {
                backend_address,
                launch_ticket,
            }) => match PatchworkAuthBackend::new(backend_address) {
                Ok(backend) => {
                    let gate = ClientPatchworkJoinGate::default();
                    gate.require_authentication();
                    let (sender, receiver) = mpsc::channel();
                    let worker_backend = backend.clone();
                    std::thread::Builder::new()
                        .name("patchwork-client-process-auth".to_owned())
                        .spawn(move || {
                            let result = worker_backend
                                .process_session(launch_ticket)
                                .map_err(safe_http_error);
                            let _ = sender.send(result);
                        })
                        .expect("failed to start Patchwork process authentication worker");
                    bevy.app
                        .insert_resource(ClientPatchworkAuthBackend(backend))
                        .insert_resource(gate)
                        .insert_resource(ClientProcessAuthState::new(
                            ClientProcessAuthStatus::Starting,
                        ))
                        .insert_resource(ProcessAuthResult(Mutex::new(receiver)))
                        .add_systems(Update, poll_process_authentication);
                    info!("started Patchwork process authentication");
                }
                Err(error) => install_failed_bootstrap(bevy, error.to_string()),
            },
            Err(error) => install_failed_bootstrap(bevy, error.to_string()),
        }
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn install_failed_bootstrap(bevy: &mut BevyMod, reason: String) {
    let gate = ClientPatchworkJoinGate::default();
    gate.require_authentication();
    bevy.app
        .insert_resource(gate)
        .insert_resource(ClientProcessAuthState::new(
            ClientProcessAuthStatus::Failed(reason.clone()),
        ))
        .add_systems(Update, report_bootstrap_failure);
    warn!("Patchwork process authentication setup failed: {reason}");
}

fn poll_process_authentication(
    mut commands: Commands,
    result: Option<Res<ProcessAuthResult>>,
    state: Res<ClientProcessAuthState>,
    game_state: Res<State<GameState>>,
    mut session: ResMut<ClientSession>,
    mut authenticated: MessageWriter<ClientPatchworkProcessAuthenticated>,
    mut state_commands: MessageWriter<GameStateCommand>,
) {
    let Some(result) = result else {
        return;
    };
    let result = result
        .0
        .lock()
        .expect("process auth result channel poisoned")
        .try_recv();
    let result = match result {
        Ok(result) => result,
        Err(mpsc::TryRecvError::Empty) => return,
        Err(mpsc::TryRecvError::Disconnected) => Err("authentication worker stopped".to_owned()),
    };
    commands.remove_resource::<ProcessAuthResult>();
    match result {
        Ok(process) => {
            info!(
                "Patchwork process authenticated for account {}",
                process.account.uuid
            );
            authenticated.write(ClientPatchworkProcessAuthenticated {
                account_uuid: process.account.uuid.clone(),
                nickname: process.account.nickname.clone(),
                process_session_id: process.process_session_id.clone(),
            });
            state.set(ClientProcessAuthStatus::Ready(process));
        }
        Err(reason) => {
            state.set(ClientProcessAuthStatus::Failed(reason.clone()));
            session.disconnect_reason = Some(format!("Patchwork authentication failed: {reason}"));
            warn!("Patchwork process authentication failed: {reason}");
            if *game_state.get() == GameState::InGame {
                state_commands.write(GameStateCommand::ShowDisconnect);
            }
        }
    }
}

fn report_bootstrap_failure(
    state: Res<ClientProcessAuthState>,
    game_state: Res<State<GameState>>,
    mut session: ResMut<ClientSession>,
    mut commands: MessageWriter<GameStateCommand>,
    mut reported: Local<bool>,
) {
    if *reported {
        return;
    }
    let ClientProcessAuthStatus::Failed(reason) = state.status() else {
        return;
    };
    session.disconnect_reason = Some(format!("Patchwork authentication failed: {reason}"));
    if *game_state.get() == GameState::InGame {
        commands.write(GameStateCommand::ShowDisconnect);
        *reported = true;
    }
}

fn safe_http_error(error: AuthHttpError) -> String {
    error.to_string()
}
