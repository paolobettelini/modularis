use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CauldronBlockItem;

impl Item for CauldronBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cauldron_block",
        label: "Cauldron",
    };
}

impl ItemRender for CauldronBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cauldron-block:item/cauldron_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CauldronBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CauldronBlockItem as ItemRender>::RENDER;

pub struct ItemCauldronBlockMod;

impl ItemCauldronBlockMod {
    pub fn init(
        _block: &mut block_cauldron::BlockCauldronMod,
        _template: &mut voxel_model_item_templates_mod::VoxelModelItemTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
