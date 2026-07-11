use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "controls.inventory_key",
    label: "Inventory key",
    kind: SettingType::String,
    input: "keybinding",
    default: SettingDefault::String("E"),
};

pub struct ClientSettingInventoryKeyMod;

impl ClientSettingInventoryKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
