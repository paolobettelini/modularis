# Crate map

The project contains many small crates. This reference groups them by role.
Generated and message-type crates follow predictable naming and are summarized
as families.

## Composition and Bevy foundation

| Crate | Role |
| --- | --- |
| `bevy-mod` | Owns the initially empty `App` |
| `client-bevy-default-plugins-mod` | Window, renderer, logging, nearest textures |
| `client-player-physics-tick-api` | Replaceable client physics frequency contract |
| `client-player-physics-tick-20hz-vanilla-mod` | Vanilla 20 Hz client physics provider |
| `server-tick-api` | Server target tick rate and measured TPS resources |
| `server-tick-rate-20hz-default-impl` | Default 20 TPS server provider |
| `server-tick-metrics-mod` | Measures observed server TPS |
| `server-bevy-runner-mod` | Headless minimal plugin set driven by `ServerTickApi` |
| `client-game-bootstrap-mod` | Owns/runs final client app |
| `server-game-bootstrap-mod` | Owns/runs final server app |

## Generated registries

| Owner | Generated crate | Contributors |
| --- | --- | --- |
| `block-registry-codegen` | `generated-block-registry` | `block-*` |
| `block-metadata-registry-codegen` | `generated-block-metadata` | block metadata crates |
| `item-registry-codegen` | `generated-item-registry` | `item-*` contributors |
| `item-metadata-registry-codegen` | `generated-item-metadata` | `item-*-meta` |
| `dimension-registry-codegen` | `generated-dimension-registry` | `dimension-*` |
| `biome-registry-codegen` | `generated-biome-registry` | `biome-*` identity contributors |
| `client-settings-registry-codegen` | `generated-client-settings-registry` | `client-setting-*` |
| `network-protocol-mod` / `network-codegen-utils` | `generated-network-messages` | `*-network-messages-mod` |

Support generators use `codegen-utils` and `network-codegen-utils`.

## Block domain

Contracts:

- `block-api`;
- `block-render-api`;
- `block-instance-api`;
- `block-manager-api`;
- `block-edit-events-api`;
- `block-edit-events-mod`.

Generated manager:

- `block-manager-generated-impl`.

Current block contributors:

- `block-air`;
- `block-anvil`;
- `block-basalt`;
- `block-bedrock`;
- `block-birch-leaves` and `block-birch-log`;
- `block-blackstone`;
- `block-cactus` and `block-calcite`;
- `block-cauldron`;
- `block-crafting-table`;
- `block-crimson-nylium` and `block-warped-nylium`;
- `block-diamond-block`;
- `block-diamond-ore`;
- `block-dirt`;
- `block-end-stone`;
- `block-gravel`;
- `block-glowstone`;
- `block-grass`;
- `block-moss`;
- `block-netherrack`;
- `block-oak-leaves`;
- `block-oak-log`;
- `block-oak-stairs`;
- `block-obsidian`;
- `block-packed-ice`;
- `block-red-sand` and `block-sand`;
- `block-snow`;
- `block-soul-sand` and `block-soul-soil`;
- `block-stone`;
- `block-terracotta`.

Block edit networking:

- `block-edit-network-message-types`;
- client/server send/receive mods.

## Item domain

Contracts:

- `item-api`;
- `item-render-api`;
- `item-instance-api`;
- `item-manager-api`;
- `item-use-api`.

Generated manager:

- `item-manager-generated-impl`.

Metadata:

- `item-quantity-meta`;
- `item-place-block-meta`;
- `item-favicon-meta`;
- `item-portal-igniter-meta`.

Block item contributors follow the `item-<block>-block` naming family. The
current composition contains one for every non-air block contributor listed
above: 33 placeable block items. This one-to-one relation is a selected demo
policy, not a registry requirement; a custom composition may expose blocks
without inventory items or multiple items for one block.

Standalone and tool items:

- `item-flint-and-steel`;
- `item-stick`.

## JSON voxel models

Contracts and library:

- `voxel-models-lib`;
- `voxel-model-api`.

Template asset mods:

- `voxel-model-block-templates-mod`;
- `voxel-model-item-templates-mod`;
- `voxel-model-anvil-template-mod`.

Shared providers and contracts:

- `voxel-model-assets-fs-impl`;
- `block-shape-api`;
- `block-shape-voxel-model-impl`.

Client consumers:

- `client-chunk-mesh-voxel-models-impl`;
- `client-item-model-ui-mod`.

## Chunk data and math

| Crate | Role |
| --- | --- |
| `voxel-math-api` | Block/chunk/local positions |
| `voxel-raycast-api` | Exact 3D DDA voxel traversal |
| `packed-bit-array-api` | Compact palette indices |
| `chunk-section-api` | Palette and reverse map |
| `chunk-api` | Positioned chunk |
| `chunk-network-message-types` | Chunk request/response payloads |
| `chunk-network-messages-mod` | Protocol contribution |
| `coherent-noise-api` | Noise helper contract |

