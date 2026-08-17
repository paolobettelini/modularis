use tokio::task::JoinHandle;

pub struct TheCrownNetworkMessagesMod;

impl TheCrownNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

