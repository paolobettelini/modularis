use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BrownWoolBlock;

impl Block for BrownWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:brown-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BrownWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-brown-wool:block/brown_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BrownWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BrownWoolBlock::RENDER;

pub struct BlockBrownWoolMod;

impl BlockBrownWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
