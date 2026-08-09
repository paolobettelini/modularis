use tokio::task::JoinHandle;

pub struct PlayerPermissionNetworkMessagesMod;

impl PlayerPermissionNetworkMessagesMod {
    pub fn init(_codegen: &mut permission_registry_codegen::PermissionRegistryCodegenMod) -> Self { Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
