use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CrackedDeepslateBricksBlockItem;

impl Item for CrackedDeepslateBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cracked_deepslate_bricks_block",
        label: "Cracked Deepslate Bricks",
    };
}

impl ItemRender for CrackedDeepslateBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cracked-deepslate-bricks-block:item/cracked_deepslate_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CrackedDeepslateBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <CrackedDeepslateBricksBlockItem as ItemRender>::RENDER;

pub struct ItemCrackedDeepslateBricksBlockMod;

impl ItemCrackedDeepslateBricksBlockMod {
    pub fn init(
        _block: &mut block_cracked_deepslate_bricks::BlockCrackedDeepslateBricksMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
