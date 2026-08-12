use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_damage_network_messages_mod::BlockDamageNetworkMessagesMod;
use client_block_damage_api::{ClientBlockDamageApi, ClientBlockDamageSet, SetClientBlockDamage};
use generated_network_messages::{
    BlockDamageProgressReceived, NetworkMessageSet,
};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientBlockDamageNetworkReceiveMod;
impl ClientBlockDamageNetworkReceiveMod {
    pub fn init<D: ClientBlockDamageApi>(
        bevy: &mut BevyMod,
        _damage: &mut D,
        _messages: &mut BlockDamageNetworkMessagesMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_damage
                .after(NetworkMessageSet::DispatchPackets)
                .in_set(ClientBlockDamageSet::Receive),
        );
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
fn receive_damage(mut packets: MessageReader<BlockDamageProgressReceived>, mut changes: MessageWriter<SetClientBlockDamage>) {
    for packet in packets.read() {
        let stage = packet.0.stage.min(6);
        info!("received block damage at {:?}, stage {}", packet.0.position, stage);
        changes.write(SetClientBlockDamage { position: packet.0.position, stage });
    }
}
