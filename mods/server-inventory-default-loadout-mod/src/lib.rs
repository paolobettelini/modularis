use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use inventory_core_api::{Inventory, InventoryCell};
use inventory_events_api::{
    InventoryResetRequested, InventorySyncRequested, InventoryValidationSet,
};
use inventory_events_mod::InventoryEventsMod;
use item_favicon_meta::ItemFavicon;
use item_instance_api::ItemInstance;
use item_manager_api::ItemManagerApi;
use item_place_block_meta::PlaceBlock;
use item_portal_igniter_meta::PortalIgniter;
use item_quantity_meta::Quantity;
use server_inventory_api::{ServerInventories, ServerInventoryApi};
use server_inventory_layout_api::ServerInventoryLayoutApi;
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct ServerInventoryDefaultLoadoutMod<L, I, B>(PhantomData<(L, I, B)>);

impl<L: ServerInventoryLayoutApi, I: ItemManagerApi, B: BlockManagerApi>
    ServerInventoryDefaultLoadoutMod<L, I, B>
{
    pub fn init<S: ServerInventoryApi>(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _layout: &mut L,
        _items: &mut I,
        _blocks: &mut B,
        _metadata: &mut item_metadata_registry_codegen::ItemMetadataRegistryCodegenMod,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _inventories: &mut S,
        _favicon: &mut item_favicon_meta::ItemFaviconMetaMod,
        _portal_igniter: &mut item_portal_igniter_meta::ItemPortalIgniterMetaMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            ensure_default_loadout::<L, I, B>.in_set(InventoryValidationSet::Initialize),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn ensure_default_loadout<L: ServerInventoryLayoutApi, I: ItemManagerApi, B: BlockManagerApi>(
    inventories: Res<ServerInventories>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut syncs: MessageReader<InventorySyncRequested>,
    mut resets: MessageWriter<InventoryResetRequested>,
) {
    for event in joined.read() {
        write_default_reset::<L, I, B>(event.player_id, &mut resets);
    }
    for event in syncs.read() {
        if inventories.get(event.player_id).is_none() {
            write_default_reset::<L, I, B>(event.player_id, &mut resets);
        }
    }
}

fn write_default_reset<L: ServerInventoryLayoutApi, I: ItemManagerApi, B: BlockManagerApi>(
    player_id: u64,
    resets: &mut MessageWriter<InventoryResetRequested>,
) {
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
    resets.write(InventoryResetRequested {
        player_id,
        inventory,
        selected_hotbar: 0,
    });
}

struct BlockItemSpec {
    item_id: &'static str,
    block_id: &'static str,
    quantity: Quantity,
    favicon: &'static str,
}

const BLOCK_ITEMS: &[BlockItemSpec] = &[
    BlockItemSpec {
        item_id: "demo:grass_block",
        block_id: "demo:grass",
        quantity: Quantity::Finite(64),
        favicon: "block-grass/grass.png",
    },
    BlockItemSpec {
        item_id: "demo:stone_block",
        block_id: "demo:stone",
        quantity: Quantity::Infinite,
        favicon: "block-stone/stone.png",
    },
    BlockItemSpec {
        item_id: "demo:bedrock_block",
        block_id: "demo:bedrock",
        quantity: Quantity::Infinite,
        favicon: "block-bedrock/bedrock.png",
    },
    BlockItemSpec {
        item_id: "demo:crafting_table_block",
        block_id: "demo:crafting-table",
        quantity: Quantity::Infinite,
        favicon: "block-crafting-table/crafting_table_top.png",
    },
    BlockItemSpec {
        item_id: "demo:diamond_block",
        block_id: "demo:diamond-block",
        quantity: Quantity::Infinite,
        favicon: "block-diamond-block/diamond_block.png",
    },
    BlockItemSpec {
        item_id: "demo:diamond_ore_block",
        block_id: "demo:diamond-ore",
        quantity: Quantity::Infinite,
        favicon: "block-diamond-ore/diamond_ore.png",
    },
    BlockItemSpec {
        item_id: "demo:netherrack_block",
        block_id: "demo:netherrack",
        quantity: Quantity::Infinite,
        favicon: "block-netherrack/netherrack.png",
    },
    BlockItemSpec {
        item_id: "demo:glowstone_block",
        block_id: "demo:glowstone",
        quantity: Quantity::Infinite,
        favicon: "block-glowstone/glowstone.png",
    },
    BlockItemSpec {
        item_id: "demo:end_stone_block",
        block_id: "demo:end-stone",
        quantity: Quantity::Infinite,
        favicon: "block-end-stone/end_stone.png",
    },
    BlockItemSpec {
        item_id: "demo:obsidian_block",
        block_id: "demo:obsidian",
        quantity: Quantity::Infinite,
        favicon: "block-obsidian/obsidian.png",
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
            favicon: Some(ItemFavicon::new(spec.favicon)),
            ..Default::default()
        },
    )
}

fn flint_and_steel<I: ItemManagerApi>() -> ItemInstance {
    ItemInstance::with_metadata(
        I::from_string("demo:flint-and-steel").expect("flint-and-steel item must exist"),
        generated_item_metadata::ItemMetaSet {
            quantity: Some(Quantity::Infinite),
            favicon: Some(ItemFavicon::new("item-flint-and-steel/flint-and-steel.png")),
            portal_igniter: Some(PortalIgniter::new()),
            ..Default::default()
        },
    )
}
