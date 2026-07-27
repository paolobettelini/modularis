use bevy::prelude::*;
use client_menu_api::MenuWidget;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct SettingInputContext {
    pub id: String,
    pub label: String,
    pub value: String,
    pub action: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

pub type SettingInputFactory = fn(SettingInputContext) -> MenuWidget;

#[derive(Resource, Clone, Default)]
pub struct SettingInputRegistryHandle(Arc<Mutex<HashMap<&'static str, SettingInputFactory>>>);

impl SettingInputRegistryHandle {
    pub fn register(&self, id: &'static str, factory: SettingInputFactory) {
        self.0
            .lock()
            .expect("setting input registry lock poisoned")
            .insert(id, factory);
    }

    pub fn build(&self, id: &str, context: SettingInputContext) -> Option<MenuWidget> {
        let factory = self
            .0
            .lock()
            .expect("setting input registry lock poisoned")
            .get(id)
            .copied()?;
        Some(factory(context))
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingInputStartupSet {
    RegisterInputs,
    BuildMenus,
}

pub trait SettingsInputApi: Send + Sync + 'static {}
