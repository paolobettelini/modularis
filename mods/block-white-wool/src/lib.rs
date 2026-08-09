use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WhiteWoolBlock;

impl Block for WhiteWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:white-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WhiteWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-white-wool:block/white_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WhiteWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WhiteWoolBlock::RENDER;

pub struct BlockWhiteWoolMod;

impl BlockWhiteWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
