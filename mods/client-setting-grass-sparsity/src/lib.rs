use tokio::task::JoinHandle;

pub struct ClientSettingGrassSparsityMod;

impl ClientSettingGrassSparsityMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
