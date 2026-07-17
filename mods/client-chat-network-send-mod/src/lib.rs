use bevy::prelude::*;
use bevy_mod::BevyMod;
use chat_network_message_types::{ChatSubmit, CommandSuggestionsRequest};
use client_chat_api::{
    ClientChatApi, ClientChatSet, ClientChatSubmitRequested, ClientChatSuggestionsRequested,
};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use generated_network_messages::ServerBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientChatNetworkSendMod;

impl ClientChatNetworkSendMod {
    pub fn init<C: ClientChatApi, N: ClientNetworkApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _network: &mut N,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .add_systems(Update, send_chat_requests.in_set(ClientChatSet::Send));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn send_chat_requests(
    sender: Option<Res<ClientNetworkSender>>,
    mut submitted: MessageReader<ClientChatSubmitRequested>,
    mut suggestions: MessageReader<ClientChatSuggestionsRequested>,
) {
    let Some(sender) = sender else {
        return;
    };
    for message in submitted.read() {
        if let Err(error) = sender.send(&ServerBoundMessage::ChatSubmit(ChatSubmit {
            text: message.0.clone(),
        })) {
            warn!("failed to send chat message: {error}");
        }
    }
    for request in suggestions.read() {
        if let Err(error) = sender.send(&ServerBoundMessage::CommandSuggestionsRequest(
            CommandSuggestionsRequest {
                request_id: request.request_id,
                input: request.input.clone(),
                cursor: request.cursor.min(u32::MAX as usize) as u32,
            },
        )) {
            warn!("failed to request command suggestions: {error}");
        }
    }
}
