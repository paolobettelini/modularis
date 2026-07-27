use generated_sound_registry::SoundId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaySoundPacket {
    pub sound: SoundId,
    pub volume: f32,
    pub pitch: f32,
    /// `None` means non-spatial playback. `Some` is an emitter position in
    /// world coordinates.
    pub position: Option<[f32; 3]>,
}
