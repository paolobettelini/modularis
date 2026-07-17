use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use kick_network_message_types::Kick;
use network_protocol_mod::NetworkProtocolMod;
use player_network_message_types::PlayerLeft;
use server_kick_api::{ServerKickApi, ServerKickRequested, ServerKickSet, ServerKickTarget};
use server_network_api::{ServerNetworkApi, ServerNetworkSender};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use server_player_visibility_api::{ServerPlayerVisibility, ServerPlayerVisibilityApi};
use tokio::task::JoinHandle;

const MAX_KICK_REASON_CHARS: usize = 512;

pub struct ServerPlayerKickMod;

impl ServerPlayerKickMod {
    pub fn init<
        K: ServerKickApi,
        N: ServerNetworkApi,
        E: ServerNetworkEventsApi,
        P: ServerPlayerRegistryApi,
        V: ServerPlayerVisibilityApi,
    >(
        bevy: &mut BevyMod,
        _kick: &mut K,
        _network: &mut N,
        _events: &mut E,
        _players: &mut P,
        _visibility: &mut V,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .add_systems(Update, apply_kick_requests.in_set(ServerKickSet::Apply));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_kick_requests(
    mut requests: MessageReader<ServerKickRequested>,
    mut registry: ResMut<ServerPlayerRegistry>,
    network: Res<ServerNetworkSender>,
    visibility: Res<ServerPlayerVisibility>,
    mut lifecycle: MessageWriter<ServerPlayerLeft>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for request in requests.read() {
        let address = match request.target {
            ServerKickTarget::Address(address) => Some(address),
            ServerKickTarget::Player(player_id) => registry.address_for_player(player_id),
        };
        let Some(address) = address else {
            continue;
        };
        let reason = normalized_reason(&request.reason);
        packets.write(ServerPacketOut {
            audience: ServerAudience::Address(address),
            message: ClientBoundMessage::Kick(Kick { reason }),
        });

        let Some(player) = registry.player_for_address(address).cloned() else {
            continue;
        };
        let viewers = visibility.viewers_of(&player, &registry.players());
        let Some(player) = registry.leave(address) else {
            continue;
        };
        network.remove_client(address);
        lifecycle.write(ServerPlayerLeft {
            player_id: player.id,
        });
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(viewers),
            message: ClientBoundMessage::PlayerLeft(PlayerLeft {
                player_id: player.id,
            }),
        });
    }
}

fn normalized_reason(reason: &str) -> String {
    let reason = reason.trim();
    let reason = if reason.is_empty() {
        "Disconnected by the server"
    } else {
        reason
    };
    reason.chars().take(MAX_KICK_REASON_CHARS).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_empty_and_long_reasons() {
        assert_eq!(normalized_reason("  "), "Disconnected by the server");
        assert_eq!(normalized_reason(" reason "), "reason");
        assert_eq!(normalized_reason(&"x".repeat(600)).chars().count(), 512);
    }
}
