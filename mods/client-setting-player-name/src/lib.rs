use tokio::task::JoinHandle;

pub struct ClientSettingPlayerNameMod;

impl ClientSettingPlayerNameMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
