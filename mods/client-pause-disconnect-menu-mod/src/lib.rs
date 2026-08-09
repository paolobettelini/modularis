use client_game_state_api::{GameStateApi, GameStateCommand};
use client_menu_api::{MenuApi, MenuButtonAction, MenuWidget};
use client_pause_menu_mod::ClientPauseMenuMod;
use tokio::task::JoinHandle;

/// Optional vanilla glue that adds server disconnection to the pause screen.
///
/// Leaving `client-pause-disconnect-menu-mod` out preserves the same pause
/// screen and session implementation without exposing this policy action.
pub struct ClientPauseDisconnectMenuMod;

impl ClientPauseDisconnectMenuMod {
    pub fn init<M: MenuApi, G: GameStateApi>(
        menu: &mut M,
        _game_state: &mut G,
        _pause_menu: &mut ClientPauseMenuMod,
    ) -> Self {
        assert!(
            menu.append_widget(
                "pause-menu",
                MenuWidget::Button {
                    id: "disconnect",
                    label: "Disconnect".to_string(),
                    action: MenuButtonAction::ChangeGameState(GameStateCommand::BackToMainMenu),
                },
            ),
            "the pause menu must be registered before its disconnect extension"
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
