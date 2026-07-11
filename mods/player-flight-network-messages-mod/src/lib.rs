use tokio::task::JoinHandle;

pub struct PlayerFlightNetworkMessagesMod;

impl PlayerFlightNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
