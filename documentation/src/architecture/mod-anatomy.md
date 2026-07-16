# Mod anatomy and lifecycle

A Patchwork mod is a Rust crate with a public entry type. Most entries expose
`init` and `run`.

## Minimal contributor mod

Many contributor mods need no Bevy state:

```rust
use tokio::task::JoinHandle;

pub struct BlockStoneMod;

impl BlockStoneMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
```

The crate's important work may be its exported constants and Cargo metadata,
which codegen reads.

## Bevy feature mod

A runtime feature normally receives `BevyMod` and API dependencies:

```rust
pub struct ExampleFeatureMod;

impl ExampleFeatureMod {
    pub fn init<W: ServerChunkWorldApi>(
        bevy: &mut BevyMod,
        _world: &mut W,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            example_system.in_set(SomePublicSet::Validate),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
```

The underscore dependency is still important. It tells Patchwork that the
feature requires a provider and enforces initialization order. Runtime systems
usually access the provider's Bevy resource.

## Provider mod

A provider implements a marker API and inserts a service:

```rust
pub struct MyChunkMesher;

impl MyChunkMesher {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.insert_resource(ChunkMeshService::new(mesh_chunk));
        Self
    }
}

impl ChunkMeshApi for MyChunkMesher {
    fn mesh_chunk(neighborhood: &ChunkMeshNeighborhood) -> ChunkMeshData {
        mesh_chunk(neighborhood)
    }
}
```

In this project, marker traits are often compile-time provider contracts, while
resources such as `ChunkMeshService`, `CollisionService`, or
`ServerChunkRouter` carry callable runtime behavior.

## Bootstrap mod and ownership

The neutral `bevy-mod` creates only an empty app:

```rust
pub struct BevyMod {
    pub app: App,
}

impl BevyMod {
    pub fn init() -> Self {
        Self { app: App::new() }
    }
}
```

Client and server add different plugins. Their bootstrap mods finally take
ownership:

```rust
pub fn run(&self, mut bevy: BevyMod) -> Option<Vec<JoinHandle<()>>> {
    bevy.app.run();
    None
}
```

The manifest declares:

```toml
ownership = ["bevy-mod"]
```

Only one selected mod should own a singleton value such as the final Bevy app.

## `init` should wire, not run the game

Good `init` work includes:

- inserting resources;
- registering messages;
- configuring `SystemSet` order;
- adding systems and observers;
- registering a provider in a registry;
- loading static configuration into a resource.

Avoid long blocking work in `init`. Expensive terrain generation, file loading,
or socket loops should happen in runtime services or tasks.

## Public versus private types

Types intended as cross-mod contracts should live in an API or event crate and
be public.

Types that only support one implementation should remain private:

- internal queue entries;
- retry timers;
- cache bookkeeping;
- renderer material caches;
- temporary drag state.

This prevents other mods from coupling to an implementation detail.

## Generic parameters and concrete domain dependencies

Use a generic API parameter when any provider should work:

```rust
pub fn init<W: ServerChunkWorldApi>(...)
```

Use a concrete dependency when the feature truly needs that contributor or
generated type. For example, the Nether dimension implementation depends on
the Nether provider mod because it registers that exact provider ID.

Concrete dependencies should be reviewed carefully. They often identify either
a valid feature relationship or an opportunity for a new API.

## Naming conventions

The repository commonly uses:

- `*-api`: public contracts and domain types;
- `*-impl`: replaceable implementation;
- `*-mod`: feature or integration mod;
- `*-vanilla-mod`: optional demo policy;
- `*-message-types`: serializable packet payloads;
- `*-network-messages-mod`: protocol contribution metadata;
- `*-registry-codegen`: codegen owner;
- `generated-*`: generated output;
- `client-*` and `server-*`: side-specific behavior.

These names are conventions, but they make architectural intent visible during
modpack review.
