use bevy::prelude::*;
use generated_permission_registry::PermissionId;
use std::collections::HashSet;

#[derive(Resource, Debug, Clone, Default)]
pub struct ClientPlayerPermissions {
    effective: HashSet<PermissionId>,
}

impl ClientPlayerPermissions {
    pub fn has(&self, permission: PermissionId) -> bool {
        self.effective.contains(&permission)
    }

    pub fn replace(&mut self, permissions: impl IntoIterator<Item = PermissionId>) {
        self.effective = permissions.into_iter().collect();
    }

    pub fn clear(&mut self) {
        self.effective.clear();
    }
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ClientPlayerPermissionsChanged {
    pub effective: Vec<PermissionId>,
}

pub trait ClientPlayerPermissionApi: Send + Sync + 'static {}
