# Blocks, items, and metadata

Blocks and items are generated domains, but their behavior is not centralized
in the generated enums.

## Block contributor

A block crate contributes its namespaced ID:

```toml
[package.metadata.block]
id = "example:marble"
```

The Rust crate exports logical and render definitions:

```rust
use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};

pub struct MarbleBlock;

impl Block for MarbleBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "example:marble",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MarbleBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-marble:block/marble"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MarbleBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MarbleBlock::RENDER;
```

Only the ID is duplicated between Cargo metadata and Rust because Cargo
metadata drives discovery while Rust constants drive runtime behavior.

## Block logical properties

`BlockInfo` contains:

- `id`: stable namespaced identifier;
- `is_air`: semantic empty-block check;
- `solid`: collision behavior;
- `opaque`: face culling and ambient occlusion behavior.

These values belong in Rust, not in a central manifest.

`is_air`, `solid`, and `opaque` are independent. A future glass block can be
solid but not opaque.

## Block rendering

Render shapes are:

- `Invisible`;
- `Model` for JSON-backed geometry;
- the legacy `Cube` variant, retained for alternate or transitional meshers.

The active client composition uses `RenderShape::Model`. Model and texture
assets live under:

```text
assets/models/block/<name>.json
assets/textures/block/<name>.png
```

and are referenced with namespaced resource IDs. Shared cube, column, stairs,
item, and anvil templates are exported by asset-only mods. See
[JSON voxel models and textures](./voxel-models.md).

Logical properties such as `solid` and `opaque` are still Rust data. They are
not inferred from the JSON model because collision, culling, and presentation
must remain independently replaceable.

## Block manager

`block-manager-api` exposes:

```rust
pub trait BlockManagerApi {
    fn info(block: BlockId) -> &'static BlockInfo;
    fn render_info(block: BlockId) -> &'static BlockRenderInfo;
    fn all() -> &'static [BlockId];
    fn from_string(id: &str) -> Option<BlockId>;
    fn id(block: BlockId) -> &'static str;
}
```

`block-manager-generated-impl` provides this API using the generated registry.

Consumers should depend on `BlockManagerApi` when they only need general block
properties. A feature that requires one exact block may depend on its
contributor and generated variant.

## Block instances and metadata

Placed world data is:

```rust
pub struct BlockInstance {
    pub block: BlockId,
    pub metadata: BlockMetaSet,
}
```

Construct a metadata-free block with:

```rust
BlockInstance::new(BlockId::Stone)
```

or:

```rust
BlockId::Stone.into()
```

`BlockMetaSet` is generated. It is empty today, but block instances are already:

- palette keys;
- serialized in chunks;
- stored in edit overlays;
- sent in block update packets.

Adding a metadata contributor later can distinguish two instances of the same
block ID in the palette.

Example future contributor:

```toml
[package.metadata.block_metadata]
id = "example:facing"
field = "facing"
type = "block-facing-meta::Facing"
```

Code that creates explicit metadata should use:

```rust
BlockMetaSet {
    facing: Some(Facing::North),
    ..Default::default()
}
```

## Item contributor

An item contributes an ID:

```toml
[package.metadata.item]
id = "example:marble_block"
```

and exports:

```rust
impl Item for MarbleBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "example:marble_block",
        label: "Marble",
    };
}

impl ItemRender for MarbleBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-marble-block:item/marble_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MarbleBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <MarbleBlockItem as ItemRender>::RENDER;
```

The item registry generates `ItemId` and lookup functions. The manager API
exposes all items, IDs, labels, string conversion, and render information.

## Item instances

Inventory cells contain:

```rust
Option<ItemInstance>
```

where:

```rust
pub struct ItemInstance {
    pub item: ItemId,
    pub metadata: ItemMetaSet,
}
```

An inventory slot is not a hardcoded stack type.

Current metadata:

| Metadata | Meaning |
| --- | --- |
| `Quantity` | `Finite(u32)` or `Infinite` |
| `PlaceBlock` | Block placed by vanilla item-use behavior |
| `ItemFavicon` | Optional per-instance client UI image override |
| `PortalIgniter` | Marker understood by portal ignition |

Each metadata type is a separate contributor mod.

## Behavior belongs outside metadata

Metadata describes capability or data. A separate mod implements semantics:

- quantity stacking: `server-inventory-quantity-stacking-mod`;
- quantity consumption: `server-item-quantity-consumption-mod`;
- place-block use: `server-place-block-item-use-mod`;
- favicon rendering: `client-item-favicon-ui-mod`;
- quantity rendering: `client-item-quantity-ui-mod`;
- portal ignition: `server-portal-ignite-vanilla-mod`.

Selecting `item-place-block-meta` does not automatically enable block placement.
A server may understand the data but choose another behavior.

## Future-proof instance creation

Always use generated defaults:

```rust
ItemInstance::with_metadata(
    ItemId::MarbleBlock,
    ItemMetaSet {
        quantity: Some(Quantity::Finite(64)),
        place_block: Some(PlaceBlock {
            block: BlockId::Marble,
        }),
        ..Default::default()
    },
)
```

This allows another metadata contributor to add a field without breaking the
constructor.

## Adding a block and its item

1. Create the block contributor, JSON model, and texture assets.
2. Add it to `blocks.toml`.
3. Create an item contributor.
4. Add it to `items.toml`.
5. If it should be placeable, create `PlaceBlock` metadata when instantiating
   the item.
6. If it should appear in a default loadout, update or replace the loadout mod.
7. Give the item a JSON model and export `ITEM_RENDER_INFO`.
8. Recompose both client and server.

Do not add place behavior to the item contributor itself. The item definition
must remain usable in servers with different rules.

The demo currently follows this process for every non-air block: 33 block
contributors have 33 matching block-item contributors, while `block-air`
intentionally has no inventory item. This is checked at composition through the
generated item registry, not by coupling block codegen to item codegen. A custom
server is still free to omit any of those item contributors.
