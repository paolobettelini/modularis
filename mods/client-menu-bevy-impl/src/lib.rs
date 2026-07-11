use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};
use bevy_mod::BevyMod;
use client_game_state_api::{
    GameState, GameStateApi, GameStateCommand, InGameOverlayCommand, InGameOverlayState,
};
use client_keybinding_api::parse_key_code;
use client_menu_api::{
    MenuApi, MenuBackground, MenuButtonAction, MenuNumberKind, MenuRegistryHandle, MenuScreen,
    MenuTarget, MenuValueChanged, MenuWidget,
};
use tokio::task::JoinHandle;

const NORMAL_BUTTON: Color = Color::srgb(0.16, 0.18, 0.22);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.32, 0.42);
const PRESSED_BUTTON: Color = Color::srgb(0.10, 0.55, 0.34);

pub struct MenuBevyImpl {
    registry: MenuRegistryHandle,
}

impl MenuBevyImpl {
    pub fn init<G: GameStateApi>(bevy: &mut BevyMod, _game_state: &mut G) -> Self {
        let registry = MenuRegistryHandle::default();
        bevy.app
            .insert_resource(registry.clone())
            .init_resource::<FocusedMenuInput>()
            .add_message::<MenuValueChanged>()
            .add_systems(Startup, spawn_ui_camera)
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnEnter(GameState::SettingsMenu), spawn_settings_menu)
            .add_systems(OnEnter(InGameOverlayState::PauseMenu), spawn_pause_menu)
            .add_systems(
                OnEnter(InGameOverlayState::Settings),
                spawn_pause_settings_menu,
            )
            .add_systems(
                Update,
                (
                    paint_button_interactions,
                    menu_button_interactions,
                    focus_text_input_interactions,
                    focus_keybinding_input_interactions,
                    toggle_input_interactions,
                    number_adjust_interactions,
                    textbox_keyboard_input,
                    keybinding_keyboard_input,
                )
                    .chain(),
            );
        Self { registry }
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl MenuApi for MenuBevyImpl {
    fn register_screen(&mut self, screen: MenuScreen) {
        self.registry.register_screen(screen);
    }
}

#[derive(Component)]
struct MenuButton(MenuButtonAction);

#[derive(Component)]
struct MenuTextbox {
    action: String,
    value: String,
    value_text: Entity,
    kind: MenuTextInputKind,
}

#[derive(Debug, Clone, Copy)]
enum MenuTextInputKind {
    String,
    I32,
    F32,
}

#[derive(Component)]
struct MenuKeybindingInput {
    action: String,
    value: String,
    value_text: Entity,
}

#[derive(Component)]
struct MenuToggleInput {
    action: String,
    value: bool,
    value_text: Entity,
}

#[derive(Component)]
struct MenuNumberAdjust {
    action: String,
    kind: MenuNumberKind,
    delta: f64,
}

#[derive(Debug, Clone, Copy, Default)]
enum FocusedMenuInputKind {
    #[default]
    None,
    Textbox(Entity),
    Keybinding(Entity),
}

#[derive(Resource, Default)]
struct FocusedMenuInput(FocusedMenuInputKind);

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 100,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        IsDefaultUiCamera,
    ));
}

fn spawn_main_menu(mut commands: Commands, registry: Res<MenuRegistryHandle>) {
    spawn_game_menu(&mut commands, &registry, GameState::MainMenu);
}

fn spawn_settings_menu(mut commands: Commands, registry: Res<MenuRegistryHandle>) {
    spawn_game_menu(&mut commands, &registry, GameState::SettingsMenu);
}

fn spawn_pause_menu(mut commands: Commands, registry: Res<MenuRegistryHandle>) {
    spawn_overlay_menu(&mut commands, &registry, InGameOverlayState::PauseMenu);
}

fn spawn_pause_settings_menu(mut commands: Commands, registry: Res<MenuRegistryHandle>) {
    spawn_overlay_menu(&mut commands, &registry, InGameOverlayState::Settings);
}

fn spawn_game_menu(commands: &mut Commands, registry: &MenuRegistryHandle, state: GameState) {
    let target = MenuTarget::Game(state);
    let Some(screen) = registry.screen_for(target) else {
        warn!("no menu screen registered for {target:?}");
        return;
    };
    spawn_menu(commands, screen, DespawnOnExit(state));
}

fn spawn_overlay_menu(
    commands: &mut Commands,
    registry: &MenuRegistryHandle,
    state: InGameOverlayState,
) {
    let target = MenuTarget::InGameOverlay(state);
    let Some(screen) = registry.screen_for(target) else {
        warn!("no menu screen registered for {target:?}");
        return;
    };
    spawn_menu(commands, screen, DespawnOnExit(state));
}

