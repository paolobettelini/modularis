use client_game_state_api::{GameState, GameStateApi, GameStateCommand};
use client_menu_api::{
    MenuApi, MenuBackground, MenuButtonAction, MenuScreen, MenuTarget, MenuWidget,
};
use tokio::task::JoinHandle;

pub struct MainMenuMod;

impl MainMenuMod {
    pub fn init<M: MenuApi, G: GameStateApi>(menu: &mut M, _game_state: &mut G) -> Self {
        menu.register_screen(MenuScreen {
            id: "main-menu",
            title: "Minecraft",
            target: MenuTarget::Game(GameState::MainMenu),
            background: MenuBackground::Opaque,
            widgets: vec![
                MenuWidget::Label {
                    text: "Minecraft demo with patchwork modding system".to_string(),
                },
                MenuWidget::Button {
                    id: "play",
                    label: "Play".to_string(),
                    action: MenuButtonAction::ChangeGameState(GameStateCommand::StartGame),
                },
                MenuWidget::Button {
                    id: "settings",
                    label: "Settings".to_string(),
                    action: MenuButtonAction::ChangeGameState(GameStateCommand::OpenSettings),
                },
            ],
        });
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
