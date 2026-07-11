# blocky_formats

Rust parser and lightweight runtime helpers for Hytale/Blockbench `.blockymodel` and `.blockyanim` files.

This crate is designed for a Minecraft-like game engine, especially if you are using Bevy. It does not depend on Bevy directly. Enable the optional `glam` feature if you want conversions to the math types Bevy uses.

```toml
[dependencies]
blocky_formats = { path = "../blocky_formats", features = ["glam"] }
```

## What it supports

- Read `.blockymodel` JSON.
- Read `.blockyanim` JSON.
- Preserve unknown JSON fields via `extra` maps so newer exporter fields do not immediately break your pipeline.
- Flatten the node hierarchy into `RuntimeModel`.
- Resolve animation targets by node name.
- Check name-based compatibility between a model and an animation.
- Sample animation tracks in seconds or frames.
- Convert `Vec2f`, `Vec3f`, `Quatf` into `glam` types with the `glam` feature.

## Compatibility notes

The parser accepts Hytale Blockbench quad shapes where `settings.size` is encoded as `{ "x": ..., "y": ... }` without `z`. Missing vector components default to `0.0`, while missing quaternion `w` defaults to `1.0`.

Looping animations are sampled across the duration boundary. This matters for tracks that do not contain a keyframe at frame `0`: when `holdLastKeyframe == false`, the sampler interpolates from the last keyframe back to the first keyframe instead of snapping to the first keyframe.

`interpolationType: "smooth"` is approximated with smoothstep in this lightweight sampler. Replace the sampler if you need exact authoring-tool playback.

## Basic loading

```rust
use blocky_formats::{BlockyAnimation, BlockyModel, RuntimeModel};

fn main() -> blocky_formats::Result<()> {
    let model = BlockyModel::from_path("assets/mobs/player.blockymodel")?;
    let anim = BlockyAnimation::from_path("assets/mobs/walk.blockyanim")?;

    let runtime = RuntimeModel::from(&model);
    let compatibility = runtime.check_animation_compatibility(&anim);

    println!("matched nodes: {}", compatibility.matched_node_names.len());
    println!(
        "animation tracks without model node: {}",
        compatibility.animation_nodes_missing_in_model.len()
    );

    let t = 0.25;
    if let Some(sample) = anim.sample_node_seconds("Head", t) {
        println!("Head at {t}s: {sample:?}");
    }

    Ok(())
}
```

## Bevy-style usage

```rust
use blocky_formats::{BlockyModel, RuntimeModel};
use bevy::prelude::*;

fn spawn_blocky_model(mut commands: Commands) -> blocky_formats::Result<()> {
    let model = BlockyModel::from_path("assets/models/character.blockymodel")?;
    let runtime = RuntimeModel::from(&model);

    for node in &runtime.nodes {
        let translation: Vec3 = node.position.into();
        let rotation: Quat = node.orientation.into();

        commands.spawn((
            Name::new(node.name.clone()),
            Transform::from_translation(translation).with_rotation(rotation),
        ));
    }

    Ok(())
}
```

If your Bevy version uses a different `glam` major/minor version from this crate, construct the Bevy math values manually:

```rust
let translation = bevy::math::Vec3::new(node.position.x, node.position.y, node.position.z);
let rotation = bevy::math::Quat::from_xyzw(
    node.orientation.x,
    node.orientation.y,
    node.orientation.z,
    node.orientation.w,
);
```

## Notes on units and animation time

The Hytale Blockbench plugin stores `.blockyanim` keyframe times and animation duration in 60 FPS frame units. The helper methods convert seconds to frames for you:

```rust
let sample = animation.sample_node_seconds("Body", 0.5);
let same_sample = animation.sample_node_frames("Body", 30.0);
```

For Bevy 0.19, enable the `glam` feature. This crate pins optional `glam` to `0.32`, matching Bevy 0.19's math stack.

```toml
blocky_formats = { path = "../blocky_formats", features = ["glam"] }
```

The recommended runtime integration is to spawn one Bevy entity per `.blockymodel` node, parent them according to the model hierarchy, attach a `Mesh3d`/`MeshMaterial3d` only when the node has a visible shape, and apply `.blockyanim` samples to each node's local `Transform`.

When building that hierarchy, use `RuntimeModel::resolved_local_position` for
the node transform. A child node's encoded `position` is relative to the center
of its parent's main shape, so the parent's `shape.offset` must be restored.
Use the child's own `shape.offset` only for its visual mesh translation.

Important for mirrored parts: preserve negative `shape.stretch` values. Do not clamp negative stretch to a small positive value, because this breaks left/right mirrored limbs and quads.
