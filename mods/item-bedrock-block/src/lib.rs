use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct BedrockBlockItem;

impl Item for BedrockBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:bedrock_block",
        label: "Bedrock",
    };
}

pub const ITEM_INFO: ItemInfo = BedrockBlockItem::INFO;

pub struct ItemBedrockBlockMod;

impl ItemBedrockBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
