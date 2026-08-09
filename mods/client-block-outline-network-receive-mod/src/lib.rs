use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_block_outline_api::{ClientBlockOutlineApi, SetClientBlockOutlineEnabled};
use generated_network_messages::{NetworkMessageSet, SetOutlineReceived};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientBlockOutlineNetworkReceiveMod;

impl ClientBlockOutlineNetworkReceiveMod {
    pub fn init<O: ClientBlockOutlineApi>(
        bevy: &mut BevyMod,
        _outline: &mut O,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(Update, receive_outline.after(NetworkMessageSet::DispatchPackets));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn receive_outline(
    mut packets: MessageReader<SetOutlineReceived>,
    mut changes: MessageWriter<SetClientBlockOutlineEnabled>,
) {
    for packet in packets.read() { changes.write(SetClientBlockOutlineEnabled(packet.0.0)); }
}
