# Networked entities

Entities are runtime ECS objects, not players and not entries inside chunk
palettes. The server owns their UUIDs and state. The generated `EntityKind`
identifies the type selected by composition; `Uuid` identifies one occurrence.

## Composition and responsibilities

| Crate | Responsibility |
| --- | --- |
| entity-type-api | Static type/visual descriptor contract |
| entity-registry-codegen | Reads contributor metadata and generates EntityKind |
| generated-entity-registry | Generated enum, lookup and contributor dispatch |
| entity-crystal-snail | Snail type, model, texture and idle/walk clips |
| entity-state-api | Serializable snapshot, pose and animation state |
| server-entity-api | ECS components, requests, notifications and public phases |
| server-entity-state-mod | Sequential request application to ECS entities |
| server-entity-visibility-api | Per-entity viewer sets, independent of rendering |
| server-entity-world-visibility-vanilla-mod | Default same-world-and-provider visibility |
| server-entity-network-sync-mod | Per-viewer snapshot diff, spawn/update/despawn replication |
| entity-network-message-types | Packet structs |
| entity-network-messages-mod | Packet codegen contribution |
| client-entity-api | Client replica and receive/present phases |
| client-entity-network-mod | Receives packets; clears replicas on disconnect |
| client-entity-blocky-render-mod | Model spawning, pose interpolation and clip playback |
| server-command-spawn-entity-lib | Reusable command registration and request construction |
| server-command-spawn-entity-vanilla-mod | Optional privileged command activation |

The new contract/helper crates use support metadata so downloaded compositions
include them without lifecycle initialization. None is an exclusive provider
just because its name ends with `-api`.

Bundles:

- `entities`: shared registry, content and protocol. Imported by `network`.
- `client-entities`: replica and Blocky presentation. Imported by `client`.
- `server-entities`: state and replication mechanisms, no visibility policy.
- `server-entities-vanilla`: same-scope visibility and command. Imported by
  `server-vanilla`.

TheCrown is not silently changed to install vanilla entity policy. A custom
server may import `server-entities`, fill `EntityViewers` itself, and reuse the
command library or create requests directly.

## Try the snail

Recompose both client and server: the network enum now contains entity packets.

```text
/spawnentity 0 12 0 demo:crystal_snail
/spawnentity 0 12 0 crystal_snail 0.5
```

The optional scale defaults to 1 and must be finite and positive. Coordinates
must be finite. The command requires `Privileged`, using the same permission
registry and command refresh mechanism as other restricted commands.
Completion suggests the generated type IDs after the three coordinates.

The command resolves the invoking player's world/provider route, creates a
fresh UUID and requests spawn. The default idle clip is selected if the type
declares it as its default. The snail does not walk autonomously: motion policy is deliberately
not part of entity state or rendering.

Assets belong to `mods/entity-crystal-snail/assets/`:

```text
models/crystal_snail.blockymodel
textures/crystal_snail.png
animations/idle.blockyanim
animations/walk.blockyanim
```

The texture is 512 by 512. Primitive units use `1.0 / 64.0`, independently of
the runtime scale. Adjust the contributor descriptor if this particular model
needs a different unit conversion. The assets were moved from the supplied
`crystal_snail/` directory; they are not shared with player model settings.

## Add a type

Create a support contributor and declare its own metadata:

```toml
[package.metadata.mod]
support = true

[package.metadata.entity]
id = "mygame:creature"

[dependencies]
entity-type-api = { path = "../entity-type-api" }
```

Export `pub const ENTITY_INFO: EntityTypeInfo`, with optional model/texture
paths, primitive scale and named clips. Include the contributor in the shared
composition, then compose again. Duplicate IDs and conflicting Rust variants
are rejected. The generator sorts IDs deterministically. Never persist generated
enum ordinals as permanent IDs; use the namespaced string for future storage.

A type without a model remains a valid server entity. A different renderer can
consume `ClientEntities` without selecting the Blocky renderer.

## Server API and ECS extension

The default phases are:

```text
Request -> Apply -> Visibility -> Sync
```

Request handlers/custom mechanics run before Apply. Application is exclusive
and sequential, so spawn followed by move/despawn in one update works without
a deferred-command duplicate-UUID race. Successful changes publish
`EntityChanged(uuid)`. The UUID lookup is built once per request batch.

The authoritative object has `ServerEntity(EntitySnapshot)` and
`EntityWorld(WorldScopeId)` components. Add your own ECS components for AI,
health, ownership or scripts; these do not become fields in a generated giant
struct, and are not replicated automatically.

Example inside a system with a `MessageWriter<EntityRequest>`:

```rust
requests.write(EntityRequest::SetPose {
    uuid,
    pose: EntityPose {
        position: [10.0, 4.0, 2.0],
        rotation: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
    },
});

requests.write(EntityRequest::Animate {
    uuid,
    animation: Some(EntityAnimationState {
        clip: "walk".into(),
        speed: 1.0,
        repeat: true,
        revision: 0, // application assigns the replay revision
    }),
});

// Stop playback, holding the current pose:
requests.write(EntityRequest::Animate { uuid, animation: None });

// Remove the runtime entity and all its current client replicas:
requests.write(EntityRequest::Despawn { uuid });
```

Send SetPose repeatedly from a movement mod to move an entity. The client
smooths position and quaternion rotation; scale is authoritative. Sending an
Animate request again restarts that clip through an incremented revision.
Animation assets are resolved from the type descriptor, not arbitrary paths
supplied over the network.

## Visibility and replication

The default policy compares each viewer's routed `WorldScopeId` (world
instance and provider) to the entity's scope. Dimensions alone are not enough
to separate private or minigame worlds.

For custom rules, omit `server-entity-world-visibility-vanilla-mod` and populate
`EntityViewers` in `ServerEntitySet::Visibility`. This policy can consult
Audience, scope trees, teams, distance or arbitrary components. An empty viewer
set means hidden, not broadcast. Visibility is recalculated after relocation.

Replication sends a full upsert only on a changed snapshot or newly visible
entity, and despawn when visibility is lost or the entity is deleted. Disconnected
viewers' sent-state is removed. Newly joined viewers receive current state,
even for entities created before their connection.

Packets carry an ordering sequence. The client restores order across the
separate generated ECS packet types if a despawn and subsequent reappearance
arrive during the same rendered update. Reliable ordering/delivery is supplied
by the existing TCP transport; this is not a new transport implementation.

The client reuses the existing node/visual-separated Blocky renderer and
animation systems. A shared `next_blocky_spawn_id()` allocator prevents
independent player/entity consumers from claiming one another's model spawn
responses. Late model completions after cancellation are removed.

## Current limits

This slice provides runtime spawning, transforms, animation, audience filtering,
replication and rendering. It does not install entity persistence, collision,
AI, pathfinding, automatic locomotion clips or gravity. These can be independent
systems over the authoritative ECS entities.

Newly visible clients start the selected animation locally; animation phase
is not synchronized to an absolute server clock. Interpolation is smoothing
 toward received poses, not a timestamped prediction/reconciliation system.
The vanilla visibility policy has no distance culling yet. These are explicit
extension points, not hidden rules inside the entity core.
