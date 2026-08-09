use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CoalOreBlock;

impl Block for CoalOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:coal-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CoalOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-coal-ore:block/coal_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CoalOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CoalOreBlock::RENDER;

pub struct BlockCoalOreMod;

impl BlockCoalOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
