use tokio::task::JoinHandle;

pub struct ClientSettingGrassRenderLodMod;

impl ClientSettingGrassRenderLodMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
