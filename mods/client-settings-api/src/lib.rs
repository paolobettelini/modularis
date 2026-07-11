use bevy::prelude::*;
use generated_client_settings_registry::{SettingKey, all_settings, default_value, definition};
use settings_schema_api::{SettingParseError, SettingValue};
use std::collections::HashMap;

#[derive(Resource, Debug, Clone)]
pub struct SettingsStore {
    values: HashMap<SettingKey, SettingValue>,
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct SettingChanged {
    pub key: SettingKey,
    pub value: SettingValue,
}

#[derive(Debug)]
pub enum SettingsError {
    InvalidType {
        key: SettingKey,
        value: SettingValue,
    },
    Parse(SettingParseError),
}

pub trait SettingsApi: Send + Sync + 'static {}

impl Default for SettingsStore {
    fn default() -> Self {
        let values = all_settings()
            .iter()
            .copied()
            .map(|key| (key, default_value(key)))
            .collect();
        Self { values }
    }
}

impl SettingsStore {
    pub fn contains(&self, key: SettingKey) -> bool {
        self.values.contains_key(&key)
    }

    pub fn get(&self, key: SettingKey) -> &SettingValue {
        self.values
            .get(&key)
            .expect("generated setting key must exist in the store")
    }

    pub fn get_i32(&self, key: SettingKey) -> Option<i32> {
        match self.get(key) {
            SettingValue::I32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_f32(&self, key: SettingKey) -> Option<f32> {
        match self.get(key) {
            SettingValue::F32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_string(&self, key: SettingKey) -> Option<&str> {
        match self.get(key) {
            SettingValue::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn values(&self) -> impl Iterator<Item = (SettingKey, &SettingValue)> {
        all_settings()
            .iter()
            .copied()
            .map(|key| (key, self.get(key)))
    }

    pub fn set(&mut self, key: SettingKey, value: SettingValue) -> Result<bool, SettingsError> {
        if definition(key).kind != value.kind() {
            return Err(SettingsError::InvalidType { key, value });
        }
        if self.get(key) == &value {
            return Ok(false);
        }
        self.values.insert(key, value);
        Ok(true)
    }

    pub fn set_from_text(
        &mut self,
        key: SettingKey,
        text: &str,
    ) -> Result<Option<SettingValue>, SettingsError> {
        let value =
            SettingValue::parse(definition(key).kind, text).map_err(SettingsError::Parse)?;
        if self.set(key, value.clone())? {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub fn reset(&mut self, key: SettingKey) -> bool {
        self.set(key, default_value(key)).unwrap_or(false)
    }

    pub fn reset_all(&mut self) {
        *self = Self::default();
    }
}
