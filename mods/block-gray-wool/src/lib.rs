use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GrayWoolBlock;

impl Block for GrayWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gray-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GrayWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gray-wool:block/gray_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GrayWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GrayWoolBlock::RENDER;

pub struct BlockGrayWoolMod;

impl BlockGrayWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
