use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_chat_api::{
    ClientChatApi, ClientChatMessageReceived, ClientChatSet, ClientChatSuggestionsReceived,
};
use generated_network_messages::{
    ChatMessageReceived, CommandSuggestionsResponseReceived, NetworkMessageSet,
};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientChatNetworkReceiveMod;

impl ClientChatNetworkReceiveMod {
    pub fn init<C: ClientChatApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_chat
                .after(NetworkMessageSet::DispatchPackets)
                .in_set(ClientChatSet::Receive),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_chat(
    mut messages: MessageReader<ChatMessageReceived>,
    mut suggestions: MessageReader<CommandSuggestionsResponseReceived>,
    mut chat_writer: MessageWriter<ClientChatMessageReceived>,
    mut suggestion_writer: MessageWriter<ClientChatSuggestionsReceived>,
) {
    for message in messages.read() {
        chat_writer.write(ClientChatMessageReceived(message.0.text.clone()));
    }
    for response in suggestions.read() {
        suggestion_writer.write(ClientChatSuggestionsReceived {
            request_id: response.0.request_id,
            suggestions: response.0.suggestions.clone(),
        });
    }
}
