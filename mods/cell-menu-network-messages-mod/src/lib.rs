use tokio::task::JoinHandle;

pub struct CellMenuNetworkMessagesMod;

impl CellMenuNetworkMessagesMod {
    pub fn init(_events: &mut cell_menu_events_mod::CellMenuEventsMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
