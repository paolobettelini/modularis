use bevy::prelude::*;
use bevy_mod::BevyMod;
use chat_clear_network_message_types::ClearChat;
use generated_network_messages::ClientBoundMessage;
use server_chat_api::{ClearServerPlayerChatRequested, ServerChatApi, ServerChatSet};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use tokio::task::JoinHandle;

pub struct ServerChatClearNetworkSyncMod;

impl ServerChatClearNetworkSyncMod {
    pub fn init<C: ServerChatApi, N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _network: &mut N,
    ) -> Self {
        bevy.app
            .add_systems(Update, sync_clear_chat.in_set(ServerChatSet::Sync));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_clear_chat(
    mut requests: MessageReader<ClearServerPlayerChatRequested>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for request in requests.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(request.player_id),
            message: ClientBoundMessage::ClearChat(ClearChat),
        });
    }
}
