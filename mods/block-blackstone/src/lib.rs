use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlackstoneBlock;
impl Block for BlackstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:blackstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for BlackstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-blackstone:block/blackstone"),
        textures: None,
    };
}
pub const BLOCK_INFO: BlockInfo = BlackstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlackstoneBlock::RENDER;
pub struct BlockBlackstoneMod;
impl BlockBlackstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
