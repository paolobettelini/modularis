use tokio::task::JoinHandle;

pub struct BiomeSavannaMod;

impl BiomeSavannaMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
