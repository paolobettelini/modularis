use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    sync::{Arc, RwLock},
};

pub const ROOT_SCOPE_ID: &str = "patchwork:root";
pub const CHAT_SCOPE_FACET: &str = "patchwork:chat";
pub const VISIBILITY_SCOPE_FACET: &str = "patchwork:visibility";
pub const WORLD_SCOPE_FACET: &str = "patchwork:world";

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopeNodeId(pub String);

impl ScopeNodeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn root() -> Self {
        Self::new(ROOT_SCOPE_ID)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ScopeNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeFacetId(pub String);

impl ScopeFacetId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn chat() -> Self {
        Self::new(CHAT_SCOPE_FACET)
    }

    pub fn visibility() -> Self {
        Self::new(VISIBILITY_SCOPE_FACET)
    }

    pub fn world() -> Self {
        Self::new(WORLD_SCOPE_FACET)
    }
}

impl fmt::Display for ScopeFacetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Marker placed on every runtime scope entity.
///
/// Feature mods may attach arbitrary components to the same entity. The
/// hierarchy itself intentionally carries no gameplay-specific state.
#[derive(Component, Debug, Clone)]
pub struct ServerScopeNode {
    pub id: ScopeNodeId,
    pub parent: Option<ScopeNodeId>,
}

/// Attaches an ordinary ECS entity to a runtime scope.
///
/// Player sessions use the indexed membership in `ServerScopes` because they
/// are currently registry records rather than ECS entities. NPCs, dropped
/// items, machines, projectiles, or future replicated entities can use this
/// component directly and resolve whatever facet their domain needs.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct ServerScopeMembership {
    pub scope: ScopeNodeId,
}

impl ServerScopeMembership {
    pub fn new(scope: ScopeNodeId) -> Self {
        Self { scope }
    }
}

#[derive(Debug, Clone)]
pub struct ScopeNodeDescriptor {
    pub id: ScopeNodeId,
    pub parent: ScopeNodeId,
    pub facets: Vec<ScopeFacetId>,
}

impl ScopeNodeDescriptor {
    pub fn child(id: impl Into<String>, parent: ScopeNodeId) -> Self {
        Self {
            id: ScopeNodeId::new(id),
            parent,
            facets: Vec::new(),
        }
    }

    pub fn with_facet(mut self, facet: ScopeFacetId) -> Self {
        self.facets.push(facet);
        self
    }
}

#[derive(Debug, Clone)]
struct ScopeNodeRecord {
    entity: Entity,
    parent: Option<ScopeNodeId>,
    children: HashSet<ScopeNodeId>,
    facets: HashSet<ScopeFacetId>,
}

#[derive(Debug)]
struct ScopeState {
    root: ScopeNodeId,
    nodes: HashMap<ScopeNodeId, ScopeNodeRecord>,
    players: HashMap<PlayerId, ScopeNodeId>,
}

#[derive(Resource, Clone)]
pub struct ServerScopes(Arc<RwLock<ScopeState>>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeError {
    DuplicateNode(ScopeNodeId),
    MissingNode(ScopeNodeId),
    CannotRemoveRoot,
}

impl fmt::Display for ScopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNode(id) => write!(formatter, "scope node '{id}' already exists"),
            Self::MissingNode(id) => write!(formatter, "scope node '{id}' does not exist"),
            Self::CannotRemoveRoot => formatter.write_str("the root scope cannot be removed"),
        }
    }
}

impl std::error::Error for ScopeError {}

#[derive(Debug, Clone)]
pub struct RemovedScopeSubtree {
    pub nodes: Vec<ScopeNodeId>,
    pub entities: Vec<Entity>,
    pub players: Vec<PlayerId>,
}

impl ServerScopes {
    pub fn with_root(entity: Entity) -> Self {
        let root = ScopeNodeId::root();
        let mut nodes = HashMap::new();
        nodes.insert(
            root.clone(),
            ScopeNodeRecord {
                entity,
                parent: None,
                children: HashSet::new(),
                facets: HashSet::new(),
            },
        );
        Self(Arc::new(RwLock::new(ScopeState {
            root,
            nodes,
            players: HashMap::new(),
        })))
    }

    pub fn root(&self) -> ScopeNodeId {
        self.0
            .read()
            .expect("server scope tree lock poisoned")
            .root
            .clone()
    }

