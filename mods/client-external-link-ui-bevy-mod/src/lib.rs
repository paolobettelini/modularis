use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_external_link_api::{
    ClientExternalLinkApi, ClientExternalLinkSet, ShowClientExternalLink,
};
use client_game_state_api::{
    GameState, GameStateApi, InGameOverlayCommand, InGameOverlayState,
};
use client_ui_font_api::{ClientUiFont, ClientUiFontApi};
use std::process::Command;
use tokio::task::JoinHandle;

const NORMAL_BUTTON: Color = Color::srgb(0.16, 0.18, 0.22);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.32, 0.42);
const OPEN_BUTTON: Color = Color::srgb(0.12, 0.48, 0.31);
const OPEN_HOVERED_BUTTON: Color = Color::srgb(0.16, 0.62, 0.39);

#[derive(Component)]
struct ExternalLinkRoot {
    return_to: InGameOverlayState,
}

#[derive(Component)]
enum ExternalLinkButton {
    Close,
    Open(String),
}

pub struct ClientExternalLinkUiBevyMod;

impl ClientExternalLinkUiBevyMod {
    pub fn init<
        L: ClientExternalLinkApi,
        G: GameStateApi,
        F: ClientUiFontApi,
    >(
        bevy: &mut BevyMod,
        _links: &mut L,
        _game_state: &mut G,
        _font: &mut F,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (show_link_prompt, handle_link_buttons)
                .chain()
                .in_set(ClientExternalLinkSet::Present)
                .run_if(in_state(GameState::InGame)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn show_link_prompt(
    mut commands: Commands,
    mut requests: MessageReader<ShowClientExternalLink>,
    overlay: Option<Res<State<InGameOverlayState>>>,
    roots: Query<(Entity, &ExternalLinkRoot)>,
    font: Option<Res<ClientUiFont>>,
    mut overlay_commands: MessageWriter<InGameOverlayCommand>,
) {
    let Some(request) = requests.read().last().cloned() else { return; };
    let mut return_to = overlay
        .as_ref()
        .map(|state| *state.get())
        .unwrap_or(InGameOverlayState::Playing);
    for (entity, root) in &roots {
        return_to = root.return_to;
        commands.entity(entity).despawn();
    }

    let title = limited(request.title.trim(), 120, "External link");
    let description = limited(request.description.trim(), 1_000, "");
    let url = request.url.trim().to_owned();
    let can_open = is_safe_web_url(&url);
    let font = font.map(|font| font.0.clone()).unwrap_or_default();
    overlay_commands.write(InGameOverlayCommand::OpenModal);

    commands.spawn((
        ExternalLinkRoot { return_to },
        DespawnOnExit(GameState::InGame),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(px(24)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.015, 0.02, 0.03, 0.78)),
        GlobalZIndex(190),
    )).with_children(|root| {
        root.spawn((
            Node {
                width: px(680),
                max_width: percent(94),
                max_height: percent(90),
                padding: UiRect::all(px(30)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                row_gap: px(18),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.065, 0.075, 0.105, 0.99)),
            BorderRadius::all(px(12)),
        )).with_children(|panel| {
            panel.spawn((
                Text::new(title),
                TextFont { font: font.clone(), font_size: 34.0, ..default() },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
            ));
            if !description.is_empty() {
                panel.spawn((
                    Text::new(description),
                    TextFont { font: font.clone(), font_size: 19.0, ..default() },
                    TextColor(Color::srgb(0.78, 0.81, 0.89)),
                    TextLayout::new_with_justify(Justify::Center),
                ));
            }
            panel.spawn((
                Text::new(if can_open { url.clone() } else { format!("Unsupported link: {url}") }),
                TextFont { font: font.clone(), font_size: 17.0, ..default() },
                TextColor(if can_open { Color::srgb(0.43, 0.73, 1.0) } else { Color::srgb(1.0, 0.48, 0.44) }),
                TextLayout::new_with_justify(Justify::Center),
            ));
            panel.spawn((
                Node {
                    width: percent(100),
                    height: px(52),
                    column_gap: px(12),
                    ..default()
                },
            )).with_children(|buttons| {
                spawn_button(buttons, &font, "Close", ExternalLinkButton::Close, NORMAL_BUTTON);
                if can_open {
                    spawn_button(buttons, &font, "Open in browser", ExternalLinkButton::Open(url), OPEN_BUTTON);
                }
            });
        });
    });
}

fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    action: ExternalLinkButton,
    color: Color,
) {
    parent.spawn((
        Button,
        action,
        Node {
            flex_grow: 1.0,
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(color),
        BorderRadius::all(px(7)),
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: 21.0, ..default() },
        TextColor(Color::WHITE),
    ));
}

fn handle_link_buttons(
    mut commands: Commands,
    mut buttons: Query<
        (&Interaction, &ExternalLinkButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    roots: Query<(Entity, &ExternalLinkRoot)>,
    mut overlay_commands: MessageWriter<InGameOverlayCommand>,
) {
    for (interaction, action, mut background) in &mut buttons {
        let open = matches!(action, ExternalLinkButton::Open(_));
        match *interaction {
            Interaction::Pressed => {
                if let ExternalLinkButton::Open(url) = action {
                    if let Err(error) = open_external_url(url) {
                        warn!("could not open external link: {error}");
                    }
                }
                for (entity, root) in &roots {
                    restore_overlay(root.return_to, &mut overlay_commands);
                    commands.entity(entity).despawn();
                }
            }
            Interaction::Hovered => {
                *background = if open { OPEN_HOVERED_BUTTON } else { HOVERED_BUTTON }.into();
            }
            Interaction::None => {
                *background = if open { OPEN_BUTTON } else { NORMAL_BUTTON }.into();
            }
        }
    }
}

fn restore_overlay(
    state: InGameOverlayState,
    commands: &mut MessageWriter<InGameOverlayCommand>,
) {
    commands.write(match state {
        InGameOverlayState::Playing => InGameOverlayCommand::Resume,
        InGameOverlayState::PauseMenu => InGameOverlayCommand::Pause,
        InGameOverlayState::Settings => InGameOverlayCommand::OpenSettings,
        InGameOverlayState::Inventory => InGameOverlayCommand::OpenInventory,
        InGameOverlayState::Chat => InGameOverlayCommand::OpenChat,
        InGameOverlayState::Modal => InGameOverlayCommand::OpenModal,
    });
}

fn limited(value: &str, max_chars: usize, fallback: &str) -> String {
    let value = if value.is_empty() { fallback } else { value };
    value.chars().take(max_chars).collect()
}

fn is_safe_web_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.starts_with("https://") || lower.starts_with("http://")
}

fn open_external_url(url: &str) -> Result<(), String> {
    if !is_safe_web_url(url) {
        return Err("only HTTP and HTTPS links are allowed".to_owned());
    }
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("rundll32");
        command.arg("url.dll,FileProtocolHandler").arg(url);
        command
    };
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = Command::new("open");
        command.arg(url);
        command
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = Command::new("xdg-open");
        command.arg(url);
        command
    };
    #[cfg(not(any(target_os = "windows", target_os = "macos", unix)))]
    return Err("opening a browser is not supported on this platform".to_owned());

    command.spawn().map(|_| ()).map_err(|error| error.to_string())
}
