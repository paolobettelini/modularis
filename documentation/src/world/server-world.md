# Server world providers, persistence, and residency

The server world is split into five main contracts:

1. providers generate base chunks;
2. routing selects provider and instance for a viewer;
3. storage loads and buffers durable chunks;
4. the world backend coordinates resident chunks, storage, and generation;
5. residency decides which requests and cached chunks are allowed.

## Provider registry

A provider implements:

```rust
pub trait ServerChunkProvider {
    fn generate(
        &self,
        request: &ChunkGenerationRequest,
    ) -> Option<Chunk>;
}
```

The request includes:

- `ChunkViewer`;
- `WorldInstanceId`;
- `ChunkPos`.

The registry stores several providers by `ChunkProviderId`.

Registration rejects duplicate IDs.

## Current providers

The demo selects:

- `server-chunk-provider-biomes-mod`: biome-driven Overworld terrain;
- `server-chunk-provider-nether-mod`: biome-driven Nether terrain without an
  implicit vertical floor;
- `server-chunk-provider-aether-mod`: biome-driven floating islands;
- `server-chunk-provider-perlin-mod`: alternate single-biome terrain, not active
  in the main server;
- `server-chunk-provider-checkerboard-mod`: alternate test provider, not active
  in the main server.

Providers answer all integer `ChunkPos` values.

The biome-aware providers share `server-biome-sampling-api` for dimension
filtering, cached selection, terrain-parameter blending, feature bounds, and
phased feature dispatch. They do not share geometry policy.

They use uniform fast paths for guaranteed regions:

- the biome Overworld provider combines local surface bounds with declared
  feature ranges, returning uniform air or underground chunks when no feature
  can affect them;
- Nether returns air above every local surface and relevant feature;
- Aether returns air outside all island columns and feature bounds;
- checkerboard returns simple uniform deep/sky chunks.

Exact terrain policy belongs to each provider.

## World seeds

Procedural generation depends on the replaceable `server-world-seed-api`.
`ServerWorldSeed` can own a fallback root `u64` plus a seed for each registered
world instance. Providers derive stable child streams from:

```text
world seed + feature/provider namespace
```

The namespace keeps unrelated algorithms independent. Changing a biome
selector must not silently change a terrain provider's random stream, and two
world instances must not accidentally generate the same terrain unless a
custom seed provider chooses that behavior.

The selected `server-world-seed-catalog-fs-impl` reads each world's seed from
`info.json`. It creates the file the first time that world is opened. A later
restart reads the stored value, so procedural chunks remain reproducible. The
initial seed can be made deterministic with:

```sh
PATCHWORK_WORLD_SEED=123456
```

The environment variable only affects worlds whose `info.json` does not exist
yet. Editing or removing a world directory is an explicit data-management
operation; changing the environment variable does not rewrite an existing
world.

The Overworld, Nether, Aether, alternate Perlin/checkerboard providers, and the
vanilla biome selector all derive their values from this service. Local numeric
constants in a noise expression are algorithm salts, not independent world
seeds.

`server-world-seed-random-impl` remains available for transient compositions.
Features must depend only on `server-world-seed-api`, never on either concrete
seed provider.

Because chunk Y coordinates are unbounded, world Y zero is not a valid generic
"bottom of the world". The Nether provider therefore uses biome strata at Y
zero instead of inserting bedrock there. A bounded world can add a separate
bedrock-floor feature or select a provider whose contract includes a floor.

The dimension-scoped registry, selector, shared sampler, phased features, and
provider responsibilities are described in
[Biomes and world-generation features](biomes.md).

## Adding a provider

```rust
pub const MOON_PROVIDER_ID: &str = "example:moon";

struct MoonProvider;

impl ServerChunkProvider for MoonProvider {
    fn generate(
        &self,
        request: &ChunkGenerationRequest,
    ) -> Option<Chunk> {
        if request.position.y > 0 {
            return Some(Chunk::filled(
                request.position,
                BlockId::Air,
            ));
        }
        Some(generate_moon_chunk(request.position))
    }
}
```

The provider mod registers it:

```rust
registry.register(
    ChunkProviderId::new(MOON_PROVIDER_ID),
    MoonProvider,
)?;
```

Registration does not make it active. A routing or dimension mod must select
its ID.

## Viewer-aware routing

`ServerChunkRouter` resolves:

```text
(ChunkViewer, ChunkPos)
    -> ServerChunkRoute {
         instance,
         provider
       }
```

The active dimension router asks `ServerDimensions` for a player's current
dimension and returns the dimension's instance/provider.

An alternate `server-chunk-routing-single-world-mod` exists for servers without
dimensions.

`server-chunk-routing-scopes-mod` is the runtime-hierarchy provider. It resolves
the player's primary scope, finds the nearest world facet, and uses the route
bound there. A parent can therefore share a route with all descendants, while
a child can override only its own subtree.

