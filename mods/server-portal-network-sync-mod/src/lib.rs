use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{ChunkRequestReceived, ClientBoundMessage, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use player_network_message_types::PlayerId;
use portal_network_message_types::PortalOpenedPacket;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use server_portal_api::{
    ActivePortal, ServerPortalApi, ServerPortalOpened, ServerPortalSet, ServerPortals,
};
use tokio::task::JoinHandle;
use voxel_math_api::ChunkPos;
use world_instance_api::WorldScopeId;

pub struct ServerPortalNetworkSyncMod;

impl ServerPortalNetworkSyncMod {
    pub fn init<
        P: ServerPortalApi,
        N: ServerNetworkEventsApi,
        W: ServerChunkWorldApi,
        R: ServerPlayerRegistryApi,
    >(
        bevy: &mut BevyMod,
        _portals: &mut P,
        _network: &mut N,
        _world: &mut W,
        _players: &mut R,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (
                broadcast_opened_portals.in_set(ServerPortalSet::Sync),
                send_portals_with_requested_chunks.after(NetworkMessageSet::DispatchPackets),
            ),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn broadcast_opened_portals(
    world: Res<ServerChunkWorld>,
    players: Res<ServerPlayerRegistry>,
    mut opened: MessageReader<ServerPortalOpened>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for event in opened.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(scope_viewers(
                &world,
                &players,
                &event.portal.scope,
                event.portal.frame.origin.chunk(),
            )),
            message: portal_packet(&event.portal),
        });
    }
}

fn send_portals_with_requested_chunks(
    world: Res<ServerChunkWorld>,
    players: Res<ServerPlayerRegistry>,
    portals: Res<ServerPortals>,
    mut requests: MessageReader<ChunkRequestReceived>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for request in requests.read() {
        let Some(player) = players.player_for_address(request.source) else {
            continue;
        };
        let Some(scope) = world
            .resident_key_for_player(player.id, request.message.position)
            .map(|key| key.scope())
        else {
            continue;
        };
        for portal in portals
            .in_scope(&scope)
            .filter(|portal| portal.frame.touches_chunk(request.message.position))
        {
            packets.write(ServerPacketOut {
                audience: ServerAudience::Address(request.source),
                message: portal_packet(portal),
            });
        }
    }
}

fn portal_packet(portal: &ActivePortal) -> ClientBoundMessage {
    ClientBoundMessage::PortalOpenedPacket(PortalOpenedPacket {
        frame: portal.frame,
        destination: portal.destination,
        color: portal.color,
    })
}

fn scope_viewers(
    world: &ServerChunkWorld,
    players: &ServerPlayerRegistry,
    scope: &WorldScopeId,
    position: ChunkPos,
) -> Vec<PlayerId> {
    players
        .players()
        .into_iter()
        .filter(|player| {
            world
                .resident_key_for_player(player.id, position)
                .is_some_and(|key| &key.scope() == scope)
        })
        .map(|player| player.id)
        .collect()
}
