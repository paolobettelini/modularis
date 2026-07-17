use tokio::task::JoinHandle;

pub struct BiomeAetherHighlandsMod;

impl BiomeAetherHighlandsMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
