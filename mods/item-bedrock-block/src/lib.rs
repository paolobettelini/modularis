use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BedrockBlockItem;

impl Item for BedrockBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:bedrock_block",
        label: "Bedrock",
    };
}

impl ItemRender for BedrockBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-bedrock-block:item/bedrock_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BedrockBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BedrockBlockItem as ItemRender>::RENDER;

pub struct ItemBedrockBlockMod;

impl ItemBedrockBlockMod {
    pub fn init(_block: &mut block_bedrock::BlockBedrockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
