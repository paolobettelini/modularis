use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "controls.jump_key",
    label: "Jump key",
    kind: SettingType::String,
    input: "keybinding",
    default: SettingDefault::String("Space"),
};

pub struct ClientSettingJumpKeyMod;

impl ClientSettingJumpKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
