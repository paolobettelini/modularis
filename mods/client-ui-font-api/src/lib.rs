use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct ClientUiFont(pub Handle<Font>);

pub trait ClientUiFontApi: Send + Sync + 'static {}
