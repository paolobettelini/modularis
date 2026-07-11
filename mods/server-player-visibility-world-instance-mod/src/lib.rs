use bevy_mod::BevyMod;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_player_visibility_api::{ServerPlayerVisibility, ServerPlayerVisibilityApi};
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;

pub struct ServerPlayerVisibilityWorldInstanceMod;

impl ServerPlayerVisibilityWorldInstanceMod {
    pub fn init<W: ServerChunkWorldApi>(bevy: &mut BevyMod, _world_api: &mut W) -> Self {
        let world = bevy.app.world().resource::<ServerChunkWorld>().clone();
        bevy.app
            .insert_resource(ServerPlayerVisibility::new(move |viewer, subject| {
                let position = BlockPos::new(
                    subject.position[0].floor() as i32,
                    subject.position[1].floor() as i32,
                    subject.position[2].floor() as i32,
                )
                .chunk();
                let viewer_scope = world
                    .resident_key_for_player(viewer.id, position)
                    .map(|key| key.scope());
                let subject_scope = world
                    .resident_key_for_player(subject.id, position)
                    .map(|key| key.scope());
                viewer_scope.is_some() && viewer_scope == subject_scope
            }));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerVisibilityApi for ServerPlayerVisibilityWorldInstanceMod {}
