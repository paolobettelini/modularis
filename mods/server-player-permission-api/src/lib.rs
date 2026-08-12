use bevy::prelude::*;
use generated_permission_registry::PermissionId;
use player_network_message_types::PlayerId;
use std::collections::{HashMap, HashSet};

#[derive(Resource, Default)]
pub struct ServerPlayerPermissions {
    grants: HashMap<PlayerId, HashMap<PermissionId, HashSet<String>>>,
    denials: HashMap<PlayerId, HashMap<PermissionId, HashSet<String>>>,
}

impl ServerPlayerPermissions {
    pub fn has(&self, player_id: PlayerId, requested: PermissionId) -> bool {
        if self.denials.get(&player_id)
            .and_then(|permissions| permissions.get(&requested))
            .is_some_and(|owners| !owners.is_empty())
        {
            return false;
        }
        self.explicit(player_id)
            .any(|granted| generated_permission_registry::implies(granted, requested))
    }

    pub fn explicit(&self, player_id: PlayerId) -> impl Iterator<Item = PermissionId> + '_ {
        self.grants
            .get(&player_id)
            .into_iter()
            .flat_map(|permissions| permissions.iter())
            .filter(|(_, owners)| !owners.is_empty())
            .map(|(permission, _)| *permission)
    }

    pub fn effective(&self, player_id: PlayerId) -> Vec<PermissionId> {
        generated_permission_registry::all_permissions()
            .iter()
            .copied()
            .filter(|permission| self.has(player_id, *permission))
            .collect()
    }

    pub fn set(
        &mut self,
        player_id: PlayerId,
        owner: &str,
        permission: PermissionId,
        enabled: bool,
    ) -> bool {
        let permissions = self.grants.entry(player_id).or_default();
        let owners = permissions.entry(permission).or_default();
        let changed = if enabled {
            owners.insert(owner.to_string())
        } else {
            owners.remove(owner)
        };
        if owners.is_empty() {
            permissions.remove(&permission);
        }
        if permissions.is_empty() {
            self.grants.remove(&player_id);
        }
        changed
    }

    pub fn clear_grants(&mut self, player_id: PlayerId, permission: PermissionId) -> bool {
        let Some(permissions) = self.grants.get_mut(&player_id) else {
            return false;
        };
        let changed = permissions.remove(&permission).is_some();
        if permissions.is_empty() {
            self.grants.remove(&player_id);
        }
        changed
    }

    pub fn set_denied(
        &mut self,
        player_id: PlayerId,
        owner: &str,
        permission: PermissionId,
        denied: bool,
    ) -> bool {
        let permissions = self.denials.entry(player_id).or_default();
        let owners = permissions.entry(permission).or_default();
        let changed = if denied {
            owners.insert(owner.to_string())
        } else {
            owners.remove(owner)
        };
        if owners.is_empty() {
            permissions.remove(&permission);
        }
        if permissions.is_empty() {
            self.denials.remove(&player_id);
        }
        changed
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        self.grants.remove(&player_id);
        self.denials.remove(&player_id);
    }
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct SetPlayerPermission {
    pub player_id: PlayerId,
    pub owner: String,
    pub permission: PermissionId,
    pub enabled: bool,
}

/// An explicit, owner-scoped denial takes precedence over grants inherited
/// through the permission hierarchy. This lets a gameplay policy disable a
/// narrow capability without revoking an unrelated administrative role.
#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct SetPlayerPermissionDenied {
    pub player_id: PlayerId,
    pub owner: String,
    pub permission: PermissionId,
    pub denied: bool,
}

/// Administrative or policy-level revocation across every grant owner.
/// Normal independent feature cleanup should keep using `SetPlayerPermission`
/// with its own owner and `enabled: false`.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClearPlayerPermissionGrants {
    pub player_id: PlayerId,
    pub permission: PermissionId,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerPlayerPermissionsChanged {
    pub player_id: PlayerId,
    pub effective: Vec<PermissionId>,
    pub added: Vec<PermissionId>,
    pub removed: Vec<PermissionId>,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerPermissionSet {
    Apply,
    DeriveCapabilities,
    Sync,
}

pub trait ServerPlayerPermissionApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_narrow_denial_overrides_hierarchy_without_revoking_parent() {
        let mut permissions = ServerPlayerPermissions::default();
        permissions.set(1, "test:role", PermissionId::Privileged, true);
        assert!(permissions.has(1, PermissionId::Privileged));
        assert!(permissions.has(1, PermissionId::CanInteract));

        permissions.set_denied(1, "test:mode", PermissionId::CanInteract, true);
        assert!(permissions.has(1, PermissionId::Privileged));
        assert!(!permissions.has(1, PermissionId::CanInteract));

        permissions.set_denied(1, "test:mode", PermissionId::CanInteract, false);
        assert!(permissions.has(1, PermissionId::CanInteract));
    }
}
