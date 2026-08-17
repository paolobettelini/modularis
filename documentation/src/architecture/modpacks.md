# Modpacks and feature boundaries

Modpacks are the architecture's final decision layer. APIs and feature mods make
choices possible; modpacks decide which choices become an application.

## Shared composition

`common.toml` imports:

```text
blocks
items
network
```

and selects `bevy-mod`.

This gives client and server the same generated block, item, metadata, dimension,
and protocol types.

`items.toml` imports `blocks.toml` because `PlaceBlock` metadata contains a
generated `BlockId`.

`network.toml` imports blocks and dimensions because packets serialize block
instances, chunks, and dimension IDs.

`biomes-overworld.toml`, `biomes-nether.toml`, and `biomes-aether.toml` select
dimension-specific biome identity contributors. `biomes.toml` imports those
packs, blocks, and dimensions, then selects `biome-registry-codegen`. It is not
part of `common.toml` yet because the current network protocol does not transmit
biome IDs.

## Client composition

`client.toml` imports:

```toml
modpacks = ["common", "client-vanilla", "client-graphics", "client-grass"]
```

The top-level client selects neutral state and concrete presentation:

- Bevy default plugins and window;
- settings and input editor providers;
- TCP transport;
- session state;
- chat state, network bridges, and UI;
- chunk cache, streaming, meshing, and rendering;
- first-person controller and camera;
- inventory and cell-menu UI;
- Blocky remote player rendering;
- dimension, sky, sun, and portal client state;
- generic world-context state, cache reset, and authoritative repositioning;
- bootstrap.

`client-vanilla.toml` adds optional behavior:

- jump, sprint, and sneak settings;
- crosshair;
- number-key and wheel hotbar selection;
- pause and inventory input;
- inventory number-key move/swap behavior;
- configurable chat-key input;
- client gravity prediction;
- fixed-rate movement, inertia, jump, sprint-jump, sprint, sneak, and flight controls;
- independent sneak state, speed, edge-protection, camera, and block-use routing policies;
- held-item fallback;
- crafting-table interaction;
- permission-gated creative catalog item picking;
- layered chunk priority.

`client-graphics.toml` adds:

- directional sun;
- sun shadows;
- ambient fill;
- face-direction shading;
- voxel ambient occlusion.

`client-grass.toml` is another optional presentation pack. It selects grouped
grass settings, the generated typed adapter, a wind provider, a replaceable
Fabric-style blade mesher, and the Bevy shader renderer. The logical short-grass
block remains in the shared block pack.

This split lets a custom client keep the base protocol and chunk renderer while
changing controls or graphics.

## Server composition

`server-core.toml` is the smallest reusable headless foundation:

- server configuration;
- replaceable server tick provider, metrics, and headless Bevy runner;
- TCP transport;
- server packet events and routing;
- lifecycle messages;
- sessions and timeout;
- player admission contracts;
- generated, owner-scoped player permission state and synchronization;
- generic kick contracts and authoritative cleanup;
- bootstrap.

`server-base.toml` is a convenience umbrella. It imports `server-core.toml` and
adds neutral, but optional, feature pipelines:

- authoritative inventory and cell-menu state;
- their network receive/sync bridges;
- block edit network bridges;
- chunk request handling;
- chat ECS contracts, network bridges, and the Brigadier command provider;
- flight capability state and synchronization;
- per-player outline capability state and synchronization;
- sun state and synchronization.

`server-vanilla.toml` imports `server-biomes-vanilla.toml` and the independent
`server-commands-vanilla.toml` command pack, then contains selected policy:

- the imported `server-biomes-vanilla.toml` umbrella, whose separate
  Overworld, Nether, and Aether packs contain definitions and phased features;
- the biome runtime registry, vanilla selector, and shared biome sampler;
- global-chat audience policy and case-insensitive unique player names;
- chat command routing and optional clear-chat, flight, flight-speed, kick,
  teleport, speed, scale, gravity, give, game-mode, privilege, and TPS commands;
- player-interest chunk residency;
- world-scope player visibility;
- default dimension lifecycle;
- typed vanilla permissions and the permission-to-flight-capability adapter;
- generated game-mode identities, neutral state, generic policy dispatch, and
  separate Creative, Survival, and Adventure policies;
