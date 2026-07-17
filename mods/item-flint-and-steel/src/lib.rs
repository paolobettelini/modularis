use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct FlintAndSteelItem;

impl Item for FlintAndSteelItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:flint-and-steel",
        label: "Flint and Steel",
    };
}

impl ItemRender for FlintAndSteelItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-flint-and-steel:item/flint_and_steel"),
    };
}

pub const ITEM_INFO: ItemInfo = FlintAndSteelItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = FlintAndSteelItem::RENDER;

pub struct ItemFlintAndSteelMod;

impl ItemFlintAndSteelMod {
    pub fn init(
        _templates: &mut voxel_model_item_templates_mod::VoxelModelItemTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
