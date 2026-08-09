use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BrownMushroomBlockBlock;

impl Block for BrownMushroomBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:brown-mushroom-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BrownMushroomBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-brown-mushroom-block:block/brown_mushroom_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BrownMushroomBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BrownMushroomBlockBlock::RENDER;

pub struct BlockBrownMushroomBlockMod;

impl BlockBrownMushroomBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
