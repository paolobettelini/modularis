use tokio::task::JoinHandle;

pub struct SettingMouseSensitivityMod;

impl SettingMouseSensitivityMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
