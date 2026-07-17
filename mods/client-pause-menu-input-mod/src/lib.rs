use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi, InGameOverlayCommand, InGameOverlayState};
use tokio::task::JoinHandle;

pub struct ClientPauseMenuInputMod;

impl ClientPauseMenuInputMod {
    pub fn init<G: GameStateApi>(bevy: &mut BevyMod, _game_state: &mut G) -> Self {
        bevy.app
            .add_systems(Update, escape_to_pause.run_if(in_state(GameState::InGame)));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn escape_to_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    overlay: Res<State<InGameOverlayState>>,
    mut commands: MessageWriter<InGameOverlayCommand>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    commands.write(match overlay.get() {
        InGameOverlayState::Playing => InGameOverlayCommand::Pause,
        InGameOverlayState::PauseMenu => InGameOverlayCommand::Resume,
        InGameOverlayState::Settings => InGameOverlayCommand::BackToPause,
        InGameOverlayState::Inventory => InGameOverlayCommand::Resume,
        InGameOverlayState::Chat => InGameOverlayCommand::Resume,
    });
}
