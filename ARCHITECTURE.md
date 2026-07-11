# Minecraft Simple Demo architecture

This document describes only the `minecraft_simple_demo` project. For the
general Patchwork philosophy, read the root
`LLM_COMPOSER_ARCHITECTURE_GUIDE.md`.

The demo is a small local Minecraft-like client/server game built as a set of
Patchwork mods. It is intentionally not a monolith. Blocks, items, inventory,
movement, networking, rendering and vanilla gameplay are split into separate
crates so that a modpack can choose which behavior exists.

## Modpack layout

The important modpacks are:

- `common.toml`: shared foundations used by client and server;
- `blocks.toml`: block contributors and block registry generation;
- `items.toml`: item contributors and item metadata generation;
- `dimensions.toml`: dimension contributors and generated dimension enum;
- `network.toml`: network message contributors and generated packet enums;
- `client-vanilla.toml`: optional vanilla client controls and prediction;
- `client-graphics.toml`: optional ambient light, sun, shadows and mesh shading;
- `client.toml`: the playable Bevy client;
- `server-base.toml`: server transport, sessions and authoritative pipelines;
- `server.toml`: the demo server;
- `server-vanilla.toml`: optional vanilla server gameplay features.

`client.toml` imports both its vanilla behavior and graphics packs, while
`server.toml` imports its vanilla feature pack. Those packs are deliberately
separate. `server.toml` composes
`server-base.toml`, the vanilla pack, the Perlin terrain provider, a routing
policy and a player-interest residency policy. A custom server can reuse the
base, independently select different providers, routing, residency and
visibility policies, and add only the behavior mods it wants.

## Bevy foundation

`bevy-mod` is neutral. It owns only the creation of `App::new()`.

Client and server behavior is added by separate mods:

- `client-bevy-default-plugins-mod` installs Bevy default plugins, window,
  renderer and texture settings;
- `server-bevy-runner-mod` installs a server runner without opening a window;
- `client-game-bootstrap-mod` owns and runs the client app;
- `server-game-bootstrap-mod` owns and runs the server app.

The server bootstrap is intentionally thin. It should not list gameplay
features directly. Gameplay comes from selected server feature mods and from
the modpacks imported by `server.toml`.

## Generated registries

The demo uses codegen for domain registries selected by the modpack.

Codegen owner crates do not list concrete contributors as Rust or initialization
dependencies. They scan the final composed project instead. Therefore adding or
removing a block, item, metadata or setting contributor is a modpack decision;
the codegen owner does not need to be edited.

Dimension declarations follow the same pattern. `dimension-overworld`,
`dimension-nether` and `dimension-aether` contribute IDs through Cargo
metadata. `dimension-registry-codegen` generates the shared, serializable
`Dimension` enum. Runtime services and network packets use this enum instead of
duplicating dimension strings.

### Blocks

Block mods such as `block-air`, `block-dirt`, `block-stone` and
`block-grass` contribute block definitions. `block-registry-codegen` generates
typed block IDs and registry helpers. `block-manager-generated-impl` provides
the block manager API from the generated registry.

A block contributor declares only its namespaced ID in Cargo metadata:

```toml
[package.metadata.block]
id = "demo:stone"
```

Logical and render properties live in that mod's Rust code as `BLOCK_INFO` and
`RENDER_INFO`; they are not duplicated in the manifest. `BlockRenderInfo` has no
fallback color. A visible untextured block is white, and textures are either:

```rust
BlockTextures::Uniform("block-stone/stone.png")
```

or an explicit `BlockTextures::PerFace { east, west, top, bottom, south,
north }`. Requiring all six paths prevents an omitted face from silently using
the wrong image. The crafting-table contributor demonstrates per-face textures.

### Items and metadata

Item mods such as `item-dirt-block` and `item-stone-block` contribute item
definitions. `item-registry-codegen` generates item IDs.

Metadata is separate from items:

- `item-quantity-meta` contributes finite/infinite quantity metadata;
- `item-place-block-meta` contributes metadata that says an item can place a
  specific block.
- `item-favicon-meta` contributes an optional UI favicon path for item
  instances.
- `item-portal-igniter-meta` marks an item as able to trigger portal rules. The
  flint-and-steel item uses this metadata; it does not contain a hardcoded
  destination.

`item-metadata-registry-codegen` generates the metadata set used by
`ItemInstance`.

The inventory base never assumes that an item is a stack. It stores:

