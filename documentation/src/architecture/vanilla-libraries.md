# Vanilla mechanics as reusable libraries

A vanilla feature has two different parts:

1. the mechanic itself;
2. the policy that decides when it always applies.

Keeping both parts in one Bevy mod makes the feature selectable at compile
time, but it is still too rigid for a custom server. A server may want to open
a crafting table only in one lobby, apply collision only to one game mode, or
allow block placement only after a custom permission check.

The server therefore uses three layers.

## The three layers

### Contract layer

API and event crates define data, resources, messages, and `SystemSet` phases.
They do not choose vanilla behavior.

Examples:

- `block-edit-events-api`;
- `server-player-registry-api`;
- `inventory-events-api`;
- `server-chunk-world-api`;
- `server-portal-api`.

### Mechanic library

A `*-lib` crate contains callable Rust logic. It has no
`[package.metadata.mod]` entry and Patchwork does not initialize it.

A mechanic library should:

- take all required state explicitly;
- return a decision, request, mutation result, or event;
- avoid registering an always-on Bevy system;
- avoid choosing a global audience unless that is the mechanic's input;
- avoid owning lifecycle policy.

### Vanilla glue mod

The corresponding vanilla mod is intentionally small. It listens at a public
ECS phase, calls the library, and publishes the result.

The crafting-table path is:

```text
server-crafting-table-menu-lib
    reusable block check + menu request construction

server-crafting-table-menu-vanilla-mod
    listen to every CellMenuOpenIntent
    call the library
    emit CellMenuOpenRequested when it matches
```

A custom server omits the vanilla glue and calls
`crafting_table_open_request` only when its own scope, permission, round, or
team rules permit it.

## Current reusable mechanics

| Library | Reusable operation | Vanilla glue |
| --- | --- | --- |
| `server-player-chat-lib` | format attributed player chat for a supplied audience | `server-chat-global-vanilla-mod` |
| `server-player-name-unique-lib` | case-insensitive unique-name admission rule | `server-player-name-unique-vanilla-mod` |
| `server-player-movement-collision-lib` | clamp and resolve an authoritative move against block shapes | `server-player-movement-collision-vanilla-mod` |
| `server-player-jump-lib` | test grounding and validate a jump against gravity/hitbox/world state | `server-player-jump-vanilla-mod` |
| `server-block-break-reach-lib` | evaluate vanilla reach for a pending break | `server-block-break-reach-vanilla-mod` |
| `server-block-edit-world-lib` | create a pending break and apply an allowed mutation | `server-block-edit-world-mod` |
| `server-place-block-item-use-lib` | validate and apply a `PlaceBlock` item use | `server-place-block-item-use-mod` |
| `server-crafting-table-menu-lib` | build the shared crafting-table menu request | `server-crafting-table-menu-vanilla-mod` |
| `server-inventory-default-loadout-lib` | construct the demo inventory and hotbar reset | `server-inventory-default-loadout-mod` |
| `inventory-quantity-operations-lib` | quantity stacking and one-use consumption | quantity stacking/consumption mods |
| `server-player-dimension-lifecycle-lib` | initialize and apply dimension changes | dimension lifecycle vanilla mod |
| `server-portal-ignite-lib` | evaluate a portal ignition attempt | portal ignition vanilla mod |
| `server-portal-travel-lib` | detect travel and select/create a return frame | portal travel vanilla mod |
| `server-chunk-residency-player-interest-lib` | compute resident keys for chosen viewers | player-interest residency vanilla mod |

Some vanilla mods are already only declarative data or a one-line policy and
do not need a separate algorithm library. Examples include:

- registering a portal rule;
- registering a biome definition or generation feature;
- selecting default sun values;
- publishing block interaction constants;
- granting a public capability message on join;
- registering a command whose effect is already a public request.

If one of these gains substantial reusable logic, extract that logic at that
time.

## Conditional use from a custom server

A custom orchestrator can use the same mechanic for only part of the tree:

```rust
fn handle_open_intents(
    scopes: Res<ServerScopes>,
    world: Res<ServerChunkWorld>,
    rounds: Query<&RoundRules>,
    mut intents: MessageReader<CellMenuOpenIntent>,
    mut opens: MessageWriter<CellMenuOpenRequested>,
) {
    for intent in intents.read() {
        let Some(scope) = scopes.player_scope(intent.player_id) else {
            continue;
        };
        let Some(entity) = scopes.entity(&scope) else {
            continue;
        };
        let Ok(rules) = rounds.get(entity) else {
            continue;
        };
        if !rules.allow_crafting {
            continue;
        }
        if let Some(request) = crafting_table_open_request(&world, intent) {
            opens.write(request);
        }
    }
}
```

The library does not need to know what a round is. The custom server does not
need to copy the geometry or menu construction logic.

## Libraries may still use domain types

“Reusable” does not mean “free of every dependency.” A block placement library
can depend on:

- `ServerChunkWorld`;
- `BlockInstance`;
- player hitboxes;
- item metadata;
- block positions.

Those are its domain contract. It should not depend on:

- the vanilla mod that calls it;
- a concrete filesystem storage provider;
- one top-level server modpack;
- one hardcoded lobby or global audience.

## System sets remain the composition surface

The glue mod should run in an existing public phase:

```text
receive
  -> collect
  -> validate
  -> dispatch
  -> apply effects
  -> consume
  -> synchronize
```

A custom server may:

- replace the glue;
- add validators before it;
- call the same library from another phase;
- use the mechanic only for selected scope nodes;
- combine several mechanics in one custom system.

The library returns domain output; ECS ordering still belongs to the
orchestrator.

## A monolithic `Main` mod is not an architecture failure

A custom server's main mod is allowed to contain a large amount of policy. It
may be the easiest place to understand:

- admission;
- matchmaking;
- scope allocation;
- game lifecycle;
- conditional feature activation;
- cleanup.

The warning sign is not file size by itself. The warning sign is duplicating
generic mechanics or reaching through concrete implementations.

A healthy custom main:

- depends on APIs, event contracts, and mechanic libraries;
- stores game-specific state as ECS components/resources;
- emits public requests and results;
- chooses scope and audience explicitly;
- leaves transport, storage, protocol, and reusable mechanics replaceable.

## Refactor checklist for a vanilla feature

When a vanilla mod grows, ask:

1. Can a custom server call the useful operation without selecting the
   always-on mod?
2. Are conditions such as scope, permission, or game phase inputs rather than
   hidden globals?
3. Does the library return enough information for custom synchronization?
4. Is authoritative mutation still performed exactly once?
5. Does the glue run in a public `SystemSet`?
6. Can another mod replace the policy without depending on the vanilla mod?
7. Is declarative registration being needlessly wrapped in another layer?

Extract the smallest stable mechanic. Do not create a library that simply
renames one line without adding a useful call boundary.
