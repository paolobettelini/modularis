use bevy::{
    input::keyboard::{Key, KeyboardInput},
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui::RelativeCursorPosition,
};
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_inventory_ui_api::{
    ClientInventoryUiApi, ClientInventoryUiSet, InventoryItemNameVisual,
    InventoryUiInputCapture, InventoryUiRoot, ItemCatalogVisual,
};
use client_ui_font_api::{ClientUiFont, ClientUiFontApi};
use inventory_events_api::{
    InventoryClientRenderSet, InventorySlotVisualCreated,
};
use inventory_events_mod::InventoryEventsMod;
use item_instance_api::ItemInstance;
use item_manager_api::ItemManagerApi;
use std::marker::PhantomData;
use tokio::task::JoinHandle;

const CATALOG_WIDTH: f32 = 356.0;
const CATALOG_SCROLL_LINE_PIXELS: f32 = 48.0;
const CATALOG_SCROLL_PIXEL_MULTIPLIER: f32 = 1.35;
const CATALOG_SCROLLBAR_MIN_THUMB: f32 = 28.0;

#[derive(Resource)]
struct ItemCatalogState {
    expanded: bool,
    focused: bool,
    query: String,
    revision: u64,
    rendered_revision: u64,
    rendered_root: Option<Entity>,
    scroll_y: f32,
}

impl Default for ItemCatalogState {
    fn default() -> Self {
        Self {
            expanded: true,
            focused: false,
            query: String::new(),
            revision: 0,
            rendered_revision: u64::MAX,
            rendered_root: None,
            scroll_y: 0.0,
        }
    }
}

#[derive(Component)]
struct ItemCatalogPanel;

#[derive(Component)]
struct ItemCatalogToggle;

#[derive(Component)]
struct ItemCatalogSearch;

#[derive(Component)]
struct ItemCatalogScroll;

#[derive(Component)]
struct ItemCatalogScrollRegion;

#[derive(Component)]
struct ItemCatalogScrollbarTrack;

#[derive(Component, Clone, Copy)]
struct ItemCatalogScrollbarThumb {
    target: Entity,
}

pub struct ClientItemCatalogUiMod<I>(PhantomData<I>);

impl<I: ItemManagerApi> ClientItemCatalogUiMod<I> {
    pub fn init<U: ClientInventoryUiApi, G: GameStateApi, F: ClientUiFontApi>(
        bevy: &mut BevyMod,
        _inventory_ui: &mut U,
        _game_state: &mut G,
        _font: &mut F,
        _events: &mut InventoryEventsMod,
        _items: &mut I,
    ) -> Self {
        bevy.app
            .init_resource::<ItemCatalogState>()
            .add_observer(drag_catalog_scrollbar)
            .add_systems(OnEnter(InGameOverlayState::Inventory), open_catalog)
            .add_systems(OnExit(InGameOverlayState::Inventory), close_catalog)
            .add_systems(
                Update,
                (
                    handle_catalog_buttons,
                    handle_catalog_keyboard,
                    scroll_catalog,
                    remember_catalog_scroll,
                    update_catalog_scrollbar,
                    rebuild_catalog::<I>
                        .in_set(InventoryClientRenderSet::Layout)
                        .in_set(ClientInventoryUiSet::Extensions),
                )
                    .chain()
                    .run_if(in_state(InGameOverlayState::Inventory)),
            );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn open_catalog(
    mut state: ResMut<ItemCatalogState>,
    mut capture: ResMut<InventoryUiInputCapture>,
) {
    state.focused = false;
    state.rendered_root = None;
    state.rendered_revision = u64::MAX;
    capture.0 = false;
}

fn close_catalog(mut capture: ResMut<InventoryUiInputCapture>) {
    capture.0 = false;
}

fn handle_catalog_buttons(
    toggles: Query<&Interaction, (Changed<Interaction>, With<ItemCatalogToggle>)>,
    searches: Query<&Interaction, (Changed<Interaction>, With<ItemCatalogSearch>)>,
    mut state: ResMut<ItemCatalogState>,
    mut capture: ResMut<InventoryUiInputCapture>,
) {
    if toggles.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.expanded = !state.expanded;
        state.focused = false;
        state.revision = state.revision.wrapping_add(1);
        capture.0 = false;
    }
    if searches
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        state.focused = true;
        state.revision = state.revision.wrapping_add(1);
        capture.0 = true;
    }
}

