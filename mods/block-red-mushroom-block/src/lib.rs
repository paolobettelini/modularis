use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedMushroomBlockBlock;

impl Block for RedMushroomBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-mushroom-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedMushroomBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-red-mushroom-block:block/red_mushroom_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedMushroomBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedMushroomBlockBlock::RENDER;

pub struct BlockRedMushroomBlockMod;

impl BlockRedMushroomBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
