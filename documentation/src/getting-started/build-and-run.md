# Build and run

Patchwork composes the selected mods into ordinary Cargo projects. The current
client and server outputs are written to `build-client/client` and
`build-server/server`.

## Requirements

You need:

- a Rust toolchain that supports edition 2024;
- the `patchwork` command available in `PATH`;
- Bevy's platform dependencies;
- an available local TCP port, `9999` by default.

The project expects the environment to choose Cargo's target directory. Do not
hardcode a different target directory in project scripts or documentation.

## Compose the client

From the repository root:

```sh
patchwork compose \
  --modpack client \
  --modpacks-folder ./modpacks \
  --mods-folder ./mods \
  --cache ./build-client
```

The generated manifest is:

```text
build-client/client/Cargo.toml
```

Check it with:

```sh
cargo check --manifest-path build-client/client/Cargo.toml
```

## Compose the server

```sh
patchwork compose \
  --modpack server \
  --modpacks-folder ./modpacks \
  --mods-folder ./mods \
  --cache ./build-server
```

Then check:

```sh
cargo check --manifest-path build-server/server/Cargo.toml
```

## Compose the TheCrown server

The alternate scoped parkour server uses the same client:

```sh
patchwork compose \
  --modpack thecrown \
  --modpacks-folder ./modpacks \
  --mods-folder ./mods \
  --cache ./build-thecrown
```

Check it with:

```sh
cargo check --manifest-path build-thecrown/thecrown/Cargo.toml
```

Recompose after changing:

- a modpack;
- Cargo metadata used by Patchwork;
- block, item, metadata, dimension, setting, or packet contributors;
- a generated crate contract;
- a mod's asset directory.

Changing ordinary Rust code inside a mod may only require Cargo to rebuild the
generated project, but recomposing is the safest workflow because Patchwork may
also need to update dependencies or copied assets.

## Run

Start the server first:

```sh
cargo run --manifest-path build-server/server/Cargo.toml
```

On its first start, the selected demo catalog creates:

```text
<server-executable-directory>/data/worlds/{overworld,nether,aether}/
```

Each directory contains its persistent seed and binary chunk data. Stop the
server normally, including with terminal Ctrl-C, to run the shutdown flush.
The `data` directory is runtime state and must not be treated as disposable
build output. If the executable is moved, copy its `data` directory with it to
retain the same worlds.

Start one or more clients in other terminals:

```sh
cargo run --manifest-path build-client/client/Cargo.toml
```

To run TheCrown instead of the vanilla server:

```sh
cargo run --manifest-path build-thecrown/thecrown/Cargo.toml
```

Default controls:

| Action | Default |
| --- | --- |
| Move | `W`, `A`, `S`, `D` |
| Look | Mouse |
| Break block | Left mouse button |
| Use held item | Right mouse button |
| Jump / ascend while flying | `Space` |
| Sprint | Left Control |
| Descend while flying | Shift |
| Inventory | `E` |
| Pause | `Escape` |
| Select hotbar | Number keys or mouse wheel |
| Toggle flight | Double-tap jump, if the server granted capability |

## Generated output is disposable

Do not make source changes in:

```text
build-client/
build-server/
build-thecrown/
mods/generated-*/
```

The development copies under `mods/generated-*` are useful for direct crate
checks and editor support, but their content is still generated. Change the
contributor metadata or codegen implementation instead.

## Common failures

### Missing API provider

Patchwork reports an unsatisfied API when a selected mod depends on an abstract
API but the modpack does not select an implementation that declares
`provides = "..."`.

Fix the modpack, not the consumer.

### Duplicate provider

Singleton APIs normally accept one provider. Selecting both TCP and UDP
implementations, or two primary chunk providers, may create a provider
conflict.

Choose one implementation or redesign the API to support keyed providers.

### Generated enum variant missing

If code references `BlockId::Something` but the contributor is not in the
modpack, code generation cannot create that variant.

Either include the contributor or avoid a hard dependency on that concrete
variant.

### Asset not found

Assets are copied under `assets/<mod-id>/`. A path must include that namespace:

```rust
asset_server.load("block-stone/textures/block/stone.png")
```

Do not assume another mod owns or copies the file unless the dependency is
explicit.

### Client and server protocol mismatch

Both applications must be composed from compatible network contributors.
Recompose and rebuild both sides after adding, removing, or changing packet
types.
