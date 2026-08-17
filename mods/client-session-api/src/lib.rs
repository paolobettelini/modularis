use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::{collections::HashSet, sync::{Arc, RwLock}};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientSessionSet {
    Accept,
}

/// Composable blockers for the moment in which the client may send JoinRequest.
///
/// Authentication, relay transfer validation, resource negotiation, or a future
/// mod can each own a namespaced blocker without the session implementation
/// knowing any domain-specific type.
#[derive(Resource, Clone, Default)]
pub struct ClientSessionJoinGates(Arc<RwLock<HashSet<String>>>);

impl ClientSessionJoinGates {
    pub fn block(&self, owner: impl Into<String>) {
        self.0
            .write()
            .expect("client session join gates lock poisoned")
            .insert(owner.into());
    }

    pub fn release(&self, owner: &str) {
        self.0
            .write()
            .expect("client session join gates lock poisoned")
            .remove(owner);
    }

    pub fn is_open(&self) -> bool {
        self.0
            .read()
            .expect("client session join gates lock poisoned")
            .is_empty()
    }
}

#[derive(Resource, Debug, Default)]
pub struct ClientSession {
    pub player_id: Option<PlayerId>,
    pub disconnect_reason: Option<String>,
    pub movement_epoch: u64,
    pub next_movement_sequence: u64,
    pub last_acknowledged_movement: Option<u64>,
}

impl ClientSession {
    pub fn reset_movement_stream(&mut self, movement_epoch: u64) {
        self.movement_epoch = movement_epoch;
        self.next_movement_sequence = 0;
        self.last_acknowledged_movement = None;
    }

    pub fn begin_next_movement(&mut self) -> (u64, u64) {
        let sequence = self.next_movement_sequence;
        self.next_movement_sequence = self.next_movement_sequence.wrapping_add(1);
        (self.movement_epoch, sequence)
    }

    pub fn accept_movement_epoch(&mut self, movement_epoch: u64) -> bool {
        if movement_epoch < self.movement_epoch {
            return false;
        }
        if movement_epoch > self.movement_epoch {
            self.reset_movement_stream(movement_epoch);
        }
        true
    }

    pub fn acknowledge_movement(&mut self, movement_epoch: u64, sequence: u64) -> bool {
        if !self.accept_movement_epoch(movement_epoch) {
            return false;
        }
        if self
            .last_acknowledged_movement
            .is_some_and(|acknowledged| sequence <= acknowledged)
        {
            return false;
        }
        self.last_acknowledged_movement = Some(sequence);
        true
    }
}

pub trait ClientSessionApi: Send + Sync + 'static {}
