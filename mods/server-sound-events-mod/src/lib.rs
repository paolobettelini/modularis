use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_sound_api::{PlayServerSound, ServerSoundApi, ServerSoundSet};
use tokio::task::JoinHandle;

pub struct ServerSoundEventsMod;

impl ServerSoundEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.add_message::<PlayServerSound>().configure_sets(
            Update,
            (ServerSoundSet::Publish, ServerSoundSet::Sync).chain(),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerSoundApi for ServerSoundEventsMod {}
