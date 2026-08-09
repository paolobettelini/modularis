use tokio::task::JoinHandle;

pub struct BiomeAetherVerdantCanopyMod;

impl BiomeAetherVerdantCanopyMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
