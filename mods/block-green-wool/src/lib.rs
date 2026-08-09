use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GreenWoolBlock;

impl Block for GreenWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:green-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GreenWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-green-wool:block/green_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GreenWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GreenWoolBlock::RENDER;

pub struct BlockGreenWoolMod;

impl BlockGreenWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
