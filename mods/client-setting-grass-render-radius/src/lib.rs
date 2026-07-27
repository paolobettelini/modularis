use tokio::task::JoinHandle;

pub struct ClientSettingGrassRenderRadiusMod;

impl ClientSettingGrassRenderRadiusMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
