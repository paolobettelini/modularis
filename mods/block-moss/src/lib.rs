use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MossBlock;
impl Block for MossBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:moss",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for MossBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-moss:block/moss"),
        textures: None,
    };
}
pub const BLOCK_INFO: BlockInfo = MossBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MossBlock::RENDER;
pub struct BlockMossMod;
impl BlockMossMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
