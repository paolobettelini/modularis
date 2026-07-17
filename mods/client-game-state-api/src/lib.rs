use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    SettingsMenu,
    Disconnected,
    InGame,
}

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameState = GameState::InGame)]
pub enum InGameOverlayState {
    #[default]
    Playing,
    PauseMenu,
    Settings,
    Inventory,
    Chat,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStateCommand {
    BackToMainMenu,
    ShowDisconnect,
    OpenSettings,
    StartGame,
}

impl GameStateCommand {
    pub const fn target(self) -> GameState {
        match self {
            Self::BackToMainMenu => GameState::MainMenu,
            Self::ShowDisconnect => GameState::Disconnected,
            Self::OpenSettings => GameState::SettingsMenu,
            Self::StartGame => GameState::InGame,
        }
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InGameOverlayCommand {
    Pause,
    Resume,
    OpenSettings,
    BackToPause,
    OpenInventory,
    OpenChat,
}

impl InGameOverlayCommand {
    pub const fn target(self) -> InGameOverlayState {
        match self {
            Self::Pause | Self::BackToPause => InGameOverlayState::PauseMenu,
            Self::Resume => InGameOverlayState::Playing,
            Self::OpenSettings => InGameOverlayState::Settings,
            Self::OpenInventory => InGameOverlayState::Inventory,
            Self::OpenChat => InGameOverlayState::Chat,
        }
    }
}

pub trait GameStateApi: Send + Sync + 'static {}
