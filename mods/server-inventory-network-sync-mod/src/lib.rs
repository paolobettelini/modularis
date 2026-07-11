use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use inventory_events_api::{
    HotbarSelectionSet, InventoryCellSet, InventoryResetApplied, InventoryResized,
    InventoryServerSet,
};
use inventory_events_mod::InventoryEventsMod;
use inventory_network_message_types::{
    HotbarSelectionPacket, InventoryResetPacket, InventoryResizePacket, InventorySetCellPacket,
};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use tokio::task::JoinHandle;

pub struct ServerInventoryNetworkSyncMod;

impl ServerInventoryNetworkSyncMod {
    pub fn init<N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _network_events: &mut N,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            sync_inventory_changes.in_set(InventoryServerSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_inventory_changes(
    mut resets: MessageReader<InventoryResetApplied>,
    mut resizes: MessageReader<InventoryResized>,
    mut cells: MessageReader<InventoryCellSet>,
    mut selections: MessageReader<HotbarSelectionSet>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for event in resets.read() {
        send(
            &mut packets,
            event.player_id,
            ClientBoundMessage::InventoryResetPacket(InventoryResetPacket {
                inventory: event.inventory.clone(),
                selected_hotbar: event.selected_hotbar,
            }),
        );
    }
    for event in resizes.read() {
        send(
            &mut packets,
            event.player_id,
            ClientBoundMessage::InventoryResizePacket(InventoryResizePacket {
                layout: event.layout.clone(),
            }),
        );
    }
    for event in cells.read() {
        send(
            &mut packets,
            event.player_id,
            ClientBoundMessage::InventorySetCellPacket(InventorySetCellPacket {
                cell: event.cell.clone(),
                item: event.item.clone(),
            }),
        );
    }
    for event in selections.read() {
        send(
            &mut packets,
            event.player_id,
            ClientBoundMessage::HotbarSelectionPacket(HotbarSelectionPacket { index: event.index }),
        );
    }
}

fn send(packets: &mut MessageWriter<ServerPacketOut>, player_id: u64, message: ClientBoundMessage) {
    packets.write(ServerPacketOut {
        audience: ServerAudience::Player(player_id),
        message,
    });
}
