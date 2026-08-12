use bevy::prelude::Resource;
use block_state_api::BlockId;
use std::{any::{Any, TypeId}, collections::HashMap};

/// A static property of a block type. Properties are composition-time/runtime
/// registry data and are never stored once per block position.
pub trait BlockProperty: Send + Sync + 'static {
    type Value: Any + Clone + Send + Sync + 'static;
    const ID: &'static str;
    fn default_value() -> Self::Value;
}

#[derive(Resource, Default)]
pub struct BlockProperties {
    values: HashMap<(TypeId, BlockId), Box<dyn Any + Send + Sync>>,
}

impl BlockProperties {
    pub fn set<P: BlockProperty>(&mut self, block: BlockId, value: P::Value) {
        assert!(P::ID.contains(':'), "block property IDs must be namespaced");
        self.values.insert((TypeId::of::<P>(), block), Box::new(value));
    }

    pub fn get<P: BlockProperty>(&self, block: BlockId) -> P::Value {
        self.values.get(&(TypeId::of::<P>(), block))
            .and_then(|value| value.downcast_ref::<P::Value>())
            .cloned()
            .unwrap_or_else(P::default_value)
    }
}
