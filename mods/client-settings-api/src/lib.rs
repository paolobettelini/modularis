use bevy::prelude::*;
use generated_client_settings_registry::{SettingKey, all_settings, default_value, definition};
use settings_schema_api::{SettingNumberRange, SettingParseError, SettingValue};
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
    NonFiniteNumber {
        key: SettingKey,
    },
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

    pub fn get_bool(&self, key: SettingKey) -> Option<bool> {
        match self.get(key) {
            SettingValue::Bool(value) => Some(*value),
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
        let value = normalize_value(key, value)?;
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
        let parsed =
            SettingValue::parse(definition(key).kind, text).map_err(SettingsError::Parse)?;
        let value = normalize_value(key, parsed.clone())?;
        let was_normalized = value != parsed;
        if self.set(key, value.clone())? || was_normalized {
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

fn normalize_value(key: SettingKey, value: SettingValue) -> Result<SettingValue, SettingsError> {
    let Some(range) = definition(key).number_range else {
        return match value {
            SettingValue::F32(value) if !value.is_finite() => {
                Err(SettingsError::NonFiniteNumber { key })
            }
            value => Ok(value),
        };
    };
    match value {
        SettingValue::I32(value) => Ok(SettingValue::I32(clamp_i32(value, range))),
        SettingValue::F32(value) if value.is_finite() => {
            Ok(SettingValue::F32(clamp_f32(value, range)))
        }
        SettingValue::F32(_) => Err(SettingsError::NonFiniteNumber { key }),
        value => Ok(value),
    }
}

fn clamp_i32(value: i32, range: SettingNumberRange) -> i32 {
    let minimum = range
        .min
        .map(|minimum| minimum.ceil() as i32)
        .unwrap_or(i32::MIN);
    let maximum = range
        .max
        .map(|maximum| maximum.floor() as i32)
        .unwrap_or(i32::MAX);
    value.clamp(minimum, maximum)
}

fn clamp_f32(mut value: f32, range: SettingNumberRange) -> f32 {
    if let Some(minimum) = range.min {
        value = value.max(minimum as f32);
    }
    if let Some(maximum) = range.max {
        value = value.min(maximum as f32);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_settings_are_clamped_to_their_generated_contract() {
        let mut settings = SettingsStore::default();

        assert!(
            settings
                .set(SettingKey::GraphicsFov, SettingValue::F32(500.0))
                .unwrap()
        );
        assert_eq!(settings.get_f32(SettingKey::GraphicsFov), Some(120.0));

        let normalized = settings
            .set_from_text(SettingKey::GraphicsRenderDistance, "-25")
            .unwrap();
        assert_eq!(normalized, Some(SettingValue::I32(1)));
        assert_eq!(
            settings.get_i32(SettingKey::GraphicsRenderDistance),
            Some(1)
        );
    }
}
