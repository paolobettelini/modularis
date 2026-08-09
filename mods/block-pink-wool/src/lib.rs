use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PinkWoolBlock;

impl Block for PinkWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pink-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PinkWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pink-wool:block/pink_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PinkWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PinkWoolBlock::RENDER;

pub struct BlockPinkWoolMod;

impl BlockPinkWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
