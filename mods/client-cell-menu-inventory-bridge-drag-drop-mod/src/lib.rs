use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{CellMenuMoveEndpoint, LocalCellMenuInventoryMoveIntent};
use cell_menu_events_mod::CellMenuEventsMod;
use client_cell_menu_ui_api::{CellMenuItemVisual, CellMenuSlotVisual, ClientCellMenuUiApi};
use client_inventory_ui_api::{ClientInventoryUiApi, InventoryItemVisual, InventorySlotVisual};
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ClientCellMenuInventoryBridgeDragDropMod;

impl ClientCellMenuInventoryBridgeDragDropMod {
    pub fn init<C: ClientCellMenuUiApi, I: ClientInventoryUiApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _cell_menu_ui: &mut C,
        _inventory_ui: &mut I,
    ) -> Self {
        bevy.app
            .init_resource::<BridgeOperationCounter>()
            .init_resource::<HandledBridgeDrops>()
            .add_observer(drop_between_inventory_and_cell_menu);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

#[derive(Resource, Default)]
struct BridgeOperationCounter(u64);

#[derive(Resource, Default)]
struct HandledBridgeDrops {
    frame: u128,
    drops: HashSet<(PointerId, Entity)>,
}

fn drop_between_inventory_and_cell_menu(
    drop: On<Pointer<DragDrop>>,
    time: Res<Time>,
    inventory_slots: Query<&InventorySlotVisual>,
    inventory_items: Query<&InventoryItemVisual>,
    cell_menu_slots: Query<&CellMenuSlotVisual>,
    cell_menu_items: Query<&CellMenuItemVisual>,
    parents: Query<&ChildOf>,
    mut counter: ResMut<BridgeOperationCounter>,
    mut handled: ResMut<HandledBridgeDrops>,
    mut intents: MessageWriter<LocalCellMenuInventoryMoveIntent>,
) {
    let frame = time.elapsed().as_nanos();
    if handled.frame != frame {
        handled.frame = frame;
        handled.drops.clear();
    }
    let drop_key = (drop.pointer_id, drop.dropped);
    if handled.drops.contains(&drop_key) {
        return;
    }
    let Some(source) = source_endpoint(drop.dropped, &inventory_items, &cell_menu_items) else {
        return;
    };
    let Some(target) = target_endpoint(
        drop.event_target(),
        &inventory_slots,
        &cell_menu_slots,
        &parents,
    ) else {
        return;
    };
    if !is_cross_inventory_menu_move(&source, &target) {
        return;
    }
    handled.drops.insert(drop_key);
    counter.0 = counter.0.wrapping_add(1);
    intents.write(LocalCellMenuInventoryMoveIntent {
        operation_id: counter.0,
        from: source,
        to: target,
    });
}

fn source_endpoint(
    entity: Entity,
    inventory_items: &Query<&InventoryItemVisual>,
    cell_menu_items: &Query<&CellMenuItemVisual>,
) -> Option<CellMenuMoveEndpoint> {
    if let Ok(item) = inventory_items.get(entity) {
        return Some(CellMenuMoveEndpoint::PlayerInventory {
            cell: item.cell.clone(),
        });
    }
    if let Ok(item) = cell_menu_items.get(entity) {
        return Some(CellMenuMoveEndpoint::CellMenu {
            menu_id: item.menu_id.clone(),
            cell: item.cell.clone(),
        });
    }
    None
}

fn target_endpoint(
    mut entity: Entity,
    inventory_slots: &Query<&InventorySlotVisual>,
    cell_menu_slots: &Query<&CellMenuSlotVisual>,
    parents: &Query<&ChildOf>,
) -> Option<CellMenuMoveEndpoint> {
    loop {
        if let Ok(slot) = inventory_slots.get(entity) {
            return Some(CellMenuMoveEndpoint::PlayerInventory {
                cell: slot.cell.clone(),
            });
        }
        if let Ok(slot) = cell_menu_slots.get(entity) {
            return Some(CellMenuMoveEndpoint::CellMenu {
                menu_id: slot.menu_id.clone(),
                cell: slot.cell.clone(),
            });
        }
        entity = parents.get(entity).ok()?.parent();
    }
}

fn is_cross_inventory_menu_move(from: &CellMenuMoveEndpoint, to: &CellMenuMoveEndpoint) -> bool {
    matches!(
        (from, to),
        (
            CellMenuMoveEndpoint::PlayerInventory { .. },
            CellMenuMoveEndpoint::CellMenu { .. }
        ) | (
            CellMenuMoveEndpoint::CellMenu { .. },
            CellMenuMoveEndpoint::PlayerInventory { .. }
        )
    )
}
