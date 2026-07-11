use inventory_core_api::{
    InventoryLayout, InventorySectionId, InventorySectionLayout, InventorySectionRole,
};
use server_inventory_layout_api::ServerInventoryLayoutApi;
use tokio::task::JoinHandle;

pub struct ServerInventoryLayoutDefaultImpl;

impl ServerInventoryLayoutDefaultImpl {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerInventoryLayoutApi for ServerInventoryLayoutDefaultImpl {
    fn default_layout() -> InventoryLayout {
        InventoryLayout {
            sections: vec![
                InventorySectionLayout {
                    id: InventorySectionId::new("hotbar"),
                    role: InventorySectionRole::Hotbar,
                    columns: 11,
                    cells: 11,
                },
                InventorySectionLayout {
                    id: InventorySectionId::new("storage"),
                    role: InventorySectionRole::Storage,
                    columns: 9,
                    cells: 27,
                },
            ],
        }
    }
}
