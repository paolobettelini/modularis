use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PrismarineBricksBlock;

impl Block for PrismarineBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:prismarine-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PrismarineBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-prismarine-bricks:block/prismarine_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PrismarineBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PrismarineBricksBlock::RENDER;

pub struct BlockPrismarineBricksMod;

impl BlockPrismarineBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
