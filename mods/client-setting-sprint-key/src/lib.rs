use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "controls.sprint_key",
    label: "Sprint key",
    kind: SettingType::String,
    input: "keybinding",
    default: SettingDefault::String("ControlLeft"),
};

pub struct ClientSettingSprintKeyMod;

impl ClientSettingSprintKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
