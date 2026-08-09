use bevy::{
    picking::hover::HoverMap,
    picking::pointer::PointerId,
    prelude::*,
    ui::OverrideClip,
};
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use client_inventory_ui_api::{
    ClientInventoryUiApi, InventoryOperationSequence, InventorySlotVisual,
    InventoryUiInputCapture, ItemCatalogVisual,
};
use client_player_permission_api::{ClientPlayerPermissionApi, ClientPlayerPermissions};
use creative_inventory_events_api::LocalCreativeItemTakeIntent;
use creative_inventory_events_mod::CreativeInventoryEventsMod;
use generated_permission_registry::PermissionId;
use inventory_core_api::InventoryCell;
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ClientItemCatalogCreativeMod;

impl ClientItemCatalogCreativeMod {
    pub fn init<U: ClientInventoryUiApi, C: ClientInventoryCacheApi, P: ClientPlayerPermissionApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _ui: &mut U,
        _cache: &mut C,
        _permissions: &mut P,
        _events: &mut CreativeInventoryEventsMod,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<HandledCatalogDrops>()
            .add_observer(move_catalog_item)
            .add_observer(reset_catalog_item)
            .add_observer(drop_catalog_item)
            .add_systems(Update, number_take.run_if(in_state(InGameOverlayState::Inventory)));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

#[derive(Resource, Default)]
struct HandledCatalogDrops(HashSet<(PointerId, Entity)>);

fn move_catalog_item(
    drag: On<Pointer<Drag>>,
    permissions: Res<ClientPlayerPermissions>,
    mut commands: Commands,
    mut items: Query<(&mut UiTransform, &mut ZIndex, &mut GlobalZIndex, &mut Pickable), With<ItemCatalogVisual>>,
) {
    if !permissions.has(PermissionId::Privileged) { return; }
    if let Ok((mut transform, mut z, mut global_z, mut pickable)) = items.get_mut(drag.event_target()) {
        transform.translation = Val2::px(drag.distance.x, drag.distance.y);
        z.0 = 1000;
        global_z.0 = 1000;
        *pickable = Pickable::IGNORE;
        commands.entity(drag.event_target()).insert(OverrideClip);
    }
}

fn reset_catalog_item(
    drag: On<Pointer<DragEnd>>,
    mut commands: Commands,
    mut items: Query<(&mut UiTransform, &mut ZIndex, &mut GlobalZIndex, &mut Pickable), With<ItemCatalogVisual>>,
    mut handled: ResMut<HandledCatalogDrops>,
) {
    handled.0.remove(&(drag.pointer_id, drag.event_target()));
    if let Ok((mut transform, mut z, mut global_z, mut pickable)) = items.get_mut(drag.event_target()) {
        transform.translation = Val2::ZERO;
        z.0 = 1;
        global_z.0 = 121;
        *pickable = Pickable { should_block_lower: false, is_hoverable: true };
        commands.entity(drag.event_target()).remove::<OverrideClip>();
    }
}

fn drop_catalog_item(
    drop: On<Pointer<DragDrop>>,
    permissions: Res<ClientPlayerPermissions>,
    slots: Query<&InventorySlotVisual>,
    catalogs: Query<&ItemCatalogVisual>,
    parents: Query<&ChildOf>,
    mut sequence: ResMut<InventoryOperationSequence>,
    mut handled: ResMut<HandledCatalogDrops>,
    mut intents: MessageWriter<LocalCreativeItemTakeIntent>,
) {
    if !permissions.has(PermissionId::Privileged) || !handled.0.insert((drop.pointer_id, drop.dropped)) { return; }
    let Ok(source) = catalogs.get(drop.dropped) else { return; };
    let Some(target) = find_slot(drop.event_target(), &slots, &parents) else { return; };
    intents.write(LocalCreativeItemTakeIntent {
        operation_id: sequence.next(),
        item: source.item,
        to: target.cell.clone(),
    });
}

fn number_take(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<InventoryUiInputCapture>,
    permissions: Res<ClientPlayerPermissions>,
    hover: Res<HoverMap>,
    catalogs: Query<&ItemCatalogVisual>,
    cache: Res<ClientInventoryCache>,
    mut sequence: ResMut<InventoryOperationSequence>,
    mut intents: MessageWriter<LocalCreativeItemTakeIntent>,
) {
    if capture.0 || !permissions.has(PermissionId::Privileged) { return; }
    let Some(index) = pressed_number(&keyboard) else { return; };
    let Some(source) = hover.values().flat_map(|entities| entities.keys()).find_map(|entity| catalogs.get(*entity).ok()) else { return; };
    let Some(inventory) = cache.inventory.as_ref() else { return; };
    let Some(hotbar) = inventory.layout.hotbar() else { return; };
    if index >= hotbar.cells { return; }
    intents.write(LocalCreativeItemTakeIntent {
        operation_id: sequence.next(),
        item: source.item,
        to: InventoryCell { section: hotbar.id.clone(), index },
    });
}

fn find_slot<'a>(
    mut entity: Entity,
    slots: &'a Query<&InventorySlotVisual>,
    parents: &Query<&ChildOf>,
) -> Option<&'a InventorySlotVisual> {
    loop {
        if let Ok(slot) = slots.get(entity) { return Some(slot); }
        entity = parents.get(entity).ok()?.parent();
    }
}

fn pressed_number(keyboard: &ButtonInput<KeyCode>) -> Option<u32> {
    [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9]
        .into_iter()
        .position(|key| keyboard.just_pressed(key))
        .map(|index| index as u32)
}
