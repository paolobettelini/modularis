use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_chunk_streaming_api::{
    ActiveChunks, ChunkNeeded, ChunkStreamingApi, ChunkStreamingFocus, ChunkStreamingViewConfig,
    ChunkUnload,
};
use client_game_state_api::{GameState, GameStateApi};
use client_settings_api::{SettingsApi, SettingsStore};
use generated_client_settings_registry::SettingKey;
use std::collections::HashSet;
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, ChunkPos};

pub struct AroundPlayerChunkStreaming;

#[derive(Resource, Default)]
struct LastStreamingWindow(Option<(ChunkPos, i32, u64)>);

impl AroundPlayerChunkStreaming {
    pub fn init<S: SettingsApi, C: CameraApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _settings: &mut S,
        _camera: &mut C,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<ActiveChunks>()
            .init_resource::<client_voxel_frame_api::ClientVoxelFrames>()
            .init_resource::<ChunkStreamingFocus>()
            .init_resource::<ChunkStreamingViewConfig>()
            .init_resource::<LastStreamingWindow>()
            .add_message::<ChunkNeeded>()
            .add_message::<ChunkUnload>()
            .add_systems(
                Update,
                update_active_chunks.after(client_voxel_frame_api::ClientVoxelFrameSet::Receive).run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), unload_all_chunks);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ChunkStreamingApi for AroundPlayerChunkStreaming {}

fn update_active_chunks(
    camera: Query<&Transform, With<PlayerCamera>>,
    settings: Res<SettingsStore>,
    view: Res<ChunkStreamingViewConfig>,
    frames: Res<client_voxel_frame_api::ClientVoxelFrames>,
    mut focus: ResMut<ChunkStreamingFocus>,
    mut last_window: ResMut<LastStreamingWindow>,
    mut active: ResMut<ActiveChunks>,
    mut needed: MessageWriter<ChunkNeeded>,
    mut unload: MessageWriter<ChunkUnload>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    let center = BlockPos::new(
        camera.translation.x.floor() as i32,
        camera.translation.y.floor() as i32,
        camera.translation.z.floor() as i32,
    )
    .chunk();
    // Do not mark the shared focus resource as changed while the camera remains
    // in the same chunk. Consumers use change detection to reprioritize or
    // rebuild chunk-derived data, and a false positive here can keep their work
    // queues permanently pinned to the closest chunks.
    if focus.center != Some(center) {
        focus.center = Some(center);
    }
    let radius = settings
        .get_i32(SettingKey::GraphicsRenderDistance)
        .unwrap_or(8)
        .clamp(1, view.max_radius.max(1));
    let window = (center, radius, frames.registry.revision());
    if last_window.0 == Some(window) && !view.is_changed() && !active.positions.is_empty() {
        return;
    }
    last_window.0 = Some(window);

    let mut desired = view.volume.positions(center,radius).into_iter().map(voxel_frame_api::VoxelChunkAddress::root).collect::<HashSet<_>>();
    if let Some(scope) = &frames.scope {
        desired.extend(frames.registry.interested_chunks(scope,camera.translation.as_dvec3()+frames.render_origin,radius as f64*16.0));
    }

    for position in desired.difference(&active.positions).copied() {
        needed.write(ChunkNeeded { position });
    }
    for position in active.positions.difference(&desired).copied() {
        unload.write(ChunkUnload { position });
    }
    active.positions = desired;
}

fn unload_all_chunks(mut active: ResMut<ActiveChunks>, mut unload: MessageWriter<ChunkUnload>) {
    for position in active.positions.drain() {
        unload.write(ChunkUnload { position });
    }
}