Custom routing may choose based on:

- player ID;
- team;
- region;
- permission;
- current game match;
- coordinate;
- server-only versus player query.

If routing changes for an existing player, client synchronization must reset or
version the affected cache.

`server-player-world-api` and the `PlayerWorldChanged` packet provide that
generic transition without pretending that every world switch is a dimension
change.

## World scope and resident keys

The full cache key is:

```rust
ResidentChunkKey {
    instance: WorldInstanceId,
    provider: ChunkProviderId,
    position: ChunkPos,
}
```

`instance + provider` becomes a `WorldScopeId`.

Two viewers can query the same `ChunkPos` but receive different resident keys.
This prevents edits in one instance from affecting another.

## World catalog

`server-world-catalog-api` maps runtime instances to durable directories:

```rust
WorldDirectory {
    id: WorldId,
    instance: WorldInstanceId,
    root: PathBuf,
}
```

`WorldId` is the stable folder name. It is deliberately separate from
`Dimension`: two worlds may use the same dimension and terrain provider while
having different IDs, seeds, chunks, entities, and players.

The catalog rejects duplicate world IDs, instances, and roots. It also limits
world IDs to safe folder-name characters. A custom server can replace the
catalog implementation without changing routing, generation, or storage.

The demo selects `server-world-catalog-build-server-impl`, which registers:

```text
build-server/worlds/
├── overworld/
├── nether/
└── aether/
```

These paths are a demo policy, not part of the catalog API. A production server
can provide paths from settings, command-line arguments, a save selector, or a
multi-tenant world service.

## Dynamic world backend

`server-chunk-world-dynamic-impl` owns only the resident chunk cache and
coordinates the other contracts:

```text
chunks:      ResidentChunkKey -> Chunk
unpersisted: ResidentChunkKey set
```

On a query:

1. route the viewer;
2. return resident chunk if present;
3. ask `ServerChunkStorage` for the exact instance/provider/position key;
4. if storage misses, ask the selected provider to generate;
5. queue a generated chunk for durable storage;
6. cache and return the result.

On mutation:

1. resolve the same resident key;
2. load the chunk if needed;
3. mutate the resident chunk;
4. queue the complete compact chunk in storage's write-behind buffer;
5. return `BlockMutation` with scope, previous, and current instance.

The storage backend is therefore a decorator between the resident cache and
generation. Terrain providers do not perform file I/O and streaming code does
not know whether a chunk came from disk or procedural generation.

`server-chunk-storage-memory-impl` provides the same storage contract for
transient worlds. It keeps generated and edited chunks available across
resident-cache eviction and can discard all data for one runtime instance.

If queueing a modified chunk fails, the world marks it as unpersisted and keeps
it resident. Residency maintenance retries the write and refuses to evict that
chunk until storage accepts it. This avoids silently losing an edit because of
a temporary storage failure.

The complete filesystem format and flush policy are documented in
[Chunk coordinates and storage](chunk-storage.md).

## World operations

`ServerChunkWorld` exposes:

- viewer-aware chunk and block queries;
- player convenience methods;
- `set_block`;
- `place_block` only when current block is air;
- `break_block` only when current block is not air;
- resident key lookup;
- cache retention;
- resident key enumeration;
- explicit transient-instance discard.

The API returns structured errors:

- route unavailable;
- chunk unavailable;
- block already air;
- position occupied.

Gameplay should use these operations instead of mutating chunk maps directly.

## Residency policy

`ServerChunkResidencyConfig` has:

- horizontal radius;
- vertical radius;
- maintenance interval.

The vanilla player-interest mod builds desired resident keys around active
players and evicts other resident chunks.

Chunk request handling validates the requested position against this policy.
This prevents a client from forcing arbitrary distant chunks into server memory.

The server radius includes slack beyond the client's nominal window to account
for movement and network latency.

## Replacing residency

Alternative policies may retain:

- all edited chunks;
- chunks around entities rather than players;
- a fixed spawn region;
- LRU chunks under a byte budget;
- chunks required by scheduled machines;
- no cache at all for a cheap provider.

Keep request authorization separate from cache eviction if their rules differ.
The current vanilla policy is intentionally simple.

Its desired-key calculation lives in
`server-chunk-residency-player-interest-lib`. A custom orchestrator can reuse
the calculation for only selected viewers without installing the blanket
vanilla timer system.

## Synchronous generation limit

Storage reads, region flushes, and provider generation currently run
synchronously. Opening a large region or writing many dirty regions can block a
server tick.

An asynchronous design should add a pipeline such as:

```text
request
  -> resident lookup
  -> queued storage read
  -> queued generation on miss
  -> ready chunk
  -> resident insertion
  -> response
```

The public storage/provider and world contracts may need an async-ready result
type or a separate generation service. Do not hide a blocking file read inside
the current synchronous `generate`.
