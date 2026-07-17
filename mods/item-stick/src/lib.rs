use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StickItem;

impl Item for StickItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stick",
        label: "Stick",
    };
}

impl ItemRender for StickItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stick:item/stick"),
    };
}

pub const ITEM_INFO: ItemInfo = StickItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = StickItem::RENDER;

pub struct ItemStickMod;

impl ItemStickMod {
    pub fn init(
        _templates: &mut voxel_model_item_templates_mod::VoxelModelItemTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
