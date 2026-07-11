use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{ServerBlockBreakRequested, ServerBlockEditSet};
use block_edit_events_mod::BlockEditEventsMod;
use generated_network_messages::{BlockBreakRequestReceived, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerBlockEditNetworkReceiveMod;

impl ServerBlockEditNetworkReceiveMod {
    pub fn init<P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _events: &mut BlockEditEventsMod,
        _protocol: &mut NetworkProtocolMod,
        _players: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_edit_requests
                .after(NetworkMessageSet::DispatchPackets)
                .in_set(ServerBlockEditSet::Receive),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_edit_requests(
    players: Res<ServerPlayerRegistry>,
    mut break_packets: MessageReader<BlockBreakRequestReceived>,
    mut breaks: MessageWriter<ServerBlockBreakRequested>,
) {
    for packet in break_packets.read() {
        let Some(player) = players.player_for_address(packet.source) else {
            continue;
        };
        breaks.write(ServerBlockBreakRequested {
            player_id: player.id,
            position: packet.message.position,
        });
    }
}
