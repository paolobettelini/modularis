# Minecraft world converter

Imports a Minecraft Java Edition Anvil overworld into Modularis/Patchwork native chunks.

This version keeps the original binary world writer, but replaces the old “same name or stone” conversion with a layered block mapper designed to preserve the visual character of Minecraft worlds as well as possible with the blocks currently available in Modularis.

## Mapping strategy

For each Minecraft block the converter tries, in order:

1. an explicit mapping from `block-mappings.toml`;
2. the same block name (`minecraft:deepslate` -> `demo:deepslate`);
3. generic variant reduction, including stairs/slabs/walls/fences/doors/signs/carpets/panes, `waxed_*`, `infested_*`, wood -> log and hyphae -> stem;
4. material mappings from `block-mappings.toml`;
5. ordered semantic rules from `block-mappings.toml`;
6. a conservative semantic nearest-block score based on material, colour, wood family and block family;
7. `demo:stone` only when none of the above gives a plausible result.

The mapping file deliberately contains the policy/overrides separately from the converter code so it can be tuned without recompiling.

Notably, water and lava no longer become stone by default. Until native fluids exist they are represented with blue/orange stained glass, which generally preserves the world silhouette and colour much better. Change those two entries if another approximation looks better in your renderer.

## Usage

```fish
cargo run --release -- \
  --input ../minecraft_world \
  --output ../../data/hub \
  --mods ../../../mods
```

Use another mapping file:

```fish
cargo run --release -- \
  --input ../minecraft_world \
  --output ../../data/hub \
  --mods ../../../mods \
  --mapping ./my-block-mappings.toml
```

Disable the final semantic matcher and use only deterministic mappings:

```fish
cargo run --release -- \
  --input ../minecraft_world \
  --output ../../data/hub \
  --mods ../../../mods \
  --no-fuzzy
```

The output directory must not already exist.

## Conversion report

Every conversion writes `conversion-report.json` inside the output directory. It contains every Minecraft block kind seen in the world, occurrence count, chosen Modularis block, resolution method and semantic score where applicable.

This makes the remaining bad mappings easy to spot: sort/filter entries whose method is `fallback` or inspect low-score `semantic` mappings, then add an override to `block-mappings.toml`.

A custom report path can be supplied with `--report PATH`.

## Scope

The converter imports block types and default Modularis block states only. Minecraft entities, block entities, scheduled ticks, biomes and Minecraft-specific block state data are intentionally ignored. The output format remains version 2 and uses the same `PWBI` / `PWCR` binary structures as the previous converter.
