use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{
    ClientBoundMessage, ClientFinishReceived, KeyExchangeResponseReceived, NetworkMessageSet,
};
use network_frame_security_api::ServerFrameSecurity;
use network_frame_security_state_mod::NetworkFrameSecurityStateMod;
use network_protocol_mod::NetworkProtocolMod;
use network_transport_events_mod::{
    NetworkTransportEventsMod, ServerTransportConnected, ServerTransportDisconnectRequested,
    ServerTransportDisconnected,
};
use patchwork_auth_network_message_types::{
    AuthenticationFailed, KeyExchangeRequest, LoginSuccess,
};
use patchwork_game_auth_api::{
    AuthenticatedAccount, PatchworkSecureFrameCodec, ServerAuthenticatedAccounts,
    ServerPatchworkAccountAuthenticated,
};
use patchwork_game_auth_crypto_lib::{
    AUTH_PROTOCOL_VERSION, ConnectionRole, EphemeralPrivateKey, SecureChannel,
    canonical_transcript, constant_time_eq_32, encode_base64_32, random_nonce_32, transcript_hash,
};
use patchwork_game_auth_events_mod::PatchworkGameAuthEventsMod;
use patchwork_game_auth_http_lib::{
    RedeemHandshakeRequest, RedeemedPlayerSession, RegisterHandshakeRequest,
    ServerInstanceCredentials,
};
use server_network_api::{ServerNetworkApi, ServerNetworkSender};
use server_patchwork_auth_instance_mod::{
    ServerPatchworkAuthBackend, ServerPatchworkAuthInstanceMod, ServerPatchworkInstanceState,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Mutex, mpsc},
    time::{Duration, Instant},
};
use tokio::task::JoinHandle;
use uuid::Uuid;

const INSTANCE_WAIT_TIMEOUT: Duration = Duration::from_secs(15);

struct HandshakeMaterial {
    credentials: ServerInstanceCredentials,
    generation: u64,
    handshake_id: String,
    server_public_key: [u8; 32],
    server_nonce: [u8; 32],
    private_key: EphemeralPrivateKey,
}

enum ServerHandshakePhase {
    WaitingForInstance {
        since: Instant,
    },
    Registering {
        material: HandshakeMaterial,
        result: Mutex<mpsc::Receiver<Result<i64, String>>>,
    },
    Challenged {
        material: HandshakeMaterial,
        expires_at: Instant,
    },
    Redeeming {
        expected_hash: [u8; 32],
        channel: Option<SecureChannel>,
        result: Mutex<mpsc::Receiver<Result<RedeemedPlayerSession, String>>>,
    },
    AwaitingFinish {
        expected_hash: [u8; 32],
        redeemed: RedeemedPlayerSession,
    },
}

#[derive(Resource, Default)]
struct ServerHandshakeRuntime {
    connections: HashMap<SocketAddr, ServerHandshakePhase>,
}

pub struct ServerPatchworkAuthHandshakeMod;

