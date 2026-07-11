use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{
    HotbarSelectRequestReceived, InventoryMoveRequestReceived, InventorySyncRequestReceived,
    NetworkMessageSet, UseHeldItemRequestReceived,
};
use inventory_events_api::{
    HotbarSelectRequested, InventoryMoveRequested, InventoryServerSet, InventorySyncRequested,
    UseHeldItemRequested,
};
use inventory_events_mod::InventoryEventsMod;
use network_protocol_mod::NetworkProtocolMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

pub struct ServerInventoryNetworkReceiveMod;

impl ServerInventoryNetworkReceiveMod {
    pub fn init<P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _protocol: &mut NetworkProtocolMod,
        _players: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_inventory_intents
                .in_set(InventoryServerSet::ReceiveRequest)
                .after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_inventory_intents(
    players: Res<ServerPlayerRegistry>,
    mut moves: MessageReader<InventoryMoveRequestReceived>,
    mut selects: MessageReader<HotbarSelectRequestReceived>,
    mut uses: MessageReader<UseHeldItemRequestReceived>,
    mut syncs: MessageReader<InventorySyncRequestReceived>,
    mut move_requests: MessageWriter<InventoryMoveRequested>,
    mut select_requests: MessageWriter<HotbarSelectRequested>,
    mut use_requests: MessageWriter<UseHeldItemRequested>,
    mut sync_requests: MessageWriter<InventorySyncRequested>,
) {
    for packet in moves.read() {
        let Some(player) = players.player_for_address(packet.source) else {
            continue;
        };
        move_requests.write(InventoryMoveRequested {
            operation_id: packet.message.operation_id,
            player_id: player.id,
            from: packet.message.from.clone(),
            to: packet.message.to.clone(),
        });
    }
    for packet in selects.read() {
        let Some(player) = players.player_for_address(packet.source) else {
            continue;
        };
        select_requests.write(HotbarSelectRequested {
            player_id: player.id,
            index: packet.message.index,
        });
    }
    for packet in uses.read() {
        let Some(player) = players.player_for_address(packet.source) else {
            continue;
        };
        use_requests.write(UseHeldItemRequested {
            player_id: player.id,
            target: packet.message.target.clone(),
        });
    }
    for packet in syncs.read() {
        let Some(player) = players.player_for_address(packet.source) else {
            continue;
        };
        sync_requests.write(InventorySyncRequested {
            player_id: player.id,
        });
    }
}
