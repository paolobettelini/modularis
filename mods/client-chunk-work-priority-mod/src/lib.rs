use bevy_mod::BevyMod;
use client_chunk_work_priority_api::{ChunkWorkPriorityService, ClientChunkWorkPriorityApi};
use tokio::task::JoinHandle;

pub struct ClientChunkWorkPriorityMod;

impl ClientChunkWorkPriorityMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<ChunkWorkPriorityService>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientChunkWorkPriorityApi for ClientChunkWorkPriorityMod {}
