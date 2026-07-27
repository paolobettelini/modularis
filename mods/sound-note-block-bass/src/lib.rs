use sound_api::{Sound, SoundInfo};
use tokio::task::JoinHandle;

pub struct NoteBlockBassSound;

impl Sound for NoteBlockBassSound {
    const INFO: SoundInfo = SoundInfo {
        id: "demo:note-block-bass",
        asset_path: "sound-note-block-bass/note_block_bass.mp3",
    };
}

pub const SOUND_INFO: SoundInfo = NoteBlockBassSound::INFO;

pub struct SoundNoteBlockBassMod;

impl SoundNoteBlockBassMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
