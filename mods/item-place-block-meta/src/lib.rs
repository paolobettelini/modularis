use generated_block_registry::BlockId;
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceBlock {
    pub block: BlockId,
}

pub struct ItemPlaceBlockMetaMod;

impl ItemPlaceBlockMetaMod {
    pub fn init(_blocks: &mut block_registry_codegen::BlockRegistryCodegenMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
