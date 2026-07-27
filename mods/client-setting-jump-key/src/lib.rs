use tokio::task::JoinHandle;

pub struct ClientSettingJumpKeyMod;

impl ClientSettingJumpKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
