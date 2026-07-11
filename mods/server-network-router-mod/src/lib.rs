use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_network_api::{ServerNetworkApi, ServerNetworkSender};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerNetworkRouterMod;

impl ServerNetworkRouterMod {
    pub fn init<N: ServerNetworkApi, E: ServerNetworkEventsApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _network: &mut N,
        _events: &mut E,
        _players: &mut P,
    ) -> Self {
        bevy.app.add_systems(Update, route_server_packets);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn route_server_packets(
    network: Res<ServerNetworkSender>,
    players: Res<ServerPlayerRegistry>,
    mut packets: MessageReader<ServerPacketOut>,
) {
    for packet in packets.read() {
        match &packet.audience {
            ServerAudience::Address(address) => {
                if let Err(error) = network.send_to(*address, &packet.message) {
                    warn!("failed to send packet to {address}: {error}");
                }
            }
            ServerAudience::Player(player_id) => {
                let Some(address) = players.address_for_player(*player_id) else {
                    continue;
                };
                if let Err(error) = network.send_to(address, &packet.message) {
                    warn!("failed to send packet to player {player_id}: {error}");
                }
            }
            ServerAudience::Broadcast => {
                network.broadcast(&packet.message);
            }
            ServerAudience::BroadcastExceptAddress(address) => {
                network.broadcast_except(*address, &packet.message);
            }
            ServerAudience::BroadcastExceptPlayer(player_id) => {
                let Some(address) = players.address_for_player(*player_id) else {
                    network.broadcast(&packet.message);
                    continue;
                };
                network.broadcast_except(address, &packet.message);
            }
            ServerAudience::Players(player_ids) => {
                for player_id in player_ids {
                    let Some(address) = players.address_for_player(*player_id) else {
                        continue;
                    };
                    if let Err(error) = network.send_to(address, &packet.message) {
                        warn!("failed to send packet to player {player_id}: {error}");
                    }
                }
            }
        }
    }
}
