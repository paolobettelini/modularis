use bevy::prelude::*;
use bevy_mod::BevyMod;
use dimension_network_message_types::PlayerDimensionChanged;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_network_message_types::{PlayerJoined, PlayerLeft, PlayerMoved};
use server_dimension_api::{
    ServerDimensionApi, ServerDimensionSet, ServerDimensions, ServerPlayerDimensionChanged,
};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use sky_network_message_types::SkyColorChanged;
use tokio::task::JoinHandle;

pub struct ServerDimensionNetworkSyncMod;

impl ServerDimensionNetworkSyncMod {
    pub fn init<D: ServerDimensionApi, N: ServerNetworkEventsApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _dimensions: &mut D,
        _network: &mut N,
        _players: &mut P,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            sync_dimension_changes.in_set(ServerDimensionSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_dimension_changes(
    dimensions: Res<ServerDimensions>,
    players: Res<ServerPlayerRegistry>,
    mut changes: MessageReader<ServerPlayerDimensionChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() {
        let Some(player) = players.player(change.player_id).cloned() else {
            continue;
        };
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(change.player_id),
            message: ClientBoundMessage::PlayerDimensionChanged(PlayerDimensionChanged {
                dimension: change.current.id,
                position: change.position,
            }),
        });
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(change.player_id),
            message: ClientBoundMessage::SkyColorChanged(SkyColorChanged {
                color: change.current.sky_color,
            }),
        });
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(change.player_id),
            message: ClientBoundMessage::PlayerMoved(PlayerMoved {
                player_id: player.id,
                position: change.position,
                yaw: player.yaw,
                pitch: player.pitch,
            }),
        });

        if change.previous == change.current.id {
            continue;
        }
        let snapshot = players.players();
        let old_viewers = snapshot
            .iter()
            .filter(|viewer| viewer.id != player.id)
            .filter(|viewer| dimensions.dimension_id_for(viewer.id) == Some(change.previous))
            .map(|viewer| viewer.id)
            .collect::<Vec<_>>();
        let new_viewers = snapshot
            .iter()
            .filter(|viewer| viewer.id != player.id)
            .filter(|viewer| dimensions.dimension_id_for(viewer.id) == Some(change.current.id))
            .map(|viewer| viewer.id)
            .collect::<Vec<_>>();

        for old_player in snapshot.iter().filter(|other| {
            other.id != player.id && dimensions.dimension_id_for(other.id) == Some(change.previous)
        }) {
            packets.write(ServerPacketOut {
                audience: ServerAudience::Player(player.id),
                message: ClientBoundMessage::PlayerLeft(PlayerLeft {
                    player_id: old_player.id,
                }),
            });
        }
        for new_player in snapshot.iter().filter(|other| {
            other.id != player.id
                && dimensions.dimension_id_for(other.id) == Some(change.current.id)
        }) {
            packets.write(ServerPacketOut {
                audience: ServerAudience::Player(player.id),
                message: ClientBoundMessage::PlayerJoined(PlayerJoined {
                    player: new_player.clone(),
                }),
            });
        }
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(old_viewers),
            message: ClientBoundMessage::PlayerLeft(PlayerLeft {
                player_id: player.id,
            }),
        });
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(new_viewers),
            message: ClientBoundMessage::PlayerJoined(PlayerJoined { player }),
        });
    }
}
