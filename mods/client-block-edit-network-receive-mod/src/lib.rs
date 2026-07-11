use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{BlockBroken, BlockPlaced};
use block_edit_events_mod::BlockEditEventsMod;
use generated_network_messages::{
    BlockBrokenPacketReceived, BlockPlacedPacketReceived, NetworkMessageSet,
};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientBlockEditNetworkReceiveMod;

impl ClientBlockEditNetworkReceiveMod {
    pub fn init(
        bevy: &mut BevyMod,
        _events: &mut BlockEditEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_block_edits.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_block_edits(
    mut broken_packets: MessageReader<BlockBrokenPacketReceived>,
    mut placed_packets: MessageReader<BlockPlacedPacketReceived>,
    mut broken: MessageWriter<BlockBroken>,
    mut placed: MessageWriter<BlockPlaced>,
) {
    for packet in broken_packets.read() {
        broken.write(BlockBroken {
            position: packet.0.position,
            previous: packet.0.previous.clone(),
        });
    }
    for packet in placed_packets.read() {
        placed.write(BlockPlaced {
            position: packet.0.position,
            block: packet.0.block.clone(),
            replaced: packet.0.replaced.clone(),
        });
    }
}
