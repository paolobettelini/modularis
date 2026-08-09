use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MushroomBlockInsideBlock;

impl Block for MushroomBlockInsideBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mushroom-block-inside",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MushroomBlockInsideBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mushroom-block-inside:block/mushroom_block_inside"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MushroomBlockInsideBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MushroomBlockInsideBlock::RENDER;

pub struct BlockMushroomBlockInsideMod;

impl BlockMushroomBlockInsideMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
