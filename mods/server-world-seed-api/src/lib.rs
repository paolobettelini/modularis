use bevy::prelude::*;
use world_instance_api::WorldInstanceId;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerWorldSeed {
    root: u64,
}

impl ServerWorldSeed {
    pub const fn new(root: u64) -> Self {
        Self { root }
    }

    pub const fn root(self) -> u64 {
        self.root
    }

    pub fn derive(self, namespace: &str, instance: &WorldInstanceId) -> u64 {
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