fn handle_catalog_keyboard(
    mut keyboard: MessageReader<KeyboardInput>,
    mut state: ResMut<ItemCatalogState>,
    mut capture: ResMut<InventoryUiInputCapture>,
    mut catalogs: Query<&mut ScrollPosition, With<ItemCatalogScroll>>,
) {
    if !state.focused {
        return;
    }
    let mut changed = false;
    let mut query_changed = false;
    for event in keyboard.read() {
        if !event.state.is_pressed() {
            continue;
        }
        match (&event.logical_key, &event.text) {
            (Key::Enter | Key::Escape, _) => {
                state.focused = false;
                capture.0 = false;
                changed = true;
            }
            (Key::Backspace, _) => {
                query_changed = state.query.pop().is_some();
                changed |= query_changed;
            }
            (_, Some(inserted)) if inserted.chars().all(|chr| !chr.is_ascii_control()) => {
                state.query.push_str(inserted);
                changed = true;
                query_changed = true;
            }
            _ => {}
        }
    }
    if changed {
        state.revision = state.revision.wrapping_add(1);
    }
    if query_changed {
        state.scroll_y = 0.0;
        for mut scroll in &mut catalogs {
            scroll.y = 0.0;
        }
    }
}

fn rebuild_catalog<I: ItemManagerApi>(
    mut commands: Commands,
    roots: Query<Entity, With<InventoryUiRoot>>,
    panels: Query<Entity, With<ItemCatalogPanel>>,
    font: Res<ClientUiFont>,
    mut state: ResMut<ItemCatalogState>,
    mut visuals: MessageWriter<InventorySlotVisualCreated>,
) {
    let Ok(root) = roots.single() else {
        return;
    };
    if state.rendered_root == Some(root) && state.rendered_revision == state.revision {
        return;
    }
    for panel in &panels {
        commands.entity(panel).despawn();
    }
    let panel = spawn_catalog_panel::<I>(&mut commands, &state, &font, &mut visuals);
    commands.entity(root).add_child(panel);
    state.rendered_root = Some(root);
    state.rendered_revision = state.revision;
}

