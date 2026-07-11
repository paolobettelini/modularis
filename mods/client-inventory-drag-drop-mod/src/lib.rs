use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_inventory_ui_api::{ClientInventoryUiApi, InventoryItemVisual, InventorySlotVisual};
use inventory_events_api::LocalInventoryMoveIntent;
use inventory_events_mod::InventoryEventsMod;
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ClientInventoryDragDropMod;

impl ClientInventoryDragDropMod {
    pub fn init<U: ClientInventoryUiApi>(
        bevy: &mut BevyMod,
        _ui: &mut U,
        _events: &mut InventoryEventsMod,
    ) -> Self {
        bevy.app
            .init_resource::<InventoryOperationCounter>()
            .init_resource::<HandledInventoryDrops>()
            .add_observer(move_dragged_item)
            .add_observer(reset_dragged_item)
            .add_observer(drop_inventory_item);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

#[derive(Resource, Default)]
struct InventoryOperationCounter(u64);

#[derive(Resource, Default)]
struct HandledInventoryDrops {
    frame: u128,
    drops: HashSet<(PointerId, Entity)>,
}

fn move_dragged_item(
    drag: On<Pointer<Drag>>,
    mut query: Query<
        (
            &mut UiTransform,
            &mut ZIndex,
            &mut GlobalZIndex,
            &mut Pickable,
        ),
        With<InventoryItemVisual>,
    >,
) {
    if let Ok((mut transform, mut z_index, mut global_z_index, mut pickable)) =
        query.get_mut(drag.event_target())
    {
        transform.translation = Val2::px(drag.distance.x, drag.distance.y);
        z_index.0 = 1000;
        global_z_index.0 = 1000;
        *pickable = Pickable::IGNORE;
    }
}

fn reset_dragged_item(
    drag: On<Pointer<DragEnd>>,
    mut query: Query<
        (
            &mut UiTransform,
            &mut ZIndex,
            &mut GlobalZIndex,
            &mut Pickable,
        ),
        With<InventoryItemVisual>,
    >,
    mut handled: ResMut<HandledInventoryDrops>,
) {
    handled
        .drops
        .remove(&(drag.pointer_id, drag.event_target()));
    if let Ok((mut transform, mut z_index, mut global_z_index, mut pickable)) =
        query.get_mut(drag.event_target())
    {
        transform.translation = Val2::ZERO;
        z_index.0 = 1;
        global_z_index.0 = 121;
        *pickable = Pickable {
            should_block_lower: false,
            is_hoverable: true,
        };
    }
}

fn drop_inventory_item(
    drop: On<Pointer<DragDrop>>,
    time: Res<Time>,
    slots: Query<&InventorySlotVisual>,
    items: Query<&InventoryItemVisual>,
    parents: Query<&ChildOf>,
    mut counter: ResMut<InventoryOperationCounter>,
    mut handled: ResMut<HandledInventoryDrops>,
    mut intents: MessageWriter<LocalInventoryMoveIntent>,
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
    let mut target_entity = drop.event_target();
    let target = loop {
        if let Ok(target) = slots.get(target_entity) {
            break target;
        }
        let Ok(parent) = parents.get(target_entity) else {
            return;
        };
        target_entity = parent.parent();
    };
    let Ok(source) = items.get(drop.dropped) else {
        return;
    };
    if source.cell == target.cell {
        return;
    }
    handled.drops.insert(drop_key);
    counter.0 = counter.0.wrapping_add(1);
    intents.write(LocalInventoryMoveIntent {
        operation_id: counter.0,
        from: source.cell.clone(),
        to: target.cell.clone(),
    });
}
