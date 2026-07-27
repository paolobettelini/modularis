use tokio::task::JoinHandle;

pub struct ClientSettingFovMod;

impl ClientSettingFovMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
