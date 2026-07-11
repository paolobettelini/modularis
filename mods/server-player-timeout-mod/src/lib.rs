use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use player_network_message_types::PlayerLeft as PlayerLeftPayload;
use server_network_api::{ServerNetworkApi, ServerNetworkSender};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use server_player_visibility_api::{ServerPlayerVisibility, ServerPlayerVisibilityApi};
use tokio::task::JoinHandle;

const PLAYER_TIMEOUT_SECONDS: f64 = 30.0;

pub struct ServerPlayerTimeoutMod;

impl ServerPlayerTimeoutMod {
    pub fn init<
        N: ServerNetworkApi,
        E: ServerNetworkEventsApi,
        P: ServerPlayerRegistryApi,
        V: ServerPlayerVisibilityApi,
    >(
        bevy: &mut BevyMod,
        _network: &mut N,
        _network_events: &mut E,
        _players: &mut P,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _visibility: &mut V,
    ) -> Self {
        bevy.app.add_systems(Update, expire_inactive_players);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn expire_inactive_players(
    time: Res<Time>,
    mut registry: ResMut<ServerPlayerRegistry>,
    network: Res<ServerNetworkSender>,
    mut left: MessageWriter<ServerPlayerLeft>,
    mut packets: MessageWriter<ServerPacketOut>,
    visibility: Res<ServerPlayerVisibility>,
) {
    let older_than = time.elapsed_secs_f64() - PLAYER_TIMEOUT_SECONDS;
    for (address, player) in registry.expire_inactive(older_than) {
        let viewers = visibility.viewers_of(&player, &registry.players());
        left.write(ServerPlayerLeft {
            player_id: player.id,
        });
        network.remove_client(address);
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(viewers),
            message: ClientBoundMessage::PlayerLeft(PlayerLeftPayload {
                player_id: player.id,
            }),
        });
    }
}
