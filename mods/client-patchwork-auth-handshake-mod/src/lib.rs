use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, GameStateCommand};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use client_patchwork_auth_bootstrap_mod::{
    ClientPatchworkAuthBackend, ClientPatchworkAuthBootstrapMod,
};
use client_session_api::{ClientSession, ClientSessionApi};
use generated_network_messages::{
    AuthenticationFailedReceived, KeyExchangeRequestReceived, LoginSuccessReceived,
    NetworkMessageSet, ServerBoundMessage,
};
use network_frame_security_api::ClientFrameSecurity;
use network_frame_security_state_mod::NetworkFrameSecurityStateMod;
use network_protocol_mod::NetworkProtocolMod;
use network_transport_events_mod::{
    ClientTransportConnected, ClientTransportDisconnectRequested, ClientTransportDisconnected,
    NetworkTransportEventsMod,
};
use patchwork_auth_network_message_types::{ClientFinish, KeyExchangeResponse};
use patchwork_game_auth_api::{
    AuthenticatedAccount, ClientPatchworkGameAuthenticated, ClientPatchworkJoinGate,
    ClientProcessAuthState, ClientProcessAuthStatus, PatchworkSecureFrameCodec,
};
use patchwork_game_auth_crypto_lib::{
    AUTH_PROTOCOL_VERSION, ConnectionRole, EphemeralPrivateKey, SecureChannel,
    canonical_transcript, encode_base64_32, random_nonce_32, transcript_hash,
};
use patchwork_game_auth_events_mod::PatchworkGameAuthEventsMod;
use patchwork_game_auth_http_lib::AuthorizeHandshakeRequest;
use std::{
    sync::{Mutex, mpsc},
    time::{Duration, Instant},
};
use tokio::task::JoinHandle;

const SERVER_CHALLENGE_TIMEOUT: Duration = Duration::from_secs(20);

struct PreparedClientHandshake {
    response: KeyExchangeResponse,
    channel: SecureChannel,
    expected_hash: [u8; 32],
}

struct PendingAuthorization {
    prepared: Option<PreparedClientHandshake>,
    result: Mutex<mpsc::Receiver<Result<(), String>>>,
}

#[derive(Resource, Default)]
struct ClientHandshakeRuntime {
    connected_since: Option<Instant>,
    deferred: Option<patchwork_auth_network_message_types::KeyExchangeRequest>,
    pending: Option<PendingAuthorization>,
    awaiting_login_hash: Option<[u8; 32]>,
}

pub struct ClientPatchworkAuthHandshakeMod;

