# Portals

Portals demonstrate how one feature can be split into reusable geometry,
rules, state, item behavior, networking, travel, and rendering.

## Feature layers

```text
portal-api                     shared geometry
server-portal-api              rules, active state, ECS sets
server-portal-state-mod        resources and messages
portal rule mods               frame block, destination, color
server-portal-ignite-*         item-use recognition and frame validation
server-portal-network-sync     audience-aware packets
server-portal-travel-*         collision, dimension request, return portal
client-portal-render-*         transparent visual
```

## Portal geometry

`PortalFrame` contains:

- outer origin;
- axis `X` or `Z`.

The vanilla frame occupies a `4x5` outer rectangle with a `2x3` interior.

The four outer corners are optional. `required_frame_blocks()` yields ten
required blocks. `interior_blocks()` yields six positions.

`find_ignitable_frame` tests candidate origins and axes around the clicked
interior face.

Geometry is shared and independent from:

- obsidian;
- glowstone;
- Nether;
- Aether;
- rendering color.

## Portal rule

```rust
pub struct PortalRule {
    pub id: String,
    pub frame_block: BlockId,
    pub destination: Dimension,
    pub return_destination: Dimension,
    pub color: [f32; 4],
}
```

The rule chooses destination based on source:

- from the destination dimension, travel to `return_destination`;
- otherwise travel to `destination`.

Current rules:

| Rule | Frame | Forward destination | Color |
| --- | --- | --- | --- |
| Nether | Obsidian | Nether | transparent red |
| Aether | Glowstone | Aether | transparent blue |

Rules are separate vanilla mods. A custom server can select either, both, or
neither.

The rule registry rejects duplicate rule IDs and duplicate frame blocks.

## Ignition

Portal ignition listens to `HeldItemUseDispatched` in:

```text
InventoryServerSet::ApplyWorldEffects
ServerPortalSet::Ignite
```

It requires:

- item metadata `PortalIgniter`;
- a block target;
- clicked block matching a registered frame rule;
- a valid hollow frame;
- air in every interior position;
- a resolvable world scope.

On success it inserts an `ActivePortal`, emits `ServerPortalOpened`, and emits
`ItemUseSucceeded`.

Quantity consumption remains a separate inventory feature.

The flint-and-steel item is not hardcoded in portal code. Any item instance with
the metadata can ignite a portal.

## Active portal state

An active portal stores:

```rust
pub struct ActivePortal {
    pub scope: WorldScopeId,
    pub frame: PortalFrame,
    pub frame_block: BlockId,
    pub destination: Dimension,
    pub destination_position: Option<[f32; 3]>,
    pub color: [f32; 4],
}
```

Scope is essential. Identical frame coordinates can exist in several
dimensions or instances.

Duplicate frame/scope insertions are ignored.

## Travel

The vanilla travel mod runs after authoritative movement apply. It:

1. resolves the player's current world scope;
2. finds an active portal intersecting that player's current scaled hitbox;
3. checks a cooldown;
4. emits `RequestPlayerDimensionChange`;
5. remembers enough information to create a return portal.

It does not directly mutate dimension state.

## Return portals

When the dimension change is applied, the travel mod creates a return frame
near the destination spawn if no matching portal exists.

It:

- places required frame blocks through `ServerChunkWorld`;
- clears interior blocks;
- emits ordinary block edit result messages;
- inserts a linked active portal with destination position beside the source
  frame;
- emits `ServerPortalOpened`.

Therefore normal chunk update synchronization informs other players.

The Nether return portal uses obsidian. The Aether return portal uses glowstone.

## Network synchronization

The portal packet is generic:

```text
PortalOpenedPacket {
    frame,
    destination: Dimension,
    color
}
```

It is sent only to players in the relevant world scope.

The client renderer does not branch on Nether or Aether. It renders a thin
unlit cuboid in the frame interior using the packet color.

Portal visuals are removed when:

- their chunk unloads;
- dimension changes;
- the game exits.

## Adding a portal rule

If existing `4x5` geometry and ignition behavior are sufficient:

1. add a frame block and destination dimension;
2. register a `PortalRule` in a new mod;
3. add it to a feature modpack;
4. ensure the destination definition/provider exists;
5. recompose both sides.

If geometry differs, add a new ignition feature rather than complicating the
generic `PortalRule` with unrelated optional fields.

## Current limitations

- active portal state is in memory;
- destroying a frame does not currently include a full portal invalidation
  pipeline;
- return portal placement searches a small fixed set near spawn;
- only vertical X/Z frames are modeled;
- the client visual is a static transparent cuboid.

These are feature-level limits, not reasons to merge portal behavior into the
dimension or chunk world.
