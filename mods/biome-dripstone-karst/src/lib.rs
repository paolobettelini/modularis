use tokio::task::JoinHandle;

pub struct BiomeDripstoneKarstMod;

impl BiomeDripstoneKarstMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