    pub fn contains(&self, id: &ScopeNodeId) -> bool {
        self.0
            .read()
            .expect("server scope tree lock poisoned")
            .nodes
            .contains_key(id)
    }

    pub fn entity(&self, id: &ScopeNodeId) -> Option<Entity> {
        self.0
            .read()
            .expect("server scope tree lock poisoned")
            .nodes
            .get(id)
            .map(|node| node.entity)
    }

    pub fn parent(&self, id: &ScopeNodeId) -> Option<ScopeNodeId> {
        self.0
            .read()
            .expect("server scope tree lock poisoned")
            .nodes
            .get(id)
            .and_then(|node| node.parent.clone())
    }

    pub fn children(&self, id: &ScopeNodeId) -> Vec<ScopeNodeId> {
        let state = self.0.read().expect("server scope tree lock poisoned");
        let Some(node) = state.nodes.get(id) else {
            return Vec::new();
        };
        let mut children = node.children.iter().cloned().collect::<Vec<_>>();
        children.sort();
        children
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        descriptor: ScopeNodeDescriptor,
    ) -> Result<Entity, ScopeError> {
        let mut state = self.0.write().expect("server scope tree lock poisoned");
        if state.nodes.contains_key(&descriptor.id) {
            return Err(ScopeError::DuplicateNode(descriptor.id));
        }
        if !state.nodes.contains_key(&descriptor.parent) {
            return Err(ScopeError::MissingNode(descriptor.parent));
        }

        let entity = commands
            .spawn(ServerScopeNode {
                id: descriptor.id.clone(),
                parent: Some(descriptor.parent.clone()),
            })
            .id();
        state
            .nodes
            .get_mut(&descriptor.parent)
            .expect("parent was checked above")
            .children
            .insert(descriptor.id.clone());
        state.nodes.insert(
            descriptor.id,
            ScopeNodeRecord {
                entity,
                parent: Some(descriptor.parent),
                children: HashSet::new(),
                facets: descriptor.facets.into_iter().collect(),
            },
        );
        Ok(entity)
    }

    pub fn add_facet(&self, node: &ScopeNodeId, facet: ScopeFacetId) -> Result<bool, ScopeError> {
        let mut state = self.0.write().expect("server scope tree lock poisoned");
        let record = state
            .nodes
            .get_mut(node)
            .ok_or_else(|| ScopeError::MissingNode(node.clone()))?;
        Ok(record.facets.insert(facet))
    }

    pub fn remove_facet(
        &self,
        node: &ScopeNodeId,
        facet: &ScopeFacetId,
    ) -> Result<bool, ScopeError> {
        let mut state = self.0.write().expect("server scope tree lock poisoned");
        let record = state
            .nodes
            .get_mut(node)
            .ok_or_else(|| ScopeError::MissingNode(node.clone()))?;
        Ok(record.facets.remove(facet))
    }

    pub fn resolve_facet(&self, origin: &ScopeNodeId, facet: &ScopeFacetId) -> Option<ScopeNodeId> {
        let state = self.0.read().expect("server scope tree lock poisoned");
        resolve_facet_in_state(&state, origin, facet)
    }

    pub fn resolve_player_facet(
        &self,
        player_id: PlayerId,
        facet: &ScopeFacetId,
    ) -> Option<ScopeNodeId> {
        let state = self.0.read().expect("server scope tree lock poisoned");
        let scope = state.players.get(&player_id)?;
        resolve_facet_in_state(&state, scope, facet)
    }

    pub fn assign_player(
        &self,
        player_id: PlayerId,
        target: ScopeNodeId,
    ) -> Result<Option<ScopeNodeId>, ScopeError> {
        let mut state = self.0.write().expect("server scope tree lock poisoned");
        if !state.nodes.contains_key(&target) {
            return Err(ScopeError::MissingNode(target));
        }
        Ok(state.players.insert(player_id, target))
    }

    pub fn remove_player(&self, player_id: PlayerId) -> Option<ScopeNodeId> {
        self.0
            .write()
            .expect("server scope tree lock poisoned")
            .players
            .remove(&player_id)
    }

    pub fn player_scope(&self, player_id: PlayerId) -> Option<ScopeNodeId> {
        self.0
            .read()
            .expect("server scope tree lock poisoned")
            .players
            .get(&player_id)
            .cloned()
    }

