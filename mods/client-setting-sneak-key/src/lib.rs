use tokio::task::JoinHandle;

pub struct ClientSettingSneakKeyMod;

impl ClientSettingSneakKeyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
