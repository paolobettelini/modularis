use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_player_permission_api::{
    ClientPlayerPermissionApi, ClientPlayerPermissions, ClientPlayerPermissionsChanged,
};
use generated_network_messages::{NetworkMessageSet, SetPlayerPermissionsReceived};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientPlayerPermissionNetworkReceiveMod;

impl ClientPlayerPermissionNetworkReceiveMod {
    pub fn init<P: ClientPlayerPermissionApi>(
        bevy: &mut BevyMod,
        _permissions: &mut P,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_permissions.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn receive_permissions(
    mut packets: MessageReader<SetPlayerPermissionsReceived>,
    mut permissions: ResMut<ClientPlayerPermissions>,
    mut changed: MessageWriter<ClientPlayerPermissionsChanged>,
) {
    for packet in packets.read() {
        permissions.replace(packet.0.effective.iter().copied());
        changed.write(ClientPlayerPermissionsChanged { effective: packet.0.effective.clone() });
    }
}
