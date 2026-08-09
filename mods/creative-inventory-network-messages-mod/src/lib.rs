use tokio::task::JoinHandle;

pub struct CreativeInventoryNetworkMessagesMod;

impl CreativeInventoryNetworkMessagesMod {
    pub fn init(_items: &mut item_registry_codegen::ItemRegistryCodegenMod) -> Self { Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
