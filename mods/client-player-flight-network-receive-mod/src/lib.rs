use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{NetworkMessageSet, PlayerFlightCapabilityChangedReceived};
use network_protocol_mod::NetworkProtocolMod;
use player_flight_api::{LocalFlightCapabilityChanged, LocalPlayerFlight, PlayerFlightApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerFlightNetworkReceiveMod;

impl ClientPlayerFlightNetworkReceiveMod {
    pub fn init<F: PlayerFlightApi>(
        bevy: &mut BevyMod,
        _flight: &mut F,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_flight_capability.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_flight_capability(
    mut packets: MessageReader<PlayerFlightCapabilityChangedReceived>,
    mut flight: ResMut<LocalPlayerFlight>,
    mut changed: MessageWriter<LocalFlightCapabilityChanged>,
) {
    for packet in packets.read() {
        flight.capability_enabled = packet.0.enabled;
        if !packet.0.enabled {
            flight.flying = false;
        }
        changed.write(LocalFlightCapabilityChanged {
            enabled: packet.0.enabled,
        });
    }
}
