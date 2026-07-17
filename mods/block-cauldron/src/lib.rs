use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CauldronBlock;

impl Block for CauldronBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cauldron",
        is_air: false,
        solid: true,
        opaque: false,
    };
}

impl BlockRender for CauldronBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cauldron:block/cauldron"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CauldronBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CauldronBlock::RENDER;

pub struct BlockCauldronMod;

impl BlockCauldronMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
