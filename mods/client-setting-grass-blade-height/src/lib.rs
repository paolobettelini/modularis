use tokio::task::JoinHandle;

pub struct ClientSettingGrassBladeHeightMod;

impl ClientSettingGrassBladeHeightMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
