pub struct EntityRegistryCodegenMod;
impl EntityRegistryCodegenMod {
 pub fn init()->Self { Self }
 pub fn run(&self)->Option<Vec<tokio::task::JoinHandle<()>>> {None}
}
