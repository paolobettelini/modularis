use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GravelBlock;

impl Block for GravelBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gravel",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GravelBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gravel:block/gravel"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GravelBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GravelBlock::RENDER;

pub struct BlockGravelMod;

impl BlockGravelMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
