# Server world providers and residency

The server world is split into four main contracts:

1. providers generate base chunks;
2. routing selects provider and instance for a viewer;
3. the world backend caches chunks and stores edits;
4. residency decides which requests and cached chunks are allowed.

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
`ServerWorldSeed` owns one root `u64` and derives stable child seeds from:

```text
root seed + feature/provider namespace + WorldInstanceId
```

The namespace keeps unrelated algorithms independent. Changing a biome
selector must not silently change a terrain provider's random stream, and two
world instances must not accidentally generate the same terrain unless a
custom seed provider chooses that behavior.

The selected `server-world-seed-random-impl` creates a random root at server
startup. For reproducible development or a fixed world, set:

```sh
PATCHWORK_WORLD_SEED=123456
```

The Overworld, Nether, Aether, alternate Perlin/checkerboard providers, and the
vanilla biome selector all derive their values from this service. Local numeric
constants in a noise expression are algorithm salts, not independent world
seeds.

A persistent server should replace the random provider with a mod that loads
and saves the root seed next to the world data. Features should depend only on
`server-world-seed-api`, never on that concrete implementation.

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

## Dynamic world backend

`server-chunk-world-dynamic-impl` stores:

```text
chunks: ResidentChunkKey -> generated/resident Chunk
edits:  ResidentChunkKey -> LocalBlockPos -> BlockInstance
```

On a query:

1. route the viewer;
2. return resident chunk if present;
3. ask the selected provider to generate;
4. apply sparse edits;
5. cache and return the result.

On mutation:

1. resolve the same resident key;
2. load the chunk if needed;
3. mutate the resident chunk;
4. store the new block in the sparse edit overlay;
5. return `BlockMutation` with scope, previous, and current instance.

Eviction removes base chunks but keeps sparse edits. Regeneration restores the
same changed world.

The overlay is in RAM and is not persistent storage. A file/database provider
or persistence mod is required for durable worlds.

## World operations

`ServerChunkWorld` exposes:

- viewer-aware chunk and block queries;
- player convenience methods;
- `set_block`;
- `place_block` only when current block is air;
- `break_block` only when current block is not air;
- resident key lookup;
- cache retention;
- resident key enumeration.

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

## Synchronous generation limit

Provider generation currently runs synchronously during world queries. A slow
file or procedural provider can block the Bevy schedule.

An asynchronous design should add a pipeline such as:

```text
request -> queued generation task -> ready chunk -> resident insertion -> response
```

The public provider and world contracts may need an async-ready result type or a
separate generation service. Do not hide a blocking file read inside the
current synchronous `generate`.
