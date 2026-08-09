use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_permission_network_message_types::SetPlayerPermissions;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_permission_api::{
    ServerPlayerPermissionApi, ServerPlayerPermissionSet, ServerPlayerPermissions,
    ServerPlayerPermissionsChanged,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerPermissionNetworkSyncMod;

impl ServerPlayerPermissionNetworkSyncMod {
    pub fn init<P: ServerPlayerPermissionApi, N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _permissions: &mut P,
        _network: &mut N,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_changed_permissions, sync_joined_players)
                .chain()
                .in_set(ServerPlayerPermissionSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn sync_changed_permissions(
    mut changes: MessageReader<ServerPlayerPermissionsChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() {
        send(change.player_id, change.effective.clone(), &mut packets);
    }
}

fn sync_joined_players(
    permissions: Res<ServerPlayerPermissions>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for event in joined.read() {
        send(event.player_id, permissions.effective(event.player_id), &mut packets);
    }
}

fn send(
    player_id: player_network_message_types::PlayerId,
    effective: Vec<generated_permission_registry::PermissionId>,
    packets: &mut MessageWriter<ServerPacketOut>,
) {
    packets.write(ServerPacketOut {
        audience: ServerAudience::Player(player_id),
        message: ClientBoundMessage::SetPlayerPermissions(SetPlayerPermissions { effective }),
    });
}
