use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateEmeraldOreBlock;

impl Block for DeepslateEmeraldOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-emerald-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateEmeraldOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-emerald-ore:block/deepslate_emerald_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateEmeraldOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateEmeraldOreBlock::RENDER;

pub struct BlockDeepslateEmeraldOreMod;

impl BlockDeepslateEmeraldOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
