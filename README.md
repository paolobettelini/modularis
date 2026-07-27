# Modularis

`Modularis` is a multiplayer voxel game built with Rust, Bevy, and
adopting the Patchwork architecture.
Client and server are composed at compile time from small,
replaceable mods.

Check out the AI-generated
<b>[documentation](https://paolobettelini.github.io/modularis)</b>.

<div align="center">
  <img src="./media/preview.png" alt="Preview" width="600">
</div>

## Build and run

You can either use the [Patchwork desktop application](https://github.com/paolobettelini/patchwork)
or build everything manually.

To compose and build manually, from this repository root:

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

Start the server:

```sh
cargo run --manifest-path build-server/server/Cargo.toml
```

Then start one or more clients:

```sh
cargo run --manifest-path build-client/client/Cargo.toml
```

The same client can connect to the alternate scoped parkour server:

```sh
patchwork compose \
  --modpack thecrown \
  --modpacks-folder ./modpacks \
  --mods-folder ./mods \
  --cache ./build-thecrown

cargo run --manifest-path build-thecrown/thecrown/Cargo.toml
```

`thecrown` uses runtime scope nodes to host separate parkour chat groups and a
private transient world for every player.
