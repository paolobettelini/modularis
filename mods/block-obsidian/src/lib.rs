use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ObsidianBlock;

impl Block for ObsidianBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:obsidian",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ObsidianBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-obsidian:block/obsidian"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ObsidianBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ObsidianBlock::RENDER;

pub struct BlockObsidianMod;

impl BlockObsidianMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
