use bevy::prelude::*;
use bevy_mod::BevyMod;
use chunk_network_message_types::ChunkResponse;
use generated_network_messages::{ChunkRequestReceived, ClientBoundMessage, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use server_chunk_residency_api::{ServerChunkResidencyApi, ServerChunkResidencyConfig};
use server_chunk_stream_events_api::{ServerChunkSent,ServerChunkRequestBudget};
use server_chunk_stream_events_mod::ServerChunkStreamEventsMod;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::{collections::{HashSet,HashMap,VecDeque}, net::SocketAddr};
use voxel_frame_api::VoxelChunkAddress;
use tokio::task::JoinHandle;

#[derive(Resource, Default)]
struct ChunkStreamingLogState(HashSet<SocketAddr>);

#[derive(Resource,Default)]
struct QueuedChunkRequests {
    queue: VecDeque<(SocketAddr,u64,u64,VoxelChunkAddress)>,
    keys: HashSet<(u64,u64,VoxelChunkAddress)>,
    per_player: HashMap<u64,usize>,
}
pub struct ServerChunkRequestMod;

impl ServerChunkRequestMod {
    pub fn init<
        N: ServerNetworkEventsApi,
        W: ServerChunkWorldApi,
        P: ServerPlayerRegistryApi,
        R: ServerChunkResidencyApi,
    >(
        bevy: &mut BevyMod,
        _network_events: &mut N,
        _world: &mut W,
        _players: &mut P,
        _residency: &mut R,
        _stream_events: &mut ServerChunkStreamEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .init_resource::<ChunkStreamingLogState>()
            .init_resource::<QueuedChunkRequests>()
            .init_resource::<ServerChunkRequestBudget>()
            .add_systems(
                Update,
                answer_chunk_requests.after(NetworkMessageSet::DispatchPackets).after(voxel_frame_registry_api::VoxelFrameSet::Replicate),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn answer_chunk_requests(
    mut requests: MessageReader<ChunkRequestReceived>,
    mut pending: ResMut<QueuedChunkRequests>,
    budget: Res<ServerChunkRequestBudget>,
    world: Res<ServerChunkWorld>,
    players: Res<ServerPlayerRegistry>,
    residency: Res<ServerChunkResidencyConfig>,
    mut logged_streams: ResMut<ChunkStreamingLogState>,
    mut packets: MessageWriter<ServerPacketOut>,
    mut sent: MessageWriter<ServerChunkSent>,
) {
    logged_streams.0.retain(|address|players.player_for_address(*address).is_some());
    for request in requests.read() {
        let Some(player)=players.player_for_address(request.source) else {continue;};
        let epoch=request.message.movement_epoch;
        if players.movement_epoch(player.id)!=Some(epoch) {continue;}
        let key=(player.id,epoch,request.message.position);
        if pending.queue.len()>=budget.max_pending_total || pending.per_player.get(&player.id).copied().unwrap_or(0)>=budget.max_pending_per_player || !pending.keys.insert(key) {continue;}
        *pending.per_player.entry(player.id).or_default()+=1;
        pending.queue.push_back((request.source,player.id,epoch,request.message.position));
    }
    let started=std::time::Instant::now();
    for attempt in 0..budget.chunks_per_update {
        if attempt>0 && started.elapsed().as_millis()>=budget.time_budget_millis as u128 {break;}
        let Some((source,player_id,movement_epoch,address))=pending.queue.pop_front() else {break;};
        pending.keys.remove(&(player_id,movement_epoch,address));
        if let Some(count)=pending.per_player.get_mut(&player_id) {*count=count.saturating_sub(1);if *count==0{pending.per_player.remove(&player_id);}}

        let Some(player) = players.player_for_address(source) else {
            continue;
        };
        if player.id!=player_id || players.movement_epoch(player.id)!=Some(movement_epoch) { continue; }
        let center = voxel_math_api::BlockPos::new(
            player.position[0].floor() as i32,
            player.position[1].floor() as i32,
            player.position[2].floor() as i32,
        )
        .chunk();
        if logged_streams.0.insert(source) {
            info!(
                "player {} started chunk streaming from center {:?}; first request {:?}",
                player.id, center, address
            );
        }
        let in_interest=if address.frame.is_root() {
            residency.contains(center,address.local)
        } else if let Some(key)=world.resident_key_for_player(player.id,address) {
            world.frames().interested_chunks(&key.scope(),bevy::math::DVec3::new(player.position[0] as f64,player.position[1] as f64,player.position[2] as f64),residency.radius as f64*16.0).contains(&address)
        } else { false };
        if !in_interest {
            debug!(
                "ignored out-of-interest chunk request {:?} from player {}",
                address, player.id
            );
            continue;
        }
        let Some(chunk) = world.chunk_for_player(player.id, address) else {
            warn!(
                "chunk provider could not answer {:?} for player {}",
                address, player.id
            );
            continue;
        };
        packets.write(ServerPacketOut {
            audience: ServerAudience::Address(source),
            message: ClientBoundMessage::ChunkResponse(ChunkResponse { chunk, frame: address.frame, movement_epoch: movement_epoch }),
        });
        if let Some(key) = world.resident_key_for_player(player.id, address) {
            sent.write(ServerChunkSent { player_id: player.id, key });
        }
    }
}
