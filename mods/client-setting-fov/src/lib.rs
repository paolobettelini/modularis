use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "graphics.fov",
    label: "Field of view (degrees)",
    kind: SettingType::F32,
    input: "f32",
    default: SettingDefault::F32(75.0),
};

pub struct ClientSettingFovMod;

impl ClientSettingFovMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
