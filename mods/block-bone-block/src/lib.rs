use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BoneBlockBlock;

impl Block for BoneBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:bone-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BoneBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-bone-block:block/bone_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BoneBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BoneBlockBlock::RENDER;

pub struct BlockBoneBlockMod;

impl BlockBoneBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
