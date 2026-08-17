use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_external_link_api::{
    ClientExternalLinkApi, ClientExternalLinkSet, ShowClientExternalLink,
};
use generated_network_messages::{NetworkMessageSet, ShowExternalLinkReceived};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientExternalLinkNetworkReceiveMod;

impl ClientExternalLinkNetworkReceiveMod {
    pub fn init<L: ClientExternalLinkApi>(
        bevy: &mut BevyMod,
        _links: &mut L,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_links
                .after(NetworkMessageSet::DispatchPackets)
                .in_set(ClientExternalLinkSet::Receive),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn receive_links(
    mut packets: MessageReader<ShowExternalLinkReceived>,
    mut links: MessageWriter<ShowClientExternalLink>,
) {
    for packet in packets.read() {
        links.write(ShowClientExternalLink {
            title: packet.0.title.clone(),
            description: packet.0.description.clone(),
            url: packet.0.url.clone(),
        });
    }
}
