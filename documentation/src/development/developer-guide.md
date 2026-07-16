# Developer guide

This chapter summarizes the workflow for changing the project without breaking
its composability.

## Start from the policy question

Before editing code, ask:

> Could another client or server reasonably want a different behavior here?

If yes, the behavior should usually be:

- a feature mod;
- a provider behind an API;
- an ECS validator/effect;
- a contributor selected by codegen;
- a modpack choice.

Examples:

- "all players receive flight" is policy;
- "flight capability can be set and synchronized" is infrastructure;
- "obsidian opens a Nether portal" is policy;
- "portal frame geometry and active portal state exist" are reusable contracts;
- "chunks around players remain resident" is policy;
- "the world can retain a set of resident keys" is infrastructure.

## Find the contract before the implementation

Search for:

- `*-api`;
- public messages;
- public `SystemSet`;
- provider metadata;
- selected modpack entries.

Use the implementation to understand behavior, but depend on the API when
possible.

## Choose one extension style

### Additive feature

Use when behavior should coexist with others.

Examples:

- inventory effect listener;
- block edit validator;
- lighting stage;
- join listener;
- setting contributor.

### Exclusive provider

Use when one implementation should be selected.

Examples:

- client network transport;
- server network transport;
- collision service;
- chunk mesher;
- player renderer;
- settings/menu implementation.

Declare `provides` and depend on the API.

### Keyed provider

Use when several implementations must coexist.

Examples:

- chunk providers;
- future recipe or biome registries;
- settings input factories.

Register by stable ID.

### Generated contributor

Use when the selected type set must become a typed enum or struct.

Examples:

- block;
- item;
- item/block metadata;
- dimension;
- setting;
- network packet.

## Keep network bridges thin

A network receive mod should:

1. read a generated typed packet event;
2. map source address to authenticated player if needed;
3. emit a transport-independent ECS request.

A network send mod should:

1. read an applied domain result;
2. choose an audience;
3. emit `ServerPacketOut`.

Do not place gameplay policy in packet conversion.

## Use ECS phases deliberately

Register a system in the phase that matches its promise:

- receive: decode/authenticate;
- collect: create mutable pending work;
- validate: deny or transform;
- apply: mutate authoritative state;
- sync: notify clients;
- render decorations: add UI visuals after layout exists.

If no suitable phase exists, add a public set to the API crate rather than
using fragile `.before(other_private_system)`.

## Preserve source-of-truth rules

Server-authoritative domains:

- world edits;
- inventory;
- shared menus;
- dimensions;
- capabilities;
- player registry.

Client caches may predict, but server sync must overwrite prediction.

## Preserve identity

When adding state, choose keys carefully:

- player state: `PlayerId`;
- world state: `WorldScopeId` plus domain position/ID;
- resident chunk: `ResidentChunkKey`;
- menu: `CellMenuId`;
- shared access: `Audience`;
- local render entity: Bevy `Entity`, stored behind a stable domain ID.

Do not serialize Bevy entity IDs across the network.

## Prefer events over direct feature calls

If using an item should:

- place a block;
- consume quantity;
- grant experience;
- emit particles;

then one direct `use_item()` function with all four behaviors is difficult to
replace.

Prefer:

```text
HeldItemUseDispatched
  -> world effect features
  -> ItemUseSucceeded
  -> quantity/experience/feedback features
```

## Keep defaults future-proof

Generated metadata sets grow over time:

```rust
ItemMetaSet {
    my_field: Some(value),
    ..Default::default()
}
```

The same applies to block metadata and normal Rust configuration structs when
appropriate.

## Update the right documentation

When an architectural contract changes:

- update the relevant chapter;
- update the crate map if a family changed;
- update extension steps;
- update current limitations if the change resolves one;
- avoid creating another root architecture file.

This mdBook is the canonical architecture documentation.
