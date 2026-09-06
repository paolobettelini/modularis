# Gravity-relative character collision plan

The current migration replaces axis-by-axis player resolution with a kinematic
capsule sweep against oriented voxel-model boxes. Capsule dimensions come from
the existing scaled hitbox; feet are the origin and capsule axis is player up.

Boundaries:
1. collision-api: character requests, contacts, surface identity and query contract.
2. character-collision-lib: distance, sweep, depenetration, slide, support and step.
3. voxel-frame-collision-lib: gather root and transformed model shapes through
   existing frame broad phase; no movement or gameplay policy.
4. client/server adapters: supply cache/routed-world data and current poses.
5. character controller: gravity-relative response and support attachment.
6. frame motion: authoritative trajectories, replication and presentation.

No rigid body engine is introduced. Gameplay/runtime checks must cover arbitrary
gravity, small scaled players, model gaps, moving/rotating supports, detachment,
teleports and phase ordering before calling the migration complete.

## Implemented and checked

The selected client/server compositions now use the gravity-relative capsule
resolver and frame geometry adapter. Support attachment and authoritative frame
trajectories are independently selected mods. Presentation-only animation does
not change collision geometry. Player movement includes an optional, untrusted
support-relative address validated against server contact history.

Client and server compilation succeeded. Six focused geometry tests passed:
rotated gravity, thin obstacles, scaled clearance, step-up, jump separation and
steep slopes. No interactive gameplay validation was performed. The remaining
runtime checks above are still necessary, especially latency during first
landing and discontinuous platform movement. See
[character collisions](../gameplay/character-collisions.md) for the public APIs
and current limitations.
