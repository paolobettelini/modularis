use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_sun_api::{ClientSunApi, ClientSunSettings, ClientSunSettingsChanged};
use generated_network_messages::{NetworkMessageSet, SunSettingsChangedReceived};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientSunNetworkReceiveMod;

impl ClientSunNetworkReceiveMod {
    pub fn init<S: ClientSunApi>(
        bevy: &mut BevyMod,
        _sun: &mut S,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_sun_settings.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_sun_settings(
    mut packets: MessageReader<SunSettingsChangedReceived>,
    mut current: ResMut<ClientSunSettings>,
    mut changed: MessageWriter<ClientSunSettingsChanged>,
) {
    for packet in packets.read() {
        let previous = current.0;
        current.0 = packet.0.settings;
        changed.write(ClientSunSettingsChanged {
            previous,
            current: current.0,
        });
    }
}
