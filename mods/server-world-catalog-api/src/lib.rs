use bevy::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{Arc, RwLock},
};
use world_instance_api::WorldInstanceId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorldId(String);

impl WorldId {
    pub fn new(id: impl Into<String>) -> Result<Self, WorldCatalogError> {
        let id = id.into();
        let valid = !id.is_empty()
            && id.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '-' | '_')
            });
        valid
            .then_some(Self(id.clone()))
            .ok_or(WorldCatalogError::InvalidWorldId(id))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldDirectory {
    pub id: WorldId,
    pub instance: WorldInstanceId,
    pub root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorldCatalogError {
    InvalidWorldId(String),
    DuplicateWorldId(WorldId),
    DuplicateInstance(WorldInstanceId),
    DuplicateRoot(PathBuf),
}

#[derive(Default)]
struct CatalogState {
    by_instance: HashMap<WorldInstanceId, WorldDirectory>,
    ids: HashSet<WorldId>,
    roots: HashSet<PathBuf>,
}

#[derive(Resource, Clone, Default)]
pub struct ServerWorldCatalog(Arc<RwLock<CatalogState>>);

impl ServerWorldCatalog {
    pub fn register(&self, world: WorldDirectory) -> Result<(), WorldCatalogError> {
        let mut state = self.0.write().expect("server world catalog lock poisoned");
        if state.ids.contains(&world.id) {
            return Err(WorldCatalogError::DuplicateWorldId(world.id));
        }
        if state.by_instance.contains_key(&world.instance) {
            return Err(WorldCatalogError::DuplicateInstance(world.instance));
        }
        if state.roots.contains(&world.root) {
            return Err(WorldCatalogError::DuplicateRoot(world.root));
        }
        state.ids.insert(world.id.clone());
        state.roots.insert(world.root.clone());
        state.by_instance.insert(world.instance.clone(), world);
        Ok(())
    }

    pub fn world(&self, instance: &WorldInstanceId) -> Option<WorldDirectory> {
        self.0
            .read()
            .expect("server world catalog lock poisoned")
            .by_instance
            .get(instance)
            .cloned()
    }

    pub fn worlds(&self) -> Vec<WorldDirectory> {
        let mut worlds = self
            .0
            .read()
            .expect("server world catalog lock poisoned")
            .by_instance
            .values()
            .cloned()
            .collect::<Vec<_>>();
        worlds.sort_by(|left, right| left.id.cmp(&right.id));
        worlds
    }
}

pub trait ServerWorldCatalogApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_distinct_instances_and_rejects_unsafe_folder_names() {
        assert!(WorldId::new("../world").is_err());
        let catalog = ServerWorldCatalog::default();
        catalog
            .register(WorldDirectory {
                id: WorldId::new("world-one").unwrap(),
                instance: WorldInstanceId::new("demo:world-one"),
                root: PathBuf::from("world-one"),
            })
            .unwrap();
        assert!(
            catalog
                .world(&WorldInstanceId::new("demo:world-one"))
                .is_some()
        );
    }
}