```text
Option<ItemInstance>
ItemInstance = ItemId + ItemMetaSet
```

When a mod creates an item with explicit metadata, it should use a struct update
such as `ItemMetaSet { quantity: Some(...), ..Default::default() }`. This keeps
the code future-proof when other mods add metadata fields.

Stacking, consumption, quantity rendering and favicon rendering are separate
mods. If `item-favicon-meta` is present on an item, `client-item-favicon-ui-mod`
renders the image and the fallback item label is hidden.

### Network messages

Network messages are not all declared in one central manifest. Each feature or
domain contributes its own message type crate or message contributor mod. The
network codegen then builds the generated clientbound/serverbound packet enums.

This keeps the protocol moddable. Adding a feature can add only the packet
types owned by that feature.

## Network transport and routing

The active transport is TCP:

- `client-network-tcp-impl`;
- `server-network-tcp-impl`;
- shared framing is in `network-framing-api`.

The framing layer uses length-prefixed packets. The TCP implementations queue
outgoing data and flush it without blocking the Bevy schedule for normal use.

Gameplay mods should not decide concrete routing themselves. Server-side
outgoing packets are emitted through `server-network-events-api`:

```text
ServerPacketOut {
    audience: ServerAudience,
    packet,
}
```

`server-network-router-mod` is responsible for mapping audiences to the active
transport:

- a specific address;
- a specific player;
- broadcast;
- broadcast except one address;
- broadcast except one player;
- an explicit list of players.

`server-player-visibility-api` decides which clients receive player join,
movement and leave updates. The active
`server-player-visibility-world-instance-mod` exposes only players routed to the
same world instance. A server can replace it with distance, team, permission or
other visibility rules without changing sessions or the transport. Future chat
features can emit the same explicit-player audience.

## Player sessions and registry

`server-player-session-mod` handles join/leave and movement packets at the
session layer. `server-player-registry-api` provides the server-side player
registry and public movement pipeline types.

Player lifecycle events are separate:

- `server-player-lifecycle-events-mod` exposes events for join/leave style
  behavior;
- inventory loadout, gravity sync and other features listen to those events.

The registry is not meant to be a gameplay god object. It is a shared state and
lookup service used by separate feature mods.

## Movement model

Movement follows a client-predicted, server-validated design.

The client does not wait for a server round trip before moving the local camera.
Instead:

1. client input updates local movement intent;
2. client movement/prediction applies immediate local motion;
3. the client sends `PlayerMove` and feature-specific intent packets;
4. the server validates plausibility and collisions;
5. accepted movement is synchronized to other clients;
6. invalid movement can be corrected by the server.

This avoids shaky local camera motion during jump/fall while keeping the server
authoritative.

Local-player `PlayerMoved` packets are correction samples, not continuously
refreshed interpolation targets. `client-player-network-sync-mod` consumes each
position correction once. Keeping an old server position as a target across
many frames would make it fight local gravity and create vertical jitter.

The shared movement pipeline in `server-player-registry-api` is:

```text
ServerPlayerMovementSet::Receive
ServerPlayerMovementSet::Validate
ServerPlayerMovementSet::Apply
ServerPlayerMovementSet::Sync
```

`server-player-session-mod` collects received movement into pending movement
requests, applies accepted movement and emits sync packets through
`ServerPacketOut`.

`server-player-movement-collision-vanilla-mod` is the vanilla validator that
checks movement against blocks and clamps suspicious movement. It is a vanilla
feature, not a hardcoded server base rule.

On the client, `client-player-controller-api` exposes
`PlayerPlanarMovementIntent` and this ordered pipeline:

```text
Input -> MovementModifiers -> ApplyMovementIntent -> Forces -> ForceOverrides -> Movement
```

The FPS controller writes the base direction. Optional feature mods can change
the intent without being compiled into the controller. For example,
`client-player-sprint-vanilla-mod` multiplies movement speed while the configured
sprint key is held. Removing that mod removes sprint behavior entirely.

`ForceOverrides` is the extension point for optional movement modes that must
run after ordinary forces without being hardcoded into gravity or the base FPS
controller. Flight uses this phase to replace only the velocity component along
the current gravity-up axis.

The shared block collision resolver applies height-axis movement before planar
movement. This lets a rising player clear a ledge before testing its side. The
client grounded probe is only a small contact tolerance: it cannot cancel a
real falling velocity or mark a player grounded while moving away from the
surface. These rules prevent alternating penetration corrections and repeated
small bounces while keeping collision policy outside the controller input mods.

