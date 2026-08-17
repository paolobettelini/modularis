use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;
use thecrown_common::nats::NatsClient;
use thecrown_common::token::random_token;
use thecrown_database::{Ban, Database, Player};
use thecrown_protocol::{
    GameInstanceSpec, GameMode, GameServerPacket, PlayerIdentity, PlayerStatus, ServerEntry,
    TransferPacketData, game_server_subject,
};
use tokio::sync::RwLock;
use tokio::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct State {
    pub nats_client: Arc<NatsClient>,
    pub db_client: Arc<Database>,
    inner: Arc<RwLock<NetworkState>>,
    auth_ticket_ttl: Duration,
    initial_instances_per_mode: usize,
}

#[derive(Debug, Default)]
struct NetworkState {
    game_servers: HashMap<String, GameServer>,
    instances: HashMap<String, GameInstance>,
    players: HashMap<Uuid, PlayerSession>,
    auth_tickets: HashMap<String, AuthTicket>,
}

#[derive(Debug, Clone, Serialize)]
struct GameServer {
    server_id: String,
    address: String,
    port: u16,
}

#[derive(Debug, Clone, Serialize)]
struct GameInstance {
    instance_id: String,
    mode: GameMode,
    server_id: String,
}

#[derive(Debug, Clone)]
struct PlayerSession {
    player: PlayerIdentity,
    server_id: String,
    instance_id: String,
    mode: GameMode,
}

#[derive(Debug, Clone)]
struct AuthTicket {
    player: PlayerIdentity,
    target_server_id: String,
    target_instance_id: String,
    expires_at: Instant,
}

#[derive(Debug, Serialize)]
struct DebugSnapshot {
    game_servers: Vec<GameServer>,
    instances: Vec<DebugInstance>,
    players: Vec<DebugPlayer>,
    pending_auth_tickets: usize,
}

#[derive(Debug, Serialize)]
struct DebugInstance {
    instance_id: String,
    mode: GameMode,
    server_id: String,
    online: u32,
}

#[derive(Debug, Serialize)]
struct DebugPlayer {
    uuid: Uuid,
    username: String,
    server_id: String,
    instance_id: String,
    mode: GameMode,
}

impl State {
    pub fn new(
        nats_client: Arc<NatsClient>,
        db_client: Arc<Database>,
        auth_ticket_ttl: Duration,
        initial_instances_per_mode: usize,
    ) -> Self {
        Self {
            nats_client,
            db_client,
            inner: Arc::new(RwLock::new(NetworkState::default())),
            auth_ticket_ttl,
            initial_instances_per_mode,
        }
    }

