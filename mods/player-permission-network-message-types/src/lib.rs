use generated_permission_registry::PermissionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetPlayerPermissions {
    pub effective: Vec<PermissionId>,
}
