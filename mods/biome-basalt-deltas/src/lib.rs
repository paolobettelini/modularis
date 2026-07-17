use tokio::task::JoinHandle;

pub struct BiomeBasaltDeltasMod;

impl BiomeBasaltDeltasMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