fn spawn_menu<S: States>(commands: &mut Commands, screen: MenuScreen, despawn: DespawnOnExit<S>) {
    let background = match screen.background {
        MenuBackground::Opaque => Color::srgb(0.035, 0.045, 0.065),
        MenuBackground::Transparent => Color::NONE,
    };

    commands
        .spawn((
            despawn,
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(background),
            GlobalZIndex(100),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: px(520),
                    padding: UiRect::all(px(32)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    row_gap: px(14),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.08, 0.10, 0.14, 0.97)),
                BorderRadius::all(px(12)),
            ))
            .with_children(|column| {
                column.spawn((
                    Text::new(screen.title),
                    TextFont {
                        font_size: 42.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(px(18)),
                        ..default()
                    },
                ));

                for widget in screen.widgets {
                    match widget {
                        MenuWidget::Label { text } => {
                            column.spawn((
                                Text::new(text),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.78, 0.82, 0.90)),
                            ));
                        }
                        MenuWidget::Button { label, action, .. } => {
                            column
                                .spawn((
                                    Button,
                                    MenuButton(action),
                                    Node {
                                        width: percent(100),
                                        height: px(54),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    BorderRadius::all(px(7)),
                                ))
                                .with_child((
                                    Text::new(label),
                                    TextFont {
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                        }
                        MenuWidget::Textbox {
                            label,
                            value,
                            action,
                            ..
                        } => {
                            column.spawn((
                                Text::new(label),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.72, 0.78, 0.86)),
                            ));

                            let mut textbox_entity = None;
                            column
                                .spawn((
                                    Button,
                                    Node {
                                        width: percent(100),
                                        height: px(48),
                                        padding: UiRect::horizontal(px(14)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.05, 0.06, 0.08)),
                                    BorderRadius::all(px(6)),
                                ))
                                .with_children(|textbox| {
                                    let value_text = textbox
                                        .spawn((
                                            Text::new(value.clone()),
                                            TextFont {
                                                font_size: 21.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ))
                                        .id();
                                    textbox_entity = Some(value_text);
                                })
                                .insert(MenuTextbox {
                                    action,
                                    value,
                                    value_text: textbox_entity
                                        .expect("textbox text entity should be created"),
                                    kind: MenuTextInputKind::String,
                                });
                        }
                        MenuWidget::NumberInput {
                            label,
                            value,
                            action,
                            kind,
                            step,
                            ..
                        } => {
                            column.spawn((
                                Text::new(label),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.72, 0.78, 0.86)),
                            ));

                            column
                                .spawn(Node {
                                    width: percent(100),
                                    height: px(48),
                                    column_gap: px(8),
                                    align_items: AlignItems::Stretch,
                                    ..default()
                                })
                                .with_children(|row| {
                                    spawn_number_adjust_button(
                                        row,
                                        "−",
                                        MenuNumberAdjust {
                                            action: action.clone(),
                                            kind,
                                            delta: -step,
                                        },
                                    );

                                    let mut value_text = None;
                                    row.spawn((
                                        Button,
                                        Node {
                                            flex_grow: 1.0,
                                            padding: UiRect::horizontal(px(14)),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.05, 0.06, 0.08)),
                                        BorderRadius::all(px(6)),
                                    ))
                                    .with_children(|textbox| {
                                        value_text = Some(
                                            textbox
                                                .spawn((
                                                    Text::new(value.clone()),
                                                    TextFont {
                                                        font_size: 21.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::WHITE),
                                                ))
                                                .id(),
                                        );
                                    })
                                    .insert(MenuTextbox {
                                        action: action.clone(),
                                        value,
                                        value_text: value_text
                                            .expect("number input text entity should be created"),
                                        kind: match kind {
                                            MenuNumberKind::I32 => MenuTextInputKind::I32,
                                            MenuNumberKind::F32 => MenuTextInputKind::F32,
                                        },
                                    });

                                    spawn_number_adjust_button(
                                        row,
                                        "+",
                                        MenuNumberAdjust {
                                            action,
                                            kind,
                                            delta: step,
                                        },
                                    );
                                });
                        }
                        MenuWidget::KeybindingInput {
                            label,
                            value,
                            action,
                            ..
                        } => {
                            column.spawn((
                                Text::new(label),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.72, 0.78, 0.86)),
                            ));
                            let mut value_text = None;
                            column
                                .spawn((
                                    Button,
                                    Node {
                                        width: percent(100),
                                        height: px(48),
                                        padding: UiRect::horizontal(px(14)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.05, 0.06, 0.08)),
                                    BorderRadius::all(px(6)),
                                ))
                                .with_children(|button| {
                                    value_text = Some(
                                        button
                                            .spawn((
                                                Text::new(value.clone()),
                                                TextFont {
                                                    font_size: 21.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                            ))
                                            .id(),
                                    );
                                })
                                .insert(MenuKeybindingInput {
                                    action,
                                    value,
                                    value_text: value_text
                                        .expect("keybinding text entity should be created"),
                                });
                        }
                        MenuWidget::ToggleInput {
                            label,
                            value,
                            action,
                            ..
                        } => {
                            column.spawn((
                                Text::new(label),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.72, 0.78, 0.86)),
                            ));
                            let mut value_text = None;
                            column
                                .spawn((
                                    Button,
                                    Node {
                                        width: percent(100),
                                        height: px(48),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    BorderRadius::all(px(6)),
                                ))
                                .with_children(|button| {
                                    value_text = Some(
                                        button
                                            .spawn((
                                                Text::new(toggle_label(value)),
                                                TextFont {
                                                    font_size: 21.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                            ))
                                            .id(),
                                    );
                                })
                                .insert(MenuToggleInput {
                                    action,
                                    value,
                                    value_text: value_text
                                        .expect("toggle text entity should be created"),
                                });
                        }
                    }
                }
            });
        });
}

