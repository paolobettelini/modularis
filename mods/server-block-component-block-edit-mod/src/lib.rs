use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{ServerBlockBroken, ServerBlockEditSet, ServerBlockPlaced};
use block_edit_events_mod::BlockEditEventsMod;
use server_block_component_api::{ServerBlockComponentApi, ServerBlockComponentSet, ServerBlockComponents};
use server_chunk_provider_api::ChunkProviderId;
use server_chunk_world_api::ResidentChunkKey;
use tokio::task::JoinHandle;

pub struct ServerBlockComponentBlockEditMod;
impl ServerBlockComponentBlockEditMod {
    pub fn init<C: ServerBlockComponentApi>(bevy: &mut BevyMod, _events: &mut BlockEditEventsMod, _components: &mut C) -> Self {
        bevy.app.add_systems(Update, clear_replaced_components
            .after(ServerBlockEditSet::Apply)
            .in_set(ServerBlockComponentSet::Mutate));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn clear_replaced_components(
    mut broken: MessageReader<ServerBlockBroken>,
    mut placed: MessageReader<ServerBlockPlaced>,
    mut components: ResMut<ServerBlockComponents>,
) {
    for (scope, position) in broken.read().map(|event| (&event.scope, event.position))
        .chain(placed.read().map(|event| (&event.scope, event.position)))
    {
        let key = ResidentChunkKey {
            instance: scope.instance.clone(),
            provider: ChunkProviderId::new(scope.source.clone()),
            frame: position.frame,
            position: position.chunk().local,
        };
        components.remove_all_at(&key, position.local().index() as u16);
    }
}
