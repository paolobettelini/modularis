use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{
    GameState, GameStateApi, GameStateCommand, InGameOverlayCommand, InGameOverlayState,
};
use client_menu_api::{
    MenuApi, MenuBackground, MenuButtonAction, MenuRegistryHandle, MenuScreen, MenuTarget,
    MenuValueChanged, MenuWidget,
};
use client_settings_api::{SettingChanged, SettingsApi, SettingsStore};
use client_settings_input_api::{
    SettingInputContext, SettingInputRegistryHandle, SettingInputStartupSet, SettingsInputApi,
};
use client_settings_registry_codegen::SettingsRegistryCodegenMod;
use generated_client_settings_registry::{all_settings, definition, key_from_id};
use tokio::task::JoinHandle;

pub struct SettingsBevyMenuImpl;

impl SettingsBevyMenuImpl {
    pub fn init<M: MenuApi, G: GameStateApi, I: SettingsInputApi>(
        bevy: &mut BevyMod,
        _menu: &mut M,
        _game_state: &mut G,
        _inputs: &mut I,
        _codegen: &mut SettingsRegistryCodegenMod,
    ) -> Self {
        let store = SettingsStore::default();
        bevy.app
            .insert_resource(store)
            .add_message::<SettingChanged>()
            .add_systems(
                Startup,
                register_settings_menus.in_set(SettingInputStartupSet::BuildMenus),
            )
            .add_systems(Update, apply_input_changes);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl SettingsApi for SettingsBevyMenuImpl {}

fn register_settings_menus(
    store: Res<SettingsStore>,
    inputs: Res<SettingInputRegistryHandle>,
    menu: Res<MenuRegistryHandle>,
) {
    let setting_widgets = all_settings()
        .iter()
        .copied()
        .filter_map(|key| {
            let definition = definition(key);
            let context = SettingInputContext {
                id: definition.id.to_string(),
                label: definition.label.to_string(),
                value: store.get(key).to_string(),
                action: format!("setting:{}", definition.id),
            };
            let widget = inputs.build(definition.input, context);
            if widget.is_none() {
                warn!(
                    "setting '{}' requires missing input provider '{}'",
                    definition.id, definition.input
                );
            }
            widget
        })
        .collect::<Vec<_>>();
    let mut main_widgets = setting_widgets.clone();
    main_widgets.push(MenuWidget::Button {
        id: "back",
        label: "Back".to_string(),
        action: MenuButtonAction::ChangeGameState(GameStateCommand::BackToMainMenu),
    });
    let mut pause_widgets = setting_widgets;
    pause_widgets.push(MenuWidget::Button {
        id: "back-to-pause",
        label: "Back".to_string(),
        action: MenuButtonAction::ChangeInGameOverlay(InGameOverlayCommand::BackToPause),
    });

    menu.register_screen(MenuScreen {
        id: "settings-menu",
        title: "Settings",
        target: MenuTarget::Game(GameState::SettingsMenu),
        background: MenuBackground::Opaque,
        widgets: main_widgets,
    });
    menu.register_screen(MenuScreen {
        id: "pause-settings-menu",
        title: "Settings",
        target: MenuTarget::InGameOverlay(InGameOverlayState::Settings),
        background: MenuBackground::Transparent,
        widgets: pause_widgets,
    });
}

fn apply_input_changes(
    mut changes: MessageReader<MenuValueChanged>,
    mut store: ResMut<SettingsStore>,
    registry: Res<MenuRegistryHandle>,
    mut changed_writer: MessageWriter<SettingChanged>,
) {
    for change in changes.read() {
        let Some(id) = change.action.strip_prefix("setting:") else {
            continue;
        };
        let Some(key) = key_from_id(id) else {
            warn!("unknown setting id from menu: {id}");
            continue;
        };

        match store.set_from_text(key, &change.value) {
            Ok(Some(value)) => {
                registry.update_input_value(&change.action, &value.to_string());
                changed_writer.write(SettingChanged { key, value });
            }
            Ok(None) => {}
            Err(error) => warn!("invalid value for setting {id}: {error:?}"),
        }
    }
}
