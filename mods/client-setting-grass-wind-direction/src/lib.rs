use tokio::task::JoinHandle;

pub struct ClientSettingGrassWindDirectionMod;

impl ClientSettingGrassWindDirectionMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