## Gravity and jump

Gravity is a separate feature family.

- `player-gravity-api` defines the `Gravity(Vec3)` resource and helpers for
  "down", "up" and orientation from gravity.
- `player-gravity-vanilla-mod` provides the default gravity resource.
- `player-gravity-network-messages-mod` contributes gravity sync packets.
- `server-player-gravity-network-sync-mod` sends gravity changes to clients.
- `client-player-gravity-network-receive-mod` receives server gravity updates.
- `client-player-gravity-prediction-vanilla-mod` applies local predicted
  gravity for the local player.

Jump is also separate:

- `client-player-jump-vanilla-mod` applies immediate local jump prediction and
  sends a jump intent;
- `server-player-jump-vanilla-mod` checks jump intent against the current
  grounded state, while actual movement correction stays in the movement
  validation pipeline;
- jump direction is computed opposite to the current gravity vector.

Gravity is currently a global resource. The architecture is moving toward
supporting per-player or per-world gravity, but the current demo uses one
runtime-editable gravity value for all players.

## Flight capability

Flight is split into authority, synchronization, state and vanilla controls:

- `player-flight-api` defines local capability/state, tuning and the public
  capability-change event;
- `server-player-flight-api` defines per-player server capability state and the
  `Apply -> Sync` ECS pipeline;
- `server-player-flight-capability-mod` is the neutral authoritative provider.
  Its default is disabled for every player;
- `server-player-flight-network-sync-mod` sends capability grants and
  revocations to the affected player;
- `client-player-flight-state-mod` owns client state without choosing controls;
- `client-player-flight-network-receive-mod` applies server updates and
  immediately exits flight when capability is revoked;
- `client-player-flight-vanilla-mod` implements the optional double-jump
  toggle, jump-key ascent and Shift descent;
- `server-player-flight-grant-all-vanilla-mod` is the optional policy that
  grants capability to every joining player.

The capability is therefore server-controlled, while the selected vanilla
client predicts flight locally so the camera does not wait for a network round
trip. The neutral capability provider and synchronization live in
`server-base.toml`; only grant-all lives in `server-vanilla.toml`. A custom
server can omit the grant-all policy, grant only selected players, revoke
flight at runtime, or replace the policy without changing movement, networking
or UI mods.

## Camera, player rendering and Blocky models

The local camera follows the client-predicted local controller position. This
is why jumping and falling do not depend on network latency.

Remote player rendering is selected through `client-player-render-api`. The old
`client-player-render-bevy-impl` sphere renderer is still a replaceable
implementation, but the active client modpack uses
`client-player-render-blocky-impl`.

The Blocky player renderer is deliberately layered:

- `blocky_formats` parses Hytale/Blockbench `.blockymodel` and `.blockyanim`
  files;
- `blocky-model-api` defines ECS messages/components for spawning models;
- `client-blocky-model-bevy-render-mod` turns a `.blockymodel` into Bevy
  entities;
- `blocky-animation-api` defines the animation service seam;
- `client-blocky-animation-bevy-mod` applies `.blockyanim` samples;
- `client-player-blocky-model-paths-api` exposes the active player model,
  texture, animation paths, atlas size and model tuning values;
- `client-player-blocky-model-paths-mod` is the current Outlander player asset
  provider;
- `client-player-render-blocky-impl` listens to network player events and
  requests Blocky model spawns/animations.

The Blocky runtime creates one bone entity per model node and a separate
optional visual child for the node shape:

```text
BlockyModelRoot
  BlockyModelNode        # hierarchy, position, orientation
    BlockyModelVisual?   # mesh, shape offset, shape stretch, visibility
```

This separation matters. Bone animation should move/rotate the hierarchy, while
shape animation such as `shapeStretch` or `shapeVisible` should affect only the
visual mesh, not the node's children.

Coordinates are converted from Blocky/Hytale units to Bevy world units through
the `primitive_scale` stored on each `BlockyModelNode`. Animated node positions
are applied as:

```text
base_translation + animated_position * primitive_scale * mask
```

The encoded position of a child node is relative to the center of its parent's
main shape. `blocky_formats::RuntimeModel::resolved_local_position` restores
the parent's `shape.offset` before the Bevy hierarchy is built. Reading the raw
child `position` as a direct pivot-to-pivot translation compresses the
hierarchy and creates visible gaps, especially along the vertical axis.

