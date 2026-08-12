use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use loading_screen_network_message_types::{RemoveLoadingTask, SetLoadingTask};
use network_protocol_mod::NetworkProtocolMod;
use server_loading_api::{
    ServerLoadingApi, ServerLoadingSet, ServerLoadingTasks, ServerPlayerLoadingTaskChanged,
};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::ServerPlayerReady;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::ServerPlayerSessionSet;
use tokio::task::JoinHandle;

pub struct ServerLoadingNetworkSyncMod;

impl ServerLoadingNetworkSyncMod {
    pub fn init<L: ServerLoadingApi, N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _loading: &mut L,
        _network: &mut N,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _messages: &mut loading_screen_network_messages_mod::LoadingScreenNetworkMessagesMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_changes, sync_ready_players)
                .chain()
                .in_set(ServerLoadingSet::Sync)
                .after(ServerPlayerSessionSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn sync_changes(
    mut changes: MessageReader<ServerPlayerLoadingTaskChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() {
        let message = match &change.task {
            Some(task) => ClientBoundMessage::SetLoadingTask(SetLoadingTask {
                authority: change.key.authority.clone(),
                task: task.clone(),
            }),
            None => ClientBoundMessage::RemoveLoadingTask(RemoveLoadingTask {
                authority: change.key.authority.clone(),
                id: change.key.id.clone(),
            }),
        };
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(change.key.player_id),
            message,
        });
    }
}

fn sync_ready_players(
    tasks: Res<ServerLoadingTasks>,
    mut ready: MessageReader<ServerPlayerReady>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for event in ready.read() {
        for (key, task) in tasks.tasks_for_player(event.player_id) {
            packets.write(ServerPacketOut {
                audience: ServerAudience::Player(event.player_id),
                message: ClientBoundMessage::SetLoadingTask(SetLoadingTask {
                    authority: key.authority,
                    task,
                }),
            });
        }
    }
}
