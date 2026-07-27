use audience_api::Audience;
use bevy::prelude::*;
use generated_sound_registry::SoundId;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerSoundSet {
    Publish,
    Sync,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoundPlayback {
    pub sound: SoundId,
    pub volume: f32,
    pub pitch: f32,
    pub position: Option<[f32; 3]>,
}

impl SoundPlayback {
    pub const fn new(sound: SoundId) -> Self {
        Self {
            sound,
            volume: 1.0,
            pitch: 1.0,
            position: None,
        }
    }

    pub const fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    pub const fn with_pitch(mut self, pitch: f32) -> Self {
        self.pitch = pitch;
        self
    }

    pub const fn at(mut self, position: [f32; 3]) -> Self {
        self.position = Some(position);
        self
    }
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct PlayServerSound {
    pub audience: Audience,
    pub playback: SoundPlayback,
}

pub trait ServerSoundApi: Send + Sync + 'static {}
