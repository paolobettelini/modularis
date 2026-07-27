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
use generated_client_settings_registry::{
    all_sections, all_settings, definition, key_from_id, section,
};
use settings_schema_api::SettingSection;
use std::collections::BTreeMap;
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

#[derive(Debug, Clone)]
struct SettingsMenuSection {
    definition: SettingSection,
    widgets: Vec<MenuWidget>,
}

fn register_settings_menus(
    store: Res<SettingsStore>,
    inputs: Res<SettingInputRegistryHandle>,
    menu: Res<MenuRegistryHandle>,
) {
    let mut root_settings = Vec::new();
    let mut sections = all_sections()
        .iter()
        .copied()
        .map(|definition| {
            (
                definition.id,
                SettingsMenuSection {
                    definition,
                    widgets: Vec::new(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    for key in all_settings().iter().copied() {
        let definition = definition(key);
        let context = SettingInputContext {
            id: definition.id.to_string(),
            label: definition.label.to_string(),
            value: store.get(key).to_string(),
            action: format!("setting:{}", definition.id),
            min: definition.number_range.and_then(|range| range.min),
            max: definition.number_range.and_then(|range| range.max),
        };
        let widget = inputs.build(definition.input, context);
        if widget.is_none() {
            warn!(
                "setting '{}' requires missing input provider '{}'",
                definition.id, definition.input
            );
        }
        let Some(widget) = widget else {
            continue;
        };
        if let Some(section) = section(key) {
            let section = sections
                .get_mut(section.id)
                .expect("generated setting section must have a descriptor");
            section.widgets.push(widget);
        } else {
            root_settings.push(widget);
        }
    }

    let mut main_widgets = root_widgets(&root_settings, &sections);
    let mut pause_widgets = root_widgets(&root_settings, &sections);

    main_widgets.push(MenuWidget::Button {
        id: "back",
        label: "Back".to_string(),
        action: MenuButtonAction::ChangeGameState(GameStateCommand::BackToMainMenu),
    });
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

    for section in sections.values() {
        let mut main_section_widgets = section_widgets(section, &sections);
        main_section_widgets.push(MenuWidget::Button {
            id: "back-to-settings",
            label: "Back".to_string(),
            action: MenuButtonAction::OpenScreen(
                section.definition.parent.unwrap_or("settings-menu"),
            ),
        });
        let mut pause_section_widgets = section_widgets(section, &sections);
        pause_section_widgets.push(MenuWidget::Button {
            id: "back-to-pause-settings",
            label: "Back".to_string(),
            action: MenuButtonAction::OpenScreen(
                section.definition.parent.unwrap_or("pause-settings-menu"),
            ),
        });
        menu.register_screen(MenuScreen {
            id: section.definition.id,
            title: section.definition.label,
            target: MenuTarget::Game(GameState::SettingsMenu),
            background: MenuBackground::Opaque,
            widgets: main_section_widgets,
        });
        menu.register_screen(MenuScreen {
            id: section.definition.id,
            title: section.definition.label,
            target: MenuTarget::InGameOverlay(InGameOverlayState::Settings),
            background: MenuBackground::Transparent,
            widgets: pause_section_widgets,
        });
    }
}

fn root_widgets(
    root_settings: &[MenuWidget],
    sections: &BTreeMap<&'static str, SettingsMenuSection>,
) -> Vec<MenuWidget> {
    let mut widgets = root_settings.to_vec();
    widgets.extend(section_buttons(None, sections));
    widgets
}

fn section_widgets(
    section: &SettingsMenuSection,
    sections: &BTreeMap<&'static str, SettingsMenuSection>,
) -> Vec<MenuWidget> {
    let mut widgets = section.widgets.clone();
    widgets.extend(section_buttons(Some(section.definition.id), sections));
    widgets
}

fn section_buttons(
    parent: Option<&str>,
    sections: &BTreeMap<&'static str, SettingsMenuSection>,
) -> Vec<MenuWidget> {
    sections
        .values()
        .filter(|section| section.definition.parent == parent)
        .map(|section| MenuWidget::Button {
            id: section.definition.id,
            label: section.definition.label.to_string(),
            action: MenuButtonAction::OpenScreen(section.definition.id),
        })
        .collect()
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
