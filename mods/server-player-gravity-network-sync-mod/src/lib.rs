use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_gravity_network_message_types::PlayerGravityChanged;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_gravity_api::{
    ServerPlayerGravities, ServerPlayerGravityApi, ServerPlayerGravityChanged,
    ServerPlayerGravitySet,
};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use tokio::task::JoinHandle;

pub struct ServerPlayerGravityNetworkSyncMod;

impl ServerPlayerGravityNetworkSyncMod {
    pub fn init<N: ServerNetworkEventsApi, G: ServerPlayerGravityApi>(
        bevy: &mut BevyMod,
        _network_events: &mut N,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _gravity: &mut G,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_gravity_to_new_players, sync_gravity_changes)
                .in_set(ServerPlayerGravitySet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_gravity_to_new_players(
    gravities: Res<ServerPlayerGravities>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for joined in joined.read() {
        send_gravity(
            joined.player_id,
            gravities.gravity(joined.player_id),
            &mut packets,
        );
    }
}

fn sync_gravity_changes(
    mut changes: MessageReader<ServerPlayerGravityChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() {
        send_gravity(change.player_id, change.gravity, &mut packets);
    }
}

fn send_gravity(
    player_id: player_network_message_types::PlayerId,
    gravity: Vec3,
    packets: &mut MessageWriter<ServerPacketOut>,
) {
    packets.write(ServerPacketOut {
        audience: ServerAudience::Player(player_id),
        message: ClientBoundMessage::PlayerGravityChanged(PlayerGravityChanged {
            gravity: gravity.to_array(),
        }),
    });
}
