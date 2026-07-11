use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::LocalCellMenuMoveIntent;
use cell_menu_events_mod::CellMenuEventsMod;
use client_cell_menu_ui_api::{CellMenuItemVisual, CellMenuSlotVisual, ClientCellMenuUiApi};
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ClientCellMenuDragDropMod;

impl ClientCellMenuDragDropMod {
    pub fn init<U: ClientCellMenuUiApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _ui: &mut U,
    ) -> Self {
        bevy.app
            .init_resource::<CellMenuOperationCounter>()
            .init_resource::<HandledCellMenuDrops>()
            .add_observer(move_dragged_item)
            .add_observer(reset_dragged_item)
            .add_observer(drop_cell_menu_item);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

#[derive(Resource, Default)]
struct CellMenuOperationCounter(u64);

#[derive(Resource, Default)]
struct HandledCellMenuDrops {
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
        With<CellMenuItemVisual>,
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
        With<CellMenuItemVisual>,
    >,
    mut handled: ResMut<HandledCellMenuDrops>,
) {
    handled
        .drops
        .remove(&(drag.pointer_id, drag.event_target()));
    if let Ok((mut transform, mut z_index, mut global_z_index, mut pickable)) =
        query.get_mut(drag.event_target())
    {
        transform.translation = Val2::ZERO;
        z_index.0 = 1;
        global_z_index.0 = 131;
        *pickable = Pickable {
            should_block_lower: false,
            is_hoverable: true,
        };
    }
}

fn drop_cell_menu_item(
    drop: On<Pointer<DragDrop>>,
    time: Res<Time>,
    slots: Query<&CellMenuSlotVisual>,
    items: Query<&CellMenuItemVisual>,
    parents: Query<&ChildOf>,
    mut counter: ResMut<CellMenuOperationCounter>,
    mut handled: ResMut<HandledCellMenuDrops>,
    mut intents: MessageWriter<LocalCellMenuMoveIntent>,
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
    if source.menu_id != target.menu_id || source.cell == target.cell {
        return;
    }
    handled.drops.insert(drop_key);
    counter.0 = counter.0.wrapping_add(1);
    intents.write(LocalCellMenuMoveIntent {
        operation_id: counter.0,
        menu_id: source.menu_id.clone(),
        from: source.cell.clone(),
        to: target.cell.clone(),
    });
}
