use bevy::prelude::*;
use bevy_mod::BevyMod;
use creative_inventory_events_api::CreativeItemTakeRequested;
use creative_inventory_events_mod::CreativeInventoryEventsMod;
use generated_network_messages::{CreativeItemTakeRequestReceived, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerCreativeInventoryNetworkReceiveMod;

impl ServerCreativeInventoryNetworkReceiveMod {
    pub fn init<P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _events: &mut CreativeInventoryEventsMod,
        _players: &mut P,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(Update, receive_requests.after(NetworkMessageSet::DispatchPackets));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn receive_requests(
    players: Res<ServerPlayerRegistry>,
    mut packets: MessageReader<CreativeItemTakeRequestReceived>,
    mut requests: MessageWriter<CreativeItemTakeRequested>,
) {
    for packet in packets.read() {
        let Some(player) = players.player_for_address(packet.source) else { continue; };
        requests.write(CreativeItemTakeRequested {
            operation_id: packet.message.operation_id,
            player_id: player.id,
            item: packet.message.item,
            to: packet.message.to.clone(),
        });
    }
}
