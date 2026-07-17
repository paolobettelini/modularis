use tokio::task::JoinHandle;

pub struct BiomeSoulSandValleyMod;

impl BiomeSoulSandValleyMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
