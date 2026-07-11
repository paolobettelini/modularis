use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{CellMenuClientRenderSet, CellMenuState};
use cell_menu_events_mod::CellMenuEventsMod;
use client_cell_menu_cache_api::{ClientCellMenuCache, ClientCellMenuCacheApi};
use client_cell_menu_ui_api::{CellMenuItemVisual, CellMenuSlotVisual, ClientCellMenuUiApi};
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_inventory_ui_api::InventoryItemNameVisual;
use inventory_core_api::{InventoryCell, InventorySectionLayout};
use inventory_events_api::InventorySlotVisualCreated;
use inventory_events_mod::InventoryEventsMod;
use item_manager_api::ItemManagerApi;
use std::marker::PhantomData;
use tokio::task::JoinHandle;

const SLOT_SIZE: f32 = 58.0;

pub struct ClientCellMenuUiBevyMod<I>(PhantomData<I>);

impl<I: ItemManagerApi> ClientCellMenuUiBevyMod<I> {
    pub fn init<G: GameStateApi, C: ClientCellMenuCacheApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _cache: &mut C,
        _game_state: &mut G,
        _inventory_events: &mut InventoryEventsMod,
        _items: &mut I,
    ) -> Self {
        bevy.app
            .init_resource::<RenderedCellMenuRevision>()
            .add_systems(OnEnter(InGameOverlayState::Inventory), mark_cell_menu_dirty)
            .add_systems(OnExit(InGameOverlayState::Inventory), mark_cell_menu_dirty)
            .add_systems(
                Update,
                rebuild_cell_menu::<I>
                    .run_if(in_state(InGameOverlayState::Inventory))
                    .in_set(CellMenuClientRenderSet::Layout),
            );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl<I: ItemManagerApi> ClientCellMenuUiApi for ClientCellMenuUiBevyMod<I> {}

#[derive(Component)]
struct CellMenuUiRoot;

#[derive(Resource)]
struct RenderedCellMenuRevision(u64);

impl Default for RenderedCellMenuRevision {
    fn default() -> Self {
        Self(u64::MAX)
    }
}

fn mark_cell_menu_dirty(mut revision: ResMut<RenderedCellMenuRevision>) {
    revision.0 = u64::MAX;
}

fn rebuild_cell_menu<I: ItemManagerApi>(
    mut commands: Commands,
    cache: Res<ClientCellMenuCache>,
    mut rendered: ResMut<RenderedCellMenuRevision>,
    roots: Query<Entity, With<CellMenuUiRoot>>,
    mut visuals: MessageWriter<InventorySlotVisualCreated>,
) {
    if rendered.0 == cache.content_revision {
        return;
    }
    for root in &roots {
        commands.entity(root).despawn();
    }
    rendered.0 = cache.content_revision;
    let Some(menu) = cache
        .active
        .as_ref()
        .and_then(|id| cache.menus.get(id))
        .cloned()
    else {
        return;
    };
    spawn_menu::<I>(&mut commands, &menu, &mut visuals);
}

fn spawn_menu<I: ItemManagerApi>(
    commands: &mut Commands,
    menu: &CellMenuState,
    visuals: &mut MessageWriter<InventorySlotVisualCreated>,
) {
    commands
        .spawn((
            CellMenuUiRoot,
            DespawnOnExit(InGameOverlayState::Inventory),
            Node {
                position_type: PositionType::Absolute,
                right: px(36),
                top: px(96),
                padding: UiRect::all(px(18)),
                flex_direction: FlexDirection::Column,
                row_gap: px(14),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.07, 0.08, 0.11, 0.96)),
            BorderColor::all(Color::srgb(0.35, 0.39, 0.48)),
            BorderRadius::all(px(12)),
            GlobalZIndex(130),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new(menu.title.clone()),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            for section in &menu.inventory.layout.sections {
                spawn_section::<I>(panel, menu, section, visuals);
            }
        });
}

fn spawn_section<I: ItemManagerApi>(
    parent: &mut ChildSpawnerCommands,
    menu: &CellMenuState,
    section: &InventorySectionLayout,
    visuals: &mut MessageWriter<InventorySlotVisualCreated>,
) {
    parent
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
                spawn_slot::<I>(grid, menu, cell, visuals);
            }
        });
}

fn spawn_slot<I: ItemManagerApi>(
    parent: &mut ChildSpawnerCommands,
    menu: &CellMenuState,
    cell: InventoryCell,
    visuals: &mut MessageWriter<InventorySlotVisualCreated>,
) {
    parent
        .spawn((
            CellMenuSlotVisual {
                menu_id: menu.id.clone(),
                cell: cell.clone(),
            },
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
            let Some(item) = menu.inventory.get(&cell).cloned() else {
                return;
            };
            let item_entity = slot
                .spawn((
                    CellMenuItemVisual {
                        menu_id: menu.id.clone(),
                        cell,
                    },
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
                    GlobalZIndex(131),
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
