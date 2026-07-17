use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{
    ChatSubmitReceived, CommandSuggestionsRequestReceived, NetworkMessageSet,
};
use network_protocol_mod::NetworkProtocolMod;
use server_chat_api::{
    MAX_CHAT_INPUT_BYTES, ServerChatApi, ServerChatInputReceived, ServerChatSet,
    ServerCommandSuggestionsRequested,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerChatNetworkReceiveMod;

impl ServerChatNetworkReceiveMod {
    pub fn init<C: ServerChatApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _protocol: &mut NetworkProtocolMod,
        _players: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_chat_packets
                .after(NetworkMessageSet::DispatchPackets)
                .in_set(ServerChatSet::Receive),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_chat_packets(
    mut submitted: MessageReader<ChatSubmitReceived>,
    mut suggestion_requests: MessageReader<CommandSuggestionsRequestReceived>,
    players: Res<ServerPlayerRegistry>,
    mut chat: MessageWriter<ServerChatInputReceived>,
    mut suggestions: MessageWriter<ServerCommandSuggestionsRequested>,
) {
    for packet in submitted.read() {
        let Some(player) = players.player_for_address(packet.source) else {
            continue;
        };
        let text = truncate_utf8(packet.message.text.trim(), MAX_CHAT_INPUT_BYTES);
        if !text.is_empty() {
            chat.write(ServerChatInputReceived {
                player_id: player.id,
                text,
            });
        }
    }

    for packet in suggestion_requests.read() {
        let Some(player) = players.player_for_address(packet.source) else {
            continue;
        };
        let input = truncate_utf8(&packet.message.input, MAX_CHAT_INPUT_BYTES);
        let cursor = (packet.message.cursor as usize).min(input.len());
        if !input.is_char_boundary(cursor) {
            continue;
        }
        suggestions.write(ServerCommandSuggestionsRequested {
            player_id: player.id,
            request_id: packet.message.request_id,
            input,
            cursor,
        });
    }
}

fn truncate_utf8(value: &str, maximum_bytes: usize) -> String {
    if value.len() <= maximum_bytes {
        return value.to_string();
    }
    let mut end = maximum_bytes;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_string()
}
