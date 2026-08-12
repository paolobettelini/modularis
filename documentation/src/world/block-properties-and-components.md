# Block properties and sparse components

The world separates three kinds of block data because they have different
cardinality, ownership, storage, and replication requirements.

```text
BlockState
  BlockId + compact discrete state
  dense, palette-compressed, streamed with chunks

BlockProperties
  defaults/static facts for a BlockId
  registered by mods, not saved once per position

BlockComponents
  optional data for one world position
  sparse, independently persisted, independently replicated
```

This distinction is a hard architecture boundary. Putting per-position data in
the palette makes compression depend on how many unique values happen to exist
in a chunk. A damage counter, UUID, timer, or inventory could otherwise turn a
small palette into thousands of entries.

## Block state is the palette key

`BlockState` contains a generated `BlockStateSet`. State contributors are for
small enumerations that are part of the represented voxel:

- facing direction;
- slab half;
- open or closed;
- a bounded growth stage;
- a small model variant.

The chunk core knows only `BlockState`. It does not know which state fields are
selected by a composition. `BlockStateSet::default()` must be used when a mod
does not set a field, so later contributors remain source-compatible.

## Static block properties

`block-properties-api` provides a type-indexed registry. A property type owns a
stable namespaced ID, value type, and common default:

```rust
pub struct BlockDurabilityProperty;

impl BlockProperty for BlockDurabilityProperty {
    type Value = BlockDurability;
    const ID: &'static str = "vanilla:durability";

    fn default_value() -> BlockDurability {
        BlockDurability::breakable(100)
    }
}
```

A feature mod can override one block type without editing a block enum or
central struct. Keep every optional block/property pairing in its own
contributor mod:

```rust
let mut properties = world.resource_mut::<BlockProperties>();
properties.set::<BlockDurabilityProperty>(
    BlockId::Obsidian,
    BlockDurability::breakable(600),
);
```

`BlockProperties::get` returns the registered override or the property's
default. A new property requires only its own type and the mods that consume or
register it. It does not add a field to every block and is never serialized per
world position.

The active provider is `block-properties-registry-mod`. Vanilla durability
overrides are independent contributors such as
`block-durability-obsidian-vanilla-mod` and
`block-durability-bedrock-vanilla-mod`. Each contributor formally depends on
the one block it configures and registers exactly one `(property, BlockId)`
value. It must not mention unrelated optional blocks.

`block-durabilities-vanilla` is only a convenience modpack that selects the
current vanilla policy set. It contains no Rust behavior. A custom composition
can import that modpack, ignore individual contributors, or select only the
specific durability mods it wants. Aggregating optional block IDs inside one
Rust mod would make those blocks inseparable and is an architecture violation.

## Sparse per-position components

`block-component-api` defines independently typed components:

```rust
#[derive(Serialize, Deserialize)]
pub struct BlockDamage(pub u32);

impl BlockComponent for BlockDamage {
    const ID: &'static str = "vanilla:block-damage";
    const VERSION: u32 = 1;
}
```

The stable ID is the persistence identity. The Rust `TypeId` is used only for
typed runtime access. A component can override `encode` and `decode_version`
when its data format evolves; the default codec uses CBOR.

Storage is shaped as:

```text
chunk key
  -> local block index
       -> component ID
            -> decoded value or preserved opaque payload
```

There is no 4096-entry component array. A chunk with no components has no
component map and no component persistence record. Unknown component IDs and
versions are preserved as opaque payloads during load/save, so temporarily
missing feature mods do not force the generic container to understand their
types.

`block-component-registry-mod` owns the codec registry. A component contributor
such as `block-damage-component-mod` registers only its own codec.

## Durability without palette expansion

Durability demonstrates how properties and components cooperate. Its static
property is one of:

- `BlockDurability::Breakable(0)` for an instant break;
- `BlockDurability::Breakable(n)` for a positive durability;
- `BlockDurability::Unbreakable`.

For finite blocks:

```text
default durability = BlockProperties[BlockDurabilityProperty, BlockId]
damage             = BlockComponents[position, BlockDamage] or 0
remaining          = default durability - damage
```

An untouched block stores no `BlockDamage`. Damage is inserted only after a
successful Survival hit. Restoring the block to its default removes the
component instead of writing zero. Breaking or replacing a block also clears
all components at that position.