The optional translation mask is an ECS component used by feature mods that need
to ignore part of a clip's positional animation. The player renderer uses it to
ignore vertical pelvis bob from the walk clip, because the gameplay/network
transform already owns the player's world height.

Shape meshes are generated centered around their visual origin using absolute
shape size. `shape.offset` becomes the visual child's local translation, and
the sign/value of `shape.stretch` is preserved in the visual child's
`Transform.scale`. This avoids negative primitive dimensions and keeps mesh
geometry aligned with animation samples.

Player orientation can be updated from network packets, and gravity helpers
define how "up" should be interpreted. The player asset provider can also
define a yaw offset for imported model forward-axis conventions.

The local player's own avatar is not rendered on that player's client.

## Input and hotbar selection

Input is split into small feature mods:

- `client-input-bevy-impl` is a neutral Bevy input/cursor backend;
- `client-crosshair-bevy-mod` owns the centered `+` HUD crosshair and can be
  omitted or replaced independently from camera/raycast logic;
- `client-pause-menu-input-mod` owns Escape-to-pause behavior;
- `client-inventory-toggle-input-mod` owns the configurable inventory toggle;
- `client-hotbar-selection-input-mod` handles number-key hotbar selection;
- the mouse-wheel hotbar cycling feature is separate from number-key selection;
- jump key settings are contributed by `client-setting-jump-key`;
- sprint key settings are contributed by `client-setting-sprint-key` and use
  left Control by default;
- inventory key settings are contributed by `client-setting-inventory-key`.

The backend does not reference jump or inventory setting keys. This means a
client composition can omit those features without leaving generated enum
references in the base input implementation.

The server can also send a packet telling the client which hotbar cell is
selected. That packet is part of the inventory/hotbar synchronization family,
not the input backend.

## Block interaction and world edits

Block edit logic uses a pending-request pipeline rather than direct mutation.

The public set chain is defined by `block-edit-events-api`:

```text
ServerBlockEditSet::Receive
ServerBlockEditSet::Collect
ServerBlockEditSet::Validate
ServerBlockEditSet::Apply
ServerBlockEditSet::Sync
```

The current shape is:

- client raycast/input mods create block break/place intentions;
- network send mods translate those intentions into packets;
- `server-block-edit-network-receive-mod` authenticates the packet source and
  emits a server request containing the acting player ID;
- `server-block-edit-world-mod` collects and applies accepted edits to the
  chunk world;
- `server-block-edit-network-send-mod` syncs successful edits through
  `ServerPacketOut`.

Validators can be inserted before `Apply` to deny, mutate or react to block
edits. This is the point where permissions, protected regions, tools, damage or
custom rules should be added.

Client interaction distance is not hardcoded in the raycast feature.
`client-block-interaction-rules-api` exposes the current reach resource and
`client-block-interaction-rules-vanilla-mod` provides the demo value. A custom
client composition can replace that provider. Voxel traversal treats only an
actual equal-time grid crossing as an edge or corner; a ray merely close to an
edge keeps the face of the voxel it really entered, so placement uses the exact
`VoxelRayHit::adjacent` cell.

Block outlines are another independent client feature family:

- `client-block-outline-api` defines owner-keyed outline commands and the
  `Collect -> Apply -> Draw` ECS set chain;
- `client-block-outline-bevy-mod` is the replaceable Bevy/Gizmos renderer and
  can display requests from several mods at once;
- `client-looked-block-outline-vanilla-mod` performs the vanilla camera raycast
  within the configured reach and owns only its `vanilla:looked-block` outline.

The renderer knows nothing about cameras, chunks, reach or vanilla targeting.
Other mods can therefore add selection, region or debug outlines through the
same API, while a client can omit the looked-block behavior or replace the
renderer entirely.

The current vanilla client composes only `client-crosshair-bevy-mod`, which
renders the centered HUD `+`. The outline API, renderer and looked-block policy
remain available as optional mods for a client that wants target highlighting.
They do not compete for cursor positioning and either feature can be selected
without changing raycast code.

The vanilla pack currently selects `server-block-break-reach-vanilla-mod` and a
replaceable `server-block-interaction-rules-api` provider. Placement and breaking
share gravity-aware eye position and reach rules instead of assuming world +Y.

