# Introduction

This book documents the architecture of `Modularis`, a small
multiplayer voxel game built with Rust, Bevy, and Patchwork.

The project is not designed as one game crate with a plugin folder attached to
it. The game is the result of composing many Rust crates at compile time. Those
crates are called **mods**, and a **modpack** selects the final set of mods that
becomes an executable.

That distinction explains most architectural decisions in the repository:

- blocks, items, dimensions, settings, and network packets are generated from
  the contributors selected by a modpack;
- client and server share domain types but select different runtime behavior;
- server authority is separated from vanilla gameplay policy;
- Bevy resources, messages, components, and `SystemSet` values are used as
  contracts between mods;
- a concrete implementation is usually hidden behind a small API crate;
- optional behavior is placed in a feature mod, often in a `*-vanilla-mod`
  crate;
- generated build directories are disposable output, not source code.

The goal is not only to demonstrate Minecraft-like mechanics. The demo is also
an example of how to build a game framework where another developer can create
a very different server by choosing different providers and policies.

For example, a custom server may:

- route different players to different chunk providers;
- expose players only to selected viewers;
- remove block placement while keeping blocks and inventories;
- replace the terrain generator without changing the chunk cache;
- grant flight only to some players;
- use a different inventory layout or item-use pipeline;
- add a new dimension and portal rule without editing a central dimension enum;
- replace the chunk mesher or player renderer on the client.

This book explains both the current implementation and the intended extension
points. Each feature chapter follows the same questions:

1. What public contract does the feature expose?
2. Which mod currently implements it?
3. Which parts are neutral infrastructure and which parts are vanilla policy?
4. How does data move through the ECS schedule?
5. How can another mod extend or replace the behavior?

## Reading paths

If you are new to the project, read these chapters first:

1. [Project overview](./getting-started/project-overview.md)
2. [Build and run](./getting-started/build-and-run.md)
3. [Patchwork composition](./architecture/patchwork-composition.md)
4. [Mod anatomy and lifecycle](./architecture/mod-anatomy.md)
5. [APIs, providers, and ECS contracts](./architecture/apis-and-events.md)

If you want to add gameplay, continue with:

- [Blocks, items, and metadata](./world/blocks-and-items.md)
- [Block interaction and world edits](./gameplay/block-interaction.md)
- [Inventory, hotbar, and item use](./gameplay/inventory.md)
- [Extension cookbook](./development/extension-cookbook.md)

If you want to work on terrain or rendering, continue with:

- [Chunk coordinates and storage](./world/chunk-storage.md)
- [Server world providers and residency](./world/server-world.md)
- [Client streaming, meshing, and rendering](./world/client-chunks.md)
- [Graphics, lighting, sky, and outlines](./client/graphics.md)

## Terminology

This book uses the following terms:

- **mod**: a Rust crate selected by Patchwork;
- **modpack**: a TOML composition that selects mods and imports other
  modpacks;
- **API crate**: a crate that defines a stable contract without choosing a
  concrete policy;
- **provider**: a mod selected as an implementation of an API;
- **feature mod**: an optional behavior that participates through events,
  resources, or API contracts;
- **contributor mod**: a mod that contributes metadata to code generation;
- **generated crate**: a crate created by Patchwork codegen from the final
  composition;
- **vanilla mod**: the demo's default Minecraft-like behavior, kept optional;
- **world scope**: the namespace formed by a world instance and chunk provider;
- **audience**: the players allowed to observe or interact with some state.

The names are architectural roles. A single crate may fill more than one role,
but keeping the roles clear makes dependencies easier to review.
