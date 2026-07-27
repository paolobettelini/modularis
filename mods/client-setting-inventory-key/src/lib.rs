use tokio::task::JoinHandle;

pub struct ClientSettingInventoryKeyMod;

impl ClientSettingInventoryKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