## Block placement as a vanilla item feature

Right-click does not hardcode "place stone" or "place dirt" anymore.

The vanilla placement feature is:

- an item has optional `PlaceBlock { block: BlockId }` metadata;
- `server-place-block-item-use-mod` listens to held-item use events;
- if the metadata exists and placement is valid, it calls
  `ServerChunkWorld::place_block`;
- on success it emits item-use success events;
- quantity consumption is handled by a separate item quantity consumption mod.

This means a server can have items and blocks without enabling vanilla block
placement, or it can replace placement with a different rule.

## Chunk world

`server-chunk-world-api` defines the server-side chunk world operations:

- viewer-aware block and chunk queries;
- viewer-aware place, set and break operations;
- lazy resident-cache keys;
- cache retention/eviction operations.

Chunk behavior is split into independent layers:

- `server-chunk-provider-api` is a registry of named chunk sources. Several
  providers can be registered in the same server;
- `server-chunk-routing-api` maps `(viewer, chunk position)` to a provider and a
  `WorldInstanceId`;
- `server-chunk-routing-dimensions-mod` is the active routing provider and maps
  each player through the dimension registry;
- `server-chunk-routing-single-world-mod` remains a simpler alternative for a
  server that does not need dimensions;
- `server-chunk-world-dynamic-impl` is the lazy cache and edit-overlay world
  implementation;
- `server-chunk-residency-api` describes the active interest window;
- `server-chunk-residency-player-interest-vanilla-mod` keeps only loaded chunks
  around current players and periodically evicts the others;
- `server-chunk-provider-perlin-mod` is the active terrain source;
- `server-chunk-provider-nether-mod` is a separate Nether terrain source using
  bedrock, netherrack and occasional obsidian;
- `server-chunk-provider-aether-mod` generates floating grass islands with
  dirt, stone and occasional glowstone;
- `server-chunk-provider-checkerboard-mod` remains an unbounded alternative
  primary provider for testing or custom modpacks.

The provider is no longer responsible for deciding which chunks stay in RAM.
Chunks are generated on demand at any coordinate. Eviction removes the packed
base chunk from the resident cache, while sparse block edits stay in a separate
overlay and are reapplied if the provider regenerates the chunk.

The request mod validates requests against the selected residency policy. This
prevents a client from forcing arbitrary distant chunks into memory while still
allowing a custom server to replace the policy. The vanilla player-interest
provider keeps one chunk of slack beyond the client's maximum render radius so
requests near a chunk boundary remain valid while the latest predicted player
position is in transit.

`WorldInstanceId` and provider/source form a `WorldScopeId`. That scope is part
of cache keys and block mutations. Breaking, placement, collision and
crafting-table lookup use the acting player as query context. Block updates are
sent only to players routed to the same scope.

Placed blocks are stored as `BlockInstance`, not only `BlockId`:

```text
BlockInstance = BlockId + BlockMetaSet
```

`block-metadata-registry-codegen` generates `BlockMetaSet`. It can currently be
empty, but the storage/network shape already supports future metadata such as
orientation, growth stage, ownership, color or custom state.

Chunk storage uses compact sections:

```text
ChunkSection
  palette: index -> block instance
  reverse map: block instance -> index
  bits per entry
  packed u64 data
```

`packed-bit-array-api` contains the bit-packing helper. The same compact model
is suitable for memory storage and network payloads.

## Dimensions and portals

Dimensions are registry entries, not branches hardcoded into the chunk world:

```text
DimensionDefinition
  id
  WorldInstanceId
  ChunkProviderId
  sky color
  spawn position
```

The implementation is split into:

- `server-dimension-registry-mod`: provides the generic dimension API and
  change-event pipeline;
- `server-dimension-overworld-mod`: contributes the default Overworld;
- `server-dimension-nether-mod`: contributes the Nether and associates it with
  the Nether chunk provider;
- `server-dimension-aether-mod`: contributes the Aether and associates it with
  the floating-island provider;
- `server-player-dimension-lifecycle-vanilla-mod`: assigns joining players and
  applies requested transitions;
- `server-dimension-network-sync-mod`: sends dimension, position, sky and
  player-visibility changes;
- client dimension state, chunk reset, player reposition and sky mods, each
  independently replaceable.

Dimension registration rejects duplicate IDs and duplicate defaults. A
non-default contributor never becomes the default merely because it initialized
first.

