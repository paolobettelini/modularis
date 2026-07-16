# Blocky models and animation

Remote players use Hytale/Blockbench `.blockymodel` and `.blockyanim` files.

The integration is layered:

```text
blocky_formats
  -> blocky-model-api
  -> Bevy model renderer
  -> blocky-animation-api
  -> Bevy animation sampler
  -> player asset provider
  -> network player renderer
```

## Parser crate

`blocky_formats` is Bevy-independent. It supports:

- model JSON parsing;
- animation JSON parsing;
- unknown-field preservation;
- hierarchy flattening into `RuntimeModel`;
- node-name compatibility checks;
- frame/second sampling;
- linear and approximate smooth interpolation;
- looping across duration boundaries.

Animation time is stored in 60 FPS frame units.

## Spawn contract

Feature mods request:

```rust
SpawnBlockyModel {
    spawn_id,
    model_path,
    texture_path,
    texture_size,
    transform,
    scale,
    primitive_scale,
}
```

The renderer emits:

```rust
BlockyModelSpawned {
    spawn_id,
    root,
    model_path,
}
```

`spawn_id` lets a higher-level feature associate an asynchronous message-style
spawn with its domain object.

## Entity hierarchy

Runtime entities are:

```text
BlockyModelRoot
└── BlockyModelNode            bone/pivot hierarchy
    ├── BlockyModelVisual?     shape mesh only
    └── child BlockyModelNode
```

The node stores:

- root;
- runtime node index/name;
- primitive scale;
- base translation/rotation/scale;
- optional visual entity.

The visual stores:

- mesh/material;
- shape offset;
- shape stretch;
- base visibility.

## Why node and visual are separate

Bone animation must transform child nodes.

Shape animation such as `shapeStretch` and `shapeVisible` must affect only the
mesh of that node.

If mesh and bone share one transform, applying shape stretch also scales every
child bone, causing overlapping or separated body parts.

## Runtime local position

The Blocky format encodes a child node position relative to the center of its
parent's main shape.

The renderer uses:

```rust
RuntimeModel::resolved_local_position(node_index)
```

This restores the parent shape offset before building the Bevy hierarchy.

Using raw child `position` as direct pivot-to-pivot translation compresses the
hierarchy, especially vertically.

The child's own `shape.offset` belongs only on its visual child.

## Coordinate conversion

The renderer consistently converts Blocky/Hytale coordinates to Bevy:

```rust
Vec3::new(x, y, -z)
Quat::from_xyzw(-x, -y, z, w).normalize()
```

The same convention is used for:

- node positions;
- animation positions;
- mesh vertices and normals;
- quaternions.

Applying a coordinate conversion to only animation or only hierarchy data would
misalign the model.

## Primitive geometry

Box and quad meshes are centered around the visual origin.

Primitive size uses absolute values. Negative `shape.stretch` is not converted
into negative mesh dimensions; its sign is preserved in visual transform scale.

This supports mirrored limbs without corrupting primitive construction.

Texture UVs use:

- per-face layout;
- optional texture dimensions;
- mirror flags;
- 90-degree rotations.

The material is cached by optional texture path.

## Animation contract

Feature mods request:

```rust
PlayBlockyAnimation {
    root,
    animation_path,
    speed,
    playback,
}
```

Playback modes:

- once;
- loop;
- ping-pong.

Idle and walk use normal looping.

The animation system caches parsed clips and applies samples to each node by
name.

Animated node translation:

```text
base_translation
  + converted_position * primitive_scale * translation_mask
```

Animated rotation:

```text
converted_delta_rotation * base_rotation
```

Visual shape stretch and visibility are applied only to
`BlockyModelVisual`.

Queries use `Without<BlockyModelVisual>` and `Without<BlockyModelNode>` to keep
mutable `Transform` access disjoint in Bevy ECS.

## Translation masks

`BlockyAnimationTranslationMask` lets a higher-level feature suppress selected
translation axes for one bone.

The player asset provider locks vertical translation for `Pelvis` during walk.
Gameplay/network transform already owns player world height, so authored pelvis
locomotion should not move the whole avatar into the ground.

The mask is a feature-level component, not a parser rule.

## Player asset provider

`client-player-blocky-model-paths-api` exposes:

- model path;
- optional texture and atlas size;
- idle and optional walk clips;
- model scale;
- primitive scale;
- yaw offset;
- node names with vertical translation lock.

The active provider owns:

```text
player.blockymodel
Outlander_1.png
idle.blockyanim
walk.blockyanim
```

Current primitive scale is `1/64`.

Changing player assets requires replacing only this provider when the format
and behavior remain compatible.

## Network player renderer

The renderer:

1. receives join snapshots and join messages;
2. skips the local player's ID;
3. requests a Blocky model spawn;
4. associates spawn IDs with player IDs;
5. starts looping idle;
6. updates transform from movement/rotation packets;
7. switches to walk on lateral movement;
8. returns to idle after `0.20` seconds without lateral motion;
9. projects player name labels into viewport space;
10. removes leave and stale entities.

Movement detection removes the gravity-up component before testing lateral
distance.

Player root rotation combines gravity alignment, network yaw, and asset yaw
offset.

## Name labels

Labels are Bevy UI text entities, not children in world space.

Every frame, the renderer:

- computes a world point above the avatar;
- calls `world_to_viewport`;
- updates absolute UI coordinates;
- hides labels behind/outside the camera.

The avatar and label are both indexed in `RenderedNetworkPlayers` and removed
together.

## Adding another Blocky entity

A mob feature can:

1. own its assets in one mod;
2. emit `SpawnBlockyModel`;
3. track `spawn_id`;
4. attach domain components to the returned root;
5. emit `PlayBlockyAnimation`;
6. update only the root world transform.

It does not need to depend on the player renderer.

## Current limits

- file loading is synchronous and filesystem-based;
- model support focuses on boxes and quads;
- exact exporter interpolation may differ;
- material/shading modes are simplified;
- animation blending is not implemented;
- skeletal clip transitions restart the selected clip;
- UV animation data is parsed but not fully applied;
- cache invalidation for changed files is not implemented.

Future improvements should extend parser, renderer, and animation services
independently.
