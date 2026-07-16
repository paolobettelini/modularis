# Client systems

The client is composed from independent state, input, simulation, cache, and
presentation mods.

```text
Bevy window and game state
        │
input/settings/menu contracts
        │
local player prediction
        │
network session and caches
        │
chunk/player/portal rendering
        │
optional graphics policy
```

## Neutral client infrastructure

Examples:

- Bevy default plugin installation;
- game and overlay states;
- input backend;
- generated settings registry;
- typed setting input registry;
- TCP network provider;
- chunk cache and render APIs;
- sun and sky state;
- Blocky model/animation services.

## Optional client behavior

Examples:

- pause key;
- inventory key;
- number-key and wheel hotbar selection;
- jump, sprint, and flight controls;
- crafting-table right-click handling;
- layered chunk work priority;
- face shading and ambient occlusion;
- target block outline.

Keeping controls in feature mods lets another client:

- use a gamepad;
- omit an inventory screen;
- replace first-person movement;
- change chunk work policy;
- provide a different renderer.

The following chapters cover settings/UI, graphics, and Blocky player models.
