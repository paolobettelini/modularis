# Dimensions and world scopes

Dimensions are generated IDs with server-side definitions. They are not
hardcoded branches in the chunk world.

## Generated dimension IDs

Contributor:

```toml
[package.metadata.dimension]
id = "demo:nether"
```

Generated enum:

```rust
pub enum Dimension {
    Aether,
    Nether,
    Overworld,
}
```

This enum is shared by:

- dimension registry;
- player dimension state;
- network messages;
- portal rules.

## Server definition

Runtime properties live in:

```rust
pub struct DimensionDefinition {
    pub id: Dimension,
    pub instance: WorldInstanceId,
    pub provider: ChunkProviderId,
    pub sky_color: [f32; 4],
    pub spawn: [f32; 3],
}
```

The current definitions are:

- Overworld: primary biome-driven provider, blue sky, default spawn;
- Nether: Nether provider, dark red sky;
- Aether: Aether provider, blue sky, elevated spawn.

The Overworld registration explicitly marks itself as default. Registration
order does not choose the default.

Duplicate IDs and duplicate defaults are rejected.

## Per-player dimension state

`ServerDimensions` maps `PlayerId -> Dimension`.

If a player has no explicit entry, lookup falls back to the default dimension.

The active chunk router uses this state to choose:

- world instance;
- chunk provider.

Player visibility also resolves world scope, so dimensions affect both terrain
and entity replication.

## Dimension change pipeline

```text
RequestPlayerDimensionChange
  -> ServerDimensionSet::Apply
  -> registry position update
  -> ServerPlayerDimensionChanged
  -> ServerDimensionSet::Sync
```

The request contains:

- player;
- target dimension;
- optional target position.

The lifecycle mod applies it and falls back to the dimension spawn.

The sync mod sends:

- generated dimension ID;
- player position;
- sky color;
- local movement correction;
- player visibility leave/join deltas.

Travel features only emit the request. They do not clear client chunks or
manually edit visibility.

## Client dimension pipeline

The client has:

```text
ClientDimensionSet::Receive
  -> ResetWorld
  -> ApplyPlayer
```

Receiving a new dimension emits `ClientDimensionChanged`.

Independent mods:

- update dimension state;
- clear chunks and active streaming window;
- move the local player;
- clear portal visuals;
- apply sky state.

This avoids a dimension receive mod directly owning all client world systems.

## World instance versus dimension

A `Dimension` is a generated semantic ID.

A `WorldInstanceId` identifies one runtime instance. Several instances may use
the same dimension rules and provider.

For example, a server could create:

```text
Dimension::Overworld
  instance "match:1"
  instance "match:2"
```

The current registry has one static instance per dimension, but chunk routing
and cache keys already use `WorldInstanceId`.

## Adding a dimension

1. Add a dimension contributor with a namespaced ID.
2. Add it to `dimensions.toml`.
3. Add or reuse a chunk provider.
4. Add a server dimension registration mod.
5. Select both in `server.toml`.
6. Add travel or lifecycle behavior if players need to enter it.
7. Recompose client and server so packets share the new enum variant.

Example registration:

```rust
dimensions.register(
    DimensionDefinition {
        id: Dimension::Moon,
        instance: WorldInstanceId::new("example:moon"),
        provider: ChunkProviderId::new(MOON_PROVIDER_ID),
        sky_color: [0.01, 0.01, 0.03, 1.0],
        spawn: [0.0, 20.0, 0.0],
    },
    false,
)?;
```

Do not edit a central dimension match in the chunk world.

## Current scope limitation

Client chunk cache keys contain only `ChunkPos`, not `WorldScopeId`. This is
safe because the client clears the cache on real dimension changes.

A client that displays several worlds at once would need scope-aware cache and
render keys.
