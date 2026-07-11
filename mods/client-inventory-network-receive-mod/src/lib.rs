use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{
    HotbarSelectionPacketReceived, InventoryResetPacketReceived, InventoryResizePacketReceived,
    InventorySetCellPacketReceived, NetworkMessageSet,
};
use inventory_events_api::{
    ClientHotbarSelectionSet, ClientInventoryCellSet, ClientInventoryReset, ClientInventoryResized,
    InventoryClientSet,
};
use inventory_events_mod::InventoryEventsMod;
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientInventoryNetworkReceiveMod;

impl ClientInventoryNetworkReceiveMod {
    pub fn init(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_inventory_sync
                .in_set(InventoryClientSet::ReceiveSync)
                .after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_inventory_sync(
    mut resets: MessageReader<InventoryResetPacketReceived>,
    mut resizes: MessageReader<InventoryResizePacketReceived>,
    mut cells: MessageReader<InventorySetCellPacketReceived>,
    mut selections: MessageReader<HotbarSelectionPacketReceived>,
    mut reset_events: MessageWriter<ClientInventoryReset>,
    mut resize_events: MessageWriter<ClientInventoryResized>,
    mut cell_events: MessageWriter<ClientInventoryCellSet>,
    mut selection_events: MessageWriter<ClientHotbarSelectionSet>,
) {
    for packet in resets.read() {
        reset_events.write(ClientInventoryReset {
            inventory: packet.0.inventory.clone(),
            selected_hotbar: packet.0.selected_hotbar,
        });
    }
    for packet in resizes.read() {
        resize_events.write(ClientInventoryResized {
            layout: packet.0.layout.clone(),
        });
    }
    for packet in cells.read() {
        cell_events.write(ClientInventoryCellSet {
            cell: packet.0.cell.clone(),
            item: packet.0.item.clone(),
        });
    }
    for packet in selections.read() {
        selection_events.write(ClientHotbarSelectionSet {
            index: packet.0.index,
        });
    }
}
