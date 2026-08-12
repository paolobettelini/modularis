use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{ServerBlockBroken, ServerBlockEditSet};
use block_edit_events_mod::BlockEditEventsMod;
use server_block_breaking_events_api::{ServerBlockBreakingSet, ServerValidatedBlockBreak};
use server_block_breaking_events_mod::ServerBlockBreakingEventsMod;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_creative_block_breaking_vanilla_lib::apply_creative_break;
use tokio::task::JoinHandle;

pub struct ServerCreativeBlockBreakingVanillaMod;
impl ServerCreativeBlockBreakingVanillaMod {
    pub fn init<W: ServerChunkWorldApi>(bevy: &mut BevyMod, _edits: &mut BlockEditEventsMod, _breaking: &mut ServerBlockBreakingEventsMod, _world: &mut W) -> Self {
        bevy.app.add_systems(
            Update,
            apply_creative_breaks
                .in_set(ServerBlockBreakingSet::ApplyEffects)
                .in_set(ServerBlockEditSet::Apply),
        );
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn apply_creative_breaks(
    world: Res<ServerChunkWorld>,
    mut requests: MessageReader<ServerValidatedBlockBreak>,
    mut broken: MessageWriter<ServerBlockBroken>,
) {
    for request in requests.read() {
        match apply_creative_break(&world, request) {
            Ok(Some(event)) => { broken.write(event); }
            Ok(None) => {}
            Err(error) => debug!("ignored creative block break: {error:?}"),
        }
    }
}
