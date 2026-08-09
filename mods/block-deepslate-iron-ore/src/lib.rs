use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateIronOreBlock;

impl Block for DeepslateIronOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-iron-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateIronOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-iron-ore:block/deepslate_iron_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateIronOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateIronOreBlock::RENDER;

pub struct BlockDeepslateIronOreMod;

impl BlockDeepslateIronOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
