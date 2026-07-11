use client_game_state_api::{GameStateApi, InGameOverlayCommand, InGameOverlayState};
use client_menu_api::{
    MenuApi, MenuBackground, MenuButtonAction, MenuScreen, MenuTarget, MenuWidget,
};
use tokio::task::JoinHandle;

pub struct ClientPauseMenuMod;

impl ClientPauseMenuMod {
    pub fn init<M: MenuApi, G: GameStateApi>(menu: &mut M, _game_state: &mut G) -> Self {
        menu.register_screen(MenuScreen {
            id: "pause-menu",
            title: "Paused",
            target: MenuTarget::InGameOverlay(InGameOverlayState::PauseMenu),
            background: MenuBackground::Transparent,
            widgets: vec![
                MenuWidget::Button {
                    id: "resume",
                    label: "Resume".to_string(),
                    action: MenuButtonAction::ChangeInGameOverlay(InGameOverlayCommand::Resume),
                },
                MenuWidget::Button {
                    id: "settings",
                    label: "Settings".to_string(),
                    action: MenuButtonAction::ChangeInGameOverlay(
                        InGameOverlayCommand::OpenSettings,
                    ),
                },
            ],
        });
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