impl ServerPatchworkAuthHandshakeMod {
    #[allow(clippy::too_many_arguments)]
    pub fn init<N: ServerNetworkApi>(
        bevy: &mut BevyMod,
        _network: &mut N,
        _protocol: &mut NetworkProtocolMod,
        _security_state: &mut NetworkFrameSecurityStateMod,
        _transport_events: &mut NetworkTransportEventsMod,
        _events: &mut PatchworkGameAuthEventsMod,
        _instance: &mut ServerPatchworkAuthInstanceMod,
    ) -> Self {
        bevy.app
            .init_resource::<ServerHandshakeRuntime>()
            .init_resource::<ServerAuthenticatedAccounts>()
            .add_systems(
                Update,
                (
                    begin_connected_handshakes,
                    poll_handshake_workers,
                    receive_key_exchange_response,
                    receive_client_finish,
                    cleanup_disconnected_handshakes,
                )
                    .chain()
                    .after(NetworkMessageSet::DispatchPackets),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn begin_connected_handshakes(
    mut connected: MessageReader<ServerTransportConnected>,
    backend: Option<Res<ServerPatchworkAuthBackend>>,
    instance: Res<ServerPatchworkInstanceState>,
    mut runtime: ResMut<ServerHandshakeRuntime>,
) {
    for connection in connected.read() {
        let phase = match (backend.as_ref(), instance.credentials()) {
            (Some(backend), Some((credentials, generation))) => {
                begin_registration(backend.0.clone(), credentials, generation)
            }
            _ => ServerHandshakePhase::WaitingForInstance {
                since: Instant::now(),
            },
        };
        runtime.connections.insert(connection.address, phase);
        info!(
            "started Patchwork authentication for {}",
            connection.address
        );
    }
}

fn begin_registration(
    backend: patchwork_game_auth_http_lib::PatchworkAuthBackend,
    credentials: ServerInstanceCredentials,
    generation: u64,
) -> ServerHandshakePhase {
    let handshake_id = Uuid::new_v4().hyphenated().to_string();
    let (private_key, server_public_key) = EphemeralPrivateKey::generate();
    let server_nonce = random_nonce_32();
    let request = RegisterHandshakeRequest {
        handshake_id: handshake_id.clone(),
        protocol_version: AUTH_PROTOCOL_VERSION,
        server_public_key: encode_base64_32(&server_public_key),
        server_nonce: encode_base64_32(&server_nonce),
    };
    let worker_credentials = credentials.clone();
    let (sender, receiver) = mpsc::channel();
    std::thread::Builder::new()
        .name("patchwork-server-handshake-register".to_owned())
        .spawn(move || {
            let result = backend
                .register_handshake(&worker_credentials, &request)
                .map_err(|error| error.to_string());
            let _ = sender.send(result);
        })
        .expect("failed to start Patchwork handshake registration worker");
    ServerHandshakePhase::Registering {
        material: HandshakeMaterial {
            credentials,
            generation,
            handshake_id,
            server_public_key,
            server_nonce,
            private_key,
        },
        result: Mutex::new(receiver),
    }
}

fn poll_handshake_workers(
    backend: Option<Res<ServerPatchworkAuthBackend>>,
    instance: Res<ServerPatchworkInstanceState>,
    network: Res<ServerNetworkSender>,
    security: Res<ServerFrameSecurity>,
    mut runtime: ResMut<ServerHandshakeRuntime>,
    mut disconnect: MessageWriter<ServerTransportDisconnectRequested>,
) {
    let addresses = runtime.connections.keys().copied().collect::<Vec<_>>();
    for address in addresses {
        let Some(phase) = runtime.connections.remove(&address) else {
            continue;
        };
        let next = match phase {
            ServerHandshakePhase::WaitingForInstance { since } => {
                if let (Some(backend), Some((credentials, generation))) =
                    (backend.as_ref(), instance.credentials())
                {
                    Some(begin_registration(
                        backend.0.clone(),
                        credentials,
                        generation,
                    ))
                } else if since.elapsed() >= INSTANCE_WAIT_TIMEOUT {
                    reject_connection(
                        address,
                        "Patchwork authentication service is unavailable",
                        false,
                        &network,
                        &security,
                        &mut disconnect,
                    );
                    None
                } else {
                    Some(ServerHandshakePhase::WaitingForInstance { since })
                }
            }
            ServerHandshakePhase::Registering { material, result } => {
                let polled = result
                    .lock()
                    .expect("handshake registration result channel poisoned")
                    .try_recv();
                match polled {
                    Err(mpsc::TryRecvError::Empty) => {
                        Some(ServerHandshakePhase::Registering { material, result })
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        reject_connection(
                            address,
                            "Patchwork handshake registration worker stopped",
                            false,
                            &network,
                            &security,
                            &mut disconnect,
                        );
                        None
                    }
                    Ok(Err(reason)) => {
                        warn!("handshake registration for {address} failed: {reason}");
                        reject_connection(
                            address,
                            "Could not register a Patchwork authentication handshake",
                            false,
                            &network,
                            &security,
                            &mut disconnect,
                        );
                        None
                    }
                    Ok(Ok(expires_in)) => {
                        let current_generation =
                            instance.credentials().map(|(_, generation)| generation);
                        if current_generation != Some(material.generation) {
                            reject_connection(
                                address,
                                "Patchwork server instance changed during authentication",
                                false,
                                &network,
                                &security,
                                &mut disconnect,
                            );
                            None
                        } else {
                            let request = KeyExchangeRequest {
                                protocol_version: AUTH_PROTOCOL_VERSION,
                                handshake_id: material.handshake_id.clone(),
                                server_id: material.credentials.server_id().to_owned(),
                                server_public_key: material.server_public_key,
                                server_nonce: material.server_nonce,
                            };
                            match network
                                .send_to(address, &ClientBoundMessage::KeyExchangeRequest(request))
                            {
                                Ok(()) => {
                                    info!(
                                        "sent Patchwork key exchange challenge {} to {address}",
                                        material.handshake_id
                                    );
                                    Some(ServerHandshakePhase::Challenged {
                                        material,
                                        expires_at: Instant::now()
                                            + Duration::from_secs(expires_in.max(1) as u64),
                                    })
                                }
                                Err(error) => {
                                    warn!(
                                        "could not send authentication challenge to {address}: {error}"
                                    );
                                    disconnect
                                        .write(ServerTransportDisconnectRequested { address });
                                    None
                                }
                            }
                        }
                    }
                }
            }
            ServerHandshakePhase::Challenged {
                material,
                expires_at,
            } => {
                if Instant::now() >= expires_at {
                    reject_connection(
                        address,
                        "Patchwork authentication handshake expired",
                        false,
                        &network,
                        &security,
                        &mut disconnect,
                    );
                    None
                } else {
                    Some(ServerHandshakePhase::Challenged {
                        material,
                        expires_at,
                    })
                }
            }
            ServerHandshakePhase::Redeeming {
                expected_hash,
                mut channel,
                result,
            } => {
                let polled = result
                    .lock()
                    .expect("handshake redeem result channel poisoned")
                    .try_recv();
                match polled {
                    Err(mpsc::TryRecvError::Empty) => Some(ServerHandshakePhase::Redeeming {
                        expected_hash,
                        channel,
                        result,
                    }),
                    Err(mpsc::TryRecvError::Disconnected) => {
                        reject_after_key_exchange(
                            address,
                            "Patchwork handshake redemption worker stopped",
                            channel,
                            &network,
                            &security,
                            &mut disconnect,
                        );
                        None
                    }
                    Ok(Err(reason)) => {
                        warn!("handshake redemption for {address} failed: {reason}");
                        reject_after_key_exchange(
                            address,
                            "Patchwork did not authorize this server join",
                            channel,
                            &network,
                            &security,
                            &mut disconnect,
                        );
                        None
                    }
                    Ok(Ok(redeemed)) => {
                        let Some(channel) = channel.take() else {
                            reject_connection(
                                address,
                                "secure channel state is missing",
                                false,
                                &network,
                                &security,
                                &mut disconnect,
                            );
                            continue;
                        };
                        if let Err(error) =
                            security.activate(address, PatchworkSecureFrameCodec::new(channel))
                        {
                            warn!("could not activate secure channel for {address}: {error}");
                            disconnect.write(ServerTransportDisconnectRequested { address });
                            None
                        } else {
                            info!("Patchwork backend redeemed handshake for {address}");
                            Some(ServerHandshakePhase::AwaitingFinish {
                                expected_hash,
                                redeemed,
                            })
                        }
                    }
                }
            }
            awaiting @ ServerHandshakePhase::AwaitingFinish { .. } => Some(awaiting),
        };
        if let Some(next) = next {
            runtime.connections.insert(address, next);
        }
    }
}

fn receive_key_exchange_response(
    mut responses: MessageReader<KeyExchangeResponseReceived>,
    backend: Option<Res<ServerPatchworkAuthBackend>>,
    security: Res<ServerFrameSecurity>,
    network: Res<ServerNetworkSender>,
    mut runtime: ResMut<ServerHandshakeRuntime>,
    mut disconnect: MessageWriter<ServerTransportDisconnectRequested>,
) {
    for response in responses.read() {
        let Some(backend) = backend.as_ref() else {
            reject_connection(
                response.source,
                "Patchwork authentication service is unavailable",
                false,
                &network,
                &security,
                &mut disconnect,
            );
            continue;
        };
        let Some(phase) = runtime.connections.remove(&response.source) else {
            reject_connection(
                response.source,
                "unexpected Patchwork key exchange response",
                false,
                &network,
                &security,
                &mut disconnect,
            );
            continue;
        };
        let ServerHandshakePhase::Challenged {
            material,
            expires_at,
        } = phase
        else {
            reject_connection(
                response.source,
                "Patchwork key exchange response arrived in the wrong state",
                false,
                &network,
                &security,
                &mut disconnect,
            );
            continue;
        };
        if Instant::now() >= expires_at || response.message.handshake_id != material.handshake_id {
            reject_connection(
                response.source,
                "Patchwork authentication handshake is invalid or expired",
                false,
                &network,
                &security,
                &mut disconnect,
            );
            continue;
        }
        let prepared = (|| {
            let transcript = canonical_transcript(
                AUTH_PROTOCOL_VERSION,
                &material.handshake_id,
                material.credentials.server_id(),
                &material.server_public_key,
                &response.message.client_public_key,
                &material.server_nonce,
                &response.message.client_nonce,
            )?;
            let expected_hash = transcript_hash(&transcript);
            if !constant_time_eq_32(&expected_hash, &response.message.handshake_hash) {
                return Err(patchwork_game_auth_crypto_lib::AuthCryptoError::AuthenticationFailed);
            }
            let shared = material
                .private_key
                .agree(response.message.client_public_key)?;
            let channel = SecureChannel::derive(ConnectionRole::Server, &shared, &expected_hash)?;
            Ok::<_, patchwork_game_auth_crypto_lib::AuthCryptoError>((expected_hash, channel))
        })();
        let (expected_hash, channel) = match prepared {
            Ok(prepared) => prepared,
            Err(error) => {
                warn!(
                    "invalid key exchange response from {}: {error}",
                    response.source
                );
                reject_connection(
                    response.source,
                    "invalid Patchwork key exchange response",
                    false,
                    &network,
                    &security,
                    &mut disconnect,
                );
                continue;
            }
        };
        if let Err(error) = security.pause(response.source) {
            warn!(
                "could not pause secure transport for {}: {error}",
                response.source
            );
            disconnect.write(ServerTransportDisconnectRequested {
                address: response.source,
            });
            continue;
        }
        let redeem_request = RedeemHandshakeRequest {
            client_public_key: encode_base64_32(&response.message.client_public_key),
            client_nonce: encode_base64_32(&response.message.client_nonce),
            handshake_hash: encode_base64_32(&expected_hash),
        };
        let credentials = material.credentials;
        let handshake_id = material.handshake_id;
        let backend = backend.0.clone();
        let (sender, receiver) = mpsc::channel();
        std::thread::Builder::new()
            .name("patchwork-server-handshake-redeem".to_owned())
            .spawn(move || {
                let result = backend
                    .redeem_handshake(&credentials, &handshake_id, &redeem_request)
                    .map_err(|error| error.to_string());
                let _ = sender.send(result);
            })
            .expect("failed to start Patchwork handshake redemption worker");
        runtime.connections.insert(
            response.source,
            ServerHandshakePhase::Redeeming {
                expected_hash,
                channel: Some(channel),
                result: Mutex::new(receiver),
            },
        );
    }
}

fn receive_client_finish(
    mut finishes: MessageReader<ClientFinishReceived>,
    network: Res<ServerNetworkSender>,
    security: Res<ServerFrameSecurity>,
    accounts: Res<ServerAuthenticatedAccounts>,
    mut runtime: ResMut<ServerHandshakeRuntime>,
    mut authenticated: MessageWriter<ServerPatchworkAccountAuthenticated>,
    mut disconnect: MessageWriter<ServerTransportDisconnectRequested>,
) {
    for finish in finishes.read() {
        let Some(phase) = runtime.connections.remove(&finish.source) else {
            reject_connection(
                finish.source,
                "unexpected encrypted authentication finish",
                true,
                &network,
                &security,
                &mut disconnect,
            );
            continue;
        };
        let ServerHandshakePhase::AwaitingFinish {
            expected_hash,
            redeemed,
        } = phase
        else {
            reject_connection(
                finish.source,
                "authentication finish arrived in the wrong state",
                true,
                &network,
                &security,
                &mut disconnect,
            );
            continue;
        };
        if !constant_time_eq_32(&expected_hash, &finish.message.handshake_hash) {
            reject_connection(
                finish.source,
                "authentication finish did not match the current handshake",
                true,
                &network,
                &security,
                &mut disconnect,
            );
            continue;
        }
        let account = AuthenticatedAccount {
            account_uuid: redeemed.account.uuid,
            nickname: redeemed.account.nickname,
            player_session_id: redeemed.player_session_id,
            admission: redeemed.admission,
            source_server_id: redeemed.source_server_id,
        };
        accounts.authenticate(finish.source, account.clone());
        authenticated.write(ServerPatchworkAccountAuthenticated {
            address: finish.source,
            account: account.clone(),
        });
        let login = LoginSuccess {
            player_session_id: account.player_session_id.clone(),
            account_uuid: account.account_uuid.clone(),
            nickname: account.nickname.clone(),
            admission: account.admission.clone(),
            source_server_id: account.source_server_id.clone(),
        };
        if let Err(error) = network.send_to(finish.source, &ClientBoundMessage::LoginSuccess(login))
        {
            warn!("could not send LoginSuccess to {}: {error}", finish.source);
            disconnect.write(ServerTransportDisconnectRequested {
                address: finish.source,
            });
            continue;
        }
        info!(
            "Patchwork account {} authenticated from {}",
            account.account_uuid, finish.source
        );
    }
}

fn cleanup_disconnected_handshakes(
    mut disconnected: MessageReader<ServerTransportDisconnected>,
    accounts: Res<ServerAuthenticatedAccounts>,
    mut runtime: ResMut<ServerHandshakeRuntime>,
) {
    for disconnected in disconnected.read() {
        runtime.connections.remove(&disconnected.address);
        accounts.remove_address(disconnected.address);
    }
}

fn reject_connection(
    address: SocketAddr,
    reason: &str,
    channel_is_secure: bool,
    network: &ServerNetworkSender,
    security: &ServerFrameSecurity,
    disconnect: &mut MessageWriter<ServerTransportDisconnectRequested>,
) {
    if !channel_is_secure {
        security.register_plaintext(address);
    }
    let reason = reason.chars().take(512).collect::<String>();
    if let Err(error) = network.send_to(
        address,
        &ClientBoundMessage::AuthenticationFailed(AuthenticationFailed {
            reason: reason.clone(),
        }),
    ) {
        warn!("could not send authentication failure to {address}: {error}");
    }
    warn!("rejected Patchwork authentication from {address}: {reason}");
    disconnect.write(ServerTransportDisconnectRequested { address });
}

fn reject_after_key_exchange(
    address: SocketAddr,
    reason: &str,
    channel: Option<SecureChannel>,
    network: &ServerNetworkSender,
    security: &ServerFrameSecurity,
    disconnect: &mut MessageWriter<ServerTransportDisconnectRequested>,
) {
    let Some(channel) = channel else {
        disconnect.write(ServerTransportDisconnectRequested { address });
        return;
    };
    if let Err(error) = security.activate(address, PatchworkSecureFrameCodec::new(channel)) {
        warn!("could not activate rejection channel for {address}: {error}");
        disconnect.write(ServerTransportDisconnectRequested { address });
        return;
    }
    reject_connection(address, reason, true, network, security, disconnect);
}