Portal geometry is shared in `portal-api`. Server state, rule registration,
ignition, network synchronization, travel and client rendering are separate
mods. The vanilla ignition engine recognizes a hollow `4x5` frame only when the
used face borders its `2x3` interior. The four outer corners are optional.
Travel emits
`RequestPlayerDimensionChange`; it does not directly mutate chunk caches,
player visibility or rendering.

Portal behavior is data contributed by rule mods:

- `server-nether-portal-rule-vanilla-mod`: obsidian frame, red interior,
  destination Nether;
- `server-aether-portal-rule-vanilla-mod`: glowstone frame, blue interior,
  destination Aether.

`PortalOpenedPacket` is generic and contains `PortalFrame`, generated
`Dimension` destination and color. The client renderer does not branch on a
specific dimension.

The active portal is server state scoped by `WorldScopeId`. When a player uses
an unlinked portal, the generic travel mod creates and activates a matching
return portal beside the destination spawn. It uses the same frame block and
color, so the Nether gets an obsidian return portal and the Aether gets a
glowstone return portal. That return portal links back beside the source frame.
The generated frame changes are also emitted through the normal block-edit ECS
events so other players in that world scope receive them.

On a dimension change the client clears old chunks and portal visuals, applies
the new spawn, changes the sky color and streams chunks through the selected
provider. Player join/leave snapshots are updated in both directions so players
in distinct dimensions cannot retain stale avatars of each other.

The portal rules are in `server-vanilla.toml`. A custom server can keep the
dimension registry and routing while selecting either vanilla rule, omitting
both, or replacing ignition/travel with different feature mods.

## Chunk streaming and rendering

Client chunk behavior is split across several mods:

- `client-chunk-streaming-around-player-impl` decides which chunks are needed;
- `client-chunk-request-network-mod` asks the server for missing chunks;
- `client-chunk-cache-network-impl` receives chunk responses and block updates;
- `client-chunk-mesh-naive-cubes-impl` builds simple cube meshes;
- `client-chunk-render-bevy-impl` spawns/despawns Bevy render entities.

`client-chunk-request-network-mod` treats `ActiveChunks` plus the local cache as
the recoverable source of truth. Every frame it reconciles pending requests:
active chunks missing from the cache are requested or retried, while requests
for inactive chunks are discarded. This makes streaming recover from an event
ordering race, a dropped/rejected request, or a dimension reset instead of
leaving permanent holes. A dimension reset is performed only for an actual
dimension transition, not for a repeated initial state notification.

The streaming center uses all three camera coordinates. Its
`ChunkStreamingViewConfig` defines a moving horizontal and vertical window;
the demo keeps two chunk layers above and below the player to limit memory, but
the window follows arbitrary positive or negative `ChunkPos.y` values. This
makes the world vertically unbounded without keeping an unbounded column in RAM.
The vanilla server residency policy allows the same vertical window plus one
chunk of latency slack. Terrain providers already answer every chunk coordinate:
solid worlds continue below their surface, while empty space and Aether gaps
produce ordinary air chunks.

The mesh implementation is deliberately simple and replaceable. A greedy mesher
or lighting-aware renderer should provide the same mesh/render API instead of
rewriting streaming or networking.

## Client lighting and graphics

Light ownership is no longer hidden in `client-game-bootstrap-mod`. The
bootstrap only wires and runs the application. Neutral sun state, network
receive behavior and the small unlit sun-disc renderer live in `client.toml`;
graphics policy is selected through `client-graphics.toml`:

- `client-ambient-light-vanilla-mod` configures lightweight ambient light;
- `client-sun-directional-light-bevy-mod` renders the sun as a directional
  light;
- `client-sun-shadows-vanilla-mod` independently enables its shadows;
- `client-chunk-face-shading-vanilla-mod` adds direction-dependent face
  brightness;
- `client-chunk-ambient-occlusion-vanilla-mod` adds inexpensive voxel AO at
  mesh vertices and selects the better quad diagonal.

The neutral `client-chunk-vertex-lighting-pipeline-mod` remains in the client
base because the selected mesher consumes its API. With no graphics stages it
returns white vertex colors. The mesher snapshots registered stages once per
chunk, so it does not lock a registry for every vertex. Missing neighbor chunks
are treated as unoccluded until their arrival triggers a remesh. Mesh
neighborhoods include all 26 adjacent chunks because AO samples at an edge or
corner can cross two or three chunk axes; per-frame remesh requests are
deduplicated.

