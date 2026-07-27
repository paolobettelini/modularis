use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, SpatialListener, Volume};
use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_sound_api::{ClientSoundApi, ClientSoundSet, PlayClientSound};
use generated_sound_registry::{SoundId, all_sounds, asset_path};
use std::collections::HashMap;
use tokio::task::JoinHandle;

const LISTENER_EAR_GAP: f32 = 0.2;

#[derive(Resource, Default)]
struct ClientSoundAssets(HashMap<SoundId, Handle<AudioSource>>);

pub struct ClientSoundBevyAudioImpl;

impl ClientSoundBevyAudioImpl {
    pub fn init<S: ClientSoundApi, C: CameraApi>(
        bevy: &mut BevyMod,
        _default_plugins: &mut ClientBevyDefaultPluginsMod,
        _sound: &mut S,
        _camera: &mut C,
    ) -> Self {
        bevy.app
            .init_resource::<ClientSoundAssets>()
            .add_systems(Startup, preload_sounds)
            .add_systems(Update, ensure_spatial_listener)
            .add_systems(Update, play_sounds.in_set(ClientSoundSet::Playback));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn preload_sounds(asset_server: Res<AssetServer>, mut assets: ResMut<ClientSoundAssets>) {
    for &sound in all_sounds() {
        assets.0.insert(sound, asset_server.load(asset_path(sound)));
    }
}

fn ensure_spatial_listener(
    mut commands: Commands,
    listeners: Query<(), With<SpatialListener>>,
    cameras: Query<Entity, (With<PlayerCamera>, Without<SpatialListener>)>,
) {
    if !listeners.is_empty() {
        return;
    }
    let Ok(camera) = cameras.single() else {
        return;
    };
    commands
        .entity(camera)
        .insert(SpatialListener::new(LISTENER_EAR_GAP));
}

fn play_sounds(
    mut commands: Commands,
    assets: Res<ClientSoundAssets>,
    mut sounds: MessageReader<PlayClientSound>,
) {
    for sound in sounds.read() {
        let Some(source) = assets.0.get(&sound.sound) else {
            continue;
        };
        let spatial = sound.position.is_some();
        let settings = PlaybackSettings::DESPAWN
            .with_volume(Volume::Linear(finite_non_negative(sound.volume, 1.0)))
            .with_speed(finite_positive(sound.pitch, 1.0))
            .with_spatial(spatial);
        let mut entity = commands.spawn((AudioPlayer::new(source.clone()), settings));
        if let Some(position) = sound
            .position
            .filter(|position| position.iter().all(|component| component.is_finite()))
        {
            entity.insert(Transform::from_translation(Vec3::from_array(position)));
        }
    }
}

fn finite_non_negative(value: f32, fallback: f32) -> f32 {
    value
        .is_finite()
        .then_some(value.max(0.0))
        .unwrap_or(fallback)
}

fn finite_positive(value: f32, fallback: f32) -> f32 {
    value
        .is_finite()
        .then_some(value.max(0.01))
        .unwrap_or(fallback)
}
