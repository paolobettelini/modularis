use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "controls.sneak_key",
    label: "Sneak key",
    kind: SettingType::String,
    input: "keybinding",
    default: SettingDefault::String("ShiftLeft"),
};

pub struct ClientSettingSneakKeyMod;

impl ClientSettingSneakKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
