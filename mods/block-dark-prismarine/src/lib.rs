use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DarkPrismarineBlock;

impl Block for DarkPrismarineBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dark-prismarine",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DarkPrismarineBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dark-prismarine:block/dark_prismarine"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DarkPrismarineBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DarkPrismarineBlock::RENDER;

pub struct BlockDarkPrismarineMod;

impl BlockDarkPrismarineMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