fn spawn_catalog_panel<I: ItemManagerApi>(
    commands: &mut Commands,
    state: &ItemCatalogState,
    font: &ClientUiFont,
    visuals: &mut MessageWriter<InventorySlotVisualCreated>,
) -> Entity {
    let panel = commands
        .spawn((
            ItemCatalogPanel,
            Node {
                width: px(if state.expanded { CATALOG_WIDTH } else { 52.0 }),
                max_height: percent(88),
                padding: UiRect::all(px(if state.expanded { 14 } else { 6 })),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                row_gap: px(10),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.07, 0.08, 0.11, 0.96)),
            BorderRadius::all(px(12)),
        ))
        .id();
    commands.entity(panel).with_children(|panel| {
        panel
            .spawn((
                Button,
                ItemCatalogToggle,
                Node {
                    width: percent(100),
                    height: px(40),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.16, 0.18, 0.22)),
                BorderRadius::all(px(6)),
            ))
            .with_child((
                Text::new(if state.expanded { "Items  ‹" } else { "›" }),
                TextFont { font: font.0.clone(), font_size: 18.0, ..default() },
                TextColor(Color::WHITE),
            ));
        if !state.expanded {
            return;
        }
        panel
            .spawn((
                Button,
                ItemCatalogSearch,
                Node {
                    width: percent(100),
                    height: px(42),
                    padding: UiRect::horizontal(px(10)),
                    align_items: AlignItems::Center,
                    border: UiRect::all(px(1)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.11, 0.13, 0.17)),
                BorderColor::all(if state.focused {
                    Color::srgb(0.32, 0.72, 0.52)
                } else {
                    Color::srgb(0.30, 0.34, 0.42)
                }),
                BorderRadius::all(px(6)),
            ))
            .with_child((
                Text::new(if state.query.is_empty() {
                    "Search items…".to_string()
                } else {
                    format!("{}{}", state.query, if state.focused { "_" } else { "" })
                }),
                TextFont { font: font.0.clone(), font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.82, 0.84, 0.90)),
            ));
        panel
            .spawn((
                Node {
                    width: percent(100),
                    height: px(590),
                    max_height: px(590),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: px(7),
                    ..default()
                },
                ItemCatalogScrollRegion,
                RelativeCursorPosition::default(),
            ))
            .with_children(|region| {
                let scroll_entity = region
                    .spawn((
                        ItemCatalogScroll,
                        ScrollPosition(Vec2::new(0.0, state.scroll_y)),
                        Node {
                            flex_grow: 1.0,
                            height: percent(100),
                            overflow: Overflow::scroll_y(),
                            ..default()
                        },
                    ))
                    .with_children(|scroll| {
                        scroll
                            .spawn(Node {
                                width: percent(100),
                                display: Display::Grid,
                                grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                                column_gap: px(6),
                                row_gap: px(6),
                                ..default()
                            })
                            .with_children(|grid| {
                                let query = state.query.to_lowercase();
                                for item in I::all().iter().copied().filter(|item| {
                                    query.is_empty()
                                        || I::id(*item).to_lowercase().contains(&query)
                                        || I::label(*item).to_lowercase().contains(&query)
                                }) {
                                    spawn_catalog_item::<I>(grid, item, font, visuals);
                                }
                            });
                    })
                    .id();
                region
                    .spawn((
                        ItemCatalogScrollbarTrack,
                        Node {
                            width: px(10),
                            height: percent(100),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.12, 0.14, 0.18, 0.96)),
                        BorderRadius::all(px(5)),
                        Pickable {
                            should_block_lower: false,
                            is_hoverable: true,
                        },
                    ))
                    .with_child((
                        ItemCatalogScrollbarThumb { target: scroll_entity },
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(1),
                            right: px(1),
                            top: px(0),
                            height: px(CATALOG_SCROLLBAR_MIN_THUMB),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.38, 0.43, 0.52)),
                        BorderRadius::all(px(4)),
                        Pickable {
                            should_block_lower: true,
                            is_hoverable: true,
                        },
                    ));
            });
    });
    panel
}

