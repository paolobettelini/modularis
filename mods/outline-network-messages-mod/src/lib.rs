use tokio::task::JoinHandle;

pub struct OutlineNetworkMessagesMod;

impl OutlineNetworkMessagesMod {
    pub fn init() -> Self { Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
