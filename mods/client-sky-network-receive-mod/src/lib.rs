use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_sky_api::{ClientSkyApi, ClientSkyColor};
use generated_network_messages::{NetworkMessageSet, SkyColorChangedReceived};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientSkyNetworkReceiveMod;

impl ClientSkyNetworkReceiveMod {
    pub fn init<S: ClientSkyApi>(
        bevy: &mut BevyMod,
        _sky: &mut S,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_sky_color.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_sky_color(
    mut packets: MessageReader<SkyColorChangedReceived>,
    mut sky: ResMut<ClientSkyColor>,
) {
    for packet in packets.read() {
        sky.0 = packet.0.color;
    }
}
