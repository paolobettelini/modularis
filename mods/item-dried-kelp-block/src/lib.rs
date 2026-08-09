use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DriedKelpBlockItem;

impl Item for DriedKelpBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dried_kelp_block",
        label: "Dried Kelp",
    };
}

impl ItemRender for DriedKelpBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dried-kelp-block:item/dried_kelp_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DriedKelpBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DriedKelpBlockItem as ItemRender>::RENDER;

pub struct ItemDriedKelpBlockMod;

impl ItemDriedKelpBlockMod {
    pub fn init(_block: &mut block_dried_kelp::BlockDriedKelpMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
