use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{CellMenuCellSet, CellMenuClosed, CellMenuOpened, CellMenuServerSet};
use cell_menu_events_mod::CellMenuEventsMod;
use cell_menu_network_message_types::{
    CellMenuClosePacket, CellMenuOpenPacket, CellMenuPacket, CellMenuSetCellPacket,
};
use generated_network_messages::ClientBoundMessage;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use tokio::task::JoinHandle;

pub struct ServerCellMenuNetworkSyncMod;

impl ServerCellMenuNetworkSyncMod {
    pub fn init<N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _network_events: &mut N,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            sync_cell_menu_changes.in_set(CellMenuServerSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_cell_menu_changes(
    mut opened: MessageReader<CellMenuOpened>,
    mut closed: MessageReader<CellMenuClosed>,
    mut cells: MessageReader<CellMenuCellSet>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for event in opened.read() {
        send(
            &mut packets,
            event.viewer,
            ClientBoundMessage::CellMenuPacket(CellMenuPacket::Open(CellMenuOpenPacket {
                menu: event.menu.clone(),
            })),
        );
    }
    for event in cells.read() {
        send(
            &mut packets,
            event.viewer,
            ClientBoundMessage::CellMenuPacket(CellMenuPacket::SetCell(CellMenuSetCellPacket {
                menu_id: event.menu_id.clone(),
                cell: event.cell.clone(),
                item: event.item.clone(),
            })),
        );
    }
    for event in closed.read() {
        send(
            &mut packets,
            event.viewer,
            ClientBoundMessage::CellMenuPacket(CellMenuPacket::Close(CellMenuClosePacket {
                menu_id: event.menu_id.clone(),
            })),
        );
    }
}

fn send(packets: &mut MessageWriter<ServerPacketOut>, player_id: u64, message: ClientBoundMessage) {
    packets.write(ServerPacketOut {
        audience: ServerAudience::Player(player_id),
        message,
    });
}
