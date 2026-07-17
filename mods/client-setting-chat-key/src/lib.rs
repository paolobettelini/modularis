use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "controls.chat_key",
    label: "Chat key",
    kind: SettingType::String,
    input: "keybinding",
    default: SettingDefault::String("T"),
};

pub struct ClientSettingChatKeyMod;

impl ClientSettingChatKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
