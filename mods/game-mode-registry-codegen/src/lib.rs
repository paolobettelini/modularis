use tokio::task::JoinHandle;

pub struct GameModeRegistryCodegenMod;
impl GameModeRegistryCodegenMod {
    pub fn init() -> Self { Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