fn spawn_number_adjust_button(
    parent: &mut ChildSpawnerCommands,
    label: &'static str,
    adjust: MenuNumberAdjust,
) {
    parent
        .spawn((
            Button,
            adjust,
            Node {
                width: px(48),
                height: px(48),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            BorderRadius::all(px(6)),
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
}

fn paint_button_interactions(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut buttons {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
        .into();
    }
}

fn menu_button_interactions(
    buttons: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut state_commands: MessageWriter<GameStateCommand>,
    mut overlay_commands: MessageWriter<InGameOverlayCommand>,
    mut focused: ResMut<FocusedMenuInput>,
) {
    for (interaction, MenuButton(action)) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        focused.0 = FocusedMenuInputKind::None;
        match action {
            MenuButtonAction::ChangeGameState(command) => {
                state_commands.write(*command);
            }
            MenuButtonAction::ChangeInGameOverlay(command) => {
                overlay_commands.write(*command);
            }
        }
    }
}

fn focus_text_input_interactions(
    inputs: Query<(Entity, &Interaction), (Changed<Interaction>, With<MenuTextbox>)>,
    mut focused: ResMut<FocusedMenuInput>,
) {
    for (entity, interaction) in &inputs {
        if *interaction == Interaction::Pressed {
            focused.0 = FocusedMenuInputKind::Textbox(entity);
        }
    }
}

fn focus_keybinding_input_interactions(
    inputs: Query<(Entity, &Interaction, &MenuKeybindingInput), Changed<Interaction>>,
    mut texts: Query<&mut Text>,
    mut focused: ResMut<FocusedMenuInput>,
) {
    for (entity, interaction, input) in &inputs {
        if *interaction != Interaction::Pressed {
            continue;
        }
        focused.0 = FocusedMenuInputKind::Keybinding(entity);
        if let Ok(mut text) = texts.get_mut(input.value_text) {
            **text = "Press a key…".to_string();
        }
    }
}

fn toggle_input_interactions(
    mut inputs: Query<(&Interaction, &mut MenuToggleInput), Changed<Interaction>>,
    mut texts: Query<&mut Text>,
    mut changed: MessageWriter<MenuValueChanged>,
    mut focused: ResMut<FocusedMenuInput>,
) {
    for (interaction, mut input) in &mut inputs {
        if *interaction != Interaction::Pressed {
            continue;
        }
        focused.0 = FocusedMenuInputKind::None;
        input.value = !input.value;
        if let Ok(mut text) = texts.get_mut(input.value_text) {
            **text = toggle_label(input.value).to_string();
        }
        changed.write(MenuValueChanged {
            action: input.action.clone(),
            value: input.value.to_string(),
        });
    }
}

fn number_adjust_interactions(
    buttons: Query<(&Interaction, &MenuNumberAdjust), Changed<Interaction>>,
    mut inputs: Query<&mut MenuTextbox>,
    mut texts: Query<&mut Text>,
    mut changed: MessageWriter<MenuValueChanged>,
    mut focused: ResMut<FocusedMenuInput>,
) {
    for (interaction, adjust) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        focused.0 = FocusedMenuInputKind::None;
        for mut input in &mut inputs {
            if input.action != adjust.action {
                continue;
            }
            let next = match adjust.kind {
                MenuNumberKind::I32 => input
                    .value
                    .parse::<i32>()
                    .ok()
                    .map(|value| value.saturating_add(adjust.delta as i32).to_string()),
                MenuNumberKind::F32 => input
                    .value
                    .parse::<f64>()
                    .ok()
                    .map(|value| format_float(value + adjust.delta)),
            };
            let Some(next) = next else {
                continue;
            };
            input.value = next.clone();
            if let Ok(mut text) = texts.get_mut(input.value_text) {
                **text = next.clone();
            }
            changed.write(MenuValueChanged {
                action: input.action.clone(),
                value: next,
            });
            break;
        }
    }
}

fn textbox_keyboard_input(
    mut keyboard_input: MessageReader<KeyboardInput>,
    mut textboxes: Query<(Entity, &mut MenuTextbox)>,
    mut texts: Query<&mut Text>,
    mut changed: MessageWriter<MenuValueChanged>,
    mut focused: ResMut<FocusedMenuInput>,
) {
    for keyboard_input in keyboard_input.read() {
        if !keyboard_input.state.is_pressed() {
            continue;
        }

        for (entity, mut textbox) in &mut textboxes {
            if !matches!(focused.0, FocusedMenuInputKind::Textbox(focused) if focused == entity) {
                continue;
            }

            let mut value_changed = false;
            match (&keyboard_input.logical_key, &keyboard_input.text) {
                (Key::Enter, _) | (Key::Escape, _) => {
                    focused.0 = FocusedMenuInputKind::None;
                }
                (Key::Backspace, _) => {
                    value_changed = textbox.value.pop().is_some();
                }
                (_, Some(inserted)) if inserted.chars().all(is_printable_char) => {
                    let mut candidate = textbox.value.clone();
                    candidate.push_str(inserted);
                    if accepts_candidate(textbox.kind, &candidate) {
                        textbox.value = candidate;
                        value_changed = true;
                    }
                }
                _ => {}
            }

            if value_changed {
                if let Ok(mut text) = texts.get_mut(textbox.value_text) {
                    **text = textbox.value.clone();
                }
                if is_committable(textbox.kind, &textbox.value) {
                    changed.write(MenuValueChanged {
                        action: textbox.action.clone(),
                        value: textbox.value.clone(),
                    });
                }
            }
        }
    }
}

fn keybinding_keyboard_input(
    mut keyboard_input: MessageReader<KeyboardInput>,
    mut inputs: Query<&mut MenuKeybindingInput>,
    mut texts: Query<&mut Text>,
    mut changed: MessageWriter<MenuValueChanged>,
    mut focused: ResMut<FocusedMenuInput>,
) {
    let FocusedMenuInputKind::Keybinding(entity) = focused.0 else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        focused.0 = FocusedMenuInputKind::None;
        return;
    };
    for keyboard_input in keyboard_input.read() {
        if !keyboard_input.state.is_pressed() {
            continue;
        }
        if keyboard_input.key_code == KeyCode::Escape {
            if let Ok(mut text) = texts.get_mut(input.value_text) {
                **text = input.value.clone();
            }
            focused.0 = FocusedMenuInputKind::None;
            return;
        }

        let value = format!("{:?}", keyboard_input.key_code);
        if parse_key_code(&value).is_none() {
            continue;
        }
        input.value = value;
        if let Ok(mut text) = texts.get_mut(input.value_text) {
            **text = input.value.clone();
        }
        changed.write(MenuValueChanged {
            action: input.action.clone(),
            value: input.value.clone(),
        });
        focused.0 = FocusedMenuInputKind::None;
        return;
    }
}

fn accepts_candidate(kind: MenuTextInputKind, candidate: &str) -> bool {
    match kind {
        MenuTextInputKind::String => candidate.chars().all(is_printable_char),
        MenuTextInputKind::I32 => {
            candidate.is_empty() || candidate == "-" || candidate.parse::<i32>().is_ok()
        }
        MenuTextInputKind::F32 => {
            matches!(candidate, "" | "-" | "." | "-.") || candidate.parse::<f32>().is_ok()
        }
    }
}

fn is_committable(kind: MenuTextInputKind, value: &str) -> bool {
    match kind {
        MenuTextInputKind::String => true,
        MenuTextInputKind::I32 => value.parse::<i32>().is_ok(),
        MenuTextInputKind::F32 => value.parse::<f32>().is_ok(),
    }
}

fn format_float(value: f64) -> String {
    let value = format!("{value:.4}");
    value
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

fn toggle_label(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
}

fn is_printable_char(chr: char) -> bool {
    !chr.is_ascii_control()
}
