use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_game_state_api::{GameState, GameStateApi};
use client_loading_api::{ClientLoadingApi, ClientLoadingSet, ClientLoadingTasks};
use client_ui_font_api::{ClientUiFont, ClientUiFontApi};
use tokio::task::JoinHandle;

#[derive(Component)]
struct LoadingScreenRoot;

pub struct ClientLoadingUiBevyMod;

impl ClientLoadingUiBevyMod {
    pub fn init<L: ClientLoadingApi, G: GameStateApi, F: ClientUiFontApi>(
        bevy: &mut BevyMod,
        _plugins: &mut ClientBevyDefaultPluginsMod,
        _loading: &mut L,
        _game: &mut G,
        _font: &mut F,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            render_loading_screen
                .run_if(in_state(GameState::InGame))
                .in_set(ClientLoadingSet::Render),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn render_loading_screen(
    mut commands: Commands,
    tasks: Res<ClientLoadingTasks>,
    roots: Query<Entity, With<LoadingScreenRoot>>,
    font: Res<ClientUiFont>,
) {
    if !tasks.is_changed() { return; }
    for root in &roots { commands.entity(root).despawn(); }
    if tasks.is_empty() { return; }

    let mut visible = tasks.tasks().map(|(key, task)| (key.clone(), task.clone())).collect::<Vec<_>>();
    visible.sort_by(|(left_key, left), (right_key, right)| {
        left.label.cmp(&right.label).then_with(|| left_key.id.cmp(&right_key.id))
    });
    let background = if tasks.has_blocking_tasks() {
        Color::srgba(0.015, 0.02, 0.03, 0.94)
    } else {
        Color::srgba(0.015, 0.02, 0.03, 0.72)
    };

    commands.spawn((
        LoadingScreenRoot,
        DespawnOnExit(GameState::InGame),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(background),
        GlobalZIndex(200),
    )).with_children(|root| {
        root.spawn((
            Node {
                width: px(620),
                max_width: percent(88),
                padding: UiRect::all(px(24)),
                flex_direction: FlexDirection::Column,
                row_gap: px(18),
                ..default()
            },
            BackgroundColor(Color::srgba(0.055, 0.065, 0.085, 0.98)),
            BorderRadius::all(px(12)),
        )).with_children(|panel| {
            panel.spawn((
                Text::new("Loading"),
                TextFont { font: font.0.clone(), font_size: 30.0, ..default() },
                TextColor(Color::WHITE),
            ));
            for (_, task) in visible {
                panel.spawn((Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(7),
                    ..default()
                },)).with_children(|row| {
                    row.spawn((
                        Text::new(format!("{}  {:>3}%", task.label, (task.progress * 100.0).round() as u32)),
                        TextFont { font: font.0.clone(), font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.92, 0.94, 1.0)),
                    ));
                    row.spawn((
                        Node { width: percent(100), height: px(12), padding: UiRect::all(px(2)), ..default() },
                        BackgroundColor(Color::srgb(0.12, 0.14, 0.18)),
                        BorderRadius::all(px(6)),
                    )).with_child((
                        Node { width: percent(task.progress * 100.0), height: percent(100), ..default() },
                        BackgroundColor(Color::srgb(0.30, 0.72, 0.50)),
                        BorderRadius::all(px(4)),
                    ));
                    row.spawn((
                        Text::new(task.status),
                        TextFont { font: font.0.clone(), font_size: 15.0, ..default() },
                        TextColor(Color::srgb(0.68, 0.72, 0.80)),
                    ));
                });
            }
        });
    });
}
