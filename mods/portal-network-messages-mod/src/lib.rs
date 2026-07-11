use tokio::task::JoinHandle;

pub struct PortalNetworkMessagesMod;

impl PortalNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
