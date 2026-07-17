use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LocalPlayerSneak {
    pub active: bool,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalPlayerSneakChanged {
    pub active: bool,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerSneakSet {
    Input,
}

pub trait PlayerSneakApi: Send + Sync + 'static {}