The damage value never enters `BlockStateSet`, the local chunk palette, the
world block index, or the normal chunk packet.

The vanilla breaking pipeline is split into replaceable layers:

```text
held left mouse
  -> BlockBreakRequest packets
  -> permission/reach validation
  -> ServerValidatedBlockBreak
       -> Creative listener: immediate mutation
       -> Survival listener: sparse damage accumulation
  -> ServerBlockDamageChanged
  -> scoped damage packet
  -> client six-stage overlay
```

The Survival library groups requests by resolved `ResidentChunkKey` and local
position. Repeated packets from one player in one server update count once,
while distinct players contribute independently. Damage remains after players
stop mining and survives save/load. The current amount per player tick is a
vanilla policy constant, not a chunk-core rule.

Blocks with durability zero mutate on the first validated hit and never
allocate damage.
Unbreakable blocks ignore damage and also never allocate it.

Decoded component values are stored behind a type-erased box. Reads must
dispatch the downcast through the erased inner value, not through the `Box`
container itself. The component API has a regression test for an immediate
`set` followed by typed `get` and `entries`; this is essential because damage
accumulation reads its newly written sparse delta in the same server update.

The animated `ShortGrass` block uses a dedicated
`block-durability-short-grass-vanilla-mod` contributor with
`BlockDurability::Breakable(0)`. The generic durability registry does not know
about short grass, and a composition that omits that block also omits its
override.

The client overlay is another optional mod. It consumes only replicated
discrete stages `0..=6` and renders transparent shape-aware overlays using
`block_breakage1.png` through `block_breakage6.png`. A server can retain damage
without selecting that presentation, or replicate it with another policy.

## Persistence domains

Block components do not extend the region chunk payload. They use
`server-world-data-storage-api`, an opaque write-behind service addressed by:

```rust
WorldDataKey {
    instance,
    domain: "modularis:block-components",
    source,
    partition: chunk_position,
}
```

The block-component domain encodes records containing:

- local block index;
- namespaced component ID;
- component version;
- opaque payload length and bytes.

There is no generated mega-struct containing every possible component. The
binary container can carry components introduced by unrelated mods.

`server-world-data-storage-fs-impl` buffers opaque records and atomically
flushes them under each world's `data/` directory. The storage API is not named
after blocks: future entity, fluid, or item-drop systems can own other
namespaced domains and choose different payloads without entering chunk or
block-component formats. The memory provider supports tests and transient
servers. After dirty data has entered the readable write-behind queue, clean
component maps for chunks outside the resident set are evicted from RAM.

## Persistence is not replication

Registering a persistent component does not put it in normal chunk streaming.
These are independent choices:

- server-only data may never leave the server;
- interaction data may be sent only when a menu opens;
- visible data may be sent only to viewers in the same world scope;
- some data may have a prediction-specific protocol.

Damage uses `block-damage-network-messages-mod` and
`server-block-damage-network-sync-mod`. The sync mod sends progress to matching
world viewers and sends existing damage after the corresponding chunk has been
streamed. The generic component store has no networking dependency.

## Crafting-table state

The crafting table opens a shared `CellMenu`. Its cells currently belong to the
cell-menu subsystem, keyed by the world/block menu ID and governed by an
audience. They are not encoded in `BlockState`, so no palette migration was
needed.

That state is also not silently moved into block components: a cell menu has
its own operation, audience, and replication lifecycle. If persistent crafting
tables are selected later, a bridge mod can serialize the menu payload as a
namespaced block component or as a dedicated menu persistence domain. The
chunk core must remain unaware of either choice.

## Adding another component

1. Define a small serializable type in a feature-owned crate.
2. Implement `BlockComponent` with a namespaced ID and version.
3. Add a registration mod that inserts its codec.
4. Read and mutate it through `ServerBlockComponents` at a public ECS phase.
5. Remove it when the value returns to the semantic default.
6. Select a persistence policy only if the data must survive restart.
7. Add a separate replication mod only for the audiences that need the data.

This same path can later support an owner, machine state, custom override,
timer, or block inventory without changing `ChunkSection`, `BlockStateSet`, or
the world block index.
