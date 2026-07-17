use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SnowBlock;

impl Block for SnowBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:snow",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SnowBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-snow:block/snow"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SnowBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SnowBlock::RENDER;

pub struct BlockSnowMod;

impl BlockSnowMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
