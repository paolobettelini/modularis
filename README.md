# Minecraft Simple Demo

`minecraft_simple_demo` is a multiplayer voxel game built with Rust, Bevy, and
Patchwork. Client and server are composed at compile time from small,
replaceable mods.

The architecture, feature pipelines, extension guides, and crate map live in
the mdBook under [`documentation/`](documentation/).

Build the documentation with:

```sh
cd documentation
mdbook build
```

## Compose

From this repository root:

```sh
patchwork compose \
  --modpack server \
  --modpacks-folder ./modpacks \
  --mods-folder ./mods \
  --cache ./build-server

patchwork compose \
  --modpack client \
  --modpacks-folder ./modpacks \
  --mods-folder ./mods \
  --cache ./build-client
```

## Run

Start the server:

```sh
cargo run --manifest-path build-server/server/Cargo.toml
```

Then start one or more clients:

```sh
cargo run --manifest-path build-client/client/Cargo.toml
```

Generated build directories and `mods/generated-*` crates are derived output.
Change mods, contributor metadata, code generators, or modpacks, then compose
again.
