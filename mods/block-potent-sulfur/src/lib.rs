use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PotentSulfurBlock;

impl Block for PotentSulfurBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:potent-sulfur",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PotentSulfurBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-potent-sulfur:block/potent_sulfur"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PotentSulfurBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PotentSulfurBlock::RENDER;

pub struct BlockPotentSulfurMod;

impl BlockPotentSulfurMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
