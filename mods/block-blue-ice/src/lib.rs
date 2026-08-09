use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlueIceBlock;

impl Block for BlueIceBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:blue-ice",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlueIceBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-blue-ice:block/blue_ice"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlueIceBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlueIceBlock::RENDER;

pub struct BlockBlueIceMod;

impl BlockBlueIceMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
