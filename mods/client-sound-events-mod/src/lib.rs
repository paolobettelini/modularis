use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_sound_api::{ClientSoundApi, ClientSoundSet, PlayClientSound};
use tokio::task::JoinHandle;

pub struct ClientSoundEventsMod;

impl ClientSoundEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.add_message::<PlayClientSound>().configure_sets(
            Update,
            (ClientSoundSet::Receive, ClientSoundSet::Playback).chain(),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientSoundApi for ClientSoundEventsMod {}
