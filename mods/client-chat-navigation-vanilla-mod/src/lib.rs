use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};
use bevy_mod::BevyMod;
use client_chat_api::{
    ClientChatApi, ClientChatComposer, ClientChatSet, ClientChatSubmitRequested,
    ClientChatSuggestionsRequested, MAX_CHAT_HISTORY,
};
use client_game_state_api::{GameStateApi, InGameOverlayState};
use std::collections::VecDeque;
use tokio::task::JoinHandle;

#[derive(Resource, Debug, Default)]
struct ChatNavigationState {
    submitted: VecDeque<String>,
    history_cursor: Option<usize>,
    history_draft: String,
    last_history_value: Option<String>,
}

pub struct ClientChatNavigationVanillaMod;

impl ClientChatNavigationVanillaMod {
    pub fn init<C: ClientChatApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<ChatNavigationState>()
            .add_systems(Update, navigate_chat_input.in_set(ClientChatSet::Input))
            .add_systems(Update, record_submitted_input.in_set(ClientChatSet::Send));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn navigate_chat_input(
    mut keyboard: MessageReader<KeyboardInput>,
    overlay: Option<Res<State<InGameOverlayState>>>,
    mut composer: ResMut<ClientChatComposer>,
    mut navigation: ResMut<ChatNavigationState>,
    mut requests: MessageWriter<ClientChatSuggestionsRequested>,
) {
    let Some(overlay) = overlay else {
        return;
    };
    if *overlay.get() != InGameOverlayState::Chat {
        return;
    }
    for input in keyboard.read() {
        if !input.state.is_pressed() {
            continue;
        }
        let direction = match input.logical_key {
            Key::ArrowUp => -1,
            Key::ArrowDown => 1,
            _ => continue,
        };
        if navigation.history_cursor.is_some() || composer.suggestions.is_empty() {
            navigate_history(direction, &mut composer, &mut navigation);
            request_suggestions(&mut composer, &mut requests);
        } else {
            navigate_suggestions(direction, &mut composer);
        }
    }
}

fn navigate_suggestions(direction: i32, composer: &mut ClientChatComposer) {
    let len = composer.suggestions.len();
    let selected = match (composer.selected_suggestion, direction) {
        (None, value) if value < 0 => len - 1,
        (None, _) => 0,
        (Some(0), value) if value < 0 => len - 1,
        (Some(index), value) if value < 0 => index - 1,
        (Some(index), _) => (index + 1) % len,
    };
    composer.selected_suggestion = Some(selected);
    composer.input.clone_from(&composer.suggestions[selected]);
}

fn navigate_history(
    direction: i32,
    composer: &mut ClientChatComposer,
    navigation: &mut ChatNavigationState,
) {
    if navigation.submitted.is_empty() {
        return;
    }
    if navigation.history_cursor.is_some()
        && navigation.last_history_value.as_deref() != Some(composer.input.as_str())
    {
        navigation.history_cursor = None;
        navigation.last_history_value = None;
    }
    let next = if direction < 0 {
        match navigation.history_cursor {
            None => {
                navigation.history_draft.clone_from(&composer.input);
                Some(navigation.submitted.len() - 1)
            }
            Some(index) => Some(index.saturating_sub(1)),
        }
    } else {
        match navigation.history_cursor {
            Some(index) if index + 1 < navigation.submitted.len() => Some(index + 1),
            Some(_) => None,
            None => return,
        }
    };
    navigation.history_cursor = next;
    composer.input = next
        .and_then(|index| navigation.submitted.get(index).cloned())
        .unwrap_or_else(|| navigation.history_draft.clone());
    composer.selected_suggestion = None;
    navigation.last_history_value = Some(composer.input.clone());
}

fn request_suggestions(
    composer: &mut ClientChatComposer,
    writer: &mut MessageWriter<ClientChatSuggestionsRequested>,
) {
    composer.latest_request_id = composer.latest_request_id.wrapping_add(1);
    composer.suggestions.clear();
    composer.selected_suggestion = None;
    if composer.input.starts_with('/') {
        writer.write(ClientChatSuggestionsRequested {
            request_id: composer.latest_request_id,
            input: composer.input.clone(),
            cursor: composer.input.len(),
        });
    }
}

fn record_submitted_input(
    mut submitted: MessageReader<ClientChatSubmitRequested>,
    mut navigation: ResMut<ChatNavigationState>,
) {
    for submitted in submitted.read() {
        if navigation.submitted.back() != Some(&submitted.0) {
            navigation.submitted.push_back(submitted.0.clone());
            while navigation.submitted.len() > MAX_CHAT_HISTORY {
                navigation.submitted.pop_front();
            }
        }
        navigation.history_cursor = None;
        navigation.history_draft.clear();
        navigation.last_history_value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggestion_navigation_wraps_and_updates_input() {
        let mut composer = ClientChatComposer {
            input: "/".to_string(),
            suggestions: vec!["/flight".to_string(), "/flightspeed".to_string()],
            ..default()
        };
        navigate_suggestions(1, &mut composer);
        assert_eq!(composer.input, "/flight");
        navigate_suggestions(1, &mut composer);
        assert_eq!(composer.input, "/flightspeed");
        navigate_suggestions(1, &mut composer);
        assert_eq!(composer.input, "/flight");
    }

    #[test]
    fn history_restores_the_draft_after_the_newest_entry() {
        let mut composer = ClientChatComposer {
            input: "draft".to_string(),
            ..default()
        };
        let mut navigation = ChatNavigationState::default();
        navigation.submitted.push_back("first".to_string());
        navigation.submitted.push_back("second".to_string());
        navigate_history(-1, &mut composer, &mut navigation);
        assert_eq!(composer.input, "second");
        navigate_history(1, &mut composer, &mut navigation);
        assert_eq!(composer.input, "draft");
    }
}
