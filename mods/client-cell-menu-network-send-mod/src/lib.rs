use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{
    LocalCellMenuCloseIntent, LocalCellMenuInventoryMoveIntent, LocalCellMenuMoveIntent,
};
use cell_menu_events_mod::CellMenuEventsMod;
use cell_menu_network_message_types::{
    CellMenuCloseRequest, CellMenuInventoryMoveRequest, CellMenuMoveRequest, CellMenuRequest,
};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use generated_network_messages::ServerBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientCellMenuNetworkSendMod;

impl ClientCellMenuNetworkSendMod {
    pub fn init<N: ClientNetworkApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _network: &mut N,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(Update, send_cell_menu_intents);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn send_cell_menu_intents(
    sender: Option<Res<ClientNetworkSender>>,
    mut moves: MessageReader<LocalCellMenuMoveIntent>,
    mut inventory_moves: MessageReader<LocalCellMenuInventoryMoveIntent>,
    mut closes: MessageReader<LocalCellMenuCloseIntent>,
) {
    let Some(sender) = sender else {
        return;
    };
    for event in moves.read() {
        let _ = sender.send(&ServerBoundMessage::CellMenuRequest(CellMenuRequest::Move(
            CellMenuMoveRequest {
                operation_id: event.operation_id,
                menu_id: event.menu_id.clone(),
                from: event.from.clone(),
                to: event.to.clone(),
            },
        )));
    }
    for event in inventory_moves.read() {
        let _ = sender.send(&ServerBoundMessage::CellMenuRequest(
            CellMenuRequest::InventoryMove(CellMenuInventoryMoveRequest {
                operation_id: event.operation_id,
                from: event.from.clone(),
                to: event.to.clone(),
            }),
        ));
    }
    for event in closes.read() {
        let _ = sender.send(&ServerBoundMessage::CellMenuRequest(
            CellMenuRequest::Close(CellMenuCloseRequest {
                menu_id: event.menu_id.clone(),
            }),
        ));
    }
}
