use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_session_api::{ClientSession, ClientSessionApi};
use client_ui_font_api::{ClientUiFont, ClientUiFontApi};
use tokio::task::JoinHandle;

#[derive(Component)]
struct SessionRejectionNotice;

pub struct ClientSessionRejectionUiMod;

impl ClientSessionRejectionUiMod {
    pub fn init<S: ClientSessionApi, G: GameStateApi, F: ClientUiFontApi>(
        bevy: &mut BevyMod,
        _session: &mut S,
        _game_state: &mut G,
        _font: &mut F,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            show_rejection_reason.run_if(in_state(GameState::MainMenu)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn show_rejection_reason(
    mut commands: Commands,
    mut session: ResMut<ClientSession>,
    font: Option<Res<ClientUiFont>>,
    existing: Query<(), With<SessionRejectionNotice>>,
) {
    if !existing.is_empty() {
        return;
    }
    let Some(font) = font else {
        return;
    };
    let Some(reason) = session.rejection_reason.take() else {
        return;
    };
    commands.spawn((
        SessionRejectionNotice,
        DespawnOnExit(GameState::MainMenu),
        Text::new(reason),
        TextFont {
            font: font.0.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.36, 0.34)),
        Node {
            position_type: PositionType::Absolute,
            left: percent(20),
            right: percent(20),
            bottom: px(28),
            justify_content: JustifyContent::Center,
            ..default()
        },
        GlobalZIndex(180),
    ));
}
