use chrono::Utc;
use thecrown_protocol::{
    AccomodatePlayerData, DataTransferResult, GameMode, GameServerPacket, ParkourRecordUpdate,
    RelayPacket,
    game_server_subject,
};

use crate::state::State;

pub async fn handle_msg(state: &State, packet: RelayPacket) -> Option<RelayPacket> {
    match packet {
        RelayPacket::PlayerWantsToJoin { player } => {
            log::info!("join request: {} ({})", player.username, player.uuid);

            match state.db_get_active_ban(player.uuid).await {
                Ok(Some(ban)) => {
                    let time_left_seconds = ban.ban_end.map(|end| {
                        (end.and_utc().timestamp() - Utc::now().timestamp()).max(0)
                    });
                    return Some(RelayPacket::AccomodatePlayer {
                        data: AccomodatePlayerData::Ban {
                            reason: ban.reason,
                            time_left_seconds,
                        },
                    });
                }
                Ok(None) => {}
                Err(error) => {
                    log::error!("ban lookup failed for {}: {error:#}", player.uuid);
                    return Some(RelayPacket::AccomodatePlayer {
                        data: AccomodatePlayerData::Unavailable {
                            reason: "database unavailable".to_owned(),
                        },
                    });
                }
            }

            if let Err(error) = state.db_upsert_player(player.clone()).await {
                log::error!("player upsert failed for {}: {error:#}", player.uuid);
                return Some(RelayPacket::AccomodatePlayer {
                    data: AccomodatePlayerData::Unavailable {
                        reason: "database unavailable".to_owned(),
                    },
                });
            }

            match state.transfer_new_player_to_hub(player).await {
                Some(transfer) => Some(RelayPacket::AccomodatePlayer {
                    data: AccomodatePlayerData::Join { transfer },
                }),
                None => Some(RelayPacket::AccomodatePlayer {
                    data: AccomodatePlayerData::Unavailable {
                        reason: "no hub instance is currently available".to_owned(),
                    },
                }),
            }
        }

        RelayPacket::RegisterServer {
            server_id,
            address,
            port,
        } => {
            log::info!("registering game server {server_id} at {address}:{port}");
            if let Err(error) = state
                .register_game_server(server_id.clone(), address, port)
                .await
            {
                log::error!("failed to register game server {server_id}: {error:#}");
            }
            None
        }

        RelayPacket::UnregisterServer { server_id } => {
            log::info!("unregistering game server {server_id}");
            state.unregister_game_server(&server_id).await;
            None
        }

        RelayPacket::AuthUserJoin {
            player_uuid,
            server_id,
            instance_id,
            cookie,
        } => {
            let value = state
                .try_auth_user(&cookie, player_uuid, &server_id, &instance_id)
                .await;
            Some(RelayPacket::ServeAuthResult { value })
        }

        RelayPacket::PlayerQuit {
            player_uuid,
            server_id,
            instance_id,
        } => {
            if state.player_quit(player_uuid, &server_id, &instance_id).await {
                log::info!("player {player_uuid} left {server_id}/{instance_id}");
            } else {
                log::debug!(
                    "ignored stale quit for {player_uuid} from {server_id}/{instance_id}"
                );
            }
            None
        }

        RelayPacket::GetOnlinePlayers => Some(RelayPacket::ServeOnlinePlayers {
            count: state.get_online_players().await,
        }),

        RelayPacket::WhisperCommand {
            sender_uuid,
            target_uuid,
            message,
        } => {
            let Some((target_server_id, sender)) =
                state.whisper_target(sender_uuid, target_uuid).await
            else {
                return Some(RelayPacket::WhisperCommandResponse { status: false });
            };

            let packet = GameServerPacket::WhisperCommand {
                sender,
                target_uuid,
                message,
            };
            let status = state
                .nats_client
                .publish(game_server_subject(&target_server_id), &packet)
                .await
                .map(|_| true)
                .unwrap_or_else(|error| {
                    log::error!("failed to forward whisper: {error:#}");
                    false
                });
            Some(RelayPacket::WhisperCommandResponse { status })
        }

        RelayPacket::WhisperCommandByName {
            sender_uuid,
            target_username,
            message,
        } => {
            let target = match state.db_get_player_by_username(target_username).await {
                Ok(Some(player)) => player.uuid,
                Ok(None) => return Some(RelayPacket::WhisperCommandResponse { status: false }),
                Err(error) => {
                    log::error!("whisper player lookup failed: {error:#}");
                    return Some(RelayPacket::WhisperCommandResponse { status: false });
                }
            };
            let Some((target_server_id, sender)) = state.whisper_target(sender_uuid, target).await
            else {
                return Some(RelayPacket::WhisperCommandResponse { status: false });
            };
            let packet = GameServerPacket::WhisperCommand {
                sender,
                target_uuid: target,
                message,
            };
            let status = state
                .nats_client
                .publish(game_server_subject(&target_server_id), &packet)
                .await
                .map(|_| true)
                .unwrap_or_else(|error| {
                    log::error!("failed to forward whisper: {error:#}");
                    false
                });
            Some(RelayPacket::WhisperCommandResponse { status })
        }

        RelayPacket::GetPlayerStatus { player_uuid } => {
            Some(RelayPacket::ServePlayerStatus {
                status: state.get_player_status(player_uuid).await,
            })
        }

        RelayPacket::GetParkourRecord { player_uuid } => {
            let record = match state.db_get_parkour_record(player_uuid).await {
                Ok(record) => record,
                Err(error) => {
                    log::error!("parkour record lookup failed for {player_uuid}: {error:#}");
                    None
                }
            };
            Some(RelayPacket::ServeParkourRecord { record })
        }

        RelayPacket::SubmitParkourRecord { player_uuid, score } => {
            let update = match state.db_submit_parkour_record(player_uuid, score).await {
                Ok(Some((previous, current, improved))) => Some(ParkourRecordUpdate {
                    previous,
                    current,
                    improved,
                }),
                Ok(None) => None,
                Err(error) => {
                    log::error!("parkour record update failed for {player_uuid}: {error:#}");
                    None
                }
            };
            Some(RelayPacket::ServeParkourRecordUpdate { update })
        }

        RelayPacket::PlayerEnterSpecificServer {
            player_uuid,
            instance_id,
        } => Some(RelayPacket::ServePlayerTransfer {
            data: match state
                .transfer_online_player_to_instance(player_uuid, &instance_id)
                .await
            {
                Some(transfer) => DataTransferResult::Join { transfer },
                None => DataTransferResult::NotFound,
            },
        }),

        RelayPacket::PlayerEnterLobby { player_uuid } => {
            Some(RelayPacket::ServePlayerTransfer {
                data: match state
                    .transfer_online_player_to_mode(player_uuid, GameMode::Hub)
                    .await
                {
                    Some(transfer) => DataTransferResult::Join { transfer },
                    None => DataTransferResult::NotFound,
                },
            })
        }

        RelayPacket::PlayerEnterMode { player_uuid, mode } => {
            Some(RelayPacket::ServePlayerTransfer {
                data: match state.transfer_online_player_to_mode(player_uuid, mode).await {
                    Some(transfer) => DataTransferResult::Join { transfer },
                    None => DataTransferResult::NotFound,
                },
            })
        }

        RelayPacket::TrySendPlayerToSpecificServer {
            player_uuid,
            instance_id,
        } => {
            if let Some((source_server_id, transfer)) = state
                .prepare_forced_transfer(player_uuid, &instance_id)
                .await
            {
                let packet = GameServerPacket::ExecuteTransfer {
                    player_uuid,
                    transfer,
                };
                if let Err(error) = state
                    .nats_client
                    .publish(game_server_subject(&source_server_id), &packet)
                    .await
                {
                    log::error!("failed to send forced transfer: {error:#}");
                }
            }
            None
        }

        RelayPacket::GetLobbyServers => Some(RelayPacket::ServeLobbyServers {
            servers: state.get_servers(GameMode::Hub).await,
        }),

        RelayPacket::GetModeServers { mode } => Some(RelayPacket::ServeModeServers {
            servers: state.get_servers(mode).await,
        }),

        RelayPacket::GetDebug => Some(RelayPacket::ServeDebug {
            debug: state.get_debug().await,
        }),

        RelayPacket::AccomodatePlayer { .. }
        | RelayPacket::ServeAuthResult { .. }
        | RelayPacket::ServeOnlinePlayers { .. }
        | RelayPacket::WhisperCommandResponse { .. }
        | RelayPacket::ServePlayerStatus { .. }
        | RelayPacket::ServeParkourRecord { .. }
        | RelayPacket::ServeParkourRecordUpdate { .. }
        | RelayPacket::ServePlayerTransfer { .. }
        | RelayPacket::ServeLobbyServers { .. }
        | RelayPacket::ServeModeServers { .. }
        | RelayPacket::ServeDebug { .. } => {
            log::warn!("relay received a response-only packet on its request subject");
            None
        }
    }
}
