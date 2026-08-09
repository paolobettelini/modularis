use bevy::prelude::*;
use player_network_message_types::{
    MovementEpoch, MovementSequence, NetworkPlayer, PlayerId, PlayerMove,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerMovementSet {
    Receive,
    Validate,
    Apply,
    Sync,
}

/// Shared ordering boundary for every authoritative relocation mechanism.
///
/// Teleports, respawns, dimension transitions and scoped-world changes must
/// apply after movement for the current update and before movement sync.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerRelocationSet {
    Apply,
    Sync,
}

/// Public join/leave phases used by custom server orchestration.
///
/// In particular, `Initialize` runs after the player has a stable `PlayerId`
/// but before `JoinAccepted` and the initial visibility snapshot are sent.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerSessionSet {
    Receive,
    Validate,
    Register,
    Initialize,
    Sync,
    Cleanup,
}

#[derive(Debug, Clone)]
pub struct PendingServerPlayerMove {
    pub source: SocketAddr,
    pub player_id: PlayerId,
    pub movement_epoch: MovementEpoch,
    pub sequence: MovementSequence,
    pub current_position: Vec3,
    pub requested_position: Vec3,
    pub accepted_position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub rejected: bool,
}

#[derive(Resource, Default)]
pub struct PendingServerPlayerMoves {
    pub moves: Vec<PendingServerPlayerMove>,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ServerPlayerMovementApplied {
    pub player_id: PlayerId,
    pub movement_epoch: MovementEpoch,
    pub sequence: MovementSequence,
    pub previous_position: Vec3,
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub corrected: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ServerPlayerMovementStream {
    pub epoch: MovementEpoch,
    pub last_received_sequence: Option<MovementSequence>,
    pub last_applied_sequence: Option<MovementSequence>,
}

#[derive(Resource, Clone)]
pub struct ServerPlayerMovementValidator {
    validate: Arc<dyn Fn(Vec3, Vec3) -> Vec3 + Send + Sync>,
}

impl ServerPlayerMovementValidator {
    pub fn new<V>(validate: V) -> Self
    where
        V: Fn(Vec3, Vec3) -> Vec3 + Send + Sync + 'static,
    {
        Self {
            validate: Arc::new(validate),
        }
    }

    pub fn validate(&self, current: Vec3, requested: Vec3) -> Vec3 {
        (self.validate)(current, requested)
    }
}

#[derive(Resource, Default)]
pub struct ServerPlayerRegistry {
    next_id: PlayerId,
    by_address: HashMap<SocketAddr, PlayerId>,
    players: HashMap<PlayerId, NetworkPlayer>,
    movement_streams: HashMap<PlayerId, ServerPlayerMovementStream>,
    last_seen_at: HashMap<SocketAddr, f64>,
}

impl ServerPlayerRegistry {
    pub fn join(&mut self, address: SocketAddr, name: String, now: f64) -> NetworkPlayer {
        if let Some(player) = self.player_for_address(address) {
            return player.clone();
        }
        self.next_id += 1;
        let player = NetworkPlayer {
            id: self.next_id,
            name,
            position: [0.0, 2.0, 0.0],
            yaw: 0.0,
            pitch: 0.0,
        };
        self.by_address.insert(address, player.id);
        self.last_seen_at.insert(address, now);
        self.players.insert(player.id, player.clone());
        self.movement_streams
            .insert(player.id, ServerPlayerMovementStream::default());
        player
    }

    pub fn leave(&mut self, address: SocketAddr) -> Option<NetworkPlayer> {
        let id = self.by_address.remove(&address)?;
        self.last_seen_at.remove(&address);
        self.movement_streams.remove(&id);
        self.players.remove(&id)
    }

    pub fn move_player(
        &mut self,
        address: SocketAddr,
        movement: &PlayerMove,
        now: f64,
    ) -> Option<NetworkPlayer> {
        self.move_player_projected(address, movement, now, None)
    }

    pub fn move_player_projected(
        &mut self,
        address: SocketAddr,
        movement: &PlayerMove,
        now: f64,
        gravity: Option<Vec3>,
    ) -> Option<NetworkPlayer> {
        self.move_player_projected_validated(address, movement, now, gravity, None)
    }

    pub fn move_player_projected_validated(
        &mut self,
        address: SocketAddr,
        movement: &PlayerMove,
        now: f64,
        gravity: Option<Vec3>,
        validator: Option<&ServerPlayerMovementValidator>,
    ) -> Option<NetworkPlayer> {
        let id = *self.by_address.get(&address)?;
        if !self.accept_movement_packet(id, movement.movement_epoch, movement.sequence) {
            return None;
        }
        let player = self.players.get_mut(&id)?;
        let current = Vec3::from_array(player.position);
        let requested = Vec3::from_array(movement.position);
        let delta = requested - current;
        let projected_position = gravity
            .map(|gravity| gravity.normalize_or_zero())
            .filter(|direction| direction.length_squared() > 0.0)
            .map(|direction| current + delta - direction * delta.dot(direction))
            .unwrap_or(requested);
        let projected_position = validator
            .map(|validator| validator.validate(current, projected_position))
            .unwrap_or(projected_position);
        player.position = projected_position.to_array();
        player.yaw = movement.yaw;
        player.pitch = movement.pitch;
        if let Some(stream) = self.movement_streams.get_mut(&id) {
            stream.last_applied_sequence = Some(movement.sequence);
        }
        self.last_seen_at.insert(address, now);
        Some(player.clone())
    }

    pub fn player_for_address(&self, address: SocketAddr) -> Option<&NetworkPlayer> {
        let id = self.by_address.get(&address)?;
        self.players.get(id)
    }

    pub fn touch_address(&mut self, address: SocketAddr, now: f64) -> bool {
        if !self.by_address.contains_key(&address) {
            return false;
        }
        self.last_seen_at.insert(address, now);
        true
    }

    pub fn players(&self) -> Vec<NetworkPlayer> {
        self.players.values().cloned().collect()
    }

    pub fn player(&self, player_id: PlayerId) -> Option<&NetworkPlayer> {
        self.players.get(&player_id)
    }

    pub fn set_player_position(
        &mut self,
        player_id: PlayerId,
        position: [f32; 3],
    ) -> Option<NetworkPlayer> {
        self.relocate_player(player_id, position)
            .map(|(player, _)| player)
    }

    /// Relocates a player and invalidates every movement packet produced from
    /// the previous authoritative position.
    pub fn relocate_player(
        &mut self,
        player_id: PlayerId,
        position: [f32; 3],
    ) -> Option<(NetworkPlayer, MovementEpoch)> {
        let player = self.players.get_mut(&player_id)?;
        player.position = position;
        let player = player.clone();
        let stream = self.movement_streams.entry(player_id).or_default();
        stream.epoch = stream.epoch.wrapping_add(1);
        stream.last_received_sequence = None;
        stream.last_applied_sequence = None;
        Some((player, stream.epoch))
    }

    pub fn movement_epoch(&self, player_id: PlayerId) -> Option<MovementEpoch> {
        self.movement_streams.get(&player_id).map(|stream| stream.epoch)
    }

    /// Accepts a packet header once. Replayed, reordered and pre-relocation
    /// packets are rejected before they can enter gameplay validation.
    pub fn accept_movement_packet(
        &mut self,
        player_id: PlayerId,
        movement_epoch: MovementEpoch,
        sequence: MovementSequence,
    ) -> bool {
        let Some(stream) = self.movement_streams.get_mut(&player_id) else {
            return false;
        };
        if stream.epoch != movement_epoch
            || stream
                .last_received_sequence
                .is_some_and(|received| sequence <= received)
        {
            return false;
        }
        stream.last_received_sequence = Some(sequence);
        true
    }

    pub fn set_player_rotation(
        &mut self,
        player_id: PlayerId,
        yaw: f32,
        pitch: f32,
    ) -> Option<NetworkPlayer> {
        let player = self.players.get_mut(&player_id)?;
        player.yaw = yaw;
        player.pitch = pitch;
        Some(player.clone())
    }

    pub fn apply_player_move(
        &mut self,
        address: SocketAddr,
        player_id: PlayerId,
        movement_epoch: MovementEpoch,
        sequence: MovementSequence,
        position: Vec3,
        yaw: f32,
        pitch: f32,
        now: f64,
    ) -> Option<NetworkPlayer> {
        if self.by_address.get(&address).copied() != Some(player_id) {
            return None;
        }
        let stream = self.movement_streams.get_mut(&player_id)?;
        if stream.epoch != movement_epoch
            || stream
                .last_applied_sequence
                .is_some_and(|applied| sequence <= applied)
            || stream
                .last_received_sequence
                .is_none_or(|received| sequence > received)
        {
            return None;
        }
        stream.last_applied_sequence = Some(sequence);
        let player = self.players.get_mut(&player_id)?;
        player.position = position.to_array();
        player.yaw = yaw;
        player.pitch = pitch;
        self.last_seen_at.insert(address, now);
        Some(player.clone())
    }

    pub fn address_for_player(&self, player_id: PlayerId) -> Option<SocketAddr> {
        self.by_address
            .iter()
            .find_map(|(address, id)| (*id == player_id).then_some(*address))
    }

    pub fn expire_inactive(&mut self, older_than: f64) -> Vec<(SocketAddr, NetworkPlayer)> {
        let expired = self
            .last_seen_at
            .iter()
            .filter_map(|(address, last_seen)| (*last_seen < older_than).then_some(*address))
            .collect::<Vec<_>>();
        expired
            .into_iter()
            .filter_map(|address| self.leave(address).map(|player| (address, player)))
            .collect()
    }
}

pub trait ServerPlayerRegistryApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expires_only_inactive_players() {
        let mut registry = ServerPlayerRegistry::default();
        let old = "127.0.0.1:10001".parse().unwrap();
        let active = "127.0.0.1:10002".parse().unwrap();
        registry.join(old, "Old".to_string(), 1.0);
        registry.join(active, "Active".to_string(), 9.0);

        let expired = registry.expire_inactive(5.0);

        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0].0, old);
        assert!(registry.player_for_address(old).is_none());
        assert!(registry.player_for_address(active).is_some());
    }
}