fn spawn_catalog_item<I: ItemManagerApi>(
    parent: &mut ChildSpawnerCommands,
    item: item_manager_api::ItemId,
    font: &ClientUiFont,
    visuals: &mut MessageWriter<InventorySlotVisualCreated>,
) {
    parent
        .spawn((
            Node {
                min_height: px(92),
                padding: UiRect::all(px(4)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            BackgroundColor(Color::srgba(0.13, 0.15, 0.19, 0.98)),
            BorderRadius::all(px(5)),
        ))
        .with_children(|card| {
            let item_entity = card
                .spawn((
                    ItemCatalogVisual { item },
                    Node {
                        width: px(56),
                        height: px(56),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
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
                    Text::new(I::label(item)),
                    Visibility::Hidden,
                ))
                .id();
            card.spawn((
                Text::new(I::label(item)),
                TextFont { font: font.0.clone(), font_size: 10.0, ..default() },
                TextColor(Color::srgb(0.86, 0.88, 0.94)),
                TextLayout::new_with_justify(Justify::Center),
                Pickable::IGNORE,
            ));
            visuals.write(InventorySlotVisualCreated {
                entity: item_entity,
                item: ItemInstance::new(item),
            });
        });
}

fn scroll_catalog(
    mut wheel: MessageReader<MouseWheel>,
    mut catalogs: Query<(&mut ScrollPosition, &ComputedNode), With<ItemCatalogScroll>>,
    regions: Query<&RelativeCursorPosition, With<ItemCatalogScrollRegion>>,
    mut state: ResMut<ItemCatalogState>,
) {
    let cursor_inside = regions.single().is_ok_and(RelativeCursorPosition::cursor_over);
    let Ok((mut scroll, computed)) = catalogs.single_mut() else {
        return;
    };
    let range =
        (computed.content_size().y - computed.size().y).max(0.0) * computed.inverse_scale_factor();
    for event in wheel.read() {
        if !cursor_inside {
            continue;
        }
        let delta = match event.unit {
            MouseScrollUnit::Line => event.y * CATALOG_SCROLL_LINE_PIXELS,
            MouseScrollUnit::Pixel => event.y * CATALOG_SCROLL_PIXEL_MULTIPLIER,
        };
        scroll.y = (scroll.y - delta).clamp(0.0, range);
    }
    state.scroll_y = scroll.y;
}

fn remember_catalog_scroll(
    catalogs: Query<&ScrollPosition, With<ItemCatalogScroll>>,
    mut state: ResMut<ItemCatalogState>,
) {
    if let Ok(scroll) = catalogs.single() {
        state.scroll_y = scroll.y;
    }
}

fn drag_catalog_scrollbar(
    drag: On<Pointer<Drag>>,
    thumbs: Query<&ItemCatalogScrollbarThumb>,
    mut catalogs: Query<(&mut ScrollPosition, &ComputedNode), With<ItemCatalogScroll>>,
    mut state: ResMut<ItemCatalogState>,
) {
    let Ok(thumb) = thumbs.get(drag.event_target()) else {
        return;
    };
    let Ok((mut scroll, computed)) = catalogs.get_mut(thumb.target) else {
        return;
    };
    let visible = computed.size().y;
    let content = computed.content_size().y.max(visible);
    let range = (content - visible).max(0.0) * computed.inverse_scale_factor();
    if range <= 0.0 || visible <= 0.0 {
        return;
    }
    let content_per_viewport = content / visible;
    scroll.y = (scroll.y
        + drag.delta.y * computed.inverse_scale_factor() * content_per_viewport)
        .clamp(0.0, range);
    state.scroll_y = scroll.y;
}

fn update_catalog_scrollbar(
    catalogs: Query<(&ScrollPosition, &ComputedNode), With<ItemCatalogScroll>>,
    tracks: Query<&ComputedNode, With<ItemCatalogScrollbarTrack>>,
    mut thumbs: Query<
        (&ItemCatalogScrollbarThumb, &ChildOf, &mut Node, &mut Visibility),
        Without<ItemCatalogScroll>,
    >,
) {
    for (thumb, parent, mut node, mut visibility) in &mut thumbs {
        let Ok((scroll, computed)) = catalogs.get(thumb.target) else {
            continue;
        };
        let Ok(track) = tracks.get(parent.parent()) else {
            continue;
        };
        let visible = computed.size().y * computed.inverse_scale_factor();
        let content = computed.content_size().y * computed.inverse_scale_factor();
        let range = (content - visible).max(0.0);
        let track_height = track.size().y * track.inverse_scale_factor();
        if range <= 0.0 || content <= 0.0 || track_height <= 0.0 {
            *visibility = Visibility::Hidden;
            continue;
        }
        *visibility = Visibility::Inherited;
        let thumb_height = (track_height * visible / content)
            .clamp(CATALOG_SCROLLBAR_MIN_THUMB, track_height);
        let travel = (track_height - thumb_height).max(0.0);
        node.height = px(thumb_height);
        node.top = px((scroll.y / range).clamp(0.0, 1.0) * travel);
    }
}
