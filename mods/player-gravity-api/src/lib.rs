use bevy::prelude::*;

pub const DEFAULT_GRAVITY: Vec3 = Vec3::new(0.0, -20.0, 0.0);

#[derive(Resource, Debug, Clone, Copy)]
pub struct Gravity(pub Vec3);

impl Default for Gravity {
    fn default() -> Self {
        Self(DEFAULT_GRAVITY)
    }
}

impl Gravity {
    pub fn direction(self) -> Vec3 {
        gravity_direction(self.0)
    }

    pub fn up(self) -> Vec3 {
        gravity_up(self.0)
    }

    pub fn alignment(self) -> Quat {
        gravity_alignment(self.0)
    }
}

pub trait PlayerGravityApi: Send + Sync + 'static {}

pub fn gravity_direction(gravity: Vec3) -> Vec3 {
    gravity.normalize_or_zero().clamp_length_max(1.0)
}

pub fn gravity_up(gravity: Vec3) -> Vec3 {
    let direction = gravity_direction(gravity);
    if direction.length_squared() == 0.0 {
        Vec3::Y
    } else {
        -direction
    }
}

pub fn gravity_alignment(gravity: Vec3) -> Quat {
    let up = gravity_up(gravity);
    if up.length_squared() == 0.0 {
        Quat::IDENTITY
    } else {
        Quat::from_rotation_arc(Vec3::Y, up.normalize())
    }
}

pub fn project_on_gravity_plane(vector: Vec3, gravity: Vec3) -> Vec3 {
    let up = gravity_up(gravity);
    vector - up * vector.dot(up)
}
