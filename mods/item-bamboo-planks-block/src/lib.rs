use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BambooPlanksBlockItem;

impl Item for BambooPlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:bamboo_planks_block",
        label: "Bamboo Planks",
    };
}

impl ItemRender for BambooPlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-bamboo-planks-block:item/bamboo_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BambooPlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BambooPlanksBlockItem as ItemRender>::RENDER;

pub struct ItemBambooPlanksBlockMod;

impl ItemBambooPlanksBlockMod {
    pub fn init(_block: &mut block_bamboo_planks::BlockBambooPlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
