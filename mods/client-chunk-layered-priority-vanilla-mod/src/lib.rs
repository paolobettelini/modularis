use bevy_mod::BevyMod;
use client_chunk_work_priority_api::{
    ChunkWorkPriority, ChunkWorkPriorityService, ClientChunkWorkPriorityApi,
};
use tokio::task::JoinHandle;
use voxel_math_api::ChunkPos;

pub struct ClientChunkLayeredPriorityVanillaMod;

impl ClientChunkLayeredPriorityVanillaMod {
    pub fn init<P: ClientChunkWorkPriorityApi>(bevy: &mut BevyMod, _priority: &mut P) -> Self {
        bevy.app
            .world_mut()
            .resource_mut::<ChunkWorkPriorityService>()
            .priority = layered_priority;
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn layered_priority(position: ChunkPos, focus: Option<ChunkPos>) -> ChunkWorkPriority {
    let Some(focus) = focus else {
        return ChunkWorkPriority::default();
    };
    let x = i64::from(position.x) - i64::from(focus.x);
    let y = i64::from(position.y) - i64::from(focus.y);
    let z = i64::from(position.z) - i64::from(focus.z);
    ChunkWorkPriority {
        // Complete the current XZ plane before progressively loading layers
        // above and below it.
        layer: y.unsigned_abs().min(u64::from(u32::MAX)) as u32,
        distance: (x * x + z * z) as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_xz_plane_precedes_closer_vertical_layers() {
        let focus = Some(ChunkPos::new(0, 10, 0));
        assert!(
            layered_priority(ChunkPos::new(8, 10, 8), focus)
                < layered_priority(ChunkPos::new(0, 11, 0), focus)
        );
    }
}
