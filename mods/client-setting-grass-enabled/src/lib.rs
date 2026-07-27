use tokio::task::JoinHandle;

pub struct ClientSettingGrassEnabledMod;

impl ClientSettingGrassEnabledMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
