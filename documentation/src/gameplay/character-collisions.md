# Gravity-relative character collision and moving supports

The character remains kinematic. No rigid-body engine, impulses between bodies,
mass solver or frame-versus-frame physics simulation is installed.

## Shape and coordinate contract

The active character collider is a capsule whose axis is the player's normalized
gravity-up vector. Its origin is the foot point. Radius and height come from the
existing scaled PlayerHitbox. The capsule's segment runs from feet + up * radius
to feet + up * (height - radius); very short capsules clamp radius to half-height.

This deliberately replaces the old Y-aligned box. It avoids choosing an
arbitrary tangent orientation for a square collider under changing gravity.
Small characters retain small capsules and can enter low/narrow model gaps.
Skin and probe/step distances scale with character dimensions.

Voxel obstacles still come from BlockShapeService's model-box union. Root boxes
use identity pose; extra-frame boxes are rigidly transformed into oriented
world boxes. The adapter uses the frame BVH and only occupied extra-frame
chunks. Model geometry is not replaced by a full cube. Unloaded client data
blocks traversal in every direction; there is no artificial Y=0 stone floor.

## Boundaries

| Concern | Crate |
| --- | --- |
| Queries, contacts, support identity and backend | collision-api |
| Capsule distance, sweep, resolution, slope/step mechanics | character-collision-lib |
| Root/frame/model geometry gathering | voxel-frame-collision-lib |
| Client cache and frame adapter | client-collision-block-aabb-impl |
| Routed server geometry and movement checks | server-player-movement-collision-lib |
| Server jump validation | server-player-jump-lib |
| Client support state | client-player-surface-api |
| Client attachment/carry/detach policy | client-player-surface-motion-mod |
| Server anchor validation/rebasing | server-player-surface-lib |
| Default server observation and activation | server-player-surface-motion-vanilla-mod |
| Authoritative trajectory contracts | voxel-frame-kinematic-api |
| Authoritative trajectory sampling | server-voxel-frame-kinematic-mod |
| Curve math and interpolation | voxel-frame-movement-lib |
| Replication / client playback / renderer | existing separate voxel-frame adapters |

The client provider keeps its historical name for modpack compatibility; its
active character path is no longer axis-by-axis AABB resolution. Legacy
CollisionService entry points remain for older providers, while character
consumers use resolve_character and character_support.

A custom server may omit vanilla movement/support validators and call the
libraries conditionally. CharacterQuery exposes up, dimensions, slope threshold,
probe distance, step height and an optional excluded surface. No block IDs or
game modes appear in the mathematical solver.

## Solver

Penetration recovery also corrects overlaps smaller than the collision skin.
Non-walkable slopes add a gravity-relative wall constraint before contact-plane
clipping; subtracting upward motion after clipping would push the character
back into the slope and can trap sideways movement on tilted frames.

The capsule-to-box distance is computed in the oriented box's local coordinates.
The closest segment/AABB distance is piecewise quadratic; the implementation
evaluates the intervals split by box-plane crossings.

Movement uses conservative advancement, followed by iterative contact-plane
clipping. It does not check only the final position, so thin model elements are
not intentionally skipped at high speed. Existing penetration is resolved with
bounded separating corrections.

Ground support is a sweep along -up and requires a normal satisfying:

```text
normal.dot(up) >= slope_cosine
```

The default threshold is approximately 45 degrees. Contact normals, rather than
hit-X/Y/Z flags, remove inward velocity. Steep contacts do not manufacture an
upward displacement from horizontal input. Ground checks compare separation
velocity with the surface normal, so uphill tangent motion is not mistaken for
a jump.

Step-up is attempted only for supported movement blocked in the gravity plane:
clearance along up, movement across, then a downward sweep to a walkable surface.
The stepped result must make better planar progress than ordinary sliding.
A jump clears Grounded before movement, disabling grounded snap/step behavior.

The solver has bounded iterations. It is not a general solution for a character
crushed between mutually interpenetrating moving solids. Such gameplay needs an
explicit crush/relocation policy; no damage or teleport policy is hidden here.

## Moving-surface attachment

The support identifier is opaque to collision-api. The voxel adapter uses a
frame UUID; static root is zero.

The client stores a local foot anchor and previous support pose. Before movement,
it transforms that anchor by the latest pose, obtaining a displacement containing
both translation and rotation. It sweeps this transport against other geometry,
then settles against the current geometry. The character's up remains its own
gravity-up; the platform does not silently change gravity.

On jump or walking off a surface, the attachment releases and adds its surface
velocity once. The finite difference of the transformed anchor includes the
rotational contribution usually written omega × radius. The trajectory library
also exposes a point_velocity sampler for other consumers.

World exit, removed surfaces and position corrections invalidate attachments.
Support processing runs around the existing public PlayerControllerSet phases,
not inside the network receive system.

## Authoritative frame motion

Use SetVoxelFrameMotion to move real collision geometry:

```rust,ignore
motions.write(SetVoxelFrameMotion {
    scope,
    frame,
    target,
    initial_elapsed_seconds: 0.0,
    movement: FrameMovement::Animated {
        duration_ms: 2000,
        translation_easing: Easing::Linear,
        rotation_easing: Easing::Linear,
        repeat: RepeatMode::PingPong,
    },
});
```

Select server-voxel-frame-kinematic-mod to register the message and sampler.
VoxelFrameMotionSet::Collect and Apply run before server movement collection.
The sampler updates the shared frame registry, so bounds, queries and existing
pose persistence observe the same authoritative geometry.

Completed tracks retain their final state for late viewers. Set an Instant
motion to stop/replace a track. Trajectories themselves are transient; normal
frame persistence saves sampled poses, not an implicit animation domain.

Replication sends the trajectory and its elapsed server time when necessary,
including for new viewers, instead of restarting the same animation each tick.
The client samples collision poses at fixed updates. The renderer separately
samples the track at display time.

SendVoxelFrameMotion remains a **presentation-only** request. Its packet is
marked affects_collision=false and does not move the client's collision registry.
Use the authoritative request above for platforms intended to carry players.
A presentation-only animation may deliberately diverge from interactable geometry;
it must not be used as a physical platform.

## Network validation on supports

PlayerMove now optionally carries an opaque surface ID and frame-local foot
position. This is untrusted input, not authorization to attach anywhere.

The server observes actual support contacts and retains an anchor with scope,
epoch, local foot and last authoritative player position. Only a matching
previously observed support can rebase a subsequent movement request onto the
current platform pose. Ordinary displacement limits and collision resolution
then run on that relative movement. This prevents platform translation from
being counted entirely as voluntary movement.

This is not a global synchronized-clock or rollback-physics system. The client
starts trajectories using the elapsed time in a received snapshot; network delay
can shift display phase. Support-relative validation handles the attached case,
but high-latency first landings and discontinuous Loop/Instant relocations still
need in-game testing. Loop is a discontinuous return to the initial transform,
not a continuous path.

Recompose both client and server after these wire changes.

## Checks

Focused mechanic tests cover rotated gravity, thin obstacles, small-character
clearance, step-up, upward jump separation and non-walkable slopes. Composition
checks cover the selected client/server adapters. Interactive cases to verify:

- arbitrary gravity with scaled characters and stairs/custom model gaps;
- idle, walk, jump and walk-off on translated/rotated frames;
- narrow ceilings, corners and opposing surfaces;
- relocation/disconnect while attached;
- latency during first landing and frame interest re-entry.

Do not infer a complete gameplay validation from compilation alone.
