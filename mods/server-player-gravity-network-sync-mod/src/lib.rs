use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_gravity_network_message_types::PlayerGravityChanged;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use tokio::task::JoinHandle;

#[derive(Resource, Debug, Clone, Copy)]
struct LastSyncedGravity(Vec3);

pub struct ServerPlayerGravityNetworkSyncMod;

impl ServerPlayerGravityNetworkSyncMod {
    pub fn init<N: ServerNetworkEventsApi, G: PlayerGravityApi>(
        bevy: &mut BevyMod,
        _network_events: &mut N,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _gravity: &mut G,
    ) -> Self {
        let initial = bevy.app.world().resource::<Gravity>().0;
        bevy.app
            .insert_resource(LastSyncedGravity(initial))
            .add_systems(
                Update,
                (sync_gravity_to_new_players, broadcast_gravity_changes),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_gravity_to_new_players(
    gravity: Res<Gravity>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    let message = ClientBoundMessage::PlayerGravityChanged(PlayerGravityChanged {
        gravity: gravity.0.to_array(),
    });
    for joined in joined.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(joined.player_id),
            message: message.clone(),
        });
    }
}

fn broadcast_gravity_changes(
    gravity: Res<Gravity>,
    mut last: ResMut<LastSyncedGravity>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    if gravity.0.abs_diff_eq(last.0, 0.0001) {
        return;
    }
    last.0 = gravity.0;
    packets.write(ServerPacketOut {
        audience: ServerAudience::Broadcast,
        message: ClientBoundMessage::PlayerGravityChanged(PlayerGravityChanged {
            gravity: gravity.0.to_array(),
        }),
    });
}
