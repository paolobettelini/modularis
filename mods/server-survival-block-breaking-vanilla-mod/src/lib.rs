use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_component_api::BlockComponentRegistry;
use block_damage_events_api::ServerBlockDamageChanged;
use block_damage_events_mod::BlockDamageEventsMod;
use block_edit_events_api::{PendingBlockBreak, ServerBlockBroken, ServerBlockEditSet};
use block_edit_events_mod::BlockEditEventsMod;
use block_properties_api::BlockProperties;
use block_properties_registry_mod::BlockPropertiesRegistryMod;
use server_block_breaking_events_api::{ServerBlockBreakingSet, ServerValidatedBlockBreak};
use server_block_breaking_events_mod::ServerBlockBreakingEventsMod;
use server_block_component_api::{
    ServerBlockComponentApi, ServerBlockComponentSet, ServerBlockComponents,
};
use server_block_component_persistence_lib::load_if_needed;
use server_block_durability_lib::{apply_damage, discrete_stage, restore_default};
use block_durability_api::BlockDurability;
use server_block_edit_world_lib::apply_block_break;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_survival_block_breaking_vanilla_lib::collect_survival_damage;
use server_world_data_storage_api::{ServerWorldDataStorage, ServerWorldDataStorageApi};
use tokio::task::JoinHandle;

pub const DAMAGE_PER_PLAYER_TICK: u32 = 5;
pub const BREAKAGE_STAGES: u8 = 6;

pub struct ServerSurvivalBlockBreakingVanillaMod;
impl ServerSurvivalBlockBreakingVanillaMod {
    pub fn init<W: ServerChunkWorldApi, C: ServerBlockComponentApi, S: ServerWorldDataStorageApi>(
        bevy: &mut BevyMod,
        _edits: &mut BlockEditEventsMod,
        _damage_events: &mut BlockDamageEventsMod,
        _properties: &mut BlockPropertiesRegistryMod,
        _damage_component: &mut block_damage_component_mod::BlockDamageComponentMod,
        _breaking: &mut ServerBlockBreakingEventsMod,
        _components: &mut C,
        _world: &mut W,
        _storage: &mut S,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_survival_damage
                .in_set(ServerBlockBreakingSet::ApplyEffects)
                .in_set(ServerBlockEditSet::Apply)
                .in_set(ServerBlockComponentSet::Mutate),
        );
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn apply_survival_damage(
    world: Res<ServerChunkWorld>,
    properties: Res<BlockProperties>,
    registry: Res<BlockComponentRegistry>,
    storage: Res<ServerWorldDataStorage>,
    mut components: ResMut<ServerBlockComponents>,
    mut requests: MessageReader<ServerValidatedBlockBreak>,
    mut damage_changed: MessageWriter<ServerBlockDamageChanged>,
    mut broken: MessageWriter<ServerBlockBroken>,
    mut logged_stages: Local<std::collections::HashMap<(server_chunk_world_api::ResidentChunkKey, voxel_frame_api::VoxelBlockAddress), u8>>,
) {
    for batch in collect_survival_damage(requests.read()) {
        if let Err(error) = load_if_needed(&mut components, &batch.key, &registry, &storage) {
            error!("failed to load block damage for {:?}: {error}", batch.position);
            continue;
        }
        let status = apply_damage(
            &properties, &mut components, batch.key.clone(), batch.position.local().index() as u16,
            batch.block.block, DAMAGE_PER_PLAYER_TICK.saturating_mul(batch.players.len() as u32),
        );
        if !status.broken {
            let stage = discrete_stage(status, BREAKAGE_STAGES);
            let log_stage = if matches!(status.default, BlockDurability::Unbreakable) { u8::MAX } else { stage };
            let log_key = (batch.key.clone(), batch.position);
            if logged_stages.get(&log_key) == Some(&log_stage) {
                continue;
            }
            if matches!(status.default, BlockDurability::Unbreakable) {
                info!(
                    "survival block break at {:?} was ignored because {:?} is unbreakable",
                    batch.position, batch.block.block
                );
                logged_stages.insert(log_key, log_stage);
                continue;
            }
            info!(
                "survival block damage at {:?}: block={:?}, players={}, damage={}, remaining={:?}, stage={}/{}",
                batch.position,
                batch.block.block,
                batch.players.len(),
                status.damage,
                status.remaining,
                stage,
                BREAKAGE_STAGES
            );
            logged_stages.insert(log_key, log_stage);
            damage_changed.write(ServerBlockDamageChanged { scope: batch.key.scope(), position: batch.position, stage });
            continue;
        }
        info!(
            "survival block at {:?} broke after {} accumulated damage",
            batch.position, status.damage
        );
        logged_stages.remove(&(batch.key.clone(), batch.position));
        restore_default(&mut components, &batch.key, batch.position.local().index() as u16);
        damage_changed.write(ServerBlockDamageChanged { scope: batch.key.scope(), position: batch.position, stage: 0 });
        let player_id = *batch.players.iter().next().unwrap();
        match apply_block_break(&world, &PendingBlockBreak { player_id, position: batch.position, allowed: true }) {
            Ok(Some(event)) => { broken.write(event); }
            Ok(None) => {}
            Err(error) => debug!("ignored completed survival block break: {error:?}"),
        }
    }
}
