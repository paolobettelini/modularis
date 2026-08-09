use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PrismarineBlock;

impl Block for PrismarineBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:prismarine",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PrismarineBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-prismarine:block/prismarine"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PrismarineBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PrismarineBlock::RENDER;

pub struct BlockPrismarineMod;

impl BlockPrismarineMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
