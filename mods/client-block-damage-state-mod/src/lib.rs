use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{BlockBroken, BlockPlaced};
use block_edit_events_mod::BlockEditEventsMod;
use client_block_damage_api::{ClientBlockDamage, ClientBlockDamageApi, ClientBlockDamageChanged, ClientBlockDamageSet, SetClientBlockDamage};
use client_chunk_streaming_api::{ChunkStreamingApi, ChunkUnload};
use client_game_state_api::{GameState, GameStateApi};
use tokio::task::JoinHandle;

pub struct ClientBlockDamageStateMod;
impl ClientBlockDamageStateMod {
    pub fn init<S: ChunkStreamingApi, G: GameStateApi>(bevy: &mut BevyMod, _edits: &mut BlockEditEventsMod, _streaming: &mut S, _game: &mut G) -> Self {
        bevy.app.init_resource::<ClientBlockDamage>()
            .add_message::<SetClientBlockDamage>().add_message::<ClientBlockDamageChanged>()
            .configure_sets(Update, (ClientBlockDamageSet::Receive, ClientBlockDamageSet::Apply, ClientBlockDamageSet::Draw).chain())
            .add_systems(Update, (clear_replaced_blocks, apply_changes, clear_unloaded).chain().in_set(ClientBlockDamageSet::Apply))
            .add_systems(OnExit(GameState::InGame), clear_all);
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
impl ClientBlockDamageApi for ClientBlockDamageStateMod {}

fn clear_replaced_blocks(
    mut broken: MessageReader<BlockBroken>, mut placed: MessageReader<BlockPlaced>,
    mut changes: MessageWriter<SetClientBlockDamage>,
) {
    for position in broken.read().map(|event| event.position).chain(placed.read().map(|event| event.position)) {
        changes.write(SetClientBlockDamage { position, stage: 0 });
    }
}
fn apply_changes(
    mut state: ResMut<ClientBlockDamage>, mut changes: MessageReader<SetClientBlockDamage>,
    mut changed: MessageWriter<ClientBlockDamageChanged>,
) {
    for change in changes.read() {
        if state.stage(change.position) == change.stage { continue; }
        state.set(change.position, change.stage);
        changed.write(ClientBlockDamageChanged { position: change.position, stage: change.stage });
    }
}
fn clear_unloaded(
    mut state: ResMut<ClientBlockDamage>, mut unloads: MessageReader<ChunkUnload>,
    mut changed: MessageWriter<ClientBlockDamageChanged>,
) {
    for unload in unloads.read() {
        for position in state.remove_chunk(unload.position) { changed.write(ClientBlockDamageChanged { position, stage: 0 }); }
    }
}
fn clear_all(mut state: ResMut<ClientBlockDamage>) { state.clear(); }