impl ClientPatchworkAuthHandshakeMod {
    #[allow(clippy::too_many_arguments)]
    pub fn init<N: ClientNetworkApi, S: ClientSessionApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _network: &mut N,
        _session: &mut S,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
        _security_state: &mut NetworkFrameSecurityStateMod,
        _transport_events: &mut NetworkTransportEventsMod,
        _events: &mut PatchworkGameAuthEventsMod,
        _bootstrap: &mut ClientPatchworkAuthBootstrapMod,
    ) -> Self {
        bevy.app
            .init_resource::<ClientHandshakeRuntime>()
            .add_systems(
                Update,
                (
                    reset_on_connection,
                    receive_key_exchange_request.after(NetworkMessageSet::DispatchPackets),
                    start_deferred_handshake,
                    timeout_waiting_for_challenge,
                    poll_handshake_authorization,
                    receive_login_success.after(NetworkMessageSet::DispatchPackets),
                    receive_authentication_failure.after(NetworkMessageSet::DispatchPackets),
                    cleanup_disconnected_handshake,
                ),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn reset_on_connection(
    mut connected: MessageReader<ClientTransportConnected>,
    gate: Option<Res<ClientPatchworkJoinGate>>,
    mut runtime: ResMut<ClientHandshakeRuntime>,
) {
    if connected.read().next().is_none() {
        return;
    }
    runtime.deferred = None;
    runtime.pending = None;
    runtime.awaiting_login_hash = None;
    runtime.connected_since = Some(Instant::now());
    if let Some(gate) = gate {
        gate.reset();
    }
    info!("TCP transport established; waiting for Patchwork authentication challenge");
}

fn receive_key_exchange_request(
    mut requests: MessageReader<KeyExchangeRequestReceived>,
    process: Res<ClientProcessAuthState>,
    mut runtime: ResMut<ClientHandshakeRuntime>,
    mut session: ResMut<ClientSession>,
    mut state_commands: MessageWriter<GameStateCommand>,
    mut disconnect: MessageWriter<ClientTransportDisconnectRequested>,
) {
    for request in requests.read() {
        runtime.connected_since = None;
        info!(
            "received Patchwork authentication challenge {}",
            request.0.handshake_id
        );
        if runtime.pending.is_some() || runtime.awaiting_login_hash.is_some() {
            fail_client(
                "server started a second authentication handshake".to_owned(),
                &mut session,
                &mut state_commands,
                &mut disconnect,
            );
            continue;
        }
        match process.status() {
            ClientProcessAuthStatus::Ready(_) | ClientProcessAuthStatus::Starting => {
                runtime.deferred = Some(request.0.clone());
            }
            ClientProcessAuthStatus::Anonymous => fail_client(
                "this server requires a logged-in Patchwork launch".to_owned(),
                &mut session,
                &mut state_commands,
                &mut disconnect,
            ),
            ClientProcessAuthStatus::Failed(reason) => fail_client(
                format!("Patchwork process authentication failed: {reason}"),
                &mut session,
                &mut state_commands,
                &mut disconnect,
            ),
        }
    }
}

fn timeout_waiting_for_challenge(
    mut runtime: ResMut<ClientHandshakeRuntime>,
    mut session: ResMut<ClientSession>,
    mut state_commands: MessageWriter<GameStateCommand>,
    mut disconnect: MessageWriter<ClientTransportDisconnectRequested>,
) {
    let Some(connected_since) = runtime.connected_since else {
        return;
    };
    if connected_since.elapsed() < SERVER_CHALLENGE_TIMEOUT {
        return;
    }

    runtime.connected_since = None;
    fail_client(
        "server did not start Patchwork authentication; check that its composition includes server-patchwork-auth"
            .to_owned(),
        &mut session,
        &mut state_commands,
        &mut disconnect,
    );
}

fn start_deferred_handshake(
    backend: Option<Res<ClientPatchworkAuthBackend>>,
    process: Res<ClientProcessAuthState>,
    mut runtime: ResMut<ClientHandshakeRuntime>,
    mut session: ResMut<ClientSession>,
    mut state_commands: MessageWriter<GameStateCommand>,
    mut disconnect: MessageWriter<ClientTransportDisconnectRequested>,
) {
    if runtime.pending.is_some() || runtime.awaiting_login_hash.is_some() {
        return;
    }
    let Some(request) = runtime.deferred.clone() else {
        return;
    };
    let ClientProcessAuthStatus::Ready(process) = process.status() else {
        return;
    };
    let Some(backend) = backend else {
        fail_client(
            "Patchwork authentication backend is unavailable".to_owned(),
            &mut session,
            &mut state_commands,
            &mut disconnect,
        );
        runtime.deferred = None;
        return;
    };
    if request.protocol_version != AUTH_PROTOCOL_VERSION {
        fail_client(
            format!(
                "server uses unsupported Patchwork authentication protocol {}",
                request.protocol_version
            ),
            &mut session,
            &mut state_commands,
            &mut disconnect,
        );
        runtime.deferred = None;
        return;
    }

    let prepared = (|| {
        let (private, client_public_key) = EphemeralPrivateKey::generate();
        let client_nonce = random_nonce_32();
        let transcript = canonical_transcript(
            request.protocol_version,
            &request.handshake_id,
            &request.server_id,
            &request.server_public_key,
            &client_public_key,
            &request.server_nonce,
            &client_nonce,
        )?;
        let expected_hash = transcript_hash(&transcript);
        let shared = private.agree(request.server_public_key)?;
        let channel = SecureChannel::derive(ConnectionRole::Client, &shared, &expected_hash)?;
        Ok::<_, patchwork_game_auth_crypto_lib::AuthCryptoError>((
            PreparedClientHandshake {
                response: KeyExchangeResponse {
                    handshake_id: request.handshake_id.clone(),
                    client_public_key,
                    client_nonce,
                    handshake_hash: expected_hash,
                },
                channel,
                expected_hash,
            },
            AuthorizeHandshakeRequest {
                protocol_version: request.protocol_version,
                handshake_id: request.handshake_id.clone(),
                server_id: request.server_id.clone(),
                server_public_key: encode_base64_32(&request.server_public_key),
                client_public_key: encode_base64_32(&client_public_key),
                server_nonce: encode_base64_32(&request.server_nonce),
                client_nonce: encode_base64_32(&client_nonce),
                handshake_hash: encode_base64_32(&expected_hash),
                transfer_ticket: None,
            },
        ))
    })();
    let (prepared, authorize_request) = match prepared {
        Ok(prepared) => prepared,
        Err(error) => {
            fail_client(
                format!("invalid server authentication challenge: {error}"),
                &mut session,
                &mut state_commands,
                &mut disconnect,
            );
            runtime.deferred = None;
            return;
        }
    };

    let (sender, receiver) = mpsc::channel();
    let backend = backend.0.clone();
    std::thread::Builder::new()
        .name("patchwork-client-handshake-auth".to_owned())
        .spawn(move || {
            let result = backend
                .authorize_handshake(&process, &authorize_request)
                .map_err(|error| error.to_string());
            let _ = sender.send(result);
        })
        .expect("failed to start Patchwork handshake authorization worker");
    info!("authorizing Patchwork handshake {}", request.handshake_id);
    runtime.deferred = None;
    runtime.pending = Some(PendingAuthorization {
        prepared: Some(prepared),
        result: Mutex::new(receiver),
    });
}

fn poll_handshake_authorization(
    sender: Option<Res<ClientNetworkSender>>,
    security: Res<ClientFrameSecurity>,
    mut runtime: ResMut<ClientHandshakeRuntime>,
    mut session: ResMut<ClientSession>,
    mut state_commands: MessageWriter<GameStateCommand>,
    mut disconnect: MessageWriter<ClientTransportDisconnectRequested>,
) {
    let Some(pending) = runtime.pending.as_mut() else {
        return;
    };
    let result = pending
        .result
        .lock()
        .expect("client handshake result channel poisoned")
        .try_recv();
    let result = match result {
        Ok(result) => result,
        Err(mpsc::TryRecvError::Empty) => return,
        Err(mpsc::TryRecvError::Disconnected) => {
            Err("handshake authorization worker stopped".to_owned())
        }
    };
    let Some(prepared) = pending.prepared.take() else {
        return;
    };
    runtime.pending = None;
    if let Err(reason) = result {
        fail_client(
            format!("server join authorization failed: {reason}"),
            &mut session,
            &mut state_commands,
            &mut disconnect,
        );
        return;
    }
    let Some(sender) = sender else {
        fail_client(
            "connection closed during authentication".to_owned(),
            &mut session,
            &mut state_commands,
            &mut disconnect,
        );
        return;
    };
    if let Err(error) = sender.send(&ServerBoundMessage::KeyExchangeResponse(
        prepared.response.clone(),
    )) {
        fail_client(
            format!("could not send key exchange response: {error}"),
            &mut session,
            &mut state_commands,
            &mut disconnect,
        );
        return;
    }
    security.activate(PatchworkSecureFrameCodec::new(prepared.channel));
    if let Err(error) = sender.send(&ServerBoundMessage::ClientFinish(ClientFinish {
        handshake_hash: prepared.expected_hash,
    })) {
        security.fail();
        fail_client(
            format!("could not finish secure channel: {error}"),
            &mut session,
            &mut state_commands,
            &mut disconnect,
        );
        return;
    }
    runtime.awaiting_login_hash = Some(prepared.expected_hash);
    info!("Patchwork key exchange authorized; secure channel enabled");
}

fn receive_login_success(
    mut logins: MessageReader<LoginSuccessReceived>,
    process: Res<ClientProcessAuthState>,
    gate: Option<Res<ClientPatchworkJoinGate>>,
    mut runtime: ResMut<ClientHandshakeRuntime>,
    mut authenticated: MessageWriter<ClientPatchworkGameAuthenticated>,
    mut session: ResMut<ClientSession>,
    mut state_commands: MessageWriter<GameStateCommand>,
    mut disconnect: MessageWriter<ClientTransportDisconnectRequested>,
) {
    for login in logins.read() {
        let Some(gate) = gate.as_ref() else {
            fail_client(
                "received authenticated login on an anonymous launch".to_owned(),
                &mut session,
                &mut state_commands,
                &mut disconnect,
            );
            continue;
        };
        if runtime.awaiting_login_hash.take().is_none() {
            fail_client(
                "received LoginSuccess outside an authentication handshake".to_owned(),
                &mut session,
                &mut state_commands,
                &mut disconnect,
            );
            continue;
        }
        let ClientProcessAuthStatus::Ready(process) = process.status() else {
            fail_client(
                "local Patchwork process session disappeared".to_owned(),
                &mut session,
                &mut state_commands,
                &mut disconnect,
            );
            continue;
        };
        if login.0.account_uuid != process.account.uuid {
            fail_client(
                "server authenticated a different Patchwork account".to_owned(),
                &mut session,
                &mut state_commands,
                &mut disconnect,
            );
            continue;
        }
        let account = AuthenticatedAccount {
            account_uuid: login.0.account_uuid.clone(),
            nickname: login.0.nickname.clone(),
            player_session_id: login.0.player_session_id.clone(),
            admission: login.0.admission.clone(),
            source_server_id: login.0.source_server_id.clone(),
        };
        gate.authorize(account.clone());
        authenticated.write(ClientPatchworkGameAuthenticated {
            account: account.clone(),
        });
        info!(
            "authenticated game session {} for account {}",
            account.player_session_id, account.account_uuid
        );
    }
}

fn receive_authentication_failure(
    mut failures: MessageReader<AuthenticationFailedReceived>,
    mut session: ResMut<ClientSession>,
    mut state_commands: MessageWriter<GameStateCommand>,
    mut disconnect: MessageWriter<ClientTransportDisconnectRequested>,
) {
    for failure in failures.read() {
        fail_client(
            failure.0.reason.clone(),
            &mut session,
            &mut state_commands,
            &mut disconnect,
        );
    }
}

fn cleanup_disconnected_handshake(
    mut disconnected: MessageReader<ClientTransportDisconnected>,
    gate: Option<Res<ClientPatchworkJoinGate>>,
    mut runtime: ResMut<ClientHandshakeRuntime>,
) {
    if disconnected.read().next().is_none() {
        return;
    }
    runtime.deferred = None;
    runtime.pending = None;
    runtime.awaiting_login_hash = None;
    runtime.connected_since = None;
    if let Some(gate) = gate {
        gate.reset();
    }
}

fn fail_client(
    reason: String,
    session: &mut ClientSession,
    state_commands: &mut MessageWriter<GameStateCommand>,
    disconnect: &mut MessageWriter<ClientTransportDisconnectRequested>,
) {
    warn!("Patchwork game authentication failed: {reason}");
    session.player_id = None;
    session.disconnect_reason = Some(reason);
    state_commands.write(GameStateCommand::ShowDisconnect);
    disconnect.write(ClientTransportDisconnectRequested);
}