    pub fn members_exact(&self, scope: &ScopeNodeId) -> Vec<PlayerId> {
        let state = self.0.read().expect("server scope tree lock poisoned");
        let mut members = state
            .players
            .iter()
            .filter_map(|(player, player_scope)| (player_scope == scope).then_some(*player))
            .collect::<Vec<_>>();
        members.sort_unstable();
        members
    }

    pub fn members_in_subtree(&self, scope: &ScopeNodeId) -> Vec<PlayerId> {
        let state = self.0.read().expect("server scope tree lock poisoned");
        if !state.nodes.contains_key(scope) {
            return Vec::new();
        }
        let mut members = state
            .players
            .iter()
            .filter_map(|(player, player_scope)| {
                is_ancestor_in_state(&state, scope, player_scope).then_some(*player)
            })
            .collect::<Vec<_>>();
        members.sort_unstable();
        members
    }

    pub fn is_ancestor(&self, ancestor: &ScopeNodeId, descendant: &ScopeNodeId) -> bool {
        let state = self.0.read().expect("server scope tree lock poisoned");
        is_ancestor_in_state(&state, ancestor, descendant)
    }

    pub fn remove_subtree(&self, scope: &ScopeNodeId) -> Result<RemovedScopeSubtree, ScopeError> {
        let mut state = self.0.write().expect("server scope tree lock poisoned");
        if *scope == state.root {
            return Err(ScopeError::CannotRemoveRoot);
        }
        if !state.nodes.contains_key(scope) {
            return Err(ScopeError::MissingNode(scope.clone()));
        }

        let mut nodes = Vec::new();
        collect_subtree(&state, scope, &mut nodes);
        let removed_set = nodes.iter().cloned().collect::<HashSet<_>>();
        let mut players = state
            .players
            .iter()
            .filter_map(|(player, node)| removed_set.contains(node).then_some(*player))
            .collect::<Vec<_>>();
        players.sort_unstable();
        for player in &players {
            state.players.remove(player);
        }

        let parent = state
            .nodes
            .get(scope)
            .and_then(|record| record.parent.clone());
        if let Some(parent) = parent {
            if let Some(record) = state.nodes.get_mut(&parent) {
                record.children.remove(scope);
            }
        }

        let entities = nodes
            .iter()
            .filter_map(|node| state.nodes.remove(node).map(|record| record.entity))
            .collect();
        Ok(RemovedScopeSubtree {
            nodes,
            entities,
            players,
        })
    }
}

fn resolve_facet_in_state(
    state: &ScopeState,
    origin: &ScopeNodeId,
    facet: &ScopeFacetId,
) -> Option<ScopeNodeId> {
    let mut current = Some(origin.clone());
    while let Some(node_id) = current {
        let node = state.nodes.get(&node_id)?;
        if node.facets.contains(facet) {
            return Some(node_id);
        }
        current = node.parent.clone();
    }
    None
}

fn is_ancestor_in_state(
    state: &ScopeState,
    ancestor: &ScopeNodeId,
    descendant: &ScopeNodeId,
) -> bool {
    let mut current = Some(descendant.clone());
    while let Some(node_id) = current {
        if &node_id == ancestor {
            return true;
        }
        current = state
            .nodes
            .get(&node_id)
            .and_then(|node| node.parent.clone());
    }
    false
}

fn collect_subtree(state: &ScopeState, node: &ScopeNodeId, result: &mut Vec<ScopeNodeId>) {
    let Some(record) = state.nodes.get(node) else {
        return;
    };
    let mut children = record.children.iter().collect::<Vec<_>>();
    children.sort();
    for child in children {
        collect_subtree(state, child, result);
    }
    result.push(node.clone());
}

#[derive(Message, Debug, Clone)]
pub struct SetServerPlayerScope {
    pub player_id: PlayerId,
    pub target: ScopeNodeId,
}

