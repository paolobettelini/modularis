use bevy::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionResult {
    pub position: Vec3,
    pub grounded: bool,
    pub hit_x: bool,
    pub hit_y: bool,
    pub hit_z: bool,
}

pub trait CollisionApi: Send + Sync + 'static {}

#[derive(Resource, Clone)]
pub struct CollisionService {
    collides: Arc<dyn Fn(Vec3, f32, f32) -> bool + Send + Sync>,
    resolve: Arc<dyn Fn(Vec3, Vec3, f32, f32) -> CollisionResult + Send + Sync>,
    support_query: Option<Arc<dyn Fn(Vec3, Vec3, f32, f32, f32) -> bool + Send + Sync>>,
}

impl CollisionService {
    pub fn new<C, R>(collides: C, resolve: R) -> Self
    where
        C: Fn(Vec3, f32, f32) -> bool + Send + Sync + 'static,
        R: Fn(Vec3, Vec3, f32, f32) -> CollisionResult + Send + Sync + 'static,
    {
        Self {
            collides: Arc::new(collides),
            resolve: Arc::new(resolve),
            support_query: None,
        }
    }

    /// Adds a provider-specific support query.
    ///
    /// Support checks are frequent and only need to inspect the thin leading
    /// face of the player hitbox. Collision implementations can provide that
    /// optimized operation without exposing their world representation. A
    /// service without one keeps working through the generic resolver fallback.
    pub fn with_support_query<S>(mut self, support_query: S) -> Self
    where
        S: Fn(Vec3, Vec3, f32, f32, f32) -> bool + Send + Sync + 'static,
    {
        self.support_query = Some(Arc::new(support_query));
        self
    }

    pub fn collides(&self, position: Vec3, radius: f32, height: f32) -> bool {
        (self.collides)(position, radius, height)
    }

    pub fn resolve(
        &self,
        position: Vec3,
        movement: Vec3,
        radius: f32,
        height: f32,
    ) -> CollisionResult {
        (self.resolve)(position, movement, radius, height)
    }

    pub fn has_support(
        &self,
        position: Vec3,
        direction: Vec3,
        probe_distance: f32,
        radius: f32,
        height: f32,
    ) -> bool {
        let direction = direction.normalize_or_zero();
        if direction == Vec3::ZERO || probe_distance <= 0.0 {
            return false;
        }

        if let Some(support_query) = &self.support_query {
            return support_query(position, direction, probe_distance, radius, height);
        }

        let resolved = self.resolve(position, direction * probe_distance, radius, height);
        let travelled = (resolved.position - position).dot(direction);
        let tolerance = (probe_distance * 0.04).min(0.002);
        travelled < probe_distance - tolerance
    }
}
