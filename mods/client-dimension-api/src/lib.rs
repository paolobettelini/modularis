use bevy::prelude::*;
pub use generated_dimension_registry::Dimension;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct ClientDimension(pub Dimension);

impl Default for ClientDimension {
    fn default() -> Self {
        Self(Dimension::Overworld)
    }
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct ClientDimensionChanged {
    pub previous: Dimension,
    pub current: Dimension,
    pub position: [f32; 3],
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientDimensionSet {
    Receive,
    ResetWorld,
    ApplyPlayer,
}

pub trait ClientDimensionApi: Send + Sync + 'static {}
