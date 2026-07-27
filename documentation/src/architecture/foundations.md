# Architecture foundations

The demo follows one central rule:

> Features that can vary independently should be represented by independent
> mods and connected through explicit contracts.

This rule is applied at several scales:

- a block ID and a block placement rule are separate;
- a reusable vanilla mechanic and its always-on vanilla glue are separate;
- inventory storage and quantity stacking are separate;
- network transport and packet meaning are separate;
- chunk generation, routing, caching, residency, meshing, and rendering are
  separate;
- flight capability, capability synchronization, local state, controls, and
  grant policy are separate;
- portal geometry, portal rules, ignition, state, synchronization, travel, and
  rendering are separate.

## Infrastructure versus policy

The project distinguishes neutral infrastructure from selected policy.

Neutral infrastructure answers questions such as:

- how are packets framed?
- where is authoritative inventory state stored?
- how can a mod request a block mutation?
- how does a client cache a chunk?
- how are server packets addressed to players?

Policy answers questions such as:

- how far may a player break a block?
- should every player have flight?
- which terrain generator is used?
- what does right-clicking a crafting table do?
- which players can see each other?
- how should chunk work be prioritized?

Policy mods often include `vanilla` in their names. This is a naming signal,
not a technical requirement. The important property is that a custom modpack
can omit or replace them.

## Compile-time composition

Patchwork composition happens before Cargo compiles the final executable.
Therefore:

- missing contracts are found while composing or compiling;
- generated registries contain only selected contributors;
- the final binary has normal static Rust types;
- mods can use generics to depend on APIs instead of runtime plugin names;
- there is no runtime dynamic library loading requirement.

The cost is that changing the selected feature set requires recomposition and
recompilation. The benefit is strong typing and normal Rust optimization.

## ECS as the integration bus

Bevy ECS is the main runtime integration mechanism:

- resources expose shared services and authoritative state;
- messages represent intentions, commands, results, and synchronization;
- components attach optional behavior to entities;
- `SystemSet` chains provide insertion points;
- state and substates control lifetime and input without destroying unrelated
  systems.

An API crate usually defines these contracts. A provider or feature mod
registers systems that read and write them.

For example, block breaking is not one function call from network code to the
world:

```text
packet
  -> authenticated server request
  -> pending block edit
  -> validators
  -> world mutation
  -> domain result event
  -> audience-aware network synchronization
```

Every arrow is an extension point.

## Dependency direction

Preferred dependency direction:

```text
feature mod ──> API/event crate <── implementation mod
```

Avoid:

```text
feature mod ──> concrete implementation mod
```

unless the feature genuinely requires that implementation.

For example, a chunk request handler should depend on
`server-chunk-world-api`, not directly on
`server-chunk-world-dynamic-impl`. This lets a different modpack select a
file-backed or database-backed world.

## State identity, scopes, and visibility

The project uses different concepts for different responsibilities:

- `ScopeNodeId` identifies a node in the runtime hierarchy;
- `WorldInstanceId` identifies a concrete world instance;
- `ChunkProviderId` identifies a chunk source;
- `WorldScopeId` identifies the combined world namespace;
- `Audience` identifies who can observe or interact with state;
- `PlayerId` identifies a session player;
- domain IDs such as `CellMenuId` identify specific state objects.

These types should not be collapsed into one universal identifier. The runtime
scope tree coordinates them through independent facets, but it does not erase
their meaning. A chest and a public entity may exist in the same world while
having different audiences; two players may share chat while using different
world instances.

The detailed rules are covered in
[Runtime scope trees and facets](runtime-scopes.md), the world chapters, and
the cell-menu chapter.

## Review checklist

When reviewing a change, ask:

- Does this mod choose a policy that another server may want to replace?
- Is the mod depending on a concrete provider instead of an API?
- Is network code performing gameplay mutation directly?
- Is a generated registry owner listing concrete contributors?
- Could a `SystemSet` insertion point avoid a hardcoded callback?
- Is state keyed by the correct identity?
- Does a default behavior belong in a vanilla modpack instead of the base?
- Can a custom server call the mechanic without selecting its always-on
  vanilla glue?
- Is a mod creating an item or block instance with a future-proof metadata
  default?
- Could two selected mods cooperate, or does one silently overwrite the other?

The rest of the book applies this checklist to each subsystem.
