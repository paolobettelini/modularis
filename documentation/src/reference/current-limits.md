# Current limits and design direction

The demo has broad extension seams, but several systems are intentionally
simple.

## World and chunks

Current limits:

- chunk generation and filesystem region I/O are synchronous;
- the filesystem backend keeps opened encoded region maps in memory;
- there is no save migration framework beyond explicit binary format versions
  and serde defaults for added metadata;
- one chunk is one `16x16x16` section;
- no palette compaction;
- no revision/version field in chunk responses;
- client cache keys omit world scope and rely on reset;
- residency is a simple player window;
- biome visuals are not synchronized or rendered;
- no generated structure registry.

Design direction:

- asynchronous storage/generation pipeline;
- region-cache eviction and background atomic writes;
- explicit save migrations and recovery tooling;
- scoped/revisioned client chunks;
- byte-budget or LRU residency;
- client biome maps and visual synchronization;
- generated structure contributors and asynchronous generation stages.

## Rendering

Current limits:

- JSON model quads are emitted without greedy merging;
- block/item model selection does not yet evaluate runtime blockstate or modern
  item-property trees;
- inventory block-item previews use a resolved texture instead of a full
  isometric model render;
- the active shape provider derives AABB unions from JSON elements, but rotated
  elements use conservative enclosing AABBs rather than oriented collision;
- no transparent block pass;
- no greedy meshing;
- no background mesh jobs;
- simple standard materials;
- no texture arrays;
- static portal visual;
- basic shadow configuration.

Design direction:

- preserve mesh/render APIs while replacing implementations;
- add blockstate and per-instance model selection as separate providers;
- add a replaceable full-model inventory preview adapter;
- add optional authored shape contributors for blocks whose physical geometry
  should differ from their visual model;
- keep graphics stages composable;
- add explicit opaque/transparent render contracts.

## Movement and authority

Current limits:

- local prediction exposes one controlled-player gravity and scale resource,
  while visibility-scoped client maps retain the gravity and visual scale of
  remote subjects;
- the selected vanilla policy scales hitboxes uniformly with model scale, but
  the collision volume is still an axis-aligned radius/height box;
- server validates displacement, collision, speed, and flight limits but does
  not simulate full velocity;
- sprint state is not explicitly validated server-side;
- acceleration and drag are not reconstructed server-side;
- correction protocol has no sequence number.

Design direction:

- optional per-world defaults layered under existing per-player force state;
- input sequence/timestamp;
- authoritative velocity and acceleration validation;
- reconciliation based on acknowledged input.

## Networking

Current limits:

- generated client/server builds must match;
- no version handshake;
- in-memory unbounded outboxes;
- no authentication or encryption;
- no bandwidth metrics;
- CBOR/framing work occurs on update thread;
- maximum frame size is global.

Design direction:

- protocol version and capability negotiation;
- bounded queues/backpressure;
- optional compression;
- connection authentication;
- task-based serialization where useful.

## Inventory and menus

Current limits:

- no maximum finite stack size;
- no durability/equipment semantics;
- the selected audience resolver maps shared audiences to all online players;
- cell-menu state is not persistent;
- no recipe system;
- no transaction revision in optimistic UI;
- one generic move/swap operation.

Design direction:

- more metadata contributors and independent UI decorators;
- team/distance/permission audience resolver implementations;
- menu persistence providers;
- generated recipes;
- operation acknowledgements/revisions.

## Dimensions and portals

Current limits:

- static definitions registered at startup;
- one instance per demo dimension;
- only vertical `4x5` portal geometry;
- active portal state is not persistent;
- frame destruction does not fully invalidate portals;
- simple return portal placement.

Design direction:

- dynamic instance lifecycle;
- generic scope-aware visibility;
- pluggable portal geometry;
- state persistence and invalidation pipeline.

## Blocky models

Current limits:

- synchronous filesystem parsing;
- boxes/quads only;
- simplified shading modes;
- approximate smooth interpolation;
- no animation blending;
- UV animation not fully applied;
- no hot reload.

Design direction:

- Bevy asset loader/provider;
- richer shape/material support;
- animation graph/blending;
- parser compatibility tests against exporter fixtures.

## Architecture risks

Watch for:

- marker APIs that expose too little runtime behavior;
- concrete feature dependencies creeping into neutral base mods;
- one resource becoming a gameplay god object;
- global resources where player/world scope is needed;
- system ordering based on private function names;
- packet contributors being centralized again;
- UI mods mutating authoritative caches without reconciliation;
- broad modpacks that silently force vanilla rules.

## How to evolve safely

When resolving a limitation:

1. identify which contract is insufficient;
2. add the smallest data or phase needed;
3. keep old policy outside neutral infrastructure;
4. migrate active providers;
5. test at least one alternate composition;
6. update this book;
7. recompose and check both applications.

The goal is not to make every possible feature abstract before it exists. The
goal is to add the right seam when a real independent variation appears.