#[derive(Message, Debug, Clone)]
pub struct ServerPlayerScopeChanged {
    pub player_id: PlayerId,
    pub previous: Option<ScopeNodeId>,
    pub current: Option<ScopeNodeId>,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerScopeSet {
    ApplyMembership,
    React,
    Cleanup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopePropagation {
    Exact,
    Descendants,
    Ancestors,
    Lineage,
}

/// A reusable envelope for domain messages that are routed through the scope
/// hierarchy. Register each concrete `ScopedMessage<T>` with Bevy as needed.
#[derive(Message, Debug, Clone)]
pub struct ScopedMessage<T: Send + Sync + 'static> {
    pub origin: ScopeNodeId,
    pub propagation: ScopePropagation,
    pub payload: T,
}

impl<T: Send + Sync + 'static> ScopedMessage<T> {
    pub fn reaches(&self, scopes: &ServerScopes, listener: &ScopeNodeId) -> bool {
        match self.propagation {
            ScopePropagation::Exact => self.origin == *listener,
            ScopePropagation::Descendants => scopes.is_ancestor(&self.origin, listener),
            ScopePropagation::Ancestors => scopes.is_ancestor(listener, &self.origin),
            ScopePropagation::Lineage => {
                scopes.is_ancestor(&self.origin, listener)
                    || scopes.is_ancestor(listener, &self.origin)
            }
        }
    }
}

pub trait ServerScopeApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_node(
        scopes: &ServerScopes,
        id: &str,
        parent: &ScopeNodeId,
        facets: impl IntoIterator<Item = ScopeFacetId>,
    ) -> ScopeNodeId {
        let id = ScopeNodeId::new(id);
        let mut state = scopes.0.write().unwrap();
        state
            .nodes
            .get_mut(parent)
            .unwrap()
            .children
            .insert(id.clone());
        state.nodes.insert(
            id.clone(),
            ScopeNodeRecord {
                entity: Entity::PLACEHOLDER,
                parent: Some(parent.clone()),
                children: HashSet::new(),
                facets: facets.into_iter().collect(),
            },
        );
        id
    }

    fn tree() -> (ServerScopes, ScopeNodeId, ScopeNodeId, ScopeNodeId) {
        let scopes = ServerScopes::with_root(Entity::PLACEHOLDER);
        let root = scopes.root();
        let lobby = add_node(&scopes, "lobby", &root, [ScopeFacetId::chat()]);
        let match_scope = add_node(&scopes, "match", &lobby, [ScopeFacetId::world()]);
        let player = add_node(
            &scopes,
            "player",
            &match_scope,
            [ScopeFacetId::visibility()],
        );
        (scopes, lobby, match_scope, player)
    }

    #[test]
    fn facets_resolve_independently_to_the_nearest_ancestor() {
        let (scopes, lobby, match_scope, player) = tree();

        assert_eq!(
            scopes.resolve_facet(&player, &ScopeFacetId::chat()),
            Some(lobby)
        );
        assert_eq!(
            scopes.resolve_facet(&player, &ScopeFacetId::world()),
            Some(match_scope)
        );
        assert_eq!(
            scopes.resolve_facet(&player, &ScopeFacetId::visibility()),
            Some(player)
        );
    }

    #[test]
    fn subtree_membership_and_removal_follow_the_hierarchy() {
        let (scopes, lobby, match_scope, player) = tree();
        scopes.assign_player(7, player.clone()).unwrap();
        scopes.assign_player(9, match_scope.clone()).unwrap();

        assert_eq!(scopes.members_exact(&player), vec![7]);
        assert_eq!(scopes.members_in_subtree(&lobby), vec![7, 9]);

        let removed = scopes.remove_subtree(&match_scope).unwrap();
        assert_eq!(removed.players, vec![7, 9]);
        assert!(!scopes.contains(&player));
        assert!(!scopes.contains(&match_scope));
    }

    #[test]
    fn scoped_message_propagation_is_explicit() {
        let (scopes, lobby, match_scope, player) = tree();
        let other = add_node(&scopes, "other", &ScopeNodeId::root(), std::iter::empty());

        let descendants = ScopedMessage {
            origin: lobby.clone(),
            propagation: ScopePropagation::Descendants,
            payload: (),
        };
        assert!(descendants.reaches(&scopes, &lobby));
        assert!(descendants.reaches(&scopes, &player));

        let ancestors = ScopedMessage {
            origin: player.clone(),
            propagation: ScopePropagation::Ancestors,
            payload: (),
        };
        assert!(ancestors.reaches(&scopes, &match_scope));
        assert!(ancestors.reaches(&scopes, &ScopeNodeId::root()));
        assert!(!ancestors.reaches(&scopes, &other));

        let exact = ScopedMessage {
            origin: player.clone(),
            propagation: ScopePropagation::Exact,
            payload: (),
        };
        assert!(exact.reaches(&scopes, &player));
        assert!(!exact.reaches(&scopes, &match_scope));
    }
}
