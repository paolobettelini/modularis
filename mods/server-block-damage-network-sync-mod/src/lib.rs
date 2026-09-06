use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_component_api::BlockComponentRegistry;
use block_damage_events_api::ServerBlockDamageChanged;
use block_damage_events_mod::BlockDamageEventsMod;
use block_damage_network_message_types::BlockDamageProgress;
use block_damage_network_messages_mod::BlockDamageNetworkMessagesMod;
use block_damage_component_mod::BlockDamageComponentMod;
use block_durability_api::BlockDamage;
use block_edit_events_api::ServerBlockEditSet;
use block_properties_api::BlockProperties;
use block_properties_registry_mod::BlockPropertiesRegistryMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use server_block_component_api::{ServerBlockComponentApi, ServerBlockComponents};
use server_block_component_persistence_lib::load_if_needed;
use server_block_breaking_events_api::ServerBlockBreakingSet;
use server_block_durability_lib::{discrete_stage, status};
use server_chunk_stream_events_api::ServerChunkSent;
use server_chunk_stream_events_mod::ServerChunkStreamEventsMod;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use server_world_data_storage_api::{ServerWorldDataStorage, ServerWorldDataStorageApi};
use tokio::task::JoinHandle;
use voxel_math_api::LocalBlockPos;

pub struct ServerBlockDamageNetworkSyncMod;
impl ServerBlockDamageNetworkSyncMod {
    pub fn init<C: ServerBlockComponentApi, W: ServerChunkWorldApi, N: ServerNetworkEventsApi, P: ServerPlayerRegistryApi, S: ServerWorldDataStorageApi>(
        bevy: &mut BevyMod, _events: &mut BlockDamageEventsMod,
        _messages: &mut BlockDamageNetworkMessagesMod,
        _properties: &mut BlockPropertiesRegistryMod,
        _damage_component: &mut BlockDamageComponentMod,
        _components: &mut C, _stream: &mut ServerChunkStreamEventsMod, _world: &mut W,
        _network: &mut N, _players: &mut P, _storage: &mut S, _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (broadcast_changes, sync_sent_chunks)
                .chain()
                .in_set(ServerBlockEditSet::Sync)
                .after(ServerBlockBreakingSet::ApplyEffects),
        );
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn broadcast_changes(
    mut changes: MessageReader<ServerBlockDamageChanged>,
    world: Res<ServerChunkWorld>, players: Res<ServerPlayerRegistry>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() {
        let viewers = players.players().into_iter().filter(|player| {
            world.resident_key_for_player(player.id, change.position.chunk()).is_some_and(|key| key.scope() == change.scope)
        }).map(|player| player.id).collect::<Vec<_>>();
        if viewers.is_empty() {
            warn!(
                "block damage at {:?} reached stage {} but no player resolves to world scope {:?}",
                change.position, change.stage, change.scope
            );
            continue;
        }
        info!(
            "replicating block damage at {:?}, stage {}, to {} viewer(s)",
            change.position, change.stage, viewers.len()
        );
        send(ServerAudience::Players(viewers), change.position, change.stage, &mut packets);
    }
}

fn sync_sent_chunks(
    mut sent: MessageReader<ServerChunkSent>,
    registry: Res<BlockComponentRegistry>, storage: Res<ServerWorldDataStorage>,
    properties: Res<BlockProperties>, world: Res<ServerChunkWorld>,
    mut components: ResMut<ServerBlockComponents>, mut packets: MessageWriter<ServerPacketOut>,
) {
    for event in sent.read() {
        if let Err(error) = load_if_needed(&mut components, &event.key, &registry, &storage) {
            error!("failed to load block damage for streamed chunk {:?}: {error}", event.key.position);
            continue;
        }
        let indices = components.entries::<BlockDamage>(&event.key).into_iter().map(|(index, _)| index).collect::<Vec<_>>();
        for index in indices {
            let x = i32::from(index % 16); let z = i32::from((index / 16) % 16); let y = i32::from(index / 256);
            let local = LocalBlockPos::new(x, y, z).unwrap();
            let position = voxel_frame_api::VoxelBlockAddress::new(event.key.frame,local.to_world(event.key.position));
            let Some(block) = world.block_for_player(event.player_id, position) else { continue };
            let stage = discrete_stage(status(&properties, &components, &event.key, index, block.block), 6);
            send(ServerAudience::Player(event.player_id), position, stage, &mut packets);
        }
    }
}

fn send(audience: ServerAudience, position: voxel_frame_api::VoxelBlockAddress, stage: u8, packets: &mut MessageWriter<ServerPacketOut>) {
    packets.write(ServerPacketOut { audience, message: ClientBoundMessage::BlockDamageProgress(BlockDamageProgress { position, stage }) });
}
