use block_api::BlockInfo;
use block_manager_api::{BlockId, BlockManagerApi};
use block_registry_codegen::BlockRegistryCodegenMod;
use block_render_api::BlockRenderInfo;
use tokio::task::JoinHandle;

pub struct GeneratedBlockManager;

impl GeneratedBlockManager {
    pub fn init(_codegen: &mut BlockRegistryCodegenMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl BlockManagerApi for GeneratedBlockManager {
    fn info(block: BlockId) -> &'static BlockInfo {
        generated_block_registry::info(block)
    }

    fn render_info(block: BlockId) -> &'static BlockRenderInfo {
        generated_block_registry::render_info(block)
    }

    fn all() -> &'static [BlockId] {
        generated_block_registry::all_blocks()
    }

    fn from_string(id: &str) -> Option<BlockId> {
        generated_block_registry::from_str(id)
    }

    fn id(block: BlockId) -> &'static str {
        generated_block_registry::id(block)
    }
}
