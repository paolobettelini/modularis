use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GlowstoneBlockItem;

impl Item for GlowstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:glowstone_block",
        label: "Glowstone",
    };
}

impl ItemRender for GlowstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-glowstone-block:item/glowstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GlowstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GlowstoneBlockItem as ItemRender>::RENDER;

pub struct ItemGlowstoneBlockMod;

impl ItemGlowstoneBlockMod {
    pub fn init(_block: &mut block_glowstone::BlockGlowstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
