use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_chat_api::{ClientChatApi, ClientChatCleared, ClientChatSet};
use generated_network_messages::{ClearChatReceived, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientChatClearNetworkReceiveMod;

impl ClientChatClearNetworkReceiveMod {
    pub fn init<C: ClientChatApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_clear_chat
                .after(NetworkMessageSet::DispatchPackets)
                .in_set(ClientChatSet::Receive),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_clear_chat(
    mut packets: MessageReader<ClearChatReceived>,
    mut cleared: MessageWriter<ClientChatCleared>,
) {
    if packets.read().next().is_some() {
        cleared.write(ClientChatCleared);
    }
}
