use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "graphics.render_distance",
    label: "Render distance (chunks)",
    kind: SettingType::I32,
    input: "i32",
    default: SettingDefault::I32(8),
};

pub struct SettingRenderDistanceMod;

impl SettingRenderDistanceMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
