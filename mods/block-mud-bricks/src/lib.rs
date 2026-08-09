use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MudBricksBlock;

impl Block for MudBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mud-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MudBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mud-bricks:block/mud_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MudBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MudBricksBlock::RENDER;

pub struct BlockMudBricksMod;

impl BlockMudBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
