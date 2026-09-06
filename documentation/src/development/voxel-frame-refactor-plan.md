# Voxel frame refactor implementation plan

The root world remains a normal integer chunk grid. Additional grids have a
runtime identity and rigid pose, scoped to an existing world route. They reuse
the chunk, block-state, sparse-component and codec implementations.

## Implementation order

1. Define frame identities, typed local addresses, rigid transform operations,
   conservative bounds and replaceable spatial queries.
2. Extend resident/storage keys with frame identity. Keep structural frame
   persistence separate from chunk payloads and block-component domains.
3. Share a pure three-dimensional interest-volume mechanism between client and
   server; select spherical default interest and distance priorities through
   policy mods. Retain budgets and bounded sparse residency.
4. Replicate frame lifecycle/pose separately from local chunk data. Parent
   rendered local meshes to frame transforms and invalidate per-frame caches.
5. Carry frame addresses from nearest world-space ray hits through validation,
   authoritative mutation, damage, placement and cell-menu interactions.
6. Expose overlap validation independently from mutation and physical collision.
7. Provide on-demand frame creation and optional client pose animation. Verify positive
   and negative coordinates, rotated hit/placement, sparse components, scope
   isolation, persistence and representative complete compositions.

## Required boundaries

- Frame contracts/math: identity, local coordinates, rigid pose; no gameplay.
- Frame registry: runtime lifecycle and sparse occupied-chunk metadata.
- Spatial query service: conservative candidate filtering; replaceable index.
- Chunk interest: replaceable 3D volume; default spherical policy.
- Chunk backend: authoritative local voxel mutations, shared across frames.
- Storage adapters: frame metadata, chunk data and components in separate domains.
- Network adapters: frame lifecycle, transforms, local chunks and feature data.
- Renderer: local meshes under mutable frame transforms.
- Feature policies: vanilla durability, overlap rules and example construction.

Physics engines and far-terrain LOD are intentionally outside this refactor.
Frame transform/bounds contracts are the extension seam for those providers.

## Implemented boundaries

The stages above now have concrete API/library and adapter implementations.
See [Voxel frames](../world/voxel-frames.md) for the crate map, lifecycle,
persistence paths, public phases, spawn command, optional animation and explicit limitations.
Physical moving-platform simulation and cross-domain crash transactions remain
outside this implementation. Verification is compilation-focused; interactive
rendering and save/reload checks must still be performed in the game.
