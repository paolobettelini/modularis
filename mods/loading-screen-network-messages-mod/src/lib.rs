use tokio::task::JoinHandle;

pub struct LoadingScreenNetworkMessagesMod;

impl LoadingScreenNetworkMessagesMod {
    pub fn init() -> Self { Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
