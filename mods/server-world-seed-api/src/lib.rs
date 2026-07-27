use bevy::prelude::*;
use std::{collections::HashMap, sync::Arc};
use world_instance_api::WorldInstanceId;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct ServerWorldSeed {
    root: u64,
    instance_seeds: Arc<HashMap<WorldInstanceId, u64>>,
}

impl ServerWorldSeed {
    pub fn new(root: u64) -> Self {
        Self {
            root,
            instance_seeds: Arc::new(HashMap::new()),
        }
    }

    pub fn with_instance_seeds(root: u64, instance_seeds: HashMap<WorldInstanceId, u64>) -> Self {
        Self {
            root,
            instance_seeds: Arc::new(instance_seeds),
        }
    }

    pub const fn root(&self) -> u64 {
        self.root
    }

    pub fn seed_for(&self, instance: &WorldInstanceId) -> u64 {
        self.instance_seeds
            .get(instance)
            .copied()
            .unwrap_or_else(|| stable_hash(self.root, instance.as_str().as_bytes()))
    }

    pub fn derive(&self, namespace: &str, instance: &WorldInstanceId) -> u64 {
        if let Some(seed) = self.instance_seeds.get(instance) {
            return stable_hash(*seed, namespace.as_bytes());
        }

        // Preserve the original derivation order for compositions that still
        // provide only one root seed. Catalog-backed worlds instead use their
        // persisted per-instance seed as the root of every namespaced stream.
        let namespaced = stable_hash(self.root, namespace.as_bytes());
        stable_hash(namespaced, instance.as_str().as_bytes())
    }
}

fn stable_hash(mut value: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        value = (value ^ u64::from(*byte)).wrapping_mul(0x1000_0000_01b3);
    }
    value
}

pub trait ServerWorldSeedApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivation_is_stable_and_scoped() {
        let seed = ServerWorldSeed::new(42);
        let overworld = WorldInstanceId::new("demo:overworld");
        let nether = WorldInstanceId::new("demo:nether");
        assert_eq!(
            seed.derive("demo:terrain", &overworld),
            seed.derive("demo:terrain", &overworld)
        );
        assert_ne!(
            seed.derive("demo:terrain", &overworld),
            seed.derive("demo:terrain", &nether)
        );
        assert_ne!(
            seed.derive("demo:terrain", &overworld),
            seed.derive("demo:climate", &overworld)
        );
    }
}
