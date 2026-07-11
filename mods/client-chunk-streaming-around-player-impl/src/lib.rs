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
struct LastStreamingWindow(Option<(ChunkPos, i32, i32)>);

impl AroundPlayerChunkStreaming {
    pub fn init<S: SettingsApi, C: CameraApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _settings: &mut S,
        _camera: &mut C,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<ActiveChunks>()
            .init_resource::<ChunkStreamingFocus>()
            .init_resource::<ChunkStreamingViewConfig>()
            .init_resource::<LastStreamingWindow>()
            .add_message::<ChunkNeeded>()
            .add_message::<ChunkUnload>()
            .add_systems(
                Update,
                update_active_chunks.run_if(in_state(GameState::InGame)),
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
    focus.center = Some(center);
    let radius = settings
        .get_i32(SettingKey::GraphicsRenderDistance)
        .unwrap_or(8)
        .clamp(1, view.max_horizontal_radius.max(1));
    let vertical_radius = view.vertical_radius.max(0);
    let window = (center, radius, vertical_radius);
    if last_window.0 == Some(window) && !active.positions.is_empty() {
        return;
    }
    last_window.0 = Some(window);

    let desired = desired_chunks(center, radius, vertical_radius);

    for position in desired.difference(&active.positions).copied() {
        needed.write(ChunkNeeded { position });
    }
    for position in active.positions.difference(&desired).copied() {
        unload.write(ChunkUnload { position });
    }
    active.positions = desired;
}

fn desired_chunks(
    center: ChunkPos,
    horizontal_radius: i32,
    vertical_radius: i32,
) -> HashSet<ChunkPos> {
    let horizontal_radius = horizontal_radius.max(0);
    let vertical_radius = vertical_radius.max(0);
    let mut desired = HashSet::new();
    for y in -vertical_radius..=vertical_radius {
        for z in -horizontal_radius..=horizontal_radius {
            for x in -horizontal_radius..=horizontal_radius {
                desired.insert(ChunkPos::new(center.x + x, center.y + y, center.z + z));
            }
        }
    }
    desired
}

fn unload_all_chunks(mut active: ResMut<ActiveChunks>, mut unload: MessageWriter<ChunkUnload>) {
    for position in active.positions.drain() {
        unload.write(ChunkUnload { position });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streaming_window_follows_arbitrary_vertical_chunk_coordinates() {
        let center = ChunkPos::new(7, 120, -4);
        let chunks = desired_chunks(center, 1, 2);
        assert_eq!(chunks.len(), 3 * 3 * 5);
        assert!(chunks.contains(&ChunkPos::new(7, 118, -4)));
        assert!(chunks.contains(&ChunkPos::new(7, 122, -4)));
        assert!(!chunks.iter().any(|position| position.y == 0));
    }
}
