use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use client_inventory_ui_api::InventoryItemNameVisual;
use inventory_core_api::InventoryCell;
use inventory_events_api::{
    InventoryClientRenderSet, InventorySlotVisualCreated, LocalHotbarSelectIntent,
};
use inventory_events_mod::InventoryEventsMod;
use item_manager_api::ItemManagerApi;
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct ClientHotbarUiBevyMod<I>(PhantomData<I>);

impl<I: ItemManagerApi> ClientHotbarUiBevyMod<I> {
    pub fn init<G: GameStateApi, C: ClientInventoryCacheApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _cache: &mut C,
        _events: &mut InventoryEventsMod,
        _items: &mut I,
    ) -> Self {
        bevy.app
            .init_resource::<RenderedHotbarRevision>()
            .init_resource::<RenderedHotbarSelectionRevision>()
            .add_systems(OnEnter(GameState::InGame), mark_hotbar_dirty)
            .add_systems(
                Update,
                rebuild_hotbar::<I>
                    .run_if(in_state(GameState::InGame))
                    .in_set(InventoryClientRenderSet::Layout),
            )
            .add_systems(
                Update,
                update_hotbar_selection
                    .run_if(in_state(GameState::InGame))
                    .in_set(InventoryClientRenderSet::Layout)
                    .after(rebuild_hotbar::<I>),
            );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

#[derive(Component)]
struct HotbarRoot;

#[derive(Component)]
struct HotbarSlot(u32);

#[derive(Resource)]
struct RenderedHotbarRevision(u64);

impl Default for RenderedHotbarRevision {
    fn default() -> Self {
        Self(u64::MAX)
    }
}

#[derive(Resource)]
struct RenderedHotbarSelectionRevision(u64);

impl Default for RenderedHotbarSelectionRevision {
    fn default() -> Self {
        Self(u64::MAX)
    }
}

fn mark_hotbar_dirty(mut revision: ResMut<RenderedHotbarRevision>) {
    revision.0 = u64::MAX;
}

fn rebuild_hotbar<I: ItemManagerApi>(
    mut commands: Commands,
    cache: Res<ClientInventoryCache>,
    mut rendered: ResMut<RenderedHotbarRevision>,
    roots: Query<Entity, With<HotbarRoot>>,
    mut visuals: MessageWriter<InventorySlotVisualCreated>,
) {
    if rendered.0 == cache.content_revision {
        return;
    }
    for root in &roots {
        commands.entity(root).despawn();
    }
    rendered.0 = cache.content_revision;
    let Some(inventory) = cache.inventory.as_ref() else {
        return;
    };
    let Some(hotbar) = inventory.layout.hotbar() else {
        return;
    };
    commands
        .spawn((
            HotbarRoot,
            DespawnOnExit(GameState::InGame),
            Node {
                position_type: PositionType::Absolute,
                bottom: px(18),
                left: percent(50),
                padding: UiRect::all(px(5)),
                column_gap: px(4),
                ..default()
            },
            UiTransform::from_translation(Val2::percent(-50.0, 0.0)),
            BackgroundColor(Color::srgba(0.04, 0.05, 0.07, 0.86)),
            BorderRadius::all(px(7)),
            GlobalZIndex(80),
        ))
        .with_children(|root| {
            for index in 0..hotbar.cells {
                let selected = index == cache.selected_hotbar;
                let cell = InventoryCell {
                    section: hotbar.id.clone(),
                    index,
                };
                root.spawn((
                    HotbarSlot(index),
                    Node {
                        width: px(52),
                        height: px(52),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(px(3)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.13, 0.15, 0.19, 0.96)),
                    BorderColor::all(if selected {
                        Color::WHITE
                    } else {
                        Color::srgb(0.33, 0.36, 0.43)
                    }),
                    BorderRadius::all(px(4)),
                ))
                .observe(select_hotbar_slot)
                .with_children(|slot| {
                    let Some(item) = inventory.get(&cell).cloned() else {
                        return;
                    };
                    let item_entity = slot
                        .spawn((
                            Node {
                                width: percent(100),
                                height: percent(100),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            Pickable::IGNORE,
                        ))
                        .with_child((
                            InventoryItemNameVisual,
                            Text::new(I::label(item.item)),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            Visibility::Visible,
                            Pickable::IGNORE,
                        ))
                        .id();
                    visuals.write(InventorySlotVisualCreated {
                        entity: item_entity,
                        item,
                    });
                });
            }
        });
}

fn update_hotbar_selection(
    cache: Res<ClientInventoryCache>,
    mut rendered: ResMut<RenderedHotbarSelectionRevision>,
    mut slots: Query<(&HotbarSlot, &mut BorderColor)>,
) {
    if rendered.0 == cache.selection_revision {
        return;
    }
    rendered.0 = cache.selection_revision;
    for (slot, mut border) in &mut slots {
        border.set_all(if slot.0 == cache.selected_hotbar {
            Color::WHITE
        } else {
            Color::srgb(0.33, 0.36, 0.43)
        });
    }
}

fn select_hotbar_slot(
    click: On<Pointer<Click>>,
    slots: Query<&HotbarSlot>,
    mut selections: MessageWriter<LocalHotbarSelectIntent>,
) {
    if let Ok(slot) = slots.get(click.event_target()) {
        selections.write(LocalHotbarSelectIntent { index: slot.0 });
    }
}
