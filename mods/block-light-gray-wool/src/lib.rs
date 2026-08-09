use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightGrayWoolBlock;

impl Block for LightGrayWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-gray-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightGrayWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-gray-wool:block/light_gray_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightGrayWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightGrayWoolBlock::RENDER;

pub struct BlockLightGrayWoolMod;

impl BlockLightGrayWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
