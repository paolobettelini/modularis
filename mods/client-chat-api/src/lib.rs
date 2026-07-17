use bevy::prelude::*;
use std::collections::VecDeque;

pub const MAX_CHAT_HISTORY: usize = 100;

#[derive(Resource, Debug, Default)]
pub struct ClientChatLog {
    entries: VecDeque<String>,
}

impl ClientChatLog {
    pub fn push(&mut self, text: String) {
        self.entries.push_back(text);
        while self.entries.len() > MAX_CHAT_HISTORY {
            self.entries.pop_front();
        }
    }

    pub fn entries(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.entries.iter().map(String::as_str)
    }
}

#[derive(Resource, Debug, Default)]
pub struct ClientChatComposer {
    pub input: String,
    pub suggestions: Vec<String>,
    pub latest_request_id: u64,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ClientChatSubmitRequested(pub String);

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ClientChatSuggestionsRequested {
    pub request_id: u64,
    pub input: String,
    pub cursor: usize,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ClientChatMessageReceived(pub String);

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ClientChatSuggestionsReceived {
    pub request_id: u64,
    pub suggestions: Vec<String>,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientChatSet {
    Receive,
    Apply,
    Input,
    Send,
    Render,
}

pub trait ClientChatApi: Send + Sync + 'static {}
