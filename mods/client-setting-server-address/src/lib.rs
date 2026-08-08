use tokio::task::JoinHandle;

pub struct ClientSettingServerAddressMod;

impl ClientSettingServerAddressMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
