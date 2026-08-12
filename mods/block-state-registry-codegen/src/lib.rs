use tokio::task::JoinHandle;

pub struct BlockStateRegistryCodegenMod;
impl BlockStateRegistryCodegenMod {
    pub fn init() -> Self { Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

