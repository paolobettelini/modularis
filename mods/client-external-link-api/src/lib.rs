use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientExternalLinkSet {
    Receive,
    Present,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ShowClientExternalLink {
    pub title: String,
    pub description: String,
    pub url: String,
}

pub trait ClientExternalLinkApi: Send + Sync + 'static {}
