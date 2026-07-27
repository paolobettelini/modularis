use tokio::task::JoinHandle;

pub struct ClientSettingSprintKeyMod;

impl ClientSettingSprintKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
