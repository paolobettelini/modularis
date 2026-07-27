use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_sound_api::{ClientSoundApi, ClientSoundSet, PlayClientSound};
use generated_network_messages::{NetworkMessageSet, PlaySoundPacketReceived};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientSoundNetworkReceiveMod;

impl ClientSoundNetworkReceiveMod {
    pub fn init<S: ClientSoundApi>(
        bevy: &mut BevyMod,
        _sound: &mut S,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_sounds
                .in_set(ClientSoundSet::Receive)
                .after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_sounds(
    mut packets: MessageReader<PlaySoundPacketReceived>,
    mut sounds: MessageWriter<PlayClientSound>,
) {
    for packet in packets.read() {
        let packet = &packet.0;
        sounds.write(PlayClientSound {
            sound: packet.sound,
            volume: packet.volume,
            pitch: packet.pitch,
            position: packet.position,
        });
    }
}
