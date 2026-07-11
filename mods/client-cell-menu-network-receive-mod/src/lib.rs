use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{
    CellMenuClientSet, ClientCellMenuCellSet, ClientCellMenuClosed, ClientCellMenuOpened,
};
use cell_menu_events_mod::CellMenuEventsMod;
use cell_menu_network_message_types::CellMenuPacket;
use client_game_state_api::{GameStateApi, InGameOverlayCommand};
use generated_network_messages::{CellMenuPacketReceived, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientCellMenuNetworkReceiveMod;

impl ClientCellMenuNetworkReceiveMod {
    pub fn init<G: GameStateApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_cell_menu_sync
                .in_set(CellMenuClientSet::ReceiveSync)
                .after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_cell_menu_sync(
    mut packets: MessageReader<CellMenuPacketReceived>,
    mut open_events: MessageWriter<ClientCellMenuOpened>,
    mut close_events: MessageWriter<ClientCellMenuClosed>,
    mut cell_events: MessageWriter<ClientCellMenuCellSet>,
    mut overlay: MessageWriter<InGameOverlayCommand>,
) {
    for packet in packets.read() {
        match &packet.0 {
            CellMenuPacket::Open(open) => {
                open_events.write(ClientCellMenuOpened {
                    menu: open.menu.clone(),
                });
                overlay.write(InGameOverlayCommand::OpenInventory);
            }
            CellMenuPacket::SetCell(cell) => {
                cell_events.write(ClientCellMenuCellSet {
                    menu_id: cell.menu_id.clone(),
                    cell: cell.cell.clone(),
                    item: cell.item.clone(),
                });
            }
            CellMenuPacket::Close(close) => {
                close_events.write(ClientCellMenuClosed {
                    menu_id: close.menu_id.clone(),
                });
            }
        }
    }
}
