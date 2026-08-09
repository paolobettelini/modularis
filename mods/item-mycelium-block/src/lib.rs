use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MyceliumBlockItem;

impl Item for MyceliumBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mycelium_block",
        label: "Mycelium",
    };
}

impl ItemRender for MyceliumBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mycelium-block:item/mycelium_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MyceliumBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MyceliumBlockItem as ItemRender>::RENDER;

pub struct ItemMyceliumBlockMod;

impl ItemMyceliumBlockMod {
    pub fn init(_block: &mut block_mycelium::BlockMyceliumMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
