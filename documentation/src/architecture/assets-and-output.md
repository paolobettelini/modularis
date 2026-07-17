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
mods/block-stone/assets/textures/block/stone.png
```

is loaded as:

```rust
asset_server.load("block-stone/textures/block/stone.png")
```

The source path is not used at runtime.

Current asset owners include:

- each textured block mod;
- item mods with dedicated textures, such as `item-flint-and-steel` and
  `item-stick`;
- asset-only JSON model template mods;
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

Cross-mod asset references are allowed when the dependency is intentional. For
example, a block model can inherit a parent exported by another mod:

```json
{
  "parent": "voxel-model-block-templates-mod:block/cube_all",
  "textures": {
    "all": "block-stone:block/stone"
  }
}
```

The block mod declares `voxel-model-block-templates-mod` as a formal Patchwork
dependency even though the template entry type performs no runtime work. This
ensures the parent model is copied in every valid composition.

If a feature requires an asset from another mod, make the relationship visible
in Cargo/Patchwork dependencies. Otherwise a valid-looking path may disappear
in another composition.

## Runtime file loading

Most Bevy assets use `AssetServer`. The Blocky player parser and JSON voxel
model provider also support direct filesystem loading. The voxel provider maps
the resource ID `block-stone:block/stone` to:

```text
assets/block-stone/models/block/stone.json
```

and maps its texture ID to
`assets/block-stone/textures/block/stone.png`. This works in generated
applications because Patchwork has already copied each asset namespace.

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

Prefer lowercase stable filenames. Model IDs are generated registry data and
some asset paths may be serialized in metadata or stored in resources, so
renaming them is a compatibility change.
