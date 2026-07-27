use bevy::prelude::*;
use generated_sound_registry::SoundId;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientSoundSet {
    Receive,
    Playback,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct PlayClientSound {
    pub sound: SoundId,
    pub volume: f32,
    pub pitch: f32,
    pub position: Option<[f32; 3]>,
}

pub trait ClientSoundApi: Send + Sync + 'static {}
