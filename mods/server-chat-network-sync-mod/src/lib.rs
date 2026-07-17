use bevy::prelude::*;
use bevy_mod::BevyMod;
use chat_network_message_types::{ChatMessage, CommandSuggestionsResponse};
use generated_network_messages::ClientBoundMessage;
use server_audience_api::{ServerAudienceApi, ServerAudienceResolver};
use server_chat_api::{
    PublishServerChatMessage, ServerChatApi, ServerChatSet, ServerCommandSuggestionsReady,
};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerChatNetworkSyncMod;

impl ServerChatNetworkSyncMod {
    pub fn init<
        C: ServerChatApi,
        A: ServerAudienceApi,
        P: ServerPlayerRegistryApi,
        N: ServerNetworkEventsApi,
    >(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _audience: &mut A,
        _players: &mut P,
        _network: &mut N,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_chat_messages, sync_suggestions).in_set(ServerChatSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_chat_messages(
    mut messages: MessageReader<PublishServerChatMessage>,
    resolver: Res<ServerAudienceResolver>,
    players: Res<ServerPlayerRegistry>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    let online = players
        .players()
        .into_iter()
        .map(|player| player.id)
        .collect::<Vec<_>>();
    for message in messages.read() {
        let recipients = resolver.resolve(&message.audience, &online);
        if recipients.is_empty() {
            continue;
        }
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(recipients),
            message: ClientBoundMessage::ChatMessage(ChatMessage {
                text: message.text.clone(),
            }),
        });
    }
}

fn sync_suggestions(
    mut ready: MessageReader<ServerCommandSuggestionsReady>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for suggestions in ready.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(suggestions.player_id),
            message: ClientBoundMessage::CommandSuggestionsResponse(CommandSuggestionsResponse {
                request_id: suggestions.request_id,
                suggestions: suggestions.suggestions.clone(),
            }),
        });
    }
}
