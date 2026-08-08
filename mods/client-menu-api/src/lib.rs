use bevy::prelude::*;
use client_game_state_api::{
    GameState, GameStateCommand, InGameOverlayCommand, InGameOverlayState,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct MenuScreen {
    pub id: &'static str,
    pub title: &'static str,
    pub target: MenuTarget,
    pub background: MenuBackground,
    pub widgets: Vec<MenuWidget>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuTarget {
    Game(GameState),
    InGameOverlay(InGameOverlayState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuBackground {
    Opaque,
    Transparent,
}

#[derive(Debug, Clone)]
pub enum MenuWidget {
    Label {
        text: String,
    },
    Button {
        id: &'static str,
        label: String,
        action: MenuButtonAction,
    },
    Textbox {
        id: String,
        label: String,
        value: String,
        action: String,
    },
    TextboxButton {
        id: String,
        label: String,
        value: String,
        action: String,
        button_id: &'static str,
        button_label: String,
        button_action: MenuButtonAction,
    },
    NumberInput {
        id: String,
        label: String,
        value: String,
        action: String,
        kind: MenuNumberKind,
        step: f64,
        min: Option<f64>,
        max: Option<f64>,
    },
    KeybindingInput {
        id: String,
        label: String,
        value: String,
        action: String,
    },
    ToggleInput {
        id: String,
        label: String,
        value: bool,
        action: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuNumberKind {
    I32,
    F32,
}

#[derive(Debug, Clone, Copy)]
pub enum MenuButtonAction {
    ChangeGameState(GameStateCommand),
    ChangeInGameOverlay(InGameOverlayCommand),
    /// Replace the current menu while preserving its game/overlay state.
    OpenScreen(&'static str),
}

#[derive(Message, Debug, Clone)]
pub struct MenuValueChanged {
    pub action: String,
    pub value: String,
}

#[derive(Resource, Clone, Default)]
pub struct MenuRegistryHandle(pub Arc<Mutex<Vec<MenuScreen>>>);

impl MenuRegistryHandle {
    pub fn register_screen(&self, screen: MenuScreen) {
        self.0
            .lock()
            .expect("menu registry lock poisoned")
            .push(screen);
    }

    pub fn screen_for(&self, target: MenuTarget) -> Option<MenuScreen> {
        self.0
            .lock()
            .expect("menu registry lock poisoned")
            .iter()
            .find(|screen| screen.target == target)
            .cloned()
    }

    pub fn screen_by_id_for_target(&self, id: &str, target: MenuTarget) -> Option<MenuScreen> {
        self.0
            .lock()
            .expect("menu registry lock poisoned")
            .iter()
            .find(|screen| screen.id == id && screen.target == target)
            .cloned()
    }

    pub fn update_input_value(&self, action: &str, value: &str) {
        let mut screens = self.0.lock().expect("menu registry lock poisoned");
        for screen in screens.iter_mut() {
            for widget in screen.widgets.iter_mut() {
                match widget {
                    MenuWidget::Textbox {
                        action: widget_action,
                        value: widget_value,
                        ..
                    }
                    | MenuWidget::TextboxButton {
                        action: widget_action,
                        value: widget_value,
                        ..
                    }
                    | MenuWidget::NumberInput {
                        action: widget_action,
                        value: widget_value,
                        ..
                    }
                    | MenuWidget::KeybindingInput {
                        action: widget_action,
                        value: widget_value,
                        ..
                    } if widget_action == action => {
                        *widget_value = value.to_string();
                    }
                    MenuWidget::ToggleInput {
                        action: widget_action,
                        value: widget_value,
                        ..
                    } if widget_action == action => {
                        if let Ok(parsed) = value.parse() {
                            *widget_value = parsed;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

pub trait MenuApi: Send + Sync + 'static {
    fn register_screen(&mut self, screen: MenuScreen);
}
