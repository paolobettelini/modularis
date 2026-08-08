# Input, settings, menus, and game state

Input collection, feature keybindings, setting storage, setting editors, and
menus are separate systems.

## Input backend

`client-input-bevy-impl` owns:

- `PlayerInput`;
- WASD movement vector;
- accumulated mouse look delta;
- left-click and right-click edge detection;
- cursor grab/hide while playing;
- cursor release when leaving the playing overlay.

It does not know:

- jump key;
- sprint key;
- sneak key;
- inventory key;
- hotbar number keys;
- pause behavior.

Those are feature mods and generated settings.

## Player input resource

The backend updates one neutral snapshot:

```text
movement
look_delta
break_block_pressed
use_item_pressed
```

Movement and camera systems consume this resource. Feature-specific keyboard
systems can read Bevy keyboard state and generated settings directly.

## Generated setting definitions

Current setting contributors:

| ID | Type | Input provider | Default |
| --- | --- | --- | --- |
| `graphics.render_distance` | `i32` | `i32` | `8` |
| `controls.mouse_sensitivity` | `f32` | `f32` | `0.15` |
| `graphics.fov` | `f32` | `f32` | `75` |
| `controls.jump_key` | string | keybinding | `Space` |
| `controls.sprint_key` | string | keybinding | `ControlLeft` |
| `controls.sneak_key` | string | keybinding | `ShiftLeft` |
| `controls.inventory_key` | string | keybinding | `E` |
| `controls.chat_key` | string | keybinding | `T` |

Storage type and input provider are independent.

## Setting store

`SettingsStore` stores selected values and emits `SettingChanged`.

Consumers use typed getters:

```rust
settings.get_f32(SettingKey::GraphicsFov)
settings.get_i32(SettingKey::GraphicsRenderDistance)
settings.get_string(SettingKey::ControlsJumpKey)
```

Numeric contributors may declare optional inclusive `min` and `max` bounds.
The generated schema carries those bounds into the store and UI. Step buttons
clamp at each endpoint, direct text input cannot commit an out-of-range value,
and programmatic writes through `SettingsStore::set` are normalized as well.

Consumers still validate semantic constraints that cannot be expressed by a
simple range. For example, invalid key strings fall back to defaults.

The server connection target is no longer a generated setting. It is owned by
the networking layer and edited directly from the main menu, so it does not
participate in `SettingsStore` validation or persistence.

Codegen rejects bounds on nonnumeric settings, inverted ranges, and defaults
outside their declared range.

### Grouped setting sections

A contributor can optionally place itself in a nested settings screen:

```toml
section = "graphics/grass"
section_label = "Grass"
```

Section IDs are slash-separated paths. Codegen stores the complete section
path, generates missing ancestors, and records the parent of each section. The
generic settings menu creates the resulting navigation tree. For example:

```text
Settings
├─ Graphics
│  ├─ FOV
│  ├─ Render distance
│  └─ Grass
│     └─ grass settings
└─ Controls
   ├─ Mouse sensitivity
   └─ Keybinds
      └─ feature keys
```

Every page uses the generic vertically scrollable menu container. A feature
does not need to modify the menu provider to gain its own inner page.

Each path segment must contain only ASCII letters, numbers, `-`, or `_`.
Section IDs must be stable. Codegen rejects invalid paths and contributors that
reuse one explicit ID with different labels. A setting without `section`
remains on the root screen.

## Setting input providers

`client-settings-input-api` stores:

```text
input provider ID -> SettingInputFactory
```

Current UI provider mods:

- string;
- `i32`;
- `f32`;
- bool;
- keybinding.

Registration happens before menu construction:

```text
SettingInputStartupSet::RegisterInputs
  -> BuildMenus
```

Numeric editors use typed editing and step controls. Bool uses a toggle.
Keybinding captures a key rather than exposing a plain text field.

This means a setting contributor does not need to know Bevy UI internals.

## Adding a setting

Create a small contributor crate:

```toml
[package.metadata.setting]
id = "audio.master_volume"
label = "Master volume"
type = "f32"
input = "f32"
default = 0.8
min = 0.0
max = 1.0
section = "audio"
section_label = "Audio"
```

Add it to the client modpack and recompose.

A feature can then read the generated key.

If the setting needs a new editor, add a separate input provider mod and use its
ID in `input`.

## Adding an input editor

An editor mod should register:

```rust
registry.register("color", build_color_widget);
```

The factory receives:

```rust
SettingInputContext {
    id,
    label,
    value,
    action,
    min,
    max,
}
```

It returns a generic `MenuWidget`.

The settings menu should not gain a hardcoded branch for every future setting
type.

## Game and overlay state

Main state:

```text
MainMenu
SettingsMenu
InGame
```

In-game substate:

```text
Playing
PauseMenu
Settings
Inventory
Chat
```

Input behavior mods emit commands:

- Escape opens pause;
- resume returns to playing;
- inventory key opens inventory;
- chat key opens the chat composer;
- settings buttons switch to settings overlays.

The state provider applies commands.

## Menu API

`client-menu-api` defines generic menu widgets and actions. Separate mods define:

- main menu;
- pause menu;
- settings screen;
- inventory UI.

UI code emits state or setting actions rather than directly mutating unrelated
systems.

The menu API also exposes `MenuWidget::TextboxButton` for a text field and a
related action button on one row. The main menu uses this widget for the server
connection target and `Play`; this is ordinary menu state, not a generated
setting.

## Server connection target

`client-network-api` owns `ClientConnectionTarget`. The selected client
transport initializes it from `ClientConfigApi::default_server_address()`, and
the main menu edits the resource directly. The default remains
`127.0.0.1:9999`.

The field accepts a socket endpoint in `host:port` form. `host` may be an IP
address or a DNS name, for example:

```text
127.0.0.1:9999
play.example.net:9999
[::1]:9999
```

Pressing `Play` enters the in-game state and the selected transport resolves
and connects to the current target. Empty addresses, DNS failures, malformed
endpoints, and connection failures return through the normal disconnected UI
instead of panicking. URL schemes such as `https://` are not part of the socket
address.

There is intentionally no Network page in Settings and no player-name textbox.
Player identity is a server-side admission concern; authenticated compositions
use the Patchwork account returned by the backend.

## Input focus

Text and keybinding inputs must have one active field at a time. When focus
changes, the old field must stop receiving characters even if its value was not
submitted with Enter.

This is an important UI invariant for modular setting editors. Shared text input
state must identify one owner/editor entity.

## Hotbar controls

Number-key selection and mouse-wheel cycling are separate mods. Both emit the
same local selection intention, which is sent to the authoritative server.

The server can also send a hotbar selection packet. The client applies it
through the inventory cache rather than faking a key press.

## Crosshair

`client-crosshair-bevy-mod` spawns one centered `+` HUD text on entering the
game. It uses absolute percentage positioning and a UI translation of
`-50%, -50%`.

It is independent from raycasting and block outlines. A client may replace its
design without changing interaction coordinates.

## Extending controls

For a new action:

1. add a setting contributor if configurable;
2. add a client feature mod that reads the key;
3. emit a domain intention or state command;
4. add network contribution only if server authority needs the intent;
5. add server validation/effect separately.

Do not add every action to `client-input-bevy-impl`.
