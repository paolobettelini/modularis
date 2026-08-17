use bevy::prelude::*;
use bevy_mod::BevyMod;
use external_link_network_message_types::ShowExternalLink;
use generated_network_messages::ClientBoundMessage;
use server_audience_api::{ServerAudienceApi, ServerAudienceResolver};
use server_external_link_api::{
    PublishServerExternalLink, ServerExternalLinkApi, ServerExternalLinkSet,
};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerExternalLinkNetworkSyncMod;

impl ServerExternalLinkNetworkSyncMod {
    pub fn init<
        L: ServerExternalLinkApi,
        A: ServerAudienceApi,
        P: ServerPlayerRegistryApi,
        N: ServerNetworkEventsApi,
    >(
        bevy: &mut BevyMod,
        _links: &mut L,
        _audience: &mut A,
        _players: &mut P,
        _network: &mut N,
    ) -> Self {
        bevy.app.add_systems(Update, sync_links.in_set(ServerExternalLinkSet::Sync));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn sync_links(
    mut links: MessageReader<PublishServerExternalLink>,
    resolver: Res<ServerAudienceResolver>,
    players: Res<ServerPlayerRegistry>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    let online = players.players().into_iter().map(|player| player.id).collect::<Vec<_>>();
    for link in links.read() {
        let recipients = resolver.resolve(&link.audience, &online);
        if recipients.is_empty() { continue; }
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(recipients),
            message: ClientBoundMessage::ShowExternalLink(ShowExternalLink {
                title: link.title.clone(),
                description: link.description.clone(),
                url: link.url.clone(),
            }),
        });
    }
}
