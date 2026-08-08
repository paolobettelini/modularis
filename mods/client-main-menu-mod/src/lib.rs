use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi, GameStateCommand};
use client_menu_api::{
    MenuApi, MenuBackground, MenuButtonAction, MenuRegistryHandle, MenuScreen, MenuTarget,
    MenuValueChanged, MenuWidget,
};
use client_network_api::{ClientConnectionTarget, ClientNetworkApi};
use tokio::task::JoinHandle;

const SERVER_ADDRESS_ACTION: &str = "main-menu.server-address";

pub struct MainMenuMod;

impl MainMenuMod {
    pub fn init<M: MenuApi, G: GameStateApi, N: ClientNetworkApi>(
        bevy: &mut BevyMod,
        menu: &mut M,
        _game_state: &mut G,
        _network: &mut N,
    ) -> Self {
        let server_address = bevy
            .app
            .world()
            .resource::<ClientConnectionTarget>()
            .address()
            .to_owned();
        menu.register_screen(MenuScreen {
            id: "main-menu",
            title: "Minecraft",
            target: MenuTarget::Game(GameState::MainMenu),
            background: MenuBackground::Opaque,
            widgets: vec![
                MenuWidget::Label {
                    text: "Minecraft demo with patchwork modding system".to_string(),
                },
                MenuWidget::TextboxButton {
                    id: "server-address".to_string(),
                    label: "Server address".to_string(),
                    value: server_address,
                    action: SERVER_ADDRESS_ACTION.to_string(),
                    button_id: "play",
                    button_label: "Play".to_string(),
                    button_action: MenuButtonAction::ChangeGameState(GameStateCommand::StartGame),
                },
                MenuWidget::Button {
                    id: "settings",
                    label: "Settings".to_string(),
                    action: MenuButtonAction::ChangeGameState(GameStateCommand::OpenSettings),
                },
            ],
        });
        bevy.app.add_systems(
            Update,
            update_server_address.run_if(in_state(GameState::MainMenu)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn update_server_address(
    mut changed: MessageReader<MenuValueChanged>,
    mut target: ResMut<ClientConnectionTarget>,
    registry: Res<MenuRegistryHandle>,
) {
    for changed in changed.read() {
        if changed.action != SERVER_ADDRESS_ACTION {
            continue;
        }
        target.set_address(changed.value.clone());
        registry.update_input_value(SERVER_ADDRESS_ACTION, &changed.value);
    }
}
