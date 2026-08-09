use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use outline_network_message_types::SetOutline;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_outline_api::{
    ServerPlayerOutlineApi, ServerPlayerOutlineChanged, ServerPlayerOutlineSet, ServerPlayerOutlines,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerOutlineNetworkSyncMod;

impl ServerPlayerOutlineNetworkSyncMod {
    pub fn init<O: ServerPlayerOutlineApi, N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _outline: &mut O,
        _network: &mut N,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_changes, sync_joined).chain().in_set(ServerPlayerOutlineSet::Sync),
        );
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn sync_changes(
    mut changes: MessageReader<ServerPlayerOutlineChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() { send(change.player_id, change.enabled, &mut packets); }
}

fn sync_joined(
    state: Res<ServerPlayerOutlines>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for event in joined.read() { send(event.player_id, state.enabled(event.player_id), &mut packets); }
}

fn send(player_id: u64, enabled: bool, packets: &mut MessageWriter<ServerPacketOut>) {
    packets.write(ServerPacketOut {
        audience: ServerAudience::Player(player_id),
        message: ClientBoundMessage::SetOutline(SetOutline(enabled)),
    });
}
