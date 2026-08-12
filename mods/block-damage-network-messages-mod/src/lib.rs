use tokio::task::JoinHandle;
pub struct BlockDamageNetworkMessagesMod;
impl BlockDamageNetworkMessagesMod {
    pub fn init() -> Self { Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

