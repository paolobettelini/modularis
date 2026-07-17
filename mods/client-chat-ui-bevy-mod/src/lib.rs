use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};
use bevy_mod::BevyMod;
use client_chat_api::{
    ClientChatApi, ClientChatComposer, ClientChatLog, ClientChatSet, ClientChatSubmitRequested,
    ClientChatSuggestionsRequested,
};
use client_game_state_api::{GameState, GameStateApi, InGameOverlayCommand, InGameOverlayState};
use client_ui_font_api::{ClientUiFont, ClientUiFontApi};
use tokio::task::JoinHandle;

const MAX_VISIBLE_MESSAGES: usize = 10;
const MAX_INPUT_BYTES: usize = 512;

#[derive(Component)]
struct ChatLogText;

#[derive(Component)]
struct ChatInputText;

#[derive(Component)]
struct ChatSuggestionsText;

pub struct ClientChatUiBevyMod;

impl ClientChatUiBevyMod {
    pub fn init<C: ClientChatApi, G: GameStateApi, F: ClientUiFontApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _game_state: &mut G,
        _font: &mut F,
    ) -> Self {
        bevy.app
            .add_systems(OnEnter(GameState::InGame), spawn_chat_log)
            .add_systems(OnEnter(InGameOverlayState::Chat), spawn_chat_input)
            .add_systems(Update, handle_chat_keyboard.in_set(ClientChatSet::Input))
            .add_systems(
                Update,
                (render_chat_log, render_chat_composer).in_set(ClientChatSet::Render),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn spawn_chat_log(mut commands: Commands, font: Res<ClientUiFont>) {
    commands
        .spawn((
            DespawnOnExit(GameState::InGame),
            Node {
                position_type: PositionType::Absolute,
                left: px(18),
                bottom: px(64),
                width: px(620),
                padding: UiRect::all(px(10)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.015, 0.02, 0.03, 0.58)),
            BorderRadius::all(px(5)),
            GlobalZIndex(140),
        ))
        .with_child((
            ChatLogText,
            Text::new(""),
            TextFont {
                font: font.0.clone(),
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
}

fn spawn_chat_input(mut commands: Commands, font: Res<ClientUiFont>) {
    commands
        .spawn((
            DespawnOnExit(InGameOverlayState::Chat),
            Node {
                position_type: PositionType::Absolute,
                left: px(18),
                bottom: px(14),
                width: px(620),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                row_gap: px(4),
                ..default()
            },
            GlobalZIndex(160),
        ))
        .with_children(|root| {
            root.spawn((
                ChatSuggestionsText,
                Text::new(""),
                TextFont {
                    font: font.0.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.72, 0.84, 1.0)),
                Node {
                    display: Display::None,
                    width: percent(100),
                    padding: UiRect::axes(px(10), px(7)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.015, 0.02, 0.03, 0.96)),
                BorderRadius::all(px(4)),
            ));
            root.spawn((
                Node {
                    min_height: px(38),
                    padding: UiRect::axes(px(10), px(7)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.015, 0.02, 0.03, 0.92)),
                BorderRadius::all(px(4)),
            ))
            .with_child((
                ChatInputText,
                Text::new("> _"),
                TextFont {
                    font: font.0.clone(),
                    font_size: 19.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn handle_chat_keyboard(
    mut keyboard: MessageReader<KeyboardInput>,
    overlay: Option<Res<State<InGameOverlayState>>>,
    mut composer: ResMut<ClientChatComposer>,
    mut submit: MessageWriter<ClientChatSubmitRequested>,
    mut suggestion_requests: MessageWriter<ClientChatSuggestionsRequested>,
    mut overlay_commands: MessageWriter<InGameOverlayCommand>,
) {
    let chat_is_open = overlay
        .as_deref()
        .is_some_and(|overlay| *overlay.get() == InGameOverlayState::Chat);
    for input in keyboard.read() {
        if !input.state.is_pressed() || !chat_is_open {
            continue;
        }

        let mut changed = false;
        match (&input.logical_key, &input.text) {
            (Key::Enter, _) => {
                let text = composer.input.trim().to_string();
                if !text.is_empty() {
                    submit.write(ClientChatSubmitRequested(text));
                }
                composer.input.clear();
                composer.suggestions.clear();
                composer.selected_suggestion = None;
                overlay_commands.write(InGameOverlayCommand::Resume);
            }
            (Key::Escape, _) => {
                composer.input.clear();
                composer.suggestions.clear();
                composer.selected_suggestion = None;
                overlay_commands.write(InGameOverlayCommand::Resume);
            }
            (Key::Backspace, _) => {
                changed = composer.input.pop().is_some();
            }
            (Key::Tab, _) => {
                let selected = composer.selected_suggestion.unwrap_or(0);
                if let Some(suggestion) = composer.suggestions.get(selected).cloned() {
                    composer.input = suggestion;
                    changed = true;
                }
            }
            (_, Some(text)) if text.chars().all(is_printable) => {
                if composer.input.len() + text.len() <= MAX_INPUT_BYTES {
                    composer.input.push_str(text);
                    changed = true;
                }
            }
            _ => {}
        }

        if changed {
            request_suggestions(&mut composer, &mut suggestion_requests);
        }
    }
}

fn request_suggestions(
    composer: &mut ClientChatComposer,
    writer: &mut MessageWriter<ClientChatSuggestionsRequested>,
) {
    composer.latest_request_id = composer.latest_request_id.wrapping_add(1);
    composer.suggestions.clear();
    composer.selected_suggestion = None;
    if !composer.input.starts_with('/') {
        return;
    }
    writer.write(ClientChatSuggestionsRequested {
        request_id: composer.latest_request_id,
        input: composer.input.clone(),
        cursor: composer.input.len(),
    });
}

fn render_chat_log(log: Res<ClientChatLog>, mut text: Query<&mut Text, With<ChatLogText>>) {
    if !log.is_changed() {
        return;
    }
    let mut visible = log
        .entries()
        .rev()
        .take(MAX_VISIBLE_MESSAGES)
        .collect::<Vec<_>>();
    visible.reverse();
    let rendered = visible.join("\n");
    for mut text in &mut text {
        **text = rendered.clone();
    }
}

fn render_chat_composer(
    composer: Res<ClientChatComposer>,
    mut input: Query<&mut Text, (With<ChatInputText>, Without<ChatSuggestionsText>)>,
    mut suggestions: Query<
        (&mut Text, &mut Node),
        (With<ChatSuggestionsText>, Without<ChatInputText>),
    >,
) {
    if !composer.is_changed() {
        return;
    }
    for mut text in &mut input {
        **text = format!("> {}_", composer.input);
    }
    const MAX_VISIBLE_SUGGESTIONS: usize = 5;
    let first_visible = composer
        .selected_suggestion
        .map(|selected| {
            selected
                .saturating_add(1)
                .saturating_sub(MAX_VISIBLE_SUGGESTIONS)
        })
        .unwrap_or(0);
    let rendered = composer
        .suggestions
        .iter()
        .enumerate()
        .skip(first_visible)
        .take(MAX_VISIBLE_SUGGESTIONS)
        .map(|(index, suggestion)| {
            let marker = if composer.selected_suggestion == Some(index) {
                "▶"
            } else {
                " "
            };
            format!("{marker} {suggestion}")
        })
        .collect::<Vec<_>>()
        .join("\n");
    for (mut text, mut node) in &mut suggestions {
        **text = rendered.clone();
        node.display = if rendered.is_empty() {
            Display::None
        } else {
            Display::Flex
        };
    }
}

fn is_printable(character: char) -> bool {
    !character.is_control()
}