## Server chunk world

Contracts:

- `server-chunk-provider-api`;
- `server-primary-chunk-provider-api`;
- `server-chunk-routing-api`;
- `server-chunk-residency-api`;
- `server-chunk-world-api`;
- `server-world-seed-api`;
- `world-instance-api`.

Infrastructure:

- `server-chunk-provider-registry-mod`;
- `server-chunk-world-dynamic-impl`;
- `server-chunk-request-mod`.
- `server-world-seed-random-impl`.

Providers:

- `server-chunk-provider-biomes-mod` (active Overworld provider);
- `server-chunk-provider-perlin-mod`;
- `server-chunk-provider-nether-mod` (active biome-driven Nether provider);
- `server-chunk-provider-aether-mod` (active biome-driven Aether provider);
- `server-chunk-provider-checkerboard-mod`.

Routing:

- `server-chunk-routing-dimensions-mod`;
- `server-chunk-routing-single-world-mod`.

Residency:

- `server-chunk-residency-player-interest-vanilla-mod`.

## Biomes and generation features

Identity and codegen:

- `biome-registry-codegen`;
- `generated-biome-registry`;
- seven Overworld, five Nether, and three Aether identity contributors.

Contracts and providers:

- `server-biome-api`;
- `server-biome-selection-api`;
- `server-biome-registry-mod`;
- `server-biome-climate-selector-vanilla-mod`;
- `server-biome-sampling-api` shared provider support;
- one vanilla definition mod per biome.

Phased features:

- caves;
- diamond ores;
- sparse/dense oak trees;
- cacti;
- packed-ice patches;
- rock boulders;
- birch trees;
- glowstone clusters;
- crystal spires.

## Client chunks

Contracts:

- `client-chunk-streaming-api`;
- `client-chunk-cache-api`;
- `client-chunk-work-priority-api`;
- `client-chunk-mesh-api`;
- `client-chunk-render-api`;
- `client-chunk-vertex-lighting-api`.

Infrastructure and providers:

- `client-chunk-streaming-around-player-impl`;
- `client-chunk-cache-network-impl`;
- `client-chunk-work-priority-mod`;
- `client-chunk-layered-priority-vanilla-mod`;
- `client-chunk-request-network-mod`;
- `client-chunk-mesh-voxel-models-impl` (active JSON model provider);
- `client-chunk-mesh-naive-cubes-impl` (legacy alternate provider);
- `client-chunk-render-bevy-impl`;
- `client-chunk-vertex-lighting-pipeline-mod`;
- face-shading and ambient-occlusion vanilla mods.

Dimension reset:

- `client-chunk-reset-on-dimension-change-mod`.

## Network and sessions

Contracts:

- `network-framing-api`;
- `client-network-api`;
- `server-network-api`;
- `server-network-events-api`;
- `server-player-registry-api`;
- `server-player-visibility-api`;
- lifecycle event API/mod.
- `server-player-admission-api`;
- `server-audience-api`.

Active transport:

- `client-network-tcp-impl`;
- `server-network-tcp-impl`.

Alternate transport crates:

- client/server UDP implementations.

Routing/session:

- `server-network-router-mod`;
- `client-session-api`;
- `client-session-network-mod`;
- `server-player-session-mod`;
- `server-player-timeout-mod`;
- `server-player-visibility-world-instance-mod`.
- `server-player-name-unique-vanilla-mod`;
- `server-audience-basic-impl`.
- `server-kick-api`, `server-kick-events-mod`, and `server-player-kick-mod`.

Message type/contributor families:

- session;
- player;
- gravity/jump;
- flight capability and flight speed;
- player movement speed;
- generic kick;
- chunks;
- block edits;
- inventory/hotbar;
- cell menus;
- dimension;
- sky/sun;
- portal;
- chat and command completion.

## Chat and commands

Contracts and state:

- `client-chat-api` and `client-chat-state-mod`;
- `server-chat-api` and `server-chat-events-mod`;
- `server-command-api`.

Network and presentation:

- `chat-network-message-types` and `chat-network-messages-mod`;
- clear-chat message type/contributor plus dedicated client receive and server
  sync bridges;
- client chat send/receive, toggle-input, and Bevy UI mods;
- optional client chat history/completion navigation mod;
- server chat receive/sync mods;
- `client-setting-chat-key`.

Policies and implementations:

- `server-chat-global-vanilla-mod`;
- `server-chat-command-router-mod`;
- `server-command-brigadier-mod`;
- clear-chat, flight, flight-speed, kick, teleport, speed, gravity, and TPS
  vanilla command mods.

