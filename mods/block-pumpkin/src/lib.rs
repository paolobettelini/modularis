use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PumpkinBlock;

impl Block for PumpkinBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pumpkin",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PumpkinBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pumpkin:block/pumpkin"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PumpkinBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PumpkinBlock::RENDER;

pub struct BlockPumpkinMod;

impl BlockPumpkinMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
