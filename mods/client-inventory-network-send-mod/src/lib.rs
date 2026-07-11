use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi, InGameOverlayState};
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use generated_network_messages::{JoinAcceptedReceived, NetworkMessageSet, ServerBoundMessage};
use inventory_events_api::{
    LocalHotbarSelectIntent, LocalInventoryMoveIntent, LocalUseHeldItemIntent,
};
use inventory_events_mod::InventoryEventsMod;
use inventory_network_message_types::{
    HotbarSelectRequest, InventoryMoveRequest, InventorySyncRequest, UseHeldItemRequest,
};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientInventoryNetworkSendMod;

impl ClientInventoryNetworkSendMod {
    pub fn init<N: ClientNetworkApi, G: GameStateApi, C: ClientInventoryCacheApi>(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _network: &mut N,
        _game_state: &mut G,
        _cache: &mut C,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .init_resource::<InventorySyncRetry>()
            .add_systems(
                Update,
                (
                    send_inventory_intents,
                    sync_after_join.after(NetworkMessageSet::DispatchPackets),
                    retry_inventory_sync.run_if(in_state(GameState::InGame)),
                ),
            )
            .add_systems(
                OnEnter(InGameOverlayState::Inventory),
                request_inventory_sync,
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

#[derive(Resource)]
struct InventorySyncRetry(Timer);

impl Default for InventorySyncRetry {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

fn sync_after_join(
    sender: Option<Res<ClientNetworkSender>>,
    mut accepted: MessageReader<JoinAcceptedReceived>,
) {
    if accepted.read().next().is_some()
        && let Some(sender) = sender
    {
        let _ = sender.send(&ServerBoundMessage::InventorySyncRequest(
            InventorySyncRequest,
        ));
    }
}

fn request_inventory_sync(sender: Option<Res<ClientNetworkSender>>) {
    if let Some(sender) = sender {
        let _ = sender.send(&ServerBoundMessage::InventorySyncRequest(
            InventorySyncRequest,
        ));
    }
}

fn retry_inventory_sync(
    time: Res<Time>,
    cache: Res<ClientInventoryCache>,
    sender: Option<Res<ClientNetworkSender>>,
    mut retry: ResMut<InventorySyncRetry>,
) {
    if cache.inventory.is_some() {
        retry.0.reset();
        return;
    }
    retry.0.tick(time.delta());
    if retry.0.just_finished()
        && let Some(sender) = sender
    {
        let _ = sender.send(&ServerBoundMessage::InventorySyncRequest(
            InventorySyncRequest,
        ));
    }
}

fn send_inventory_intents(
    sender: Option<Res<ClientNetworkSender>>,
    mut moves: MessageReader<LocalInventoryMoveIntent>,
    mut selections: MessageReader<LocalHotbarSelectIntent>,
    mut uses: MessageReader<LocalUseHeldItemIntent>,
) {
    let Some(sender) = sender else {
        return;
    };
    for event in moves.read() {
        let _ = sender.send(&ServerBoundMessage::InventoryMoveRequest(
            InventoryMoveRequest {
                operation_id: event.operation_id,
                from: event.from.clone(),
                to: event.to.clone(),
            },
        ));
    }
    for event in selections.read() {
        let _ = sender.send(&ServerBoundMessage::HotbarSelectRequest(
            HotbarSelectRequest { index: event.index },
        ));
    }
    for event in uses.read() {
        let _ = sender.send(&ServerBoundMessage::UseHeldItemRequest(
            UseHeldItemRequest {
                target: event.target.clone(),
            },
        ));
    }
}
