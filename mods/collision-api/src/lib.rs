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
        }
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
}
