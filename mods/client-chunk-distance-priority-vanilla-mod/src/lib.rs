use bevy_mod::BevyMod;
use client_chunk_work_priority_api::{
    ChunkWorkPriority, ChunkWorkPriorityService, ClientChunkWorkPriorityApi,
};
use tokio::task::JoinHandle;
use voxel_math_api::ChunkPos;

pub struct ClientChunkDistancePriorityVanillaMod;

impl ClientChunkDistancePriorityVanillaMod {
    pub fn init<P: ClientChunkWorkPriorityApi>(bevy: &mut BevyMod, _priority: &mut P) -> Self {
        bevy.app
            .world_mut()
            .resource_mut::<ChunkWorkPriorityService>()
            .priority = distance_priority;
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn distance_priority(position: ChunkPos, focus: Option<ChunkPos>) -> ChunkWorkPriority {
    let Some(focus) = focus else {
        return ChunkWorkPriority::default();
    };
    ChunkWorkPriority { layer: 0, distance: chunk_interest_api::distance_squared(position,focus) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closer_chunks_precede_farther_chunks_on_every_axis() {
        let focus = Some(ChunkPos::new(0, 10, 0));
        assert!(
            distance_priority(ChunkPos::new(8, 10, 8), focus)
                > distance_priority(ChunkPos::new(0, 11, 0), focus)
        );
    }
}
