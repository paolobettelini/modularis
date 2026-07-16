# Gameplay systems

Gameplay is implemented as optional systems around shared authoritative state.

The most important separation is:

```text
input/packet -> intention -> validation -> effect -> result -> sync
```

Gameplay mods should not skip directly from packet to mutation.

## Client prediction and server authority

The demo predicts:

- local player movement;
- gravity and jump response;
- flight movement;
- inventory and cell-menu drag/drop previews.

The server remains authoritative for:

- accepted player position;
- block edits;
- inventory contents;
- cell-menu contents;
- dimensions;
- capabilities;
- portal state.

Prediction is a presentation and latency technique. It is not a second source
of truth.

## Vanilla feature boundaries

Examples of optional vanilla behavior:

- collision validation;
- jump acceptance;
- sprint multiplier;
- grant-all flight;
- block reach;
- block placement from item metadata;
- quantity stacking and consumption;
- default inventory layout/loadout;
- crafting-table menu;
- portal ignition and travel.

A custom server can keep the underlying APIs while selecting another set.

## Actor and scope

Authoritative gameplay events usually carry:

- actor `PlayerId`;
- world scope when coordinates are involved;
- operation ID for optimistic UI;
- previous and current data for synchronization.

This gives unrelated mods enough context to validate or react.

The following chapters cover movement, block interactions, inventory, and cell
menus.
