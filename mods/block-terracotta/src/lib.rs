use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct TerracottaBlock;
impl Block for TerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for TerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-terracotta:block/terracotta"),
        textures: None,
    };
}
pub const BLOCK_INFO: BlockInfo = TerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = TerracottaBlock::RENDER;
pub struct BlockTerracottaMod;
impl BlockTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
