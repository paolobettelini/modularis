# Cargo dependency sources

This repository contains two different kinds of Cargo crates under `mods/`:

- Patchwork mods, which declare `[package.metadata.mod]`;
- plain Rust libraries, which have no Patchwork mod metadata.

They deliberately use different dependency sources. This is required for both
local composition and installation from the Patchwork registry.

## Patchwork mods use sibling paths

Every dependency on a normal, API, or support mod uses a local sibling path:

```toml
[dependencies]
block-stone = { path = "../block-stone" }
block-registry-codegen = { path = "../block-registry-codegen" }
network-codegen-utils = { path = "../network-codegen-utils" }
```

All three kinds are real Patchwork projects:

```toml
[package.metadata.mod]
entry = "SomeRuntimeMod"
```

```toml
[package.metadata.mod]
api = true
```

```toml
[package.metadata.mod]
support = true
```

Patchwork must open their directories to read metadata, copy assets, resolve
providers, order lifecycle calls, or run declared codegen. The composed Cargo
project also uses paths to the exact downloaded mod folders. A Git dependency
would hide the selected registry artifact behind a second Cargo download and
would prevent domain generators from discovering it as a contributor.

## Plain libraries use Git

A crate without `[package.metadata.mod]` is not downloaded as a Patchwork
project. It is an ordinary Cargo implementation dependency. References to such
crates use the Modularis repository:

```toml
[dependencies]
block-api = { git = "https://github.com/paolobettelini/modularis.git" }
codegen-utils = { git = "https://github.com/paolobettelini/modularis.git" }
voxel-models-lib = { git = "https://github.com/paolobettelini/modularis.git" }
```

This makes a registry installation complete. The launcher downloads only the
selected Patchwork mods; Cargo obtains their plain library dependencies from
Git. Do not add `support = true` only to make a helper library downloadable.
Use a support mod when the crate itself is a selected composition artifact,
such as an asset bundle or a codegen generator.

## Local development patches

Fetching every helper library from Git would make local development slow and
would ignore uncommitted library changes. `mods/.cargo/config.toml` therefore
patches the Modularis Git source back to the adjacent local crates:

```toml
[patch."https://github.com/paolobettelini/modularis.git"]
block-api = { path = "block-api" }
codegen-utils = { path = "codegen-utils" }
```

The file contains only plain libraries. It must not patch normal, API, or
support mods, because those already use sibling paths directly.

Cargo discovers `.cargo/config.toml` from its working directory and parent
directories. A command run from `mods/` sees this configuration even when its
manifest is elsewhere:

```bash
cd mods
cargo check --manifest-path ../build-client/client/Cargo.toml
```

Patchwork runs codegen generator Cargo commands with `mods/` as their working
directory for the same reason. A command run from the repository root does not
discover `mods/.cargo/config.toml` merely because `--manifest-path` points into
`mods/`.

Using no local patch is still valid: Cargo simply compiles the Git copy. This is
the expected behavior in a downloaded registry installation.

## Synchronization script

Do not maintain hundreds of dependency declarations or patch entries by hand.
Run:

```bash
mods/.cargo/sync-modularis-deps.sh
```

Preview changes without writing:

```bash
mods/.cargo/sync-modularis-deps.sh --dry-run
```

The script scans every direct child crate of `mods/`, reads its package name,
ignores ephemeral `generated-*` development outputs, and classifies the rest by
the presence of `[package.metadata.mod]`. It then:

1. rewrites sibling Patchwork mod dependencies to `path = "../<folder>"`;
2. rewrites sibling plain-library dependencies to the Modularis Git URL;
3. preserves fields such as features and package aliases;
4. regenerates `mods/.cargo/config.toml` with local patches for plain libraries
   only.

The generated config file starts with a warning not to edit it by hand. Run the
script after adding a crate, changing whether a crate is a Patchwork mod, or
performing a broad dependency migration. A clean dry run should report zero
manifest changes.

## Codegen boundary

A domain generator first scans path dependencies in the composed project to
find selected contributor mods. That scan is intentionally path-based.

The contributor's ordinary Rust dependencies are a separate concern. If the
generated source names `block_api::BlockInfo`, the generator reads the
`block-api` dependency declaration from the contributor and carries its Cargo
source into the generated crate:

```toml
[dependencies]
block-api = { git = "https://github.com/paolobettelini/modularis.git" }
```

The generator does not need the `block-api` directory in the Patchwork mod
cache. Git and registry sources remain unchanged; a local path is canonicalized
relative to the contributor and rebased relative to the generated crate.

This distinction fixes errors such as:

```text
block contributor 'demo:air' does not depend on block-api
```

In that case the contributor did depend on `block-api`, but the old generator
treated every non-path dependency as absent. The correct invariant is:

> contributors are selected Patchwork mods and therefore paths; libraries used
> by generated code may use any supported Cargo source.
