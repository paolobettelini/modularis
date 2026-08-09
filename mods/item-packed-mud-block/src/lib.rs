use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PackedMudBlockItem;

impl Item for PackedMudBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:packed_mud_block",
        label: "Packed Mud",
    };
}

impl ItemRender for PackedMudBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-packed-mud-block:item/packed_mud_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PackedMudBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PackedMudBlockItem as ItemRender>::RENDER;

pub struct ItemPackedMudBlockMod;

impl ItemPackedMudBlockMod {
    pub fn init(_block: &mut block_packed_mud::BlockPackedMudMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
