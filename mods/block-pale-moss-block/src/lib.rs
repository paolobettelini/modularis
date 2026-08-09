use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PaleMossBlockBlock;

impl Block for PaleMossBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pale-moss-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PaleMossBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pale-moss-block:block/pale_moss_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PaleMossBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PaleMossBlockBlock::RENDER;

pub struct BlockPaleMossBlockMod;

impl BlockPaleMossBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
