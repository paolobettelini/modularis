use tokio::task::JoinHandle;

pub struct ClientSettingChatKeyMod;

impl ClientSettingChatKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
