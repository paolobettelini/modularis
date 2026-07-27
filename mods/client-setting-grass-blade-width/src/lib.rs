use tokio::task::JoinHandle;

pub struct ClientSettingGrassBladeWidthMod;

impl ClientSettingGrassBladeWidthMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
