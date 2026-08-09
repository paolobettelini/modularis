use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CryingObsidianBlock;

impl Block for CryingObsidianBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:crying-obsidian",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CryingObsidianBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-crying-obsidian:block/crying_obsidian"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CryingObsidianBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CryingObsidianBlock::RENDER;

pub struct BlockCryingObsidianMod;

impl BlockCryingObsidianMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
