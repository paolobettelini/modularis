use block_manager_api::BlockManagerApi;
use inventory_core_api::{Inventory, InventoryCell};
use inventory_events_api::InventoryResetRequested;
use item_instance_api::ItemInstance;
use item_manager_api::ItemManagerApi;
use item_place_block_meta::PlaceBlock;
use item_portal_igniter_meta::PortalIgniter;
use item_quantity_meta::Quantity;
use server_inventory_layout_api::ServerInventoryLayoutApi;

pub fn vanilla_default_reset<L, I, B>(player_id: u64) -> InventoryResetRequested
where
    L: ServerInventoryLayoutApi,
    I: ItemManagerApi,
    B: BlockManagerApi,
{
    InventoryResetRequested {
        player_id,
        inventory: vanilla_default_inventory::<L, I, B>(),
        selected_hotbar: 0,
    }
}

/// Constructs the demo's optional vanilla loadout.
///
/// A custom server may use this as-is, modify the returned inventory, provide
/// it only in selected scopes, or build a completely different loadout.
pub fn vanilla_default_inventory<L, I, B>() -> Inventory
where
    L: ServerInventoryLayoutApi,
    I: ItemManagerApi,
    B: BlockManagerApi,
{
    let mut inventory =
        Inventory::new(L::default_layout()).expect("default inventory layout must be valid");
    for (index, spec) in BLOCK_ITEMS.iter().enumerate() {
        inventory
            .set(
                InventoryCell::new("hotbar", index as u32),
                Some(block_item::<I, B>(spec)),
            )
            .expect("default hotbar cell must exist");
    }
    inventory
        .set(
            InventoryCell::new("hotbar", BLOCK_ITEMS.len() as u32),
            Some(flint_and_steel::<I>()),
        )
        .expect("default flint-and-steel hotbar cell must exist");
    for (index, spec) in STORAGE_ITEMS.iter().enumerate() {
        inventory
            .set(
                InventoryCell::new("storage", index as u32),
                Some(storage_item::<I, B>(spec)),
            )
            .expect("default storage cell must exist");
    }
    inventory
}

struct BlockItemSpec {
    item_id: &'static str,
    block_id: &'static str,
    quantity: Quantity,
}

const BLOCK_ITEMS: &[BlockItemSpec] = &[
    BlockItemSpec {
        item_id: "demo:grass_block",
        block_id: "demo:grass",
        quantity: Quantity::Finite(64),
    },
    BlockItemSpec {
        item_id: "demo:stone_block",
        block_id: "demo:stone",
        quantity: Quantity::Infinite,
    },
    BlockItemSpec {
        item_id: "demo:bedrock_block",
        block_id: "demo:bedrock",
        quantity: Quantity::Infinite,
    },
    BlockItemSpec {
        item_id: "demo:crafting_table_block",
        block_id: "demo:crafting-table",
        quantity: Quantity::Infinite,
    },
    BlockItemSpec {
        item_id: "demo:diamond_block",
        block_id: "demo:diamond-block",
        quantity: Quantity::Infinite,
    },
    BlockItemSpec {
        item_id: "demo:diamond_ore_block",
        block_id: "demo:diamond-ore",
        quantity: Quantity::Infinite,
    },
    BlockItemSpec {
        item_id: "demo:netherrack_block",
        block_id: "demo:netherrack",
        quantity: Quantity::Infinite,
    },
    BlockItemSpec {
        item_id: "demo:glowstone_block",
        block_id: "demo:glowstone",
        quantity: Quantity::Infinite,
    },
    BlockItemSpec {
        item_id: "demo:end_stone_block",
        block_id: "demo:end-stone",
        quantity: Quantity::Infinite,
    },
    BlockItemSpec {
        item_id: "demo:obsidian_block",
        block_id: "demo:obsidian",
        quantity: Quantity::Infinite,
    },
];

fn block_item<I: ItemManagerApi, B: BlockManagerApi>(spec: &BlockItemSpec) -> ItemInstance {
    ItemInstance::with_metadata(
        I::from_string(spec.item_id).expect("default item must exist"),
        generated_item_metadata::ItemMetaSet {
            quantity: Some(spec.quantity),
            place_block: Some(PlaceBlock {
                block: B::from_string(spec.block_id).expect("default block must exist"),
            }),
            ..Default::default()
        },
    )
}

fn flint_and_steel<I: ItemManagerApi>() -> ItemInstance {
    ItemInstance::with_metadata(
        I::from_string("demo:flint-and-steel").expect("flint-and-steel item must exist"),
        generated_item_metadata::ItemMetaSet {
            quantity: Some(Quantity::Infinite),
            portal_igniter: Some(PortalIgniter::new()),
            ..Default::default()
        },
    )
}

struct StorageItemSpec {
    item_id: &'static str,
    block_id: Option<&'static str>,
}

const STORAGE_ITEMS: &[StorageItemSpec] = &[
    StorageItemSpec {
        item_id: "demo:stick",
        block_id: None,
    },
    StorageItemSpec {
        item_id: "demo:anvil_block",
        block_id: Some("demo:anvil"),
    },
    StorageItemSpec {
        item_id: "demo:oak_stairs_block",
        block_id: Some("demo:oak-stairs"),
    },
    StorageItemSpec {
        item_id: "demo:cauldron_block",
        block_id: Some("demo:cauldron"),
    },
];

fn storage_item<I: ItemManagerApi, B: BlockManagerApi>(spec: &StorageItemSpec) -> ItemInstance {
    let mut metadata = generated_item_metadata::ItemMetaSet {
        quantity: Some(Quantity::Infinite),
        ..Default::default()
    };
    if let Some(block_id) = spec.block_id {
        metadata.place_block = Some(PlaceBlock {
            block: B::from_string(block_id).expect("default storage block must exist"),
        });
    }
    ItemInstance::with_metadata(
        I::from_string(spec.item_id).expect("default storage item must exist"),
        metadata,
    )
}