## Player movement

Contracts:

- `client-input-api`;
- `client-camera-api`;
- `client-player-controller-api`;
- `collision-api`;
- `block-shape-api`;
- `player-block-collision-api`;
- `player-hitbox-api`;
- `player-gravity-api`;
- `player-speed-api`;
- `player-jump-api`;
- `player-flight-api`;
- `player-flight-speed-api`;
- `player-sneak-api`;
- `server-player-flight-api`;
- `server-player-flight-speed-api`;
- `server-player-gravity-api`;
- `server-player-speed-api`.

Implementations/features:

- `client-input-bevy-impl`;
- `client-camera-first-person-bevy-impl`;
- `client-player-controller-fps-bevy-impl`;
- `client-collision-block-aabb-impl`;
- `client-player-spawn-mod`;
- gravity prediction/network mods;
- per-player server gravity state/sync mods;
- local and server player speed state/sync mods;
- client/server jump vanilla mods;
- client sprint vanilla mod;
- client sneak state, input, speed, edge-protection, camera, and block-use
  routing mods;
- client inertial acceleration/drag and sprint-jump vanilla mods;
- flight state, capability, sync, controls, and grant policy mods;
- separate client/server flight-speed state and sync mods;
- `server-player-movement-collision-vanilla-mod`;
- `client-player-network-sync-mod`.

## Player rendering and Blocky formats

Contracts/parser:

- `client-player-render-api`;
- `blocky_formats`;
- `blocky-model-api`;
- `blocky-animation-api`;
- `client-player-blocky-model-paths-api`.

Implementations:

- `client-blocky-model-bevy-render-mod`;
- `client-blocky-animation-bevy-mod`;
- `client-player-blocky-model-paths-mod`;
- `client-player-render-blocky-impl`.

Alternate renderer:

- `client-player-render-bevy-impl` sphere-based implementation.

## Inventory and cell menus

Inventory contracts:

- `inventory-core-api`;
- `inventory-events-api`;
- `inventory-events-mod`;
- `server-inventory-api`;
- `server-inventory-layout-api`;
- `client-inventory-cache-api`;
- `client-inventory-ui-api`.

Server inventory:

- authoritative state;
- default layout;
- default loadout;
- network receive/sync;
- quantity stacking;
- quantity consumption;
- place-block item use.

Client inventory:

- cache;
- optimistic move;
- network receive/send;
- UI;
- drag/drop;
- hotbar UI and selection input;
- quantity and favicon UI.

Cell menu contracts:

- `audience-api`;
- `cell-menu-api`;
- `cell-menu-events-mod`;
- `server-cell-menu-api`;
- client cache/UI APIs.

Cell menu implementation:

- server authoritative, receive, sync, and inventory bridge mods;
- client cache, UI, drag/drop, optimistic, network, and bridge mods;
- client/server crafting-table vanilla mods.

## Dimensions and portals

Dimensions:

- three contributor crates;
- server dimension API/registry;
- Overworld, Nether, Aether registration mods;
- player lifecycle and network sync;
- client dimension state/receive/position/reset mods.

Portals:

- `portal-api`;
- `server-portal-api`;
- server state, rule, ignition, sync, travel mods;
- Nether and Aether rule mods;
- portal message types/contributor;
- client portal renderer.

## Client UI, settings, and menus

Game/menu contracts:

- `client-game-state-api`;
- `client-menu-api`;
- `client-settings-api`;
- `settings-schema-api`;
- `client-settings-input-api`;
- `client-keybinding-api`.

Providers/features:

- game state Bevy implementation;
- menu Bevy implementation;
- main and pause menus;
- pause/inventory input mods;
- settings menu;
- settings input registry;
- string, integer, float, bool, keybinding editors;
- generated setting contributor crates;
- DejaVu font provider;
- crosshair;
- optional block outline API/renderer/looked-block policy.

## Sky, sun, and graphics

Contracts:

- `client-sky-api`;
- `sun-api`;
- `client-sun-api`;
- `server-sun-api`.

State/network:

- sky/sun message types and contributors;
- server state, sync, and vanilla sun;
- client state and receive mods.

Rendering/policy:

- client sky;
- sun directional light;
- sun shadows;
- ambient light;
- sun disc;
- chunk face shading;
- chunk ambient occlusion.

## Reading this map

When a crate name is unfamiliar:

1. identify its family here;
2. inspect its API crate first;
3. check which modpack selects it;
4. inspect `package.metadata.mod.dependencies`;
5. inspect the entry's `init` registration;
6. follow public messages and system sets.

This is usually faster than starting from generated `main.rs`.