- a separate default join policy that starts players Creative and Privileged;
- server-driven outline policy;
- default sun;
- Nether and Aether portal rules;
- portal ignition and travel;
- movement collision and jump validation;
- per-player gravity, movement-speed, and flight-speed state/synchronization;
- default inventory layout and loadout;
- quantity stacking and consumption;
- reach validation;
- mode-aware Creative/Survival block breaking, persistent sparse damage, and
  `PlaceBlock` item behavior;
- crafting-table menus;
- privileged creative catalog item creation.

`server.toml` selects the concrete world:

- demo world catalog rooted at `build-server/worlds`;
- per-world seed files;
- filesystem chunk storage;
- periodic and shutdown flush policies;
- chunk provider registry;
- dimension registry;
- Overworld, Nether, and Aether definitions;
- dimension-aware chunk routing;
- independent biome-driven Overworld, Nether, and Aether providers;
- dynamic resident world cache.

The biome packs are separate from `server-base.toml`. A server can import only
chosen dimension packs, omit them, or select the simple Perlin provider,
checkerboard provider, or a custom biome composition.
It can also replace filesystem persistence with an in-memory, database, remote,
or application-specific storage provider without changing terrain generation.

## TheCrown compositions

TheCrown has two top-level profiles. `thecrown-auth.toml` is a minimal entry
server: after Patchwork account authentication, it asks Relay for admission and
sends the returned transfer to the client. It never registers as an instance
host.

`thecrown-game.toml` is a Relay-managed worker. It imports only
`server-core.toml` and Patchwork authentication, then explicitly selects:

- the runtime scope tree;
- scope-backed world routing, audience, and player visibility;
- transient in-memory chunk storage;
- a sparse RAM-template chunk provider;
- generic player world-context synchronization;
- selected reusable movement policies;
- separate Relay and TheCrown session API providers;
- one custom `thecrown-game-main-mod` orchestrator.

The worker starts empty. `StartInstance` messages create exact Relay-assigned
Hub or Parkour IDs. It deliberately omits default inventory loadout, block
mutation glue, item placement, dimensions, portals, biome generation, and
crafting-table behavior. The result validates that those features are not
accidentally required by the server foundation.

The standalone Leptos web application is a separate executable and talks to
Relay over NATS; it is not a game-server mod.

See [TheCrown network and dynamic instances](../development/thecrown-network.md)
for authentication, transfer, lifecycle, and runtime topology.

## Why vanilla is not base

Suppose a custom server wants inventories but no block placement. It can use
`server-base.toml` and select the inventory layout/loadout mods it wants, while
omitting `server-place-block-item-use-mod`. A server that does not need
inventory infrastructure at all can start from `server-core.toml`.

Suppose another server wants players to move through blocks. It can omit
`server-player-movement-collision-vanilla-mod` or provide a different validator.

Suppose a server wants team worlds. It can replace:

- chunk routing;
- player visibility;
- chunk residency;
- dimension lifecycle;

without replacing TCP, packet generation, or the chunk storage backend.

These are only possible when default behavior remains outside the base.

When a vanilla behavior must apply only in some runtime scopes, omit its
always-on glue mod and call the corresponding `*-lib` mechanic from custom
orchestration. See
[Vanilla mechanics as reusable libraries](vanilla-libraries.md).

## Designing a new feature pack

A feature pack should:

1. import only foundations it genuinely needs;
2. select small behavior mods;
3. avoid selecting both sides of an exclusive provider API;
4. keep generated contributors near the domain pack that owns them;
5. avoid importing a top-level application modpack;
6. document which policies it chooses.

Example:

```toml
name = "Low gravity server rules"
description = "Movement rules for a low-gravity server."
modpacks = []
ignore = []

mods = [
    "player-gravity-low-mod",
    "server-player-gravity-network-sync-mod",
    "server-player-jump-low-gravity-mod",
]
```

The top-level server profile can import this pack instead of the vanilla gravity
selection.

## Using `ignore`

`ignore` is appropriate when importing a convenient broad pack but replacing
one selected mod:

```toml
modpacks = ["client", "my-graphics"]
ignore = ["client-sun-shadows-vanilla-mod"]
```

Use it carefully. If the ignored mod provides an API required by another
selected feature, the profile must select a replacement.

## Dependency review at the modpack level

Before accepting a modpack change, check:

- does it accidentally add a client-only mod to the server?
- does it select two exclusive providers?
- does it make a vanilla behavior mandatory through an imported base?
- are packet contributors shared by both client and server?
- are contributor and codegen owner selected together?
- does a new asset-owning mod appear in the correct application?

The modpack is where architectural promises become testable compositions.
