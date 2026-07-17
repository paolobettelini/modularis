use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BasaltBlock;
impl Block for BasaltBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:basalt",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for BasaltBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-basalt:block/basalt"),
        textures: None,
    };
}
pub const BLOCK_INFO: BlockInfo = BasaltBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BasaltBlock::RENDER;
pub struct BlockBasaltMod;
impl BlockBasaltMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
