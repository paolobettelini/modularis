use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RawIronBlockBlock;

impl Block for RawIronBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:raw-iron-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RawIronBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-raw-iron-block:block/raw_iron_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RawIronBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RawIronBlockBlock::RENDER;

pub struct BlockRawIronBlockMod;

impl BlockRawIronBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
