use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_game_state_api::{
    GameState, GameStateApi, GameStateCommand, InGameOverlayCommand, InGameOverlayState,
};
use tokio::task::JoinHandle;

pub struct GameStateBevyImpl;

impl GameStateBevyImpl {
    pub fn init(bevy: &mut BevyMod, _plugins: &mut ClientBevyDefaultPluginsMod) -> Self {
        bevy.app
            .init_state::<GameState>()
            .add_sub_state::<InGameOverlayState>()
            .add_message::<GameStateCommand>()
            .add_message::<InGameOverlayCommand>()
            .add_systems(Update, (apply_state_commands, apply_overlay_commands));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl GameStateApi for GameStateBevyImpl {}

fn apply_state_commands(
    mut commands: MessageReader<GameStateCommand>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for command in commands.read() {
        next_state.set(command.target());
    }
}

fn apply_overlay_commands(
    mut commands: MessageReader<InGameOverlayCommand>,
    mut next_state: ResMut<NextState<InGameOverlayState>>,
) {
    for command in commands.read() {
        next_state.set(command.target());
    }
}