    pub fn start_cleanup_task(&self) {
        let state = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                state.cleanup_expired_auth_tickets().await;
            }
        });
    }

    async fn cleanup_expired_auth_tickets(&self) {
        let now = Instant::now();
        let mut inner = self.inner.write().await;
        inner.auth_tickets.retain(|_, ticket| ticket.expires_at > now);
    }

    pub async fn db_upsert_player(&self, player: PlayerIdentity) -> anyhow::Result<Player> {
        let db = self.db_client.clone();
        Ok(tokio::task::spawn_blocking(move || db.upsert_player(player.uuid, &player.username))
            .await??)
    }

    pub async fn db_get_active_ban(&self, player_uuid: Uuid) -> anyhow::Result<Option<Ban>> {
        let db = self.db_client.clone();
        Ok(tokio::task::spawn_blocking(move || db.get_active_ban(player_uuid)).await??)
    }

    pub async fn db_get_player_by_username(
        &self,
        username: String,
    ) -> anyhow::Result<Option<Player>> {
        let db = self.db_client.clone();
        Ok(tokio::task::spawn_blocking(move || db.get_player_by_username(&username)).await??)
    }

    pub async fn db_get_parkour_record(
        &self,
        player_uuid: Uuid,
    ) -> anyhow::Result<Option<i32>> {
        let db = self.db_client.clone();
        Ok(tokio::task::spawn_blocking(move || {
            db.get_player(player_uuid)
                .map(|player| player.map(|player| player.parkour_record))
        })
        .await??)
    }

    pub async fn db_submit_parkour_record(
        &self,
        player_uuid: Uuid,
        score: i32,
    ) -> anyhow::Result<Option<(i32, i32, bool)>> {
        let db = self.db_client.clone();
        Ok(tokio::task::spawn_blocking(move || {
            db.update_parkour_record_if_higher(player_uuid, score)
        })
        .await??)
    }

    pub async fn register_game_server(
        &self,
        server_id: String,
        address: String,
        port: u16,
    ) -> anyhow::Result<()> {
        validate_server_id(&server_id)?;

        {
            let mut inner = self.inner.write().await;
            inner.remove_server(&server_id);
            inner.game_servers.insert(
                server_id.clone(),
                GameServer {
                    server_id: server_id.clone(),
                    address,
                    port,
                },
            );
        }

        for mode in GameMode::ALL {
            for _ in 0..self.initial_instances_per_mode {
                if let Err(error) = self.add_instance(&server_id, mode).await {
                    // Do not leave a half-registered server routable if bootstrap delivery fails.
                    self.unregister_game_server(&server_id).await;
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    pub async fn unregister_game_server(&self, server_id: &str) {
        let mut inner = self.inner.write().await;
        inner.remove_server(server_id);
    }

    /// Lifecycle primitive used by the bootstrap policy now and by a future load balancer later.
    pub async fn add_instance(
        &self,
        server_id: &str,
        mode: GameMode,
    ) -> anyhow::Result<GameInstanceSpec> {
        let spec = {
            let mut inner = self.inner.write().await;
            if !inner.game_servers.contains_key(server_id) {
                anyhow::bail!("game server {server_id} is not registered");
            }

            let instance_id = inner.next_instance_id(mode);
            let spec = GameInstanceSpec {
                instance_id: instance_id.clone(),
                mode,
            };
            inner.instances.insert(
                instance_id.clone(),
                GameInstance {
                    instance_id,
                    mode,
                    server_id: server_id.to_owned(),
                },
            );
            spec
        };

        if let Err(error) = self
            .nats_client
            .publish(
                game_server_subject(server_id),
                &GameServerPacket::StartInstance {
                    instance: spec.clone(),
                },
            )
            .await
        {
            // Publishing is part of the state transition: roll back the optimistic entry on error.
            let mut inner = self.inner.write().await;
            inner.instances.remove(&spec.instance_id);
            return Err(error);
        }
        Ok(spec)
    }

    /// Prepared for a future scaler. Only empty instances can be stopped: removing an
    /// occupied instance would make Relay forget sessions that are still alive on the
    /// game server. Pending, not-yet-redeemed tickets are invalidated atomically.
    #[allow(dead_code)]
    pub async fn remove_instance(&self, instance_id: &str) -> anyhow::Result<bool> {
        let server_id = {
            let mut inner = self.inner.write().await;
            let Some(instance) = inner.instances.get(instance_id).cloned() else {
                return Ok(false);
            };
            if inner
                .players
                .values()
                .any(|player| player.instance_id == instance_id)
            {
                anyhow::bail!("cannot stop occupied instance {instance_id}");
            }
            inner.instances.remove(instance_id);
            inner
                .auth_tickets
                .retain(|_, ticket| ticket.target_instance_id != instance_id);
            instance.server_id
        };

        self.nats_client
            .publish(
                game_server_subject(&server_id),
                &GameServerPacket::StopInstance {
                    instance_id: instance_id.to_owned(),
                },
            )
            .await?;
        Ok(true)
    }

    pub async fn get_online_players(&self) -> u64 {
        self.inner.read().await.players.len() as u64
    }

    pub async fn get_player_status(&self, player_uuid: Uuid) -> PlayerStatus {
        let inner = self.inner.read().await;
        match inner.players.get(&player_uuid) {
            Some(player) => PlayerStatus::Online {
                server_id: player.server_id.clone(),
                instance_id: player.instance_id.clone(),
                mode: player.mode,
            },
            None => PlayerStatus::Offline,
        }
    }

    pub async fn player_quit(&self, player_uuid: Uuid, server_id: &str, instance_id: &str) -> bool {
        let mut inner = self.inner.write().await;
        let matches_current_location = inner.players.get(&player_uuid).is_some_and(|player| {
            player.server_id == server_id && player.instance_id == instance_id
        });
        if matches_current_location {
            inner.players.remove(&player_uuid);
            true
        } else {
            false
        }
    }

    pub async fn transfer_new_player_to_hub(
        &self,
        player: PlayerIdentity,
    ) -> Option<TransferPacketData> {
        self.transfer_identity_to_mode(player, GameMode::Hub).await
    }

    pub async fn transfer_online_player_to_mode(
        &self,
        player_uuid: Uuid,
        mode: GameMode,
    ) -> Option<TransferPacketData> {
        let player = self.online_identity(player_uuid).await?;
        self.transfer_identity_to_mode(player, mode).await
    }

    pub async fn transfer_online_player_to_instance(
        &self,
        player_uuid: Uuid,
        instance_id: &str,
    ) -> Option<TransferPacketData> {
        let player = self.online_identity(player_uuid).await?;
        self.transfer_identity_to_instance(player, instance_id).await
    }

    async fn online_identity(&self, player_uuid: Uuid) -> Option<PlayerIdentity> {
        self.inner
            .read()
            .await
            .players
            .get(&player_uuid)
            .map(|player| player.player.clone())
    }

    async fn transfer_identity_to_mode(
        &self,
        player: PlayerIdentity,
        mode: GameMode,
    ) -> Option<TransferPacketData> {
        let mut inner = self.inner.write().await;
        let target = inner.least_loaded_instance(mode)?;
        inner.create_transfer(player, &target, self.auth_ticket_ttl)
    }

    async fn transfer_identity_to_instance(
        &self,
        player: PlayerIdentity,
        instance_id: &str,
    ) -> Option<TransferPacketData> {
        let mut inner = self.inner.write().await;
        let target = inner.instances.get(instance_id)?.clone();
        inner.create_transfer(player, &target, self.auth_ticket_ttl)
    }

    pub async fn try_auth_user(
        &self,
        token: &str,
        player_uuid: Uuid,
        server_id: &str,
        instance_id: &str,
    ) -> bool {
        let now = Instant::now();
        let mut inner = self.inner.write().await;

        // Consume before validation: a transfer ticket is strictly one-use, even on a failed attempt.
        let Some(ticket) = inner.auth_tickets.remove(token) else {
            return false;
        };
        if ticket.expires_at <= now {
            return false;
        }
        if ticket.player.uuid != player_uuid
            || ticket.target_server_id != server_id
            || ticket.target_instance_id != instance_id
        {
            return false;
        }

        let Some(instance) = inner.instances.get(instance_id).cloned() else {
            return false;
        };
        if instance.server_id != server_id {
            return false;
        }

        inner.players.insert(
            player_uuid,
            PlayerSession {
                player: ticket.player,
                server_id: server_id.to_owned(),
                instance_id: instance_id.to_owned(),
                mode: instance.mode,
            },
        );
        true
    }

    pub async fn whisper_target(
        &self,
        sender_uuid: Uuid,
        target_uuid: Uuid,
    ) -> Option<(String, PlayerIdentity)> {
        let inner = self.inner.read().await;
        let sender = inner.players.get(&sender_uuid)?.player.clone();
        let target_server = inner.players.get(&target_uuid)?.server_id.clone();
        Some((target_server, sender))
    }

    pub async fn prepare_forced_transfer(
        &self,
        player_uuid: Uuid,
        instance_id: &str,
    ) -> Option<(String, TransferPacketData)> {
        let source_server = {
            let inner = self.inner.read().await;
            inner.players.get(&player_uuid)?.server_id.clone()
        };
        let transfer = self
            .transfer_online_player_to_instance(player_uuid, instance_id)
            .await?;
        Some((source_server, transfer))
    }

    pub async fn get_servers(&self, mode: GameMode) -> Vec<ServerEntry> {
        let inner = self.inner.read().await;
        let mut servers: Vec<_> = inner
            .instances
            .values()
            .filter(|instance| instance.mode == mode)
            .map(|instance| ServerEntry {
                server_id: instance.server_id.clone(),
                instance_id: instance.instance_id.clone(),
                mode: instance.mode,
                online: inner.online_in_instance(&instance.instance_id),
            })
            .collect();
        servers.sort_by(|a, b| a.instance_id.cmp(&b.instance_id));
        servers
    }

    pub async fn get_debug(&self) -> String {
        let inner = self.inner.read().await;
        let mut game_servers: Vec<_> = inner.game_servers.values().cloned().collect();
        game_servers.sort_by(|a, b| a.server_id.cmp(&b.server_id));

        let mut instances: Vec<_> = inner
            .instances
            .values()
            .map(|instance| DebugInstance {
                instance_id: instance.instance_id.clone(),
                mode: instance.mode,
                server_id: instance.server_id.clone(),
                online: inner.online_in_instance(&instance.instance_id),
            })
            .collect();
        instances.sort_by(|a, b| a.instance_id.cmp(&b.instance_id));

        let mut players: Vec<_> = inner
            .players
            .values()
            .map(|player| DebugPlayer {
                uuid: player.player.uuid,
                username: player.player.username.clone(),
                server_id: player.server_id.clone(),
                instance_id: player.instance_id.clone(),
                mode: player.mode,
            })
            .collect();
        players.sort_by_key(|player| player.uuid);

        serde_json::to_string_pretty(&DebugSnapshot {
            game_servers,
            instances,
            players,
            pending_auth_tickets: inner.auth_tickets.len(),
        })
        .unwrap_or_else(|error| format!("{{\"error\":\"{error}\"}}"))
    }
}

impl NetworkState {
    fn next_instance_id(&self, mode: GameMode) -> String {
        // IDs only need to be unique among live instances. Reusing a free number keeps
        // the default single-server layout stable across server re-registration.
        for number in 1_u64.. {
            let candidate = format!("{}{}", mode.as_str(), number);
            if !self.instances.contains_key(&candidate) {
                return candidate;
            }
        }
        unreachable!("u64 instance id space exhausted")
    }

    fn remove_server(&mut self, server_id: &str) {
        self.game_servers.remove(server_id);
        let removed_instance_ids: Vec<String> = self
            .instances
            .values()
            .filter(|instance| instance.server_id == server_id)
            .map(|instance| instance.instance_id.clone())
            .collect();
        for instance_id in &removed_instance_ids {
            self.instances.remove(instance_id);
        }
        self.players.retain(|_, player| player.server_id != server_id);
        self.auth_tickets
            .retain(|_, ticket| ticket.target_server_id != server_id);
    }

    fn least_loaded_instance(&self, mode: GameMode) -> Option<GameInstance> {
        self.instances
            .values()
            .filter(|instance| {
                instance.mode == mode && self.game_servers.contains_key(&instance.server_id)
            })
            .min_by_key(|instance| {
                (
                    self.routing_load_in_instance(&instance.instance_id),
                    instance.instance_id.clone(),
                )
            })
            .cloned()
    }

    fn online_in_instance(&self, instance_id: &str) -> u32 {
        self.players
            .values()
            .filter(|player| player.instance_id == instance_id)
            .count() as u32
    }

    fn routing_load_in_instance(&self, instance_id: &str) -> usize {
        let online = self
            .players
            .values()
            .filter(|player| player.instance_id == instance_id)
            .count();
        let now = Instant::now();
        let pending = self
            .auth_tickets
            .values()
            .filter(|ticket| {
                ticket.target_instance_id == instance_id && ticket.expires_at > now
            })
            .count();
        online.saturating_add(pending)
    }

    fn create_transfer(
        &mut self,
        player: PlayerIdentity,
        instance: &GameInstance,
        ttl: Duration,
    ) -> Option<TransferPacketData> {
        let server = self.game_servers.get(&instance.server_id)?.clone();
        // Only the newest transfer for a UUID remains valid.
        self.auth_tickets
            .retain(|_, ticket| ticket.player.uuid != player.uuid);
        let token = random_token();
        self.auth_tickets.insert(
            token.clone(),
            AuthTicket {
                player,
                target_server_id: server.server_id.clone(),
                target_instance_id: instance.instance_id.clone(),
                expires_at: Instant::now() + ttl,
            },
        );
        Some(TransferPacketData {
            cookie: token,
            address: server.address,
            port: server.port,
            server_id: server.server_id,
            instance_id: instance.instance_id.clone(),
            mode: instance.mode,
        })
    }
}

fn validate_server_id(server_id: &str) -> anyhow::Result<()> {
    let valid = !server_id.is_empty()
        && server_id.len() <= 64
        && server_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'));
    if valid {
        Ok(())
    } else {
        anyhow::bail!(
            "invalid server_id `{server_id}`: use 1-64 ASCII letters, digits, `_` or `-`"
        )
    }
}
