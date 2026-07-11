use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{
    BlockBreakRequested, BlockBroken, BlockPlaced, PendingBlockBreaks, ServerBlockBreakRequested,
    ServerBlockBroken, ServerBlockEditSet, ServerBlockPlaced,
};
use block_registry_codegen::BlockRegistryCodegenMod;
use tokio::task::JoinHandle;

pub struct BlockEditEventsMod;

impl BlockEditEventsMod {
    pub fn init(bevy: &mut BevyMod, _blocks: &mut BlockRegistryCodegenMod) -> Self {
        bevy.app
            .init_resource::<PendingBlockBreaks>()
            .add_message::<BlockBreakRequested>()
            .add_message::<ServerBlockBreakRequested>()
            .add_message::<BlockBroken>()
            .add_message::<BlockPlaced>()
            .add_message::<ServerBlockBroken>()
            .add_message::<ServerBlockPlaced>()
            .configure_sets(
                Update,
                (
                    ServerBlockEditSet::Receive,
                    ServerBlockEditSet::Collect,
                    ServerBlockEditSet::Validate,
                    ServerBlockEditSet::Apply,
                    ServerBlockEditSet::Sync,
                )
                    .chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
