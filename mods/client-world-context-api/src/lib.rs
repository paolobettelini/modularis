use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Default, PartialEq)]
pub struct ClientWorldContext {
    pub id: Option<String>,
    pub revision: u64,
    /// Latest authoritative spawn/transition position for this world revision.
    ///
    /// Keeping it in the resource, rather than only in the transient
    /// `ClientWorldChanged` message, lets consumers wait until the local player
    /// entity exists.
    pub position: Option<[f32; 3]>,
}

impl ClientWorldContext {
    /// Applies an authoritative world-context update.
    ///
    /// Every update advances the revision so consumers can apply server
    /// relocations and clear local velocity. A cache-reset transition is
    /// returned only when the world identity actually changes.
    pub fn apply_authoritative_update(
        &mut self,
        world_id: String,
        position: [f32; 3],
    ) -> Option<ClientWorldChanged> {
        let changed_world = self.id.as_deref() != Some(world_id.as_str());
        let previous = self.id.replace(world_id.clone());
        self.revision = self.revision.wrapping_add(1);
        self.position = Some(position);

        changed_world.then(|| ClientWorldChanged {
            previous,
            current: world_id,
            revision: self.revision,
            position,
        })
    }
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct ClientWorldChanged {
    pub previous: Option<String>,
    pub current: String,
    pub revision: u64,
    pub position: [f32; 3],
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientWorldContextSet {
    Receive,
    ResetWorld,
    ApplyPlayer,
}

pub trait ClientWorldContextApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_world_relocation_updates_position_without_resetting_world() {
        let mut context = ClientWorldContext {
            id: Some("thecrown:player-1".to_string()),
            revision: 4,
            position: Some([0.5, 42.0, 0.5]),
        };

        let change =
            context.apply_authoritative_update("thecrown:player-1".to_string(), [0.5, 42.0, 0.5]);

        assert!(change.is_none());
        assert_eq!(context.revision, 5);
        assert_eq!(context.position, Some([0.5, 42.0, 0.5]));
    }

    #[test]
    fn different_world_reports_a_cache_reset_transition() {
        let mut context = ClientWorldContext {
            id: Some("world-a".to_string()),
            revision: 2,
            position: None,
        };

        let change = context
            .apply_authoritative_update("world-b".to_string(), [1.0, 2.0, 3.0])
            .expect("changing world ids must publish a transition");

        assert_eq!(change.previous.as_deref(), Some("world-a"));
        assert_eq!(change.current, "world-b");
        assert_eq!(change.revision, 3);
    }
}
