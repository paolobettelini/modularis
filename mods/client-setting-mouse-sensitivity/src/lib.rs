use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "controls.mouse_sensitivity",
    label: "Mouse sensitivity",
    kind: SettingType::F32,
    input: "f32",
    default: SettingDefault::F32(0.15),
};

pub struct SettingMouseSensitivityMod;

impl SettingMouseSensitivityMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