Sun authority is similarly separated. `server-sun-state-mod` owns optional
state, `server-sun-network-sync-mod` synchronizes joins and runtime changes, and
`server-sun-vanilla-mod` chooses the initial position, illuminance and light
color. The first two live in the server foundation; only the chosen default is
vanilla policy. Other server mods can emit `SetServerSun` at runtime without
depending on Bevy rendering.

The ambient-light policy uses a brighter, mostly neutral fill to keep cast
shadows readable, while retaining part of the server-provided sun color. This
makes extreme colors visible in both directly lit and shadowed areas. The sun
disc follows the camera at a fixed apparent distance and is unlit, so its color
matches `SunSettings::color` without receiving or casting shadows.

## Inventory, cell menus and hotbar

Inventory is server-authoritative.

The client sends intentions:

- move or swap cells;
- select hotbar cell;
- use held item;
- request sync.

The server validates and then synchronizes state back through packets:

- `InventoryResetPacket`;
- `InventoryResizePacket`;
- `InventorySetCellPacket`;
- `HotbarSelectionPacket`.

The server pipeline is:

```text
InventoryServerSet::ReceiveRequest
InventoryServerSet::Validate
InventoryServerSet::DispatchUse
InventoryServerSet::ApplyWorldEffects
InventoryServerSet::ApplyConsumption
InventoryServerSet::Sync
```

There is also a more detailed validation grouping:

```text
InventoryValidationSet::Initialize
InventoryValidationSet::Stack
InventoryValidationSet::MoveOrSwap
InventoryValidationSet::Other
```

Important inventory mods:

- `server-inventory-authoritative-mod`: owns server inventory state;
- `server-inventory-layout-default-impl`: defines default sections and hotbar
  layout;
- `server-inventory-default-loadout-mod`: gives initial items;
- `server-inventory-network-receive-mod`: receives client intentions;
- `server-inventory-network-sync-mod`: sends authoritative state;
- `server-inventory-quantity-stacking-mod`: stack behavior for quantity
  metadata;
- `server-item-quantity-consumption-mod`: consumes finite quantities after
  successful use.

The hotbar is not a separate inventory system. It is a section in the
server-defined inventory layout.

Cell menus generalize inventory-like grids beyond the personal player
inventory. They use the same `Inventory`, `InventoryLayout`, `InventoryCell`
and `ItemInstance` building blocks, but are addressed by `CellMenuId` and
guarded by an `Audience`.

The important audience modes are:

- `Audience::Personal(owner)`: only one player can open/interact;
- `Audience::Shared(key)`: multiple current viewers can see and interact with
  the same menu state.

This is the same design direction intended for future visibility systems:
chunks, players, menus and other shared state should be synchronized to an
audience instead of hardcoding "send to everyone".

`Audience` and `WorldScopeId` are deliberately related but not the same type:

- a world scope identifies which state namespace/source is being queried;
- an audience identifies which members may observe or interact with state.

A scope can be one input to an audience resolver, but it does not define the
whole audience. For example, two chests in the same world scope may be public,
personal, party-only or visible only to their current viewers. Shared
infrastructure should therefore resolve domain audiences into player IDs,
while menus, chunks, entities and future chat features retain their own policy
mods and state identities.

Important cell-menu mods:

- `cell-menu-api`: shared menu state, ECS events and client/server system sets;
- `server-cell-menu-api`: server-side authoritative menu storage;
- `server-cell-menu-authoritative-mod`: validates and applies menu operations;
- `server-cell-menu-network-receive-mod`: receives client menu intentions;
- `server-cell-menu-network-sync-mod`: syncs opens/closes/cell updates to all
  viewers in the menu audience;
- `client-cell-menu-cache-mod`: stores active client-side menu state;
- `client-cell-menu-ui-bevy-mod`: renders grid-like menu UIs;
- `client-cell-menu-drag-drop-mod`: drag/drop within a cell menu;
- `client-cell-menu-inventory-bridge-drag-drop-mod`: drag/drop between a cell
  menu and the player's inventory;
- `client-cell-menu-optimistic-move-mod`: local optimistic feedback until the
  authoritative server sync arrives.

The crafting table is implemented as a vanilla cell-menu feature:

- `block-crafting-table` and `item-crafting-table-block` define the block/item;
- `client-crafting-table-open-network-mod` sends an open-menu request when the
  player uses a crafting table block;
