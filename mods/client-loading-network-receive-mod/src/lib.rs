use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_loading_api::{
    ClientLoadingApi, ClientLoadingAuthority, ClientLoadingSet, ClientLoadingTaskKey,
    RemoveClientLoadingTask, SetClientLoadingTask,
};
use generated_network_messages::{
    NetworkMessageSet, RemoveLoadingTaskReceived, SetLoadingTaskReceived,
};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientLoadingNetworkReceiveMod;

impl ClientLoadingNetworkReceiveMod {
    pub fn init<L: ClientLoadingApi>(
        bevy: &mut BevyMod,
        _loading: &mut L,
        _messages: &mut loading_screen_network_messages_mod::LoadingScreenNetworkMessagesMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_server_loading_tasks
                .after(NetworkMessageSet::DispatchPackets)
                .in_set(ClientLoadingSet::Receive),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn receive_server_loading_tasks(
    mut sets: MessageReader<SetLoadingTaskReceived>,
    mut removals: MessageReader<RemoveLoadingTaskReceived>,
    mut set_tasks: MessageWriter<SetClientLoadingTask>,
    mut remove_tasks: MessageWriter<RemoveClientLoadingTask>,
) {
    for packet in sets.read() {
        set_tasks.write(SetClientLoadingTask {
            authority: ClientLoadingAuthority::Server(packet.0.authority.clone()),
            task: packet.0.task.clone(),
        });
    }
    for packet in removals.read() {
        remove_tasks.write(RemoveClientLoadingTask {
            key: ClientLoadingTaskKey {
                authority: ClientLoadingAuthority::Server(packet.0.authority.clone()),
                id: packet.0.id.clone(),
            },
        });
    }
}
