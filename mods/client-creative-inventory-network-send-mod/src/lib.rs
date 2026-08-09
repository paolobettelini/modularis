use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use creative_inventory_events_api::LocalCreativeItemTakeIntent;
use creative_inventory_events_mod::CreativeInventoryEventsMod;
use creative_inventory_network_message_types::CreativeItemTakeRequest;
use generated_network_messages::ServerBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientCreativeInventoryNetworkSendMod;

impl ClientCreativeInventoryNetworkSendMod {
    pub fn init<N: ClientNetworkApi>(
        bevy: &mut BevyMod,
        _events: &mut CreativeInventoryEventsMod,
        _network: &mut N,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(Update, send_intents);
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn send_intents(
    sender: Option<Res<ClientNetworkSender>>,
    mut intents: MessageReader<LocalCreativeItemTakeIntent>,
) {
    let Some(sender) = sender else { return; };
    for intent in intents.read() {
        let _ = sender.send(&ServerBoundMessage::CreativeItemTakeRequest(CreativeItemTakeRequest {
            operation_id: intent.operation_id,
            item: intent.item,
            to: intent.to.clone(),
        }));
    }
}
