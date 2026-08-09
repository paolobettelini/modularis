use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct IronBlockBlock;

impl Block for IronBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:iron-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for IronBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-iron-block:block/iron_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = IronBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = IronBlockBlock::RENDER;

pub struct BlockIronBlockMod;

impl BlockIronBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
