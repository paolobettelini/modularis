# Assets and generated output

Patchwork copies a mod's `assets/` directory into the generated application's
asset root.

The destination is namespaced:

```text
mods/<mod-id>/assets/<file>
        │
        └──> build-*/<app>/assets/<mod-id>/<file>
```

This prevents two mods from accidentally owning the same path.

## Asset path rules

A stone texture stored at:

```text
mods/block-stone/assets/stone.png
```

is loaded as:

```rust
asset_server.load("block-stone/stone.png")
```

The source path is not used at runtime.

Current asset owners include:

- each textured block mod;
- `item-flint-and-steel`;
- `client-ui-font-dejavu-mod`;
- `client-player-blocky-model-paths-mod`.

## Ownership guidelines

The mod that defines or provides the asset should own it.

Good examples:

- `block-grass` owns grass textures;
- `client-ui-font-dejavu-mod` owns its font and exposes the loaded handle through
  a resource;
- the Blocky player asset provider owns model, texture, idle animation, and
  walk animation.

Avoid placing all assets in a central client mod. That would make small domain
mods non-portable and create hidden dependencies.

## References across mods

Cross-mod asset references are allowed when the dependency is intentional. The
default inventory loadout uses block textures as item favicon metadata:

```rust
favicon: Some(ItemFavicon::new("block-stone/stone.png"))
```

That loadout already depends on the selected block and item registries.

If a feature requires an asset from another mod, make the relationship visible
in Cargo/Patchwork dependencies. Otherwise a valid-looking path may disappear
in another composition.

## Runtime file loading

Most Bevy assets use `AssetServer`. The Blocky parser currently also supports
direct filesystem loading. Its resolver interprets a relative runtime path as:

```text
assets/<relative-path>
```

This works in generated applications because Patchwork has already copied the
asset namespace.

## Generated crate output

Codegen output is another type of composed artifact. It must be treated as
derived state:

```text
contributors + modpack -> generated crate -> final application
```

Do not manually add an enum variant to `generated-block-registry`. Add a block
contributor and regenerate.

Do not manually add a packet to `generated-network-messages`. Add message types
and a network contributor.

## Generated project inspection

Useful places to inspect:

- `build-*/<app>/Cargo.toml`: final dependencies and asset-owning crates;
- `build-*/<app>/src/main.rs`: resolved initialization and ownership order;
- `build-*/generated-*/`: codegen output for that composition;
- `build-*/<app>/assets/`: copied runtime assets.

Inspection is useful for debugging, but source fixes still belong in mods and
modpacks.

## Adding an asset-owning mod

1. Create `mods/my-mod/assets/`.
2. Put only that mod's files inside it.
3. Refer to them as `my-mod/<file>`.
4. Add the mod to the consuming modpack.
5. Recompose.
6. Confirm the file exists under the generated application's assets.

Prefer lowercase stable filenames. Asset paths are serialized in item metadata
and stored in resources, so renaming them is a data compatibility change.
