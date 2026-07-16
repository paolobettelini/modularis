# Patchwork composition

Patchwork reads modpack TOML files and Cargo metadata, resolves the selected
mods, runs code generation, copies assets, and writes a normal Cargo project.

The final generated `main.rs` is explicit. It creates each mod in dependency
order, passes dependencies to `init`, and then runs ownership-consuming mods.

A simplified generated application looks like:

```rust
fn main() {
    let mut bevy_mod = bevy_mod::BevyMod::init();
    let mut protocol =
        network_protocol_mod::NetworkProtocolMod::init(&mut bevy_mod, /* ... */);
    let mut world =
        server_chunk_world_dynamic_impl::ServerChunkWorldDynamicImpl::init(
            &mut bevy_mod,
            /* providers and APIs */
        );
    let bootstrap =
        server_game_bootstrap_mod::ServerGameBootstrapMod::init(
            &mut bevy_mod,
            /* runner */
        );

    bootstrap.run(bevy_mod);
}
```

The real generated file is longer because the demo contains hundreds of small
crates, but the mechanism remains ordinary Rust.

## Modpack format

A modpack has descriptive fields and two dependency lists:

```toml
name = "Minecraft demo vanilla server features"
description = "Optional vanilla-style server gameplay features."
modpacks = []
ignore = []

mods = [
    "server-player-movement-collision-vanilla-mod",
    "server-place-block-item-use-mod",
]
```

`modpacks` imports other compositions. `mods` adds direct crates. `ignore`
removes selected mods from the composed dependency set when a profile wants to
override an imported choice.

The current top-level client imports:

```toml
modpacks = ["common", "client-vanilla", "client-graphics"]
```

The server imports:

```toml
modpacks = ["server-base", "server-vanilla"]
```

This allows a profile to reuse a broad default while replacing one policy.

## Cargo metadata for a mod

The common shape is:

```toml
[package.metadata.mod]
entry = "MyFeatureMod"

[package.metadata.mod.dependencies]
init = ["bevy-mod", "some-api"]
run = []
ownership = []
```

The `entry` type is the type Patchwork constructs.

Dependency phases have different meanings:

- `init`: mutable references passed to `init`;
- `run`: dependencies required before or during `run`;
- `ownership`: values moved into `run`.

The bootstrap mods own `bevy-mod` because they finally consume the Bevy app and
call `App::run`.

## API providers

A concrete implementation can declare:

```toml
[package.metadata.mod]
entry = "ClientTcpNetwork"
provides = "client-network-api"
```

Consumers depend on `client-network-api`, not on the TCP crate. Patchwork
selects the provider chosen by the modpack and passes its concrete type to the
consumer's generic `init` function.

For example:

```rust
impl SomeClientFeature {
    pub fn init<N: ClientNetworkApi>(
        bevy: &mut BevyMod,
        _network: &mut N,
    ) -> Self {
        // Register systems using the ClientNetworkSender resource.
        Self
    }
}
```

The generic bound creates a compile-time contract while the Bevy resource is
the runtime service.

## Composition errors are useful

Patchwork should reject or expose:

- missing mods;
- cyclic modpack imports;
- self-references;
- missing API providers;
- conflicting exclusive providers;
- invalid ownership relationships;
- codegen contributors with duplicate IDs;
- unresolved dependencies.

These errors protect the architecture. Avoid "fixing" them by adding broad
dependencies to a central mod. Select the correct provider or split the
contract more clearly.

## Generated projects are not architecture

The generated project shows the resolved result, but it is not where design
changes belong.

Use generated code to:

- inspect initialization order;
- confirm the selected provider;
- debug a missing generated variant;
- inspect the final packet enum;
- verify copied assets and Cargo dependencies.

Make changes in:

- contributor manifests;
- API crates;
- feature mods;
- implementation mods;
- modpack TOML files;
- codegen generators.

Then run Patchwork again.
