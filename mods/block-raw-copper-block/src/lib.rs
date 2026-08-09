use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RawCopperBlockBlock;

impl Block for RawCopperBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:raw-copper-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RawCopperBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-raw-copper-block:block/raw_copper_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RawCopperBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RawCopperBlockBlock::RENDER;

pub struct BlockRawCopperBlockMod;

impl BlockRawCopperBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
