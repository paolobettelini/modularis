# Minecraft world converter

This development tool imports Java Edition Anvil region files into Modularis'
native binary chunk format. It converts block types and default block states
only. Unsupported blocks, fluids and feature-specific data become stone;
entities, block entities, scheduled ticks, biomes and other Minecraft systems
are intentionally ignored.

By default it reads `../minecraft_world` and writes `../../data/hub`:

```sh
cargo run --release
```

Alternative input and output directories can be passed as positional
arguments. The converter refuses to overwrite an existing output directory.

