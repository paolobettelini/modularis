use inventory_core_api::InventoryLayout;

pub trait ServerInventoryLayoutApi: Send + Sync + 'static {
    fn default_layout() -> InventoryLayout;
}
