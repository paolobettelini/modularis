use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use tokio::task::JoinHandle;

pub struct ClientCrosshairBevyMod;

impl ClientCrosshairBevyMod {
    pub fn init<G: GameStateApi>(bevy: &mut BevyMod, _game_state: &mut G) -> Self {
        bevy.app
            .add_systems(OnEnter(GameState::InGame), spawn_crosshair);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn spawn_crosshair(mut commands: Commands) {
    commands.spawn((
        Text::new("+"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50),
            top: percent(50),
            ..default()
        },
        UiTransform::from_translation(Val2::percent(-50.0, -50.0)),
        Pickable::IGNORE,
        DespawnOnExit(GameState::InGame),
    ));
}
