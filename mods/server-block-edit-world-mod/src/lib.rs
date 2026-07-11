use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{
    PendingBlockBreak, PendingBlockBreaks, ServerBlockBreakRequested, ServerBlockBroken,
    ServerBlockEditSet,
};
use block_edit_events_mod::BlockEditEventsMod;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use tokio::task::JoinHandle;

pub struct ServerBlockEditWorldMod;

impl ServerBlockEditWorldMod {
    pub fn init<W: ServerChunkWorldApi>(
        bevy: &mut BevyMod,
        _events: &mut BlockEditEventsMod,
        _world: &mut W,
    ) -> Self {
        bevy.app
            .add_systems(
                Update,
                collect_break_requests.in_set(ServerBlockEditSet::Collect),
            )
            .add_systems(Update, break_blocks.in_set(ServerBlockEditSet::Apply));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn collect_break_requests(
    mut requests: MessageReader<ServerBlockBreakRequested>,
    mut pending: ResMut<PendingBlockBreaks>,
) {
    for request in requests.read() {
        pending.breaks.push(PendingBlockBreak {
            player_id: request.player_id,
            position: request.position,
            allowed: true,
        });
    }
}

fn break_blocks(
    world: Res<ServerChunkWorld>,
    mut pending: ResMut<PendingBlockBreaks>,
    mut broken: MessageWriter<ServerBlockBroken>,
) {
    let requests = std::mem::take(&mut pending.breaks);
    for request in requests {
        if !request.allowed {
            continue;
        }
        match world.break_block_for_player(request.player_id, request.position) {
            Ok(mutation) => {
                broken.write(ServerBlockBroken {
                    player_id: request.player_id,
                    scope: mutation.scope,
                    position: mutation.position,
                    previous: mutation.previous,
                });
            }
            Err(error) => debug!("ignored block break request: {error:?}"),
        }
    }
}
