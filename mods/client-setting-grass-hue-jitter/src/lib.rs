use tokio::task::JoinHandle;

pub struct ClientSettingGrassHueJitterMod;

impl ClientSettingGrassHueJitterMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
