use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{
    CellMenuCloseRequested, CellMenuInventoryMoveRequested, CellMenuMoveRequested,
    CellMenuOpenIntent, CellMenuServerSet,
};
use cell_menu_events_mod::CellMenuEventsMod;
use cell_menu_network_message_types::CellMenuRequest;
use generated_network_messages::{CellMenuRequestReceived, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerCellMenuNetworkReceiveMod;

impl ServerCellMenuNetworkReceiveMod {
    pub fn init<P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _protocol: &mut NetworkProtocolMod,
        _players: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_cell_menu_intents
                .in_set(CellMenuServerSet::ReceiveRequest)
                .after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_cell_menu_intents(
    players: Res<ServerPlayerRegistry>,
    mut packets: MessageReader<CellMenuRequestReceived>,
    mut move_requests: MessageWriter<CellMenuMoveRequested>,
    mut inventory_move_requests: MessageWriter<CellMenuInventoryMoveRequested>,
    mut close_requests: MessageWriter<CellMenuCloseRequested>,
    mut open_requests: MessageWriter<CellMenuOpenIntent>,
) {
    for packet in packets.read() {
        let Some(player) = players.player_for_address(packet.source) else {
            continue;
        };
        match &packet.message {
            CellMenuRequest::Move(message) => {
                move_requests.write(CellMenuMoveRequested {
                    operation_id: message.operation_id,
                    player_id: player.id,
                    menu_id: message.menu_id.clone(),
                    from: message.from.clone(),
                    to: message.to.clone(),
                });
            }
            CellMenuRequest::InventoryMove(message) => {
                inventory_move_requests.write(CellMenuInventoryMoveRequested {
                    operation_id: message.operation_id,
                    player_id: player.id,
                    from: message.from.clone(),
                    to: message.to.clone(),
                });
            }
            CellMenuRequest::Close(message) => {
                close_requests.write(CellMenuCloseRequested {
                    player_id: player.id,
                    menu_id: message.menu_id.clone(),
                });
            }
            CellMenuRequest::Open(message) => {
                open_requests.write(CellMenuOpenIntent {
                    player_id: player.id,
                    kind: message.kind.clone(),
                    anchor: message.anchor,
                });
            }
        }
    }
}
