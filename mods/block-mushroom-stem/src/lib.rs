use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MushroomStemBlock;

impl Block for MushroomStemBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mushroom-stem",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MushroomStemBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mushroom-stem:block/mushroom_stem"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MushroomStemBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MushroomStemBlock::RENDER;

pub struct BlockMushroomStemMod;

impl BlockMushroomStemMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
