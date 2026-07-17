use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi, GameStateCommand};
use client_session_api::{ClientSession, ClientSessionApi};
use client_ui_font_api::{ClientUiFont, ClientUiFontApi};
use tokio::task::JoinHandle;

const NORMAL_BUTTON: Color = Color::srgb(0.16, 0.18, 0.22);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.32, 0.42);
const PRESSED_BUTTON: Color = Color::srgb(0.10, 0.55, 0.34);

#[derive(Component)]
struct BackToHomeButton;

pub struct ClientDisconnectMenuMod;

impl ClientDisconnectMenuMod {
    pub fn init<S: ClientSessionApi, G: GameStateApi, F: ClientUiFontApi>(
        bevy: &mut BevyMod,
        _session: &mut S,
        _game_state: &mut G,
        _font: &mut F,
    ) -> Self {
        bevy.app
            .add_systems(OnEnter(GameState::Disconnected), spawn_disconnect_menu)
            .add_systems(
                Update,
                handle_back_to_home.run_if(in_state(GameState::Disconnected)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn spawn_disconnect_menu(
    mut commands: Commands,
    session: Res<ClientSession>,
    font: Option<Res<ClientUiFont>>,
) {
    let reason = session
        .disconnect_reason
        .clone()
        .unwrap_or_else(|| "Disconnected from the server".to_string());
    let font = font.map(|font| font.0.clone()).unwrap_or_default();
    commands
        .spawn((
            DespawnOnExit(GameState::Disconnected),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.035, 0.045, 0.065)),
            GlobalZIndex(180),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: px(640),
                    padding: UiRect::all(px(36)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    row_gap: px(24),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.08, 0.10, 0.14, 0.98)),
                BorderRadius::all(px(12)),
            ))
            .with_children(|column| {
                column.spawn((
                    Text::new("Disconnected"),
                    TextFont {
                        font: font.clone(),
                        font_size: 42.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Justify::Center),
                ));
                column.spawn((
                    Text::new(reason),
                    TextFont {
                        font: font.clone(),
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.48, 0.44)),
                    TextLayout::new_with_justify(Justify::Center),
                    Node {
                        width: percent(100),
                        min_height: px(72),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                ));
                column
                    .spawn((
                        Button,
                        BackToHomeButton,
                        Node {
                            width: percent(100),
                            height: px(54),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(NORMAL_BUTTON),
                        BorderRadius::all(px(7)),
                    ))
                    .with_child((
                        Text::new("Back to home"),
                        TextFont {
                            font,
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
            });
        });
}

fn handle_back_to_home(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackToHomeButton>),
    >,
    mut session: ResMut<ClientSession>,
    mut state: MessageWriter<GameStateCommand>,
) {
    for (interaction, mut background) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                *background = PRESSED_BUTTON.into();
                session.disconnect_reason = None;
                state.write(GameStateCommand::BackToMainMenu);
            }
            Interaction::Hovered => *background = HOVERED_BUTTON.into(),
            Interaction::None => *background = NORMAL_BUTTON.into(),
        }
    }
}
