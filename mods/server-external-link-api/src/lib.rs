use audience_api::Audience;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerExternalLinkSet {
    Publish,
    Sync,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct PublishServerExternalLink {
    pub audience: Audience,
    pub title: String,
    pub description: String,
    pub url: String,
}

pub trait ServerExternalLinkApi: Send + Sync + 'static {}
