use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use client_inventory_ui_api::{
    ClientInventoryUiApi, InventoryItemNameVisual, InventoryItemVisual, InventorySlotVisual,
};
use inventory_core_api::{InventoryCell, InventorySectionLayout};
use inventory_events_api::{InventoryClientRenderSet, InventorySlotVisualCreated};
use inventory_events_mod::InventoryEventsMod;
use item_manager_api::ItemManagerApi;
use std::marker::PhantomData;
use tokio::task::JoinHandle;

const SLOT_SIZE: f32 = 58.0;

pub struct ClientInventoryUiBevyMod<I>(PhantomData<I>);

impl<I: ItemManagerApi> ClientInventoryUiBevyMod<I> {
    pub fn init<G: GameStateApi, C: ClientInventoryCacheApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _cache: &mut C,
        _events: &mut InventoryEventsMod,
        _items: &mut I,
    ) -> Self {
        bevy.app
            .init_resource::<RenderedInventoryRevision>()
            .add_systems(
                OnEnter(InGameOverlayState::Inventory),
                mark_inventory_ui_dirty,
            )
            .add_systems(
                Update,
                rebuild_inventory::<I>
                    .run_if(in_state(InGameOverlayState::Inventory))
                    .in_set(InventoryClientRenderSet::Layout),
            );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl<I: ItemManagerApi> ClientInventoryUiApi for ClientInventoryUiBevyMod<I> {}

#[derive(Component)]
struct InventoryUiRoot;

#[derive(Resource)]
struct RenderedInventoryRevision(u64);

impl Default for RenderedInventoryRevision {
    fn default() -> Self {
        Self(u64::MAX)
    }
}

fn mark_inventory_ui_dirty(mut revision: ResMut<RenderedInventoryRevision>) {
    revision.0 = u64::MAX;
}

fn rebuild_inventory<I: ItemManagerApi>(
    mut commands: Commands,
    cache: Res<ClientInventoryCache>,
    mut rendered: ResMut<RenderedInventoryRevision>,
    roots: Query<Entity, With<InventoryUiRoot>>,
    mut visuals: MessageWriter<InventorySlotVisualCreated>,
) {
    if rendered.0 == cache.content_revision {
        return;
    }
    for root in &roots {
        commands.entity(root).despawn();
    }
    rendered.0 = cache.content_revision;
    let root = commands
        .spawn((
            InventoryUiRoot,
            DespawnOnExit(InGameOverlayState::Inventory),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.42)),
            GlobalZIndex(120),
        ))
        .id();
    commands.entity(root).with_children(|root| {
        root.spawn((
            Node {
                max_width: px(760),
                padding: UiRect::all(px(24)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(18),
                ..default()
            },
            BackgroundColor(Color::srgba(0.07, 0.08, 0.11, 0.96)),
            BorderRadius::all(px(12)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("Inventory"),
                TextFont {
                    font_size: 34.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            let Some(inventory) = cache.inventory.as_ref() else {
                panel.spawn((
                    Text::new("Waiting for the server..."),
                    TextColor(Color::srgb(0.75, 0.78, 0.84)),
                    Pickable::IGNORE,
                ));
                return;
            };
            for section in
                inventory.layout.sections.iter().filter(|section| {
                    section.role == inventory_core_api::InventorySectionRole::Storage
                })
            {
                spawn_section::<I>(panel, section, inventory, &mut visuals);
            }
            if let Some(hotbar) = inventory.layout.hotbar() {
                panel.spawn((
                    Node {
                        width: percent(100),
                        height: px(2),
                        margin: UiRect::vertical(px(2)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.30, 0.34, 0.42)),
                ));
                spawn_section::<I>(panel, hotbar, inventory, &mut visuals);
            }
        });
    });
}

fn spawn_section<I: ItemManagerApi>(
    parent: &mut ChildSpawnerCommands,
    section: &InventorySectionLayout,
    inventory: &inventory_core_api::Inventory,
    visuals: &mut MessageWriter<InventorySlotVisualCreated>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|section_root| {
            section_root
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(section.columns as u16, 1.0),
                    column_gap: px(5),
                    row_gap: px(5),
                    ..default()
                })
                .with_children(|grid| {
                    for index in 0..section.cells {
                        let cell = InventoryCell {
                            section: section.id.clone(),
                            index,
                        };
                        spawn_slot::<I>(grid, cell, inventory, visuals);
                    }
                });
        });
}

fn spawn_slot<I: ItemManagerApi>(
    parent: &mut ChildSpawnerCommands,
    cell: InventoryCell,
    inventory: &inventory_core_api::Inventory,
    visuals: &mut MessageWriter<InventorySlotVisualCreated>,
) {
    parent
        .spawn((
            InventorySlotVisual { cell: cell.clone() },
            Node {
                width: px(SLOT_SIZE),
                height: px(SLOT_SIZE),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(px(2)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.13, 0.15, 0.19, 0.98)),
            BorderColor::all(Color::srgb(0.30, 0.34, 0.42)),
            BorderRadius::all(px(5)),
            Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
        ))
        .with_children(|slot| {
            let Some(item) = inventory.get(&cell).cloned() else {
                return;
            };
            let item_entity = slot
                .spawn((
                    InventoryItemVisual { cell },
                    Node {
                        width: percent(100),
                        height: percent(100),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(px(4)),
                        ..default()
                    },
                    BorderRadius::all(px(4)),
                    UiTransform::default(),
                    ZIndex(1),
                    GlobalZIndex(121),
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                ))
                .with_child((
                    InventoryItemNameVisual,
                    Text::new(I::label(item.item)),
                    TextFont {
                        font_size: 13.0,
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
