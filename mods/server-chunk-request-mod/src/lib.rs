use bevy::prelude::*;
use bevy_mod::BevyMod;
use chunk_network_message_types::ChunkResponse;
use generated_network_messages::{ChunkRequestReceived, ClientBoundMessage, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use server_chunk_residency_api::{ServerChunkResidencyApi, ServerChunkResidencyConfig};
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

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
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            answer_chunk_requests.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn answer_chunk_requests(
    mut requests: MessageReader<ChunkRequestReceived>,
    world: Res<ServerChunkWorld>,
    players: Res<ServerPlayerRegistry>,
    residency: Res<ServerChunkResidencyConfig>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for request in requests.read() {
        let Some(player) = players.player_for_address(request.source) else {
            continue;
        };
        let center = voxel_math_api::BlockPos::new(
            player.position[0].floor() as i32,
            player.position[1].floor() as i32,
            player.position[2].floor() as i32,
        )
        .chunk();
        if !residency.contains(center, request.message.position) {
            debug!(
                "ignored out-of-interest chunk request {:?} from player {}",
                request.message.position, player.id
            );
            continue;
        }
        let Some(chunk) = world.chunk_for_player(player.id, request.message.position) else {
            warn!(
                "chunk provider could not answer {:?} for player {}",
                request.message.position, player.id
            );
            continue;
        };
        packets.write(ServerPacketOut {
            audience: ServerAudience::Address(request.source),
            message: ClientBoundMessage::ChunkResponse(ChunkResponse { chunk }),
        });
    }
}
