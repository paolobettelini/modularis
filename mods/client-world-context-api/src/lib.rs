use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Default, PartialEq)]
pub struct ClientWorldContext {
    pub id: Option<String>,
    pub revision: u64,
    /// Latest authoritative spawn/transition position for this world revision.
    ///
    /// Keeping it in the resource, rather than only in the transient
    /// `ClientWorldChanged` message, lets consumers wait until the local player
    /// entity exists.
    pub position: Option<[f32; 3]>,
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct ClientWorldChanged {
    pub previous: Option<String>,
    pub current: String,
    pub revision: u64,
    pub position: [f32; 3],
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientWorldContextSet {
    Receive,
    ResetWorld,
    ApplyPlayer,
}

pub trait ClientWorldContextApi: Send + Sync + 'static {}
