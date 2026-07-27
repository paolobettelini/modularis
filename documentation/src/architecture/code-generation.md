# Code generation and registries

Patchwork code generation lets the final composition define typed domain
registries.

The current generated domains are:

- blocks;
- block metadata;
- items;
- item metadata;
- dimensions;
- biomes;
- sounds;
- client settings;
- network messages and Bevy packet events.

## Contributor discovery

Codegen owner crates do not list every contributor as an initialization
dependency. They scan the final composed Cargo project.

For example, a block contributor declares:

```toml
[package.metadata.block]
id = "demo:stone"
```

The block registry generator finds every selected package with
`package.metadata.block`, validates IDs, and generates:

- the `BlockId` enum;
- `ALL_BLOCKS`;
- string conversion;
- mapping to each contributor's `BLOCK_INFO`;
- mapping to each contributor's `RENDER_INFO`.

Sound contributors follow the same pattern with `package.metadata.sound`. The
generated sound registry maps `SoundId` variants to each contributor's
`SOUND_INFO`, including its namespaced asset path.

The owner manifest declares the output:

```toml
[[package.metadata.mod.codegen]]
crate = "generated-block-registry"
version = "0.1.0"
dev_crate = "generated-block-registry"

[package.metadata.mod.codegen.generator]
crate = "block-registry-codegen"
command = "generate"
```

## Why contributors are distributed

The alternative would be one central list:

```toml
blocks = ["stone", "dirt", "grass", ...]
```

inside the registry crate. That would make the registry owner depend on every
feature and require central edits for every extension.

Distributed contributor metadata means:

- adding a block adds one block crate and one modpack entry;
- adding an item metadata field adds one metadata crate;
- adding a network feature contributes its packet types from that feature;
- removing a feature removes its generated variants.

## Generated block and item IDs

Current generated IDs are serializable Rust enums:

```rust
pub enum BlockId {
    Air,
    Bedrock,
    CraftingTable,
    // ...
    Stone,
}

pub enum ItemId {
    BedrockBlock,
    CraftingTableBlock,
    // ...
    FlintAndSteel,
}
```

Code should use the enum when it has a compile-time dependency on the
contributor. Generic systems can use manager APIs:

```rust
let block = B::from_string("demo:stone");
let id = B::id(block);
```

This is useful for data-driven mods that do not want to name a generated
variant directly.

## Metadata code generation

An item metadata contributor declares:

```toml
[package.metadata.item_metadata]
id = "demo:quantity"
field = "quantity"
type = "item_quantity_meta::Quantity"
```

The current generated set is:

```rust
#[derive(Default, Serialize, Deserialize)]
pub struct ItemMetaSet {
    pub favicon: Option<ItemFavicon>,
    pub place_block: Option<PlaceBlock>,
    pub portal_igniter: Option<PortalIgniter>,
    pub quantity: Option<Quantity>,
}
```

Every field is optional. Features test for the metadata they understand.

When constructing metadata, always keep future fields at their default:

```rust
let metadata = ItemMetaSet {
    quantity: Some(Quantity::Finite(64)),
    place_block: Some(PlaceBlock { block: BlockId::Stone }),
    ..Default::default()
};
```

Without the struct update, adding a new generated field would break every item
constructor.

Block metadata follows the same architecture:

```text
BlockInstance = BlockId + BlockMetaSet
```

The current `BlockMetaSet` is empty, but chunk palettes and packets already
store complete `BlockInstance` values. Future orientation, growth, owner, or
color metadata can therefore be added without redesigning chunk storage.

## Dimension code generation

Dimension contributor:

```toml
[package.metadata.dimension]
id = "demo:aether"
```

Generated output:

```rust
pub enum Dimension {
    Aether,
    Nether,
    Overworld,
}
```

Network messages and server definitions use this enum. Portal packets can carry
any selected dimension without a hardcoded Nether-specific message.

## Biome code generation

A biome identity contributor declares only:

```toml
[package.metadata.biome]
id = "demo:forest"
```

`biome-registry-codegen` generates `BiomeId`, `ALL_BIOMES`, string lookup, and
stable namespaced ID conversion. Climate, terrain blocks, visual properties,
and feature lists remain normal Rust registered by server definition mods.

This split keeps identity available independently from the server's selected
world-generation policy. See
[Biomes and world-generation features](../world/biomes.md).

## Settings code generation

A setting contributor declares schema and input provider independently:

```toml
[package.metadata.setting]
id = "controls.sprint_key"
label = "Sprint key"
type = "string"
input = "keybinding"
default = "ControlLeft"
```

The generated registry contains:

- `SettingKey`;
- all selected definitions;
- typed defaults;
- string-to-key conversion.

`type` is the stored data type. `input` is the UI editor provider. A keybinding
is stored as a string but rendered by the `keybinding` input mod.

## Network code generation

Packet ownership is distributed. A feature contributor declares:

```toml
[package.metadata.network.messages]
clientbound = ["sun_network_message_types::SunSettingsChanged"]
serverbound = []
```

Codegen builds:

- `ClientBoundMessage`;
- `ServerBoundMessage`;
- CBOR encode/decode helpers;
- generic received packet messages;
- one typed Bevy message for every packet variant;
- `NetworkMessageEventsPlugin`;
- `NetworkMessageSet::ReceivePackets` and `DispatchPackets`.

The transport emits one generic packet message. The generated plugin dispatches
it into typed messages such as `SunSettingsChangedReceived` or
`ChunkRequestReceived`.

Feature systems therefore do not match a central packet enum themselves.

## Adding a codegen domain

A new generated domain usually needs:

1. a contributor metadata schema;
2. an owner mod with `package.metadata.mod.codegen`;
3. a generator command;
4. a development generated crate;
5. validation for duplicate IDs and invalid type paths;
6. a modpack that selects owner and contributors;
7. consumers that depend on the owner or generated crate.

Keep domain meaning in the generator crate, not in Patchwork core. Patchwork
should understand that code must be generated, but not what a recipe, biome, or
status effect means.