- `server-crafting-table-menu-vanilla-mod` checks that the target block is a
  crafting table and opens a shared `3x3` menu anchored to the block position.

The network receiver only converts the packet into `CellMenuOpenIntent`.
Crafting-table policy consumes that transport-independent ECS message in the
validation phase. A different server can add chest, machine or permission
handlers without changing the networking mod.

No recipe logic is hardcoded into the cell-menu base. Crafting, chest behavior,
damage-on-move or any other semantics should be separate mods listening to the
cell-menu/inventory events.

## Client inventory UI

Client inventory UI is also split:

- `client-inventory-cache-mod` stores the last authoritative inventory state;
- `client-inventory-ui-bevy-mod` renders the grid and hotbar;
- `client-inventory-drag-drop-mod` handles drag/drop intentions;
- `client-inventory-optimistic-move-mod` gives immediate feedback for personal
  inventory moves;
- `client-inventory-network-send-mod` sends inventory intentions;
- `client-inventory-network-receive-mod` applies authoritative updates;
- `client-item-quantity-ui-mod` renders quantity labels.

The UI may optimistically show drag state for both inventory and cell menus, but
the server remains the source of truth. If a move is rejected or changed, the
next authoritative sync wins.

## Settings and menus

Settings are generated from contributor mods:

- render distance;
- mouse sensitivity;
- FOV;
- jump key;
- sprint key;
- inventory key;
- player name;
- server address.

`client-settings-registry-codegen` generates only the contributors selected by
the modpack. Each contributor declares an input provider ID independently from
its storage type, for example:

```toml
type = "string"
input = "keybinding"
```

Input UI is composed from separate mods: string, `i32`, `f32`, bool and
keybinding providers register factories in `client-settings-input-api` during
the ECS `Startup` schedule. `client-settings-bevy-menu-impl` builds the menu
after registration. Numeric inputs have typed editing and step buttons,
keybindings capture a key directly, and booleans use a toggle.

In-game overlays are modeled as Bevy substates. Opening pause/settings/inventory
does not destroy the world, networking or chunk cache.

## Assets

Mod assets live in each mod's `assets/` folder. Patchwork copies them into the
composed output under:

```text
assets/<mod-id>/
```

Texture-owning block mods, font mods and UI asset mods should load files from
their own namespace.

Examples:

- block texture mods own their texture files;
- `client-ui-font-dejavu-mod` owns the DejaVu font and exposes it as a Bevy
  resource for UI text that needs broad glyph support.
- `client-player-blocky-model-paths-mod` owns the current player
  `.blockymodel`, `.blockyanim` files and `Outlander_1.png` texture.

## Vanilla feature packs

`client-vanilla.toml` and `server-vanilla.toml` are the default optional feature
bundles for demo gameplay. This is where vanilla rules belong.

Examples of vanilla server features:

- player movement collision validation;
- jump validation;
- gravity sync;
- block placement from `PlaceBlock` item metadata;
- quantity consumption after successful item use;
- default inventory layout and loadout;
- player-interest chunk residency and request limits;
- world-instance player visibility.

The important rule is that "vanilla" is not "base". A different server should
be able to use the same APIs and choose a different set of rules.

## Known extension points and current limits

The project is intentionally built with future modding in mind, but not every
extension is fully generalized yet.

Current useful extension points:

- replace TCP transport with another network provider;
- register additional chunk providers and route viewers between them;
- replace Perlin terrain with a file-backed or other procedural provider;
- replace player-interest chunk residency with another cache policy;
- add block edit validators;
- add inventory validators or effects;
- add item metadata and item-use behavior;
- replace chunk meshing/rendering;
- replace Blocky player rendering or provide different Blocky assets;
- add new cell-menu semantics such as chest persistence or crafting recipes;
- add custom player visibility by replacing its API provider;
- add new settings through contributor mods.

Current limitations to watch:

- gravity is global and is not yet scoped per player or `WorldInstanceId`;
- chunk generation is synchronous; expensive file/procedural providers should
  add an asynchronous generation pipeline;
- the renderer is naive cube rendering;
- Blocky UV/animation support is functional but still young compared to a full
  Hytale runtime;
- inventory metadata is extensible, but advanced metadata UI is still basic.

When extending the demo, prefer adding a new API/event/feature crate over
editing a concrete vanilla mod. The modpack should make the final choice.
